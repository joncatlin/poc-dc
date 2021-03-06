// #[macro_use] extern crate log;
// #[macro_use] extern crate serde_derive;
#[macro_use]
extern crate log; 
extern crate env_logger;
extern crate chrono;
extern crate serde_json;
extern crate serde_derive;

use std::env;
use actix_web::{post, web, App, Error, HttpResponse, HttpServer, Result};
use bytes::Bytes; 
use std::io::Write;
use chrono::{Local, Utc, DateTime, NaiveDateTime};
use env_logger::Builder;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use json::JsonValue;
use std::thread;
use std::time::Duration;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord};
use std::sync::mpsc::{channel, Sender, Receiver};

/****************************************************************
* Data strcutures used
*/

// // The msg generated to describe the webhook call received
// #[derive(Debug)]
// #[derive(Serialize, Deserialize)]
// struct MessageEvent {
//     account_id: String,
//     id: String,
//     channel: String,
//     status: String,
//     datetime_rfc2822: String,
//     event_specific_data: String,
// }

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


// The request object received for the SMS webhook
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct SMSFormData {
    SmsSid: String,
    SmsStatus: String,
    MessageStatus: String,
    To: String,
    MessageSid: String,
    AccountSid: String,
    From: String,
    ApiVersion: String,
}

// The structure to hold the data received by the SMS webhook
#[derive(Serialize, Deserialize)]
struct SMSData {
    sms_sid: String,
    message_status: String,
    message_sid: String,
    account_sid: String,
    from: String,
    api_version: String,
    to: String,
    sms_status: String,
}

// Body of the response when the msg processed successfully
const OK_STATUS: &str = "{ \"status\" : \"Ok\" }";


//************************************************************************
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    // Use a channel for the web responders to communicate with the Kafka polling and sending thread. This
    // prevents the issue with trying to share a Kafka Producer amongst various threads.
    let (tx, rx): (Sender<MessageEvent>, Receiver<MessageEvent>) = channel();

    // Kick off a thread for the Kafka polling and sending of events
    thread::spawn(move|| {
        kafka_poll(rx)
    });

    // Start the HTTP Server and register all of the endpoints then wait for calls
    HttpServer::new(move || {
        App::new()
            .data(tx.clone())
            .service(sms_hook)
            .service(email_hook)
            .service(whatsapp_hook)
        })
        .bind("0.0.0.0:8081")?
        .run()
        .await
}


//************************************************************************
fn kafka_poll(receiver: Receiver<MessageEvent>) {

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

        // ********THIS CODE WORKS FROM SIMPLE-PRODUCR **************
        // let producer: FutureProducer = ClientConfig::new()
        // .set("bootstrap.servers", brokers)
        // .set("message.timeout.ms", "5000")
        // .create()
        // .expect("Producer creation error");












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
                // producer.send(
                //     BaseRecord::to(&*topic)
                //         .payload(&payload)
                //         .key(key),
                //     ).expect("Failed to enqueue");
                
        }
    }
}

//************************************************************************
#[post("/csc/webhooks/sms")]
async fn sms_hook(transmitter: web::Data<Sender<MessageEvent>>, form: web::Form<SMSFormData>) -> Result<HttpResponse> {

    let form = form.into_inner();
    let sms_data = SMSData { 
        message_status: form.MessageStatus, 
        message_sid: form.MessageSid,
        account_sid: form.AccountSid,
        from: form.From,
        api_version: form.ApiVersion,
        to: form.To,
        sms_status: form.SmsStatus,
        sms_sid: form.SmsSid,
    };

    let json_data = serde_json::to_string(&sms_data).unwrap();

    info!("EVENT-SMS-Received:{}", json_data);

    // Create an event to send to the rest of the system, then send it
    let event = MessageEvent {        
        account_id: "".to_string(),
        id: sms_data.message_sid,
        channel: "sms".to_string(),
        status: sms_data.message_status,
        datetime_rfc2822: Utc::now().to_rfc2822(),
        event_specific_data: json_data,
    };

    debug!("EVENT-SMS-To be sent = {}", serde_json::to_string(&event).unwrap());

    // Send the event here
    // TODO handle the return code properly
    transmitter.send(event).unwrap();

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(OK_STATUS))
}


//************************************************************************
#[post("/csc/webhooks/email")]
async fn email_hook(transmitter: web::Data<Sender<MessageEvent>>, body: Bytes) -> Result<HttpResponse, Error> {

    info!("DEBUG-EVENT-EMAIL-Received:{:?}", body);
//    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    // match result {
    //     Ok(v) => { 
    //         info!("EVENT-EMAIL-Received:{}", v.dump());

    //         let event = create_email_event(v);

    //         debug!("EVENT-EMAIL-To be sent = {}", serde_json::to_string(&event).unwrap());

    //         // Send the event here
    //         // TODO handle the return code properly
    //         transmitter.send(event).unwrap();

    //         return Ok(HttpResponse::Ok()
    //             .content_type("application/json")
    //             .body(OK_STATUS));
    //     },
    //     Err(e) => {
    //         warn!("EVENT-EMAIL-body contained {} and parse error was: {}", std::str::from_utf8(&body).unwrap(), e.to_string());
    //         let err_msg = json::object! {"err" => e.to_string()};
    //         return Ok(HttpResponse::BadRequest()
    //             .content_type("application/json")
    //             .body(err_msg.dump()
    //         ));
    //     }
    // };

    let responses: Vec<serde_json::Value> = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();
    for resp in responses {
        info!("EVENT-EMAIL-Received:{:?}", resp);

        // Create the event and produce it to a kafka topic
        let event = create_email_event(resp);
        transmitter.send(event).unwrap();
    }


    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(OK_STATUS))

}



