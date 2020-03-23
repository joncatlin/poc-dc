use futures::*;
use log::info;


use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
//use rdkafka::util::get_rdkafka_version;

//use crate::example_utils::setup_logger;

//mod example_utils;

async fn produce(brokers: &str, topic_name: &str) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // This loop is non blocking: all messages will be sent one after the other, without waiting
    // for the results.
    let futures = (0..5)
        .map(|i| {
            println!("sending for {}", i);
            // The send operation on the topic returns a future, that will be completed once the
            // result or failure from Kafka will be received.
            producer
                .send(
                    FutureRecord::to(topic_name)
                        .payload(&format!("Message {} this is jons payload", i))
                        .key(&format!("Key {}", i))
                        .headers(OwnedHeaders::new().add("header_key", "header_value")),
                    0,
                )
                .map(move |delivery_status| {
                    // This will be executed onw the result is received
                    info!("Delivery status for message {} received", i);
                    delivery_status
                })
        })
        .collect::<Vec<_>>();

    // This loop will wait until all delivery statuses have been received received.
    for future in futures {
        info!("Future completed. Result: {:?}", future.await);
    }
}

#[tokio::main]
async fn main() {

    let topic = "events";
    let brokers = "kafka1:19092,kafka2:19092,kafka3:19092";

    println!("hi jon");
    produce(brokers, topic).await;

}
