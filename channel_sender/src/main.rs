#[macro_use]
extern crate log; 
extern crate env_logger;
extern crate handlebars;
extern crate reqwest;
// #[macro_use] 
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate rand;

use std::error::Error;
use std::env;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
//use serde_json::{Value};

use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Message};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::producer::{FutureProducer, FutureRecord};
//use rdkafka::message::OwnedHeaders;

use futures::executor::block_on;

use std::fs;
use std::path::Path;

use handlebars::Handlebars;

use log::{debug, error, warn, info};
use chrono::Utc;


mod template;
mod css_interface;
mod email;
mod sms;
mod whatsapp;
mod pdf;

// Constants
static SECRET_PATH: &str = "/run/secrets/"; // Dir where to get the docker swarm secrets from

// Create a glob var to control whether or not to send msgs to the vendors. This is used to performance test the code
// without actually sending data to a vendor that would incur charges
lazy_static! {
//    static ref EXAMPLE: u8 = 42;
// static ref SEND_TO_VENDOR: bool = true;
    static ref SEND_TO_VENDOR: bool = env::var("SEND_TO_VENDOR").expect(
        "Could not find environment variable named SEND_TO_VENDOR. Without this variable being set the program will not work.")
        .parse().unwrap();
    // static ref SEND_TO_VENDOR: String = env::var("SEND_TO_VENDOR").expect(
    //     "Could not find environment variable named SEND_TO_VENDOR. Without this variable being set the program will not work.");
    
}




// Structure to hold a template id and channel to be used to send the DC through
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct TemplateChannel {
    template_id: i32,
    language_id: i32,
    channel: String,
}

// Structue of the expected payload for a digital communication event received via Kafka
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct DC {
    id: String,

    // Templates and channels to be used for the DC.
    template_channels: Vec<TemplateChannel>,

    // List of accounts to send the digital communication to
    accounts: Vec<String>,
}

// TODO figure out how to share this structure across multiple components
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
struct MessageEvent {
    account_id: String,
    id: String,
    channel: String,
    status: String,
    datetime_rfc2822: String,
    event_specific_data: String,
}

// Message Event Status. DO not use enum as the value could change from one end of the sender-receiver pair. Uses strings instead.
const SENT: &str = "sent";
const RECEIVED: &str = "received";
const QUEUED: &str = "queued";
const FAILED_RESEND: &str = "failed_resend";
const FAILED_NO_RESEND: &str = "failed_no_resend";
const OPENED: &str = "opened";
const DELIVERED: &str = "delivered";




// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;




// ****************************** NEW CODE ****************************************************
fn create_consumer() -> LoggingConsumer {
    let context = CustomContext;

    // Initialize variables from environment
    let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS").expect("Could not find environment variable named KAFKA_BOOTSTRAP_SERVERS. Without this variable being set the program will not work.");

    let consumer_topic = env::var("KAFKA_CONSUMER_TOPIC").expect("Could not find environment variable named KAFKA_CONSUMER_TOPIC. Without this variable being set the program will not work.");
    let consumer_topics = [&*consumer_topic];

    let group_id = env::var("KAFKA_GROUP_ID").expect("Could not find environment variable named KAFKA_GROUP_ID. Without this variable being set the program will not work.");
    info!("Environment variables KAFKA_BOOTSTRAP_SERVERS={}, KAFKA_CONSUMER_TOPIC={}, KAFKA_GROUP_ID={}", bootstrap_servers, consumer_topic, group_id);

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", &*group_id)
        .set("bootstrap.servers", &*bootstrap_servers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("auto.commit.interval.ms", "5000")
        .set("enable.auto.offset.store", "false")
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&consumer_topics.to_vec())
        .expect("Can't subscribe to specified topics");

    consumer
}


fn create_producer() -> FutureProducer {

    // Initialize variables from environment
    let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS").expect("Could not find environment variable named KAFKA_BOOTSTRAP_SERVERS. Without this variable being set the program will not work.");

    ClientConfig::new()
        .set("bootstrap.servers", &*bootstrap_servers)
        .set("queue.buffering.max.ms", "0") // Do not buffer
        .create()
        .expect("Producer creation failed")
}


/// Send an event to the kafka topic for further processing, this records the successful transmission of the digital comms msg 
async fn send_event(producer: &FutureProducer, topic: &String, msg: MessageEvent
) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut record = FutureRecord::to(topic);
    record = record.key(&msg.account_id);
    let payload = serde_json::to_string(&msg).unwrap();
    record = record.payload(&payload);

    // Send the message and block forever if the queue is full
    // TODO determine what is the best strategy here, if block forever is not it
    let result = producer.send(record, -1i64).await;

    match result {
        Ok(_s) => info!("Sent event to kafka topic"),
        Err(e) => error!("Error sending event to kafka topic. Reason: {}", e),
    }

    Ok(())
}



