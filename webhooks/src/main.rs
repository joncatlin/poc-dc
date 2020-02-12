use actix_web::{post, get, web, App, Error, HttpResponse, HttpServer, Responder};
use json::JsonValue;
use bytes::Bytes; 

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(index3)
            .service(sms)
            .service(sms2)
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
async fn sms() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[post("/sms2")]
async fn sms2(body: Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(injson.dump()))
}