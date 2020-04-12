#[macro_use]
extern crate log; 
extern crate env_logger;

// use env_logger::Builder;
// use log::LevelFilter;
// use chrono::{Local};
// use std::io::Write;
use serde::{Deserialize, Serialize};
use std::env;

use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
//use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::producer::{BaseProducer, BaseRecord};
use std::time::Duration;

use clap::{App, Arg};

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

fn get_accounts(num_batches: i32, num_msgs_per_batch: i32) -> Vec<Vec<String>> {

    // let num_batches = 100;
    // let num_msgs_per_batch = 10;

    let mut accounts = Vec::new();

    for batch_num in 0..(num_batches) {

        let mut batch = Vec::<String>::new();

        for msg in 0..(num_msgs_per_batch) {
            batch.push(format!("Account_{}", msg + (batch_num * num_msgs_per_batch)));
        }

        accounts.push(batch);
    }

    accounts
}


async fn produce(template_id: i32, language_id: i32, channel: &String, batches: i32, msgs_per_batch: i32) {


    // Get the bootstrap servers and topic from the environment variables
    let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS").expect("Could not find environment variable named KAFKA_BOOTSTRAP_SERVERS. Without this variable being set the program will not work.");

    let topic = env::var("KAFKA_TOPIC").expect("Could not find environment variable named KAFKA_TOPIC. Without this variable being set the program will not work.");

    info!("Environment variables KAFKA_BOOTSTRAP_SERVERS={}, KAFKA_TOPIC={}", bootstrap_servers, topic);


    // Create the producer
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", &*bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");


    // Create a DC msg as the payload
//    let tc = TemplateChannel {template_id: 1, language_id: 2, channel: "email".to_string()};
//    let tc = TemplateChannel {template_id: 2, language_id: 1, channel: "email".to_string()};

    let account_batches = get_accounts( batches, msgs_per_batch);

    info!("Got accounts: vec size is {}", account_batches.len());

    for (index, ab) in account_batches.iter().enumerate() {

        let tc = TemplateChannel {template_id: template_id, language_id: language_id, channel: channel.clone()};

        let ident = format!("ID_{}", index);
        let dc = DC {id: ident, template_channels: vec!(tc), accounts: ab.to_vec()};            
        let dc_string = serde_json::to_string(&dc).unwrap();

        info!("About to send payload: {:?}", dc_string);
        producer.send(
            BaseRecord::to(&*topic)
                .payload(&*dc_string)
                .key(&format!("Key {}", index.to_string()))
                .headers(OwnedHeaders::new().add("header_key", "header_value"))
            ).expect("Failed to enqueue");
        
        info!("Send payload index: {}", index);

    }

    
    // Poll at regular intervals to process all the asynchronous delivery events.
    for _ in 0..10 {
        producer.poll(Duration::from_millis(100));
        info!("Polling producer");
    }
    
    // And/or flush the producer before dropping it.
    producer.flush(Duration::from_secs(1));

}

#[tokio::main]
async fn main() {

    env_logger::init();

//template_id: 1, language_id: 2, channel: "email"


    let matches = App::new("test-channel-sender")
    .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
    .about("Creates messages designed to test the channel-sender app")
    .arg(
        Arg::with_name("template_id")
            .short("t")
            .long("template_id")
            .help("The identifier of the template in the msg")
            .takes_value(true)
            .default_value("1"),
    )
    .arg(
        Arg::with_name("language_id")
            .long("l")
            .help("The identifier of the language to use in the msg")
            .takes_value(true)
            .default_value("2"),
    )
    .arg(
        Arg::with_name("channel")
            .short("c")
            .long("channel")
            .help("The channel to use in the msg")
            .takes_value(true)
            .default_value("email"),
    )
    .arg(
        Arg::with_name("num_msgs")
            .short("n")
            .long("num_msgs")
            .help("The number of msgs to send")
            .takes_value(true)
            .default_value("1"),
    )
    .arg(
        Arg::with_name("batches")
            .short("b")
            .long("batches")
            .help("The number of batches of msgs to send")
            .takes_value(true)
            .default_value("1"),
    )
    .arg(
        Arg::with_name("msgs_per_batch")
            .short("m")
            .long("msgs_per_batch")
            .help("The number of msgs to send per batch")
            .takes_value(true)
            .default_value("1"),
    )
    .get_matches();


    let template_id = matches.value_of("template_id").unwrap().parse::<i32>().unwrap();
    let language_id = matches.value_of("language_id").unwrap().parse::<i32>().unwrap();
    let channel = matches.value_of("channel").unwrap();
    let batches = matches.value_of("batches").unwrap().parse::<i32>().unwrap();
    let msgs_per_batch = matches.value_of("msgs_per_batch").unwrap().parse::<i32>().unwrap();

    produce(template_id, language_id, &channel.to_string(), batches, msgs_per_batch).await;
}