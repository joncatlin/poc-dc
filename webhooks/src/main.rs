// #[macro_use] extern crate log;
// #[macro_use] extern crate serde_derive;
#[macro_use]
extern crate log; 
extern crate env_logger;
extern crate chrono;
extern crate serde_json;
extern crate serde_derive;

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

/****************************************************************
* Data strcutures used
*/

// The msg generated to describe the webhook call received
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Event {
    id: String,
    status: String,
    datetime_rfc2822: String,
    event_specific_data: String,
}

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

// Struct to hold the webstate that is passed to every responder
//#[derive(Debug)]
struct WebStateForKafka {
    producer: BaseProducer,
}


// struct Foo<'a> {
//     x: &'a i32,
// }



#[actix_rt::main]
async fn main() -> std::io::Result<()> {

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

    // let producer: BaseProducer = ClientConfig::new()
    //     .set("bootstrap.servers", "kafka1:19092,kafka2:19092,kafka3:19092")
    //     .create()
    //     .expect("Producer creation error");

    // TODO Use a THreadedProducer but it was throwing type error that could not resolve/understand
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "kafka1:19092,kafka2:19092,kafka3:19092")
        .create()
        .expect("Producer creation error");

    // Create the webstate to hold the Kafka producer so it can be used by the poll thread and
    // all of the actix-web responders
    let webstate = web::Data::new(WebStateForKafka {producer: producer.clone()});

    // Kick off a thread
    // thread::spawn(|| {
    //     let mut i = 0;
    //     loop {
    //         println!("hi number {} from the spawned thread!", i);
    //         i += 1;
    //         thread::sleep(Duration::from_millis(10000));
    //     }
    // });
    // Kick off a thread
    thread::spawn(|| {
//        simple()
        kafka_poll(producer)
    });

    // Start the HTTP Server and register all of the endpoints
    HttpServer::new(move || {
        App::new()
            .data(webstate.clone())
            .service(sms_hook)
            .service(email_hook)
            .service(whatsapp_hook)
        })
        .bind("0.0.0.0:8081")?
        .run()
        .await
}


//************************************************************************
fn kafka_poll(producer: BaseProducer) {
    // Poll Kafka to ensure all the asynchronous delivery events are processed
    // Also required for any changes in cluster etc events to be handled correctly
    loop {
        producer.poll(Duration::from_millis(100));
    }
}

//************************************************************************
#[post("/csc/webhooks/sms")]
async fn sms_hook(state: web::Data<WebStateForKafka>, form: web::Form<SMSFormData>) -> Result<HttpResponse> {

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

    log_info (&json_data);

    // Create an event to send to the rest of the system, then send it
    let event = Event { 
        id: sms_data.message_sid, 
        status: sms_data.message_status, 
        datetime_rfc2822: Utc::now().to_rfc2822(),
        event_specific_data: json_data,
    };

    let payload = serde_json::to_vec(&event).unwrap();
    let key = String::as_bytes(&event.id);

    // Send the event here
    state.producer.send(
        BaseRecord::to("destination_topic")
            .payload(&payload)
            .key(key),
        ).expect("Failed to enqueue");
    // TODO handle the return code properly


    info!("EVENT to be sent = {}", serde_json::to_string(&event).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(OK_STATUS))
}

//************************************************************************
fn log_info (data_to_be_logged: &String) {
    info!("EVENT-SMS-Received:{}", data_to_be_logged);
}



//************************************************************************
#[post("/csc/webhooks/email")]
async fn email_hook(body: Bytes) -> Result<HttpResponse, Error> {
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    match result {
        Ok(v) => { 
            info!("EVENT-EMAIL-Received:{}", v.dump());

            let event = create_email_event(v);

            // Send the event here
            info!("EVENT to be sent = {}", serde_json::to_string(&event).unwrap());

            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(OK_STATUS));
        },
        Err(e) => {
            warn!("EVENT-EMAIL-body contained {} and parse error was: {}", std::str::from_utf8(&body).unwrap(), e.to_string());
            let err_msg = json::object! {"err" => e.to_string()};
            return Ok(HttpResponse::BadRequest()
                .content_type("application/json")
                .body(err_msg.dump()
            ));
        }
    };
}



//************************************************************************
fn create_email_event(v: JsonValue) -> Event {

    // Ensure the timestamp is an i64 before converting it to the correct date/time format
    let datetime = match v["timestamp"].as_i64() {
        None => {
            // Log that the event received's timestamp is not i64 and default to using the time now
            warn!("Received a timestamp of: [{}] when expecting value of type 64 bit integer. Defaulting timstamp to time now. Full message received: {}",
                v["timestamp"], v.dump());
                Utc::now().to_rfc2822()
        },
        Some(i) => DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(i, 0), Utc).to_rfc2822()
    };
    
    // Ensure the id is a string
    let id = match v["smtp-id"].as_str() {
        Some(s) => s, 
        None => {
            // Log that the id is not a string
            warn!("Received a event with an id that is not a string: [{}]. Defaulting id to empty string. Full message received: {}",
                v["smtp_id"], v.dump());
            ""
        }, 
    };          

    // Convert the status  
    let status = match v["event"].as_str() {
        Some(s) => s, 
        None => {
            // Log that the statis is not a string
            warn!("Received an event status with that is not a string: [{}]. Defaulting status to \"unknown\". Full message received: {}",
                v["event"], v.dump());
            "unknown"
        }, 
    }; 

    // Create an event to send to the rest of the system
    let event = Event { 
        id: id.to_string(), 
        status: status.to_string(), 
        datetime_rfc2822: datetime,
        event_specific_data: v.dump(),
    };

    return event;
}




//************************************************************************
#[post("/csc/webhooks/whatsapp")]
async fn whatsapp_hook(body: Bytes) -> Result<HttpResponse, Error> {
    let result = json::parse(std::str::from_utf8(&body).unwrap());
    match result {
        Ok(v) => { 
            info!("EVENT-WHATSAPP-Received:{}", v.dump());

            let event = create_whatsapp_event(v);

            // Send the event here
            info!("EVENT to be sent = {}", serde_json::to_string(&event).unwrap());

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
fn create_whatsapp_event(v: JsonValue) -> Event {
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
    let event = Event { 
        id: id.to_string(), 
        status: status.to_string(), 
        datetime_rfc2822: datetime,
        event_specific_data: v.dump(),
    };

    return event;
}