#[tokio::main]
async fn main() {
     env_logger::init();

    let producer_topic = env::var("KAFKA_PRODUCER_TOPIC").expect("Could not find environment variable named KAFKA_PRODUCER_TOPIC. Without this variable being set the program will not work.");
    info!("Environment variable KAFKA_PRODUCER_TOPIC={}", producer_topic);

    let consumer = create_consumer();
    let producer = create_producer();

    // Get the secrets for the vendor credentials
    let sms_vendor_account_id = get_secret("sms_vendor_account_id");
    let sms_vendor_token = get_secret("sms_vendor_token");
    let email_vendor_token = get_secret("email_vendor_token");

    // Get the PDF Server URL
    let pdf_service_url = env::var("PDF_SERVICE_URL").expect("Could not find environment variable named PDF_SERVICE_URL. Without this variable being set the program will not work.");
    
    // Create the handlebars instance used to merge the templates and data
    let mut hb = handlebars::Handlebars::new();

    // Create the client used to make the rest calls to the comms vendors
    let client = reqwest::Client::new();

    let mut stream = consumer.start();

    while let Some(message) = stream.next().await {
        match message {
            Err(e) => {
                warn!("Kafka error: {}", e);
            }
            Ok(m) => {
                match m.payload_view::<str>() {
                    None => {
                        warn!("No payload in received message from kafka topic. Ignoring message with contents: {:?}", m);
                        ""
                    },
                    Some(Ok(s)) => {
                        // Process the message here
                        // Get the JSON object from the payload
                        let msg: DC = serde_json::from_str(s).unwrap();
                        info!("Payload contains DC: {:?}", msg);

                        // Process the digital communications request
                        process_request(msg, &mut hb, &sms_vendor_account_id, &sms_vendor_token, &email_vendor_token, 
                            &pdf_service_url, &client, &producer, &producer_topic);
                        s
                    },
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    },
                };
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        }
    }
}





































































// //************************************************************************
// async fn consume() {
//     let context = CustomContext;

//     // Initialize variables from environment
//     let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS").expect("Could not find environment variable named KAFKA_BOOTSTRAP_SERVERS. Without this variable being set the program will not work.");

//     let consumer_topic = env::var("KAFKA_CONSUMER_TOPIC").expect("Could not find environment variable named KAFKA_CONSUMER_TOPIC. Without this variable being set the program will not work.");
//     let consumer_topics = [&*consumer_topic];

//     let producer_topic = env::var("KAFKA_PRODUCER_TOPIC").expect("Could not find environment variable named KAFKA_PRODUCER_TOPIC. Without this variable being set the program will not work.");
//     let producer_topics = [&*producer_topic];

//     let group_id = env::var("KAFKA_GROUP_ID").expect("Could not find environment variable named KAFKA_GROUP_ID. Without this variable being set the program will not work.");
//     info!("Environment variables KAFKA_BOOTSTRAP_SERVERS={}, KAFKA_CONSUMER_TOPIC={}, , KAFKA_PRODUCER_TOPIC={}, KAFKA_GROUP_ID={}", bootstrap_servers, consumer_topic, producer_topic, group_id);

//     // Get the secrets for the vendor credentials
//     let sms_vendor_account_id = get_secret("sms_vendor_account_id");
//     let sms_vendor_token = get_secret("sms_vendor_token");
//     let email_vendor_token = get_secret("email_vendor_token");

//     // Get the PDF Server URL
//     let pdf_service_url = env::var("PDF_SERVICE_URL").expect("Could not find environment variable named PDF_SERVICE_URL. Without this variable being set the program will not work.");


//     let consumer: LoggingConsumer = ClientConfig::new()
//         .set("group.id", &*group_id)
//         .set("bootstrap.servers", &*bootstrap_servers)
//         .set("enable.partition.eof", "false")
//         .set("session.timeout.ms", "6000")
//         .set("enable.auto.commit", "true")
//         .create_with_context(context)
//         .expect("Consumer creation failed");

//     consumer
//         .subscribe(&consumer_topics.to_vec())
//         .expect("Can't subscribe to specified topics");

//     info!("Starting kafka consumer");

//     // Create the handlebars instance used to merge the templates and data
//     let mut hb = handlebars::Handlebars::new();

//     // consumer.start() returns a stream. The stream can be used ot chain together expensive steps,
//     // such as complex computations on a thread pool or asynchronous IO.
//     let mut message_stream = consumer.start();

//     // Create the client used to send rest api calls
//     let client = reqwest::Client::new();

//     while let Some(message) = message_stream.next().await {
//         match message {
//             Err(e) => warn!("Kafka error: {}", e),
//             Ok(m) => {
//                 match m.payload_view::<str>() {
//                     None => {
//                         warn!("No payload in received message from kafka topic. Ignoring message with contents: {:?}", m);
//                         ""
//                     },
//                     Some(Ok(s)) => {
//                         // Get the JSON object from the payload
//                         let msg: DC = serde_json::from_str(s).unwrap();
//                         info!("Payload contains DC: {:?}", msg);

