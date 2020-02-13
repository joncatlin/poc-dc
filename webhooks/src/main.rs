use actix_web::{post, get, web, App, Error, HttpResponse, HttpServer, Responder};
use json::JsonValue;
use bytes::Bytes; 

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

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
            .service(sms_status_update)
            .service(sms_status_update2)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}



#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[post("/sms")]
async fn sms_status_update(body: Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let injson: JsonValue = match result {
        Ok(v) => { 
            info!("EVENT-{}", v.stringify());
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(v.dump()));
        },
        Err(e) => {
            error!("EVENT-body contained {} and parse error was: {}", std::str::from_utf8(&body).unwrap(), e.to_string());
            return Ok(HttpResponse::StatusCode::BAD_REQUEST.as_u16()
                .content_type("application/json")
                .body(json::object! {"err" => e.to_string()}));
        }
    };


    // Ok(HttpResponse::Ok()
    //     .content_type("application/json")
    //     .body(injson.dump()))
//    Ok(HttpResponse::Ok())

//StatusCode::BAD_REQUEST.as_u16()
    // Generate a 400 status code when the json is invalid
}


#[post("/sms2")]
async fn sms_status_update2(body: Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string()},
    };


    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(injson.dump()))
//    Ok(HttpResponse::Ok())

//StatusCode::BAD_REQUEST.as_u16()
    // Generate a 400 status code when the json is invalid
}