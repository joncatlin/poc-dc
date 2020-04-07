#[macro_use]
extern crate log; 
extern crate env_logger;

use env_logger::Builder;
use log::LevelFilter;
use chrono::{Local};
use std::io::Write;
use serde::{Deserialize, Serialize};
use std::env;

use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
//use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::producer::{BaseProducer, BaseRecord};
use std::time::Duration;

// Constants
static ACCOUNT_ID: &str = "account_id";
static TOKEN: &str = "token";
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


async fn produce() {

    // Get the bootstrap servers and topic from the environment variables
    let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS").expect("Could not find environment variable named KAFKA_BOOTSTRAP_SERVERS. Without this variable being set the program will not work.");

    let topic = env::var("KAFKA_TOPIC").expect("Could not find environment variable named KAFKA_TOPIC. Without this variable being set the program will not work.");

    info!("Environment variables KAFKA_BOOTSTRAP_SERVERS={}, KAFKA_TOPIC={}", bootstrap_servers, topic);


    // Create a DC msg as the payload
    let tc = TemplateChannel {template_id: 1, language_id: 2, channel: "email".to_string()};
//    let tc = TemplateChannel {template_id: 1, language_id: 2, channel: "email".to_string()};
//    let tc = TemplateChannel {template_id: 2, language_id: 1, channel: "email".to_string()};

    // 50 Accounts !!!!!
    let a = vec!(
        "account11".to_string(), "account21".to_string(), "account31".to_string(), "account41".to_string(), "account51".to_string(),
        "account12".to_string(), "account22".to_string(), "account32".to_string(), "account42".to_string(), "account52".to_string(),
        "account13".to_string(), "account23".to_string(), "account33".to_string(), "account43".to_string(), "account53".to_string(),
        "account14".to_string(), "account24".to_string(), "account34".to_string(), "account44".to_string(), "account54".to_string(),
        "account15".to_string(), "account25".to_string(), "account35".to_string(), "account45".to_string(), "account55".to_string(),
        "account16".to_string(), "account26".to_string(), "account36".to_string(), "account46".to_string(), "account56".to_string(),
        "account17".to_string(), "account27".to_string(), "account37".to_string(), "account47".to_string(), "account57".to_string(),
        "account18".to_string(), "account28".to_string(), "account38".to_string(), "account48".to_string(), "account58".to_string(),
        "account19".to_string(), "account29".to_string(), "account39".to_string(), "account49".to_string(), "account59".to_string(),
        "account10".to_string(), "account20".to_string(), "account30".to_string(), "account40".to_string(), "account50".to_string(),
    );
//    let a = vec!("account1".to_string());
    let dc = DC {id: "this is the id2".to_string(), template_channels: vec!(tc), accounts: a};            
    let dc_string = serde_json::to_string(&dc).unwrap();


    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", &*bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    info!("About to send payload: {:?}", dc_string);
    
    producer.send(
        BaseRecord::to(&*topic)
            .payload(&*dc_string)
            .key(&format!("Key {}", "1"))
            .headers(OwnedHeaders::new().add("header_key", "header_value"))
        ).expect("Failed to enqueue");
    
    // Poll at regular intervals to process all the asynchronous delivery events.
    for _ in 0..10 {
        producer.poll(Duration::from_millis(100));
    }
    
    // And/or flush the producer before dropping it.
    producer.flush(Duration::from_secs(1));

}

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

    produce().await;
}