//************************************************************************
fn create_email_event(v: serde_json::Value) -> MessageEvent {
// fn create_email_event(v: JsonValue) -> MessageEvent {

    let datetime = match v.get("timestamp").unwrap().as_i64() {
        None => {
            // Log that the event received's timestamp is not i64 and default to using the time now
            warn!("Received a timestamp of: [{}] when expecting value of type 64 bit integer. Defaulting timstamp to time now. Full message received: {:?}",
            v.get("timestamp").unwrap(), v);
                Utc::now().to_rfc2822()
        },
        Some(i) => DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(i, 0), Utc).to_rfc2822()
    };

    // Ensure the id is a string
    let id = match v.get("sg_message_id").unwrap().as_str() {
        Some(s) =>  {
            // The id has some stuff on the end of it separated by a dot. Take the first part and ditch the rest.
            let split_vec: Vec<&str> = s.splitn(2,'.').collect();
            let extracted_id = split_vec[0]; 
            extracted_id
        },
        None => {
            // Log that the id is not a string
            warn!("Received a event with an id that is not a string: [{}]. Defaulting id to empty string. Full message received: {:?}",
                v.get("sg_message_id").unwrap(), v);
            ""
        }, 
    };          

    // Convert the status  
    let status = match v.get("event").unwrap().as_str() {
        Some(s) => {
            s
        }, 
        None => {
            // Log that the statis is not a string
            warn!("Received an event status with that is not a string: [{}]. Defaulting status to \"unknown\". Full message received: {:?}",
                v.get("event").unwrap(), v);
            "unknown"
        }, 
    }; 

    // Create an event to send to the rest of the system
    let event = MessageEvent { 
        account_id: "".to_string(), 
        id: id.to_string(),
        channel: "email".to_string(), 
        status: status.to_string(), 
        datetime_rfc2822: datetime,
        event_specific_data: v.to_string(),
    };

    return event;
}




//************************************************************************
#[post("/csc/webhooks/whatsapp")]
async fn whatsapp_hook(transmitter: web::Data<Sender<MessageEvent>>, body: Bytes) -> Result<HttpResponse, Error> {
    let result = json::parse(std::str::from_utf8(&body).unwrap());
    match result {
        Ok(v) => { 
            info!("EVENT-WHATSAPP-Received:{}", v.dump());

            let event = create_whatsapp_event(v);

            debug!("EVENT-WHATSAPP-To be sent = {}", serde_json::to_string(&event).unwrap());

            // Send the event here
            // TODO handle the return code properly
            transmitter.send(event).unwrap();

            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(OK_STATUS));
        },
        Err(e) => {
            warn!("EVENT-WHATSAPP-body contained {} and parse error was: {}", std::str::from_utf8(&body).unwrap(), e.to_string());
            let err_msg = json::object! {"err" => e.to_string()};
            return Ok(HttpResponse::BadRequest()
                .content_type("application/json")
                .body(err_msg.dump()
            ));
        }
    };
}



//************************************************************************
fn create_whatsapp_event(v: JsonValue) -> MessageEvent {
    // Ensure the timestamp is an i64 before converting it to the correct date/time format
    let datetime = match v["date_updated"].as_str() {
        None => {
            warn!("Received a datetime of: [{}] when expecting value of type string. Defaulting datetime to time now. Full message received: {}",
                v["date_updated"], v.dump());
                Utc::now().to_rfc2822()
        },
        Some(i) => i.to_string(),
    };

    // Ensure the id is a string
    let id = match v["sid"].as_str() {
        Some(s) => s, 
        None => {
            warn!("Received an event with an id that is not a string: [{}]. Defaulting id to empty string. Full message received: {}",
                v["sid"], v.dump());
            ""
        }, 
    };          

    // Convert the status  
    let status = match v["status"].as_str() {
        Some(s) => s, 
        None => {
            warn!("Received an event status with that is not a string: [{}]. Defaulting status to \"unknown\". Full message received: {}",
                v["status"], v.dump());
            "unknown"
        }, 
    }; 
    // TODO transform status to normalized status so it is ubiquitos across all event channels

    // Create an event to send to the rest of the system
    let event = MessageEvent { 
        account_id: "".to_string(), 
        id: id.to_string(), 
        channel: "whatsapp".to_string(), 
        status: status.to_string(), 
        datetime_rfc2822: datetime,
        event_specific_data: v.dump(),
    };

    return event;
}
