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
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use serde::{Deserialize, Serialize};

// Body of the response when the msg processed successfully
const OK_STATUS: &str = "{ \"status\" : \"Ok\" }";


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

    // Start the HTTP Server and register all of the endpoints
    HttpServer::new(|| {
        App::new()
            .service(sms_hook)
            .service(email_hook)
            .service(whatsapp_hook)
        })
        .bind("0.0.0.0:80")?
        .run()
        .await
}


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

#[post("/csc/webhooks/sms")]
async fn sms_hook(form: web::Form<SMSFormData>) -> Result<HttpResponse> {
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

    info!("EVENT-SMS-{}", serde_json::to_string(&sms_data).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(OK_STATUS))
}


#[post("/csc/webhooks/email")]
async fn email_hook(body: Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    match result {
        Ok(v) => { 
            info!("EVENT-EMAIL-{}", v.dump());
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


#[post("/csc/webhooks/whatsapp")]
async fn whatsapp_hook(body: Bytes) -> Result<HttpResponse, Error> {
    let result = json::parse(std::str::from_utf8(&body).unwrap());
    match result {
        Ok(v) => { 
            info!("EVENT-WHATSAPP-{}", v.dump());
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
