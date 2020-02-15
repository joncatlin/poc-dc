use actix_web::{post, App, Error, HttpResponse, HttpServer};
use bytes::Bytes; 

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

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
        .bind("127.0.0.1:8088")?
        .run()
        .await
}


#[post("/sms")]
async fn sms_hook(body: Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    match result {
        Ok(v) => { 
            info!("EVENT-{}", v.dump());
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(v.pretty(2)));
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


#[post("/email")]
async fn email_hook(body: Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    match result {
        Ok(v) => { 
            info!("EVENT-{}", v.dump());
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(v.pretty(2)));
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
