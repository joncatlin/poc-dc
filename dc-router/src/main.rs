// #[macro_use] extern crate log;
// #[macro_use] extern crate serde_derive;
#[macro_use]
extern crate log; 
extern crate env_logger;
// extern crate chrono;
extern crate serde_json;
extern crate serde_derive;

use std::env;
//use actix_web::{post, web, App, Error, HttpResponse, HttpServer, Result};
use actix_web::{post, web, App, HttpResponse, HttpServer, Result};
//use bytes::Bytes; 
// use std::io::Write;
// use chrono::{Local, Utc, DateTime, NaiveDateTime};
// use env_logger::Builder;
// use log::LevelFilter;
use serde::{Deserialize, Serialize};
// use json::JsonValue;
use std::thread;
use std::time::Duration;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord};
use std::sync::mpsc::{channel, Sender, Receiver};

use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct DCRouteRequest {
    correspondence_id: i32,
    account_list_handle: String,
}

// // TODO figure out how to share this structure across multiple components such as ui-services
// #[derive(Debug, Deserialize, Serialize)]
// pub struct Hierarchy {
//     pub developer: String,
//     pub project: String,
//     pub lender: String,
// }

// Structure to hold a template id and channel to be used to send the DC through
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TemplateChannel {
    template_id: i32,
    language_id: i32,
    channel: String,
}

// Structue of the expected payload for a digital communication event received via Kafka
#[derive(Clone, Debug, Serialize, Deserialize)]
struct DC {
    id: String,

    // Templates and channels to be used for the DC.
    template_channels: Vec<TemplateChannel>,

    // List of accounts to send the digital communication to
    accounts: Vec<String>,
}





//************************************************************************
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    // Use a channel for the web responders to communicate with the Kafka polling and sending thread. This
    // prevents the issue with trying to share a Kafka Producer amongst various threads.
    let (tx, rx): (Sender<DC>, Receiver<DC>) = channel();

    // Kick off a thread for the Kafka polling and sending of events
    thread::spawn(move|| {
        kafka_poll(rx)
    });

    // Start the HTTP Server and register all of the endpoints then wait for calls
    HttpServer::new(move || {
        App::new()
            .data(tx.clone())
            .service(dc_route)
            // .service(email_hook)
            // .service(whatsapp_hook)
        })
        .bind("0.0.0.0:8081")?
        .run()
        .await
}


//************************************************************************
fn kafka_poll(receiver: Receiver<DC>) {

    // Get the bootstrap servers and topic from the environment variables
    let bootstrap_servers_env = env::var("KAFKA_BOOTSTRAP_SERVERS").expect("Could not find environment variable named KAFKA_BOOTSTRAP_SERVERS. Without this variable being set the program will not work.");
    let bootstrap_servers: &str = &*bootstrap_servers_env;

    let topic_env = env::var("KAFKA_TOPIC").expect("Could not find environment variable named KAFKA_TOPIC. Without this variable being set the program will not work.");
    let topic: &str = &*topic_env;

    info!("Configured boot_strap_servers={} and topic={}", bootstrap_servers, topic);

    // TODO handle the error correctly and decide what to do
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .set("message.timeout.ms", "5000")
        // .set("bootstrap.servers", &*bootstrap_servers)
        // .set("acks", "1")
        // .set("retries", "3")
        .create()
        .expect("Producer creation error");

    // Poll Kafka to ensure all the asynchronous delivery events are processed
    // Also required for any changes in cluster etc events to be handled correctly
    loop {
        producer.poll(Duration::from_millis(100));

        // Check for events waiting on the channel to be sent via Kafka
        for event in receiver.try_iter() {
            println!("event received in kafka_poll is: {:?}", event);

            let payload = serde_json::to_vec(&event).unwrap();
            let key = String::as_bytes(&event.id);

            // Send the message to the topic
            // TODO handle the error correctly!
            match producer.send(
                BaseRecord::to(topic)
                    .payload(&payload)
                    .key(key),
                ) {
                    Ok(_) => {
                        info!("Successfully sent msg with id: {} to kafka topic {:?}", event.id, topic);
                    },
                    Err(e) => {
                        error!("Failed to send msg content: {:?} due to error: {:?}", event, e);
                    }
                };
        }
    }
}

//************************************************************************
#[post("/csc/dc-router/v1/route")]
async fn dc_route(transmitter: web::Data<Sender<DC>>, _route_rq: web::Data<DCRouteRequest>) -> Result<HttpResponse> {

    let tc1 = TemplateChannel {template_id: 1, language_id: 1, channel: "pdf".to_string()};
    let tc2 = TemplateChannel {template_id: 1, language_id: 1, channel: "email".to_string()};
    let tc3 = TemplateChannel {template_id: 1, language_id: 1, channel: "sms".to_string()};
    let accs = vec!(
        "Account_1".to_string(), "Account_2".to_string(), "Account_3".to_string(), "Account_4".to_string(), 
        "Account_5".to_string(), "Account_6".to_string(), "Account_7".to_string(), "Account_8".to_string(), 
    );

    // Create the message to be sent via kafka topic
    let dc_msg = DC {id: Uuid::new_v4().to_string(), template_channels: vec!(tc1, tc2, tc3), accounts: accs, };            

    info!("Sending DC msg content: {:?}", dc_msg);

    // let dc_string = serde_json::to_string(&dc).unwrap();

    // Create the event and produce it to a kafka topic
    // let event = create_email_event(resp);
    transmitter.send(dc_msg.clone()).unwrap();

    Ok(HttpResponse::Ok().json(dc_msg))
}


