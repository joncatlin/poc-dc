#[macro_use]
extern crate log; 
extern crate env_logger;
extern crate handlebars;
extern crate reqwest;
// #[macro_use] 
extern crate hyper;

use std::env;
use futures::StreamExt;
use env_logger::Builder;
use log::LevelFilter;
use chrono::{Local};
use serde::{Deserialize, Serialize};
use serde_json::{Value};

use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Message};
use rdkafka::topic_partition_list::TopicPartitionList;

use std::io::Write;
use std::collections::HashMap;
use uuid::Uuid;
use reqwest::Client;
use futures::executor::block_on;

//use hyper::header::{Headers, Authorization, Bearer};

//use std::io;
//use std::fs::File;
use std::fs;
use std::path::Path;

use handlebars::Handlebars;
//use handlebars::template:: {TemplateElement, HelperTemplate, Parameter, DecoratorTemplate};
//use handlebars::Path:: {Relative, Local as HBLocal};

mod template;
mod css_interface;

// Constants
static SECRET_PATH: &str = "/run/secrets/";



// Structure to hold a template id and channel to be used to send the DC through
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct TemplateChannel {
    template_id: i32,
    language_id: i32,
    channel: String,
}

// Structuer of the expected payload for a digital communication event received via Kafka
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
#[derive(Deserialize)]
struct MessageEvent {
    account_id: String,
    id: String,
    channel: String,
    status: String,
    datetime_rfc2822: String,
    event_specific_data: String,
}




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


