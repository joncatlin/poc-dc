#[macro_use]
extern crate log; 
#[macro_use]
extern crate diesel;

use std::env;
use futures::StreamExt;
// use env_logger::Builder;
// use log::LevelFilter;
//use chrono::{Local, DateTime, ParseError, NaiveDateTime, Utc};
// use chrono::{Local};
// use std::io::Write;
use serde::{Deserialize};

use self::models::*;
use self::diesel::prelude::*;

use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;

mod models;
mod schema;

use diesel::result::{DatabaseErrorKind, Error};


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
    let conn = establish_connection();

    info!("In consume_and_print");

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
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
//        .set_log_level(RDKafkaLogLevel::Debug)
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
                        let msg: MessageEvent = serde_json::from_str(s).unwrap();
                        info!("Payload contains MessageEvent: {:?}", msg);

                        // Insert the event into the event store
                        // TODO properly handle any of the errors that could occur
                        insert_account (&msg, &conn).expect("Insert account failed");
                        insert_event (&msg, &conn).expect("Insert event failedd");
                        s
                    },
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    },
                };
                info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for i in 0..headers.count() {
                        let header = headers.get(i).unwrap();
                        info!("  Header {:#?}: {:?}", header.0, header.1);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}


//************************************************************************
#[tokio::main]
async fn main() {

    env_logger::init();

    info!("Event Store Process starting");

    consume().await
}


//************************************************************************
fn establish_connection() -> PgConnection {
    let url = ::std::env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&url).unwrap()
}

















//************************************************************************
fn insert_account (msg: &MessageEvent, conn: &PgConnection) -> Result<(), diesel::result::Error> {
    use schema::account::dsl::*;
    
    // Only create an entry in the Account table if the message has an account_id. Only the initial send
    // message will have the account_id as the status update messages from the channels do not contain one.
    if msg.account_id.is_empty() {
        return Ok(());
    } else {

        let acc = Account { message_id: msg.id.to_owned(), channel: msg.channel.to_owned(), account_id: msg.account_id.to_owned() };

        match diesel::insert_into(account) 
            .values(&acc)
            .execute(conn) {
                Ok(s) => debug!("Insert successful. Account: {:?}", acc),
                Err(e) => {
                    match e {
                        Error::DatabaseError  => {
                            // Ignore this as it could be legitimate due to a service restart or the vendor sending duplicate updates
                            return Ok(())
                        },
                        _ => return Err(e),
                    // if e == Error::DatabaseError::DatabaseErrorKind::UniqueViolation {
                    //     // Ignore it as it could be a duplicate caused by various conditions
                    //     return Ok(())
                }
            }
        }
        
        return Ok(());
    }
}


//************************************************************************
fn insert_event (msg: &MessageEvent, conn: &PgConnection)   -> Result<(), diesel::result::Error> {

    use schema::event::dsl::*;

//     match DateTime::parse_from_rfc2822(&msg.datetime_rfc2822) {
//         Err(e) => {
//             error!("ParseError converting received datetime_rfc2822 to DateTime. Received datetime_rfc2822 is: {}", 
//                 msg.datetime_rfc2822);
//             e
//         },
//         Ok(dt) => {
//             // TODO there must be a better way than doing all these conversions on a date time            
// //            let naive_dt = dt.naive_utc();
//             let naive_dt = Utc::now().naive_utc();

            // Insert the event into the datastore
            let ev = Event { 
                message_id: msg.id.to_owned(), 
                channel: msg.channel.to_owned(), 
                event_status: msg.status.to_owned(), 
                event_timestamp: msg.datetime_rfc2822.to_owned(), 
                event_specific_data: msg.event_specific_data.to_owned()
            };
        
            diesel::insert_into(event)
                .values(&ev)
                .execute(conn)?;
            Ok(())
    //     }
    // };
}


// Rubbish