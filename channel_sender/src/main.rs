#[macro_use]
extern crate log; 
extern crate env_logger;
extern crate handlebars;
extern crate reqwest;
#[macro_use] 
extern crate hyper;

use std::env;
use futures::StreamExt;
use env_logger::Builder;
use log::LevelFilter;
use chrono::{Local};
use serde::{Deserialize, Serialize};

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
//use hyper::header::{Headers, Authorization, Bearer};

//use std::io;
//use std::fs::File;
use std::fs;
use std::path::Path;

mod template;

// Constants
static ACCOUNT_ID: &str = "account_id";
static TOKEN: &str = "token";
static SECRET_PATH: &str = "/run/secrets/";



// Structure to hold a template id and channel to be used to send the DC through
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct TemplateChannel {
    template_id: i32,
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

    // consumer.start() returns a stream. The stream can be used ot chain together expensive steps,
    // such as complex computations on a thread pool or asynchronous IO.
    let mut message_stream = consumer.start();

    while let Some(message) = message_stream.next().await {
        match message {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => {
                        warn!("No payload in received message from kafka topic. Ignoring message with contents: {:?}", m);
                        ""
                    },
                    Some(Ok(s)) => {
                        // Get the JSON object from the payload
                        let msg: DC = serde_json::from_str(s).unwrap();
                        info!("Payload contains DC: {:?}", msg);

                        // Process the digital communications request
                        process_request(msg);
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

    // Initialize the logger for stdout
    Builder::new()
    .format(|buf, record| {
        writeln!(buf,
            "{} [{}] - {}",
            Local::now().format("%Y-%m-%dT%H:%M:%S"),
            record.level(),
            record.args()
        )
    })
    .filter(None, LevelFilter::Info)
    .init();


    // let (version_n, version_s) = get_rdkafka_version();
    // info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    // let topics = matches.values_of("topics").unwrap().collect::<Vec<&str>>();
    // let brokers = matches.value_of("brokers").unwrap();
    // let group_id = matches.value_of("group-id").unwrap();

    consume().await
}


//************************************************************************
fn send_whatsapp() -> Result<(), Box<dyn std::error::Error>> {
    
    let account_id = "ACbd9666b3f5427bb33828653997cb357a".to_string();
    let token = "8cd2cb93c2e4bdcef546690b0868e995".to_string();

    // Send a Whatsapp msg
    let mut params = HashMap::new();
    params.insert("To", "whatsapp:+14805169974");
    params.insert("From", "whatsapp:+14155238886");
    params.insert("Body", "Hello, World! from Rust");
    params.insert("StatusCallback", "http://destini.synology.me:50012/csc/webhooks/whatsapp");
    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account_id);
    let client = Client::new();
    async {

        let res = client
            .post(&url)
            .basic_auth(account_id, Some(token))
            .form(&params)
            .send()
            .await
            .unwrap();

        let mytext = res.text().await.unwrap();
        println!("{}", mytext);
    };

    Ok(())
}


//************************************************************************
fn send_sms() -> Result<(), Box<dyn std::error::Error>> {
    let account_id = "ACbd9666b3f5427bb33828653997cb357a".to_string();
    let token = "8cd2cb93c2e4bdcef546690b0868e995".to_string();

    // Send an SMS
    let mut params = HashMap::new();
    params.insert("To", "+14805169974");
    params.insert("From", "+14804053433");
    params.insert("Body", "Hello, World! from Rust");
    params.insert("StatusCallback", "http://destini.synology.me:50012/csc/webhooks/sms");
    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account_id);
    let client = reqwest::Client::new();
    async {

        let res = client
            .post(&url)
            .basic_auth(account_id, Some(token))
            .form(&params)
            .send()
            .await
            .unwrap();

        let mytext = res.text().await.unwrap();
        println!("{}", mytext);
    };

    Ok(())
}


//************************************************************************
fn send_email(account_fields: &HashMap<String, String>, email_content: &String, api_key: &String) -> Result<(), Box<dyn std::error::Error>> {

    info!("Starting to send_email. account_fields: {:?}, email_content: {:?}, api_key: {:?}", account_fields, email_content, api_key);

    // var apiKey = "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";
    // //            var apiKey = Environment.GetEnvironmentVariable("SENDGRID_APIKEY");
    //             var client = new SendGridClient(apiKey);
    //             var from = new EmailAddress("jonc@destini.com", "Jon Catlin");
    //             var subject = "Testing sending with SendGrid";
    //             var to = new EmailAddress("jon.catlin@destini.com", "Jon T Catlin");
    //             var plainTextContent = "This is an easy api to use";
    //             var htmlContent = "<strong>Just figuring out how to use this api</strong>";
    //             var msg = MailHelper.CreateSingleEmail(from, to, subject, plainTextContent, htmlContent);
    //             var response = await client.SendEmailAsync(msg);

    // Get the fields from the account needed for an email
    let email_to = account_fields.get(&"email_to".to_string()).unwrap();
    let email_from = account_fields.get(&"email_from".to_string()).unwrap();
    let email_subject = account_fields.get(&"email_from".to_string()).unwrap();

    let filled_email_struct = format!(
        r#"{{"personalizations": [{{"to": [{{"email": "{}"}}],"subject": "{}"}}],"from": {{"email": "{}"}},"content": [{{"type": "text/html","value": "{}"}}]}}"#, 
        email_to, email_subject, email_from, email_content
    );

    // let account_id = "ACbd9666b3f5427bb33828653997cb357a".to_string();
    // let token = "8cd2cb93c2e4bdcef546690b0868e995".to_string();

    // Set up the authorization for the call
    // let mut headers = Headers::new();
    // headers.set(
    //    Authorization(
    //        Bearer {
    //            token: api_key.to_owned()
    //        }
    //    )
    // );

    let url = "https://api.sendgrid.com/v3/mail/send HTTP/1.1";

    let client = reqwest::Client::new();
    async {

        let res = client
            .post(url)
            .bearer_auth(api_key)
//            .header(headers)
            .body(filled_email_struct)
            .send()
            .await
            .unwrap();

        let mytext = res.text().await.unwrap();
        println!("{}", mytext);
    };

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
fn process_request (msg: DC) {

    // For each account and channel template combination, send the communication
    for template_channel in &msg.template_channels {

        // Get the fields used in the template so that the data can be requested
        let template_fields: Vec<String> = get_template_fields(template_channel.template_id);

        for account in &msg.accounts {
            
            // Get the fields for the account
            let account_fields = get_account_fields(&account, &template_fields);

            // Combine the template with the fields retrieved for the account
            let populated_template = "Hi this is a temporary message".to_string();

            // Send the digital communication for the account through the correct channel

            // Send a status message to the event store registering that a DC has occured for that account
            match &*template_channel.channel {
                "email" => {
                    let api_key = "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".to_string();
                    send_email(&account_fields, &populated_template, &api_key);
                    info!("");
                },
                "sms" => {
                    send_sms();
                    info!("");
                },
                "pdf" => {
                    send_pdf();
                    info!("");
                },
                "whatsapp" => {
                    send_whatsapp();
                    info!("");
                },
                ch => error!("Unknown channel specified in received msg. Channel found is: {}", ch),
            }
        }
    }
}


//************************************************************************
fn get_template_fields (template_id: i32) -> Vec<String> {
    vec!()
}


//************************************************************************
fn get_account_fields (account: &String, template_fields: &Vec<String>) -> HashMap<String, String> {






}


#[derive(Serialize, Deserialize, Debug)]
struct Account {
    days_delinquent: i16,
    amount_due: f64,
    account_number: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct Contract {
    id: i16,
    first_name: String,
    last_name: String,
    gender: String,
    email: String,
    address1: String,
    address2: String,
    address3: String,
    city: String,
    state: String,
    zip: String,
    client: String,
    accounts: Vec<Account>,
    currency: String,
}


fn get_data() -> Vec<UserData> {

    let file_contents = fs::read_to_string("./mock_data.json").expect("error on read string from file");

    let array: Vec<UserData> = serde_json::from_str(&file_contents).expect("");

    array
}