//************************************************************************
async fn consume() {
    let context = CustomContext;

    // Get the bootstrap servers and topic from the environment variables
    let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS").expect("Could not find environment variable named KAFKA_BOOTSTRAP_SERVERS. Without this variable being set the program will not work.");
    let topic = env::var("KAFKA_TOPIC").expect("Could not find environment variable named KAFKA_TOPIC. Without this variable being set the program will not work.");
    let topics = [&*topic];
    let group_id = env::var("KAFKA_GROUP_ID").expect("Could not find environment variable named KAFKA_GROUP_ID. Without this variable being set the program will not work.");
    info!("Environment variables KAFKA_BOOTSTRAP_SERVERS={}, KAFKA_TOPIC={}, KAFKA_GROUP_ID={}", bootstrap_servers, topic, group_id);

    // Get the secrets for the vendor credentials
    let sms_vendor_account_id = get_secret("sms_vendor_account_id");
    let sms_vendor_token = get_secret("sms_vendor_token");
    let email_vendor_token = get_secret("email_vendor_token");


    // Rubbish
    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", &*group_id)
        .set("bootstrap.servers", &*bootstrap_servers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    info!("Starting kafka consumer");

    // Create the handlebars instance used to merge the templates and data
    let mut hb = handlebars::Handlebars::new();

    // consumer.start() returns a stream. The stream can be used ot chain together expensive steps,
    // such as complex computations on a thread pool or asynchronous IO.
    let mut message_stream = consumer.start();

    while let Some(message) = message_stream.next().await {
        match message {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                match m.payload_view::<str>() {
                    None => {
                        warn!("No payload in received message from kafka topic. Ignoring message with contents: {:?}", m);
                        ""
                    },
                    Some(Ok(s)) => {
                        // Get the JSON object from the payload
                        let msg: DC = serde_json::from_str(s).unwrap();
                        info!("Payload contains DC: {:?}", msg);

                        // Process the digital communications request
                        process_request(msg, &mut hb, &sms_vendor_account_id, &sms_vendor_token, &email_vendor_token);
                        s
                    },
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    },
                };
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}


//************************************************************************
#[tokio::main]
async fn main() {
    env_logger::init();
    // // Initialize the logger for stdout
    // Builder::new()
    // .format(|buf, record| {
    //     writeln!(buf,
    //         "{} [{}] - {}",
    //         Local::now().format("%Y-%m-%dT%H:%M:%S"),
    //         record.level(),
    //         record.args()
    //     )
    // })
    // .filter(None, LevelFilter::Info)
    // .init();


    // let (version_n, version_s) = get_rdkafka_version();
    // info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    // let topics = matches.values_of("topics").unwrap().collect::<Vec<&str>>();
    // let brokers = matches.value_of("brokers").unwrap();
    // let group_id = matches.value_of("group-id").unwrap();

    consume().await
}


//************************************************************************
async fn send_whatsapp(account_fields: &Value, whatsapp_content: String, vendor_acc_id: &String, vendor_token: &String) -> Result<(), Box<dyn std::error::Error>> {
//    fn send_whatsapp() -> Result<(), Box<dyn std::error::Error>> {
    
    // Send a Whatsapp msg
    let mut params = HashMap::new();
    let whatsapp_to = format!("whatsapp:{}", account_fields["phone_mobile"]);
    params.insert("To", whatsapp_to);
    params.insert("From", "whatsapp:+14155238886".to_string());
    params.insert("Body", whatsapp_content);
    params.insert("StatusCallback", "http://destini.synology.me:50012/csc/webhooks/whatsapp".to_string());

    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", vendor_acc_id);

    let client = Client::new();
    let res = client
        .post(&url)
        .basic_auth(vendor_acc_id, Some(vendor_token))
        .form(&params)
        .send()
        .await
        .unwrap();

    let response_text = res.text().await.unwrap();
    debug!("Response from send_sms reqwest: {}", response_text);

    // TODO if the response status is not 200 then an error needs to be generated
    // TODO need to deal with the errors that could come back from reqwest

    Ok(())
}


//************************************************************************
async fn send_sms(account_fields: &Value, sms_content: String, vendor_acc_id: &String, vendor_token: &String) -> Result<(), Box<dyn std::error::Error>> {

    debug!("Starting to send_sms. account_fields: {:?}, sms_content: {:?}", account_fields, sms_content);


    
    // Send an SMS
    let mut params = HashMap::new();
    params.insert("To", account_fields["phone_mobile"].to_string());
//    params.insert("From", "+14804053433".to_string());
    params.insert("From", "+15005550006".to_string());
    params.insert("Body", sms_content);
    params.insert("StatusCallback", "http://destini.synology.me:50012/csc/webhooks/sms".to_string());
    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", vendor_acc_id);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .basic_auth(vendor_acc_id, Some(vendor_token))
        .form(&params)
        .send()
        .await
        .unwrap();

    let response_text = res.text().await.unwrap();
    debug!("Response from send_sms reqwest: {}", response_text);

    // TODO if the response status is not 200 then an error needs to be generated
    // TODO need to deal with the errors that could come back from reqwest

    Ok(())
}


//************************************************************************
async fn send_email(account_fields: &Value, email_content: String, api_key: &String) -> Result<(), Box<dyn std::error::Error>> {

    debug!("Starting to send_email. account_fields: {:?}, email_content: {:?}, api_key: {:?}", account_fields, email_content, api_key);

    // Get the fields for the email
    let email_to = &account_fields["email"];
    let email_from = &account_fields["email_from"];
    let email_subject = format!("A message from - {}", account_fields["client_name"]);

    // Create the body of the request
    let filled_email_struct = format!(
        r#"{{"personalizations": [{{"to": [{{"email": "{}"}}],"subject": "{}"}}],"from": {{"email": "{}"}},"content": [{{"type": "text/html","value": "{}"}}]}}"#, 
        email_to, email_subject, email_from, email_content
    );
    debug!("filled_email_struct contains: {}", filled_email_struct);

    let url = "https://api.sendgrid.com/v3/mail/send";

    // TODO look into passing this in to increase performance, instead of building it each time
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .body(filled_email_struct)
        .send()
        .await
        .unwrap();


    // TODO if the response status is not 200 then an error needs to be generated
    // TODO need to deal with the errors that could come back from reqwest
    if !res.status().is_success() {
        error!("Response from send_email reqwest was failure. Status: {}, Text: {}", res.status(), res.text().await.unwrap());
    }

    Ok(())
}


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



//************************************************************************
fn send_pdf() -> Result<(), Box<dyn std::error::Error>> {
    
    // Build request to send to the makepdf service
    info!("Started conversion of pdf");
    let url = format!("http://docker01:8083/convert/html");
    
    let form = reqwest::blocking::multipart::Form::new()
        .text("x", "not used")
        .file("files", "./index.html").unwrap();

    let client = reqwest::blocking::Client::new();

    let resp = client
        .post(&url)
        .multipart(form)
        .send();
    info!("Conversion complete, started file creation");

    match resp {
        Ok(mut r) => {
            println!("success!");
//            let mytext = r.text()?;


            let filename = format!("./output/pdf-{}.pdf", Uuid::new_v4());
            let path = Path::new(&filename);
        
            let mut file = std::fs::File::create(&path)?;
            r.copy_to(&mut file)?;
        
            // match save_in_file(mytext) {
            //     Ok(_) => Ok(()),
            //     Err(e) => {println!("Error {}", e); Ok(())},
            // }
            Ok(())
        },
        Err(e) => Err(e),
    };
    info!("File creation complete");

    Ok(())
}


//************************************************************************
fn process_request (msg: DC, mut hb: &mut Handlebars, sms_vendor_account_id: &String, sms_vendor_token: &String, email_vendor_token: &String) {
    
    // For each account and channel template combination, send the communication
    for template_channel in &msg.template_channels {

        // Register the template
        let template_file_name = format!("/templates/temp_{}_lang_{}_{}.html", template_channel.template_id, template_channel.language_id, template_channel.channel);
        let path = Path::new(&template_file_name);
        hb.register_template_file(&template_file_name, path).expect("Template registration error");

        // Get the fields used in the template so that the data can be requested
        let template = hb.get_template(&template_file_name).unwrap();
        let mut template_fields = Vec::<String>::new();
        template::get_template_fields(&template.elements, &mut template_fields);

        for account in &msg.accounts {
            
            // Get the fields for the account
            let account_fields = css_interface::get_account_fields(&account, &template_fields);

            // Combine the template with the fields retrieved for the account
            let populated_template = hb.render(&template_file_name, &account_fields).expect("render error");

            // Send the digital communication for the account through the correct channel
            let result = match &*template_channel.channel {
                "email" => {
                    block_on(send_email(&account_fields, populated_template, &email_vendor_token));
                },
                "sms" => {
                    block_on(send_sms(&account_fields, populated_template, &sms_vendor_account_id, &sms_vendor_token));
                },
                "pdf" => {
                    send_pdf();
                },
                "whatsapp" => {
                    block_on(send_whatsapp(&account_fields, populated_template, &sms_vendor_account_id, &sms_vendor_token));
                },
                ch => error!("Unknown channel specified in received msg. Channel found is: {}", ch),
            };

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
}