//                         // Process the digital communications request
//                         process_request(msg, &mut hb, &sms_vendor_account_id, &sms_vendor_token, &email_vendor_token, &pdf_service_url, &client);
//                         s
//                     },
//                     Some(Err(e)) => {
//                         warn!("Error while deserializing message payload: {:?}", e);
//                         ""
//                     },
//                 };
//                 consumer.commit_message(&m, CommitMode::Async).unwrap();
//             }
//         };
//     }
// }


//************************************************************************
//#[tokio::main]
// async fn main() {
//     env_logger::init();

//     consume().await
// }


//************************************************************************
fn get_secret(name: &str) -> String {
    let file_name = format!("{}{}", SECRET_PATH, name);
    let secret = match fs::read_to_string(&file_name) {
        Ok(s) => s,
        Err(e) => {
            error!("Error occurred trying to find a secret named:{}. The error was:{}", &file_name, e);
            panic!("secret not found, cannot continue")
        },
    };
    secret
}



// fn return_result(error_str:String)
// -> Result<String, Box<dyn std::error::Error>> {

//     return Err(Box::new(error_str));
//     Ok("It should not get this far".to_string())
// }



//************************************************************************
fn process_request (msg: DC, mut hb: &mut Handlebars, sms_vendor_account_id: &String, 
    sms_vendor_token: &String, email_vendor_token: &String, pdf_service_url: &String,
    client: &reqwest::Client, producer: &FutureProducer, producer_topic: &String
)  -> Result<(), Box<dyn std::error::Error>> {
    
    debug!("In process_request");

    // For each account and channel template combination, send the communication
    for template_channel in &msg.template_channels {

        // Register the template
        let template_file_name = format!("./templates/temp_{}_lang_{}_{}.html", template_channel.template_id, template_channel.language_id, template_channel.channel);
        let path = Path::new(&template_file_name);
        hb.register_template_file(&template_file_name, path).expect("Template registration error");

        // Get the fields used in the template so that the data can be requested
        let template = hb.get_template(&template_file_name).unwrap();
        let mut template_fields = Vec::<String>::new();
        template::get_template_fields(&template.elements, &mut template_fields);

        for account in &msg.accounts {
            
            debug!("Process DC for account: {:?}", account);

            // Get the fields for the account
            let account_fields = css_interface::get_account_fields(&account, &template_fields);

            // Combine the template with the fields retrieved for the account
            let populated_template = hb.render(&template_file_name, &account_fields).expect("render error");

            // Send the digital communication for the account through the correct channel
            debug!("Calling channel processing for: {:?}", &*template_channel.channel);

            // TODO. Determine how to get the event specific data back from the sending methods
            let result = match &*template_channel.channel {
                "email" =>      block_on(email::send_email(&account_fields, populated_template, &email_vendor_token, client)),
                "sms" =>        block_on(sms::send_sms(&account_fields, populated_template, &sms_vendor_account_id, &sms_vendor_token, client)),
                "pdf" =>        block_on(pdf::send_pdf(&account_fields, populated_template, &pdf_service_url, client)),
                "whatsapp" =>   block_on(whatsapp::send_whatsapp(&account_fields, populated_template, &sms_vendor_account_id, &sms_vendor_token, client)),
                ch =>           Ok("ERROR UNKNOWN CHANNEL".to_string()),
//                ch =>           Err(Box::new(Error("Unknown channel"))),
            };

            if result.is_ok() {
                let sent_msg_id = result.unwrap();
                
                // Generate and send an event that details the successful sending of a DC msg
                let msg_event = MessageEvent {account_id: account.to_string(), id: sent_msg_id, channel: template_channel.channel.to_string(), status: SENT.to_string(),
                    datetime_rfc2822: Utc::now().to_rfc2822(),
                    event_specific_data: "".to_string(),
                };
//                let result = block_on(send_event(&producer, &producer_topic, msg_event).await.expect("Failed to send event"));
                let result = block_on(send_event(&producer, &producer_topic, msg_event));

            } else {
                error!("Failed to send event to the vendors service, account_id: {}, channel: {}, reason: {}", account, 
                    &*template_channel.channel, result.unwrap_err());
            }

            // match result {
            //     Ok(()) => info!("Sent {} for account_id: {} using template named: {}", 
            //         &*template_channel.channel, &account_fields["account_id"], template_file_name),
            //     Err(e) => error!("Failed to send {} for account_id: {} using template named: {}. Error is: {}", 
            //         &*template_channel.channel, &account_fields["account_id"], template_file_name, e)
            // }

            // TODO Send a status message to the event store registering that a DC has occured for that account
        }

        // Unregister the template
        hb.unregister_template(&template_file_name);
    }

    Ok(())
}
