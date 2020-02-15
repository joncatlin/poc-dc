use actix_web::{post, web, App, Error, HttpResponse, HttpServer, Result};
use bytes::Bytes; 

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

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
        })
        .bind("0.0.0.0:80")?
//        .bind("127.0.0.1:80")?
        .run()
        .await
}



#[derive(Serialize, Deserialize)]
struct SMSData {
    SmsSid: String,
    SmsStatus: String,
}


// var smsSid = Request.Form["SmsSid"];
// var messageStatus = Request.Form["MessageStatus"];
#[post("/csc/webhooks/sms")]
async fn sms_hook(form: web::Form<SMSData>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Welcome smsSid={} ans messageStatus={}!", form.SmsSid, form.SmsStatus)))
}



// async fn sms_hook(form: web::Form<FormData>) -> Result<String> {

//     Ok(format!("Welcome smsSid={} ans messageStatus={}!", form.smsSid, form.messageStatus))
// }


// #[post("/csc/webhooks/sms")]
// async fn sms_hook(body: Bytes) -> Result<HttpResponse, Error> {
//     // body is loaded, now we can deserialize json-rust
//     let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
//     match result {
//         Ok(v) => { 
//             info!("EVENT-{}", v.dump());
//             return Ok(HttpResponse::Ok()
//                 .content_type("application/json")
//                 .body(OK_STATUS));
//         },
//         Err(e) => {
//             warn!("EVENT-body contained {} and parse error was: {}", std::str::from_utf8(&body).unwrap(), e.to_string());
//             let err_msg = json::object! {"err" => e.to_string()};
//             return Ok(HttpResponse::BadRequest()
//                 .content_type("application/json")
//                 .body(err_msg.dump()
//             ));
//         }
//     };
// }


#[post("/csc/webhooks/email")]
async fn email_hook(body: Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    match result {
        Ok(v) => { 
            info!("EVENT-{}", v.dump());
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(OK_STATUS));
        },
        Err(e) => {
            warn!("EVENT-body contained {} and parse error was: {}", std::str::from_utf8(&body).unwrap(), e.to_string());
            let err_msg = json::object! {"err" => e.to_string()};
            return Ok(HttpResponse::BadRequest()
                .content_type("application/json")
                .body(err_msg.dump()
            ));
        }
    };
}
