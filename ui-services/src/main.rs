#[macro_use] extern crate log;
#[macro_use] extern crate postgres_derive;

use env_logger;

// Do not mess with this statement as the JSon error handler starts to break. Not sure why!!!!
use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, FromRequest, error, };

use dotenv::dotenv;
use tokio_postgres::NoTls;

// Modules
mod handlers;
mod app_errors;
mod models;
mod config;
mod db;

// Crate use statements
//use crate::handlers::{channel, category, correspondence, language, category_mapping, channel_config, template_file};
use crate::handlers::{channel, category, correspondence, language, category_mapping, channel_config, client_preference, template_file};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(channel::get_channels)
            .service(channel::get_channel_by_name)
            .service(channel::upsert_channels)
            .service(channel::delete_channels)

            .service(category::get_categories)
            .service(category::upsert_categories)
            .service(category::delete_categories)

            .service(correspondence::get_correspondences)
            .service(correspondence::upsert_correspondences)
            .service(correspondence::delete_correspondences)

            .service(language::get_languages)
            .service(language::upsert_languages)
            .service(language::delete_languages)


            .service(category_mapping::get_mapped_categories)
            .service(category_mapping::get_unmapped_correspondences)
            .service(category_mapping::upsert_category_mappings)
            .service(category_mapping::delete_category_mappings)
            .service(category_mapping::get_category_mapping)

            .service(channel_config::get_channel_configs)
            .service(channel_config::upsert_channel_configs)
            .service(channel_config::delete_channel_configs)

            .service(client_preference::get_client_preferences)
            .service(client_preference::upsert_client_preferences)
            .service(client_preference::delete_client_preferences)

            .service(template_file::upload_template_html)
            .service(template_file::upload_template)
            .service(template_file::download_template)
            .service(template_file::download_document)

            // Below are the data structures used. These need to be added 
            // to the error handler so that a json error can be captured and 
            // reported back to the caller
            // TODO. Look into putting these into a seperate file. There is a mechanism to do that.
            .app_data(web::Json::<Vec<models::Channel>>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))

            .app_data(web::Json::<Vec<models::Category>>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))

            .app_data(web::Json::<Vec<models::Correspondence>>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))

            .app_data(web::Json::<Vec<models::Language>>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))

            .app_data(web::Json::<Vec<models::CategoryMapping>>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))

            .app_data(web::Json::<Vec<models::ChannelConfig>>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))

            .app_data(web::Json::<models::ClientPreferenceQuery>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))
        })
        .bind(config.server_addr.clone())?
        .run();

    info!("Server running on internal container port at http://{}/", config.server_addr);

    server.await
}


//***********************************************************************************
fn json_error_handler(err: error::JsonPayloadError, req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => {
            HttpResponse::UnsupportedMediaType().body(detail)
        }
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };

    warn!("Client calling API is doing something wrong. Message is: {:?}", req);

    actix_web::error::InternalError::from_response(err, resp).into()
}


// ***************** FUTURE IN CASE WE NEED THIS
// /// This handler manually load request payload and parse json-rust
// async fn index_mjsonrust(body: Bytes) -> Result<HttpResponse, Error> {
//     // body is loaded, now we can deserialize json-rust
//     let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
//     let injson: JsonValue = match result {
//         Ok(v) => v,
//         Err(e) => json::object! {"err" => e.to_string() },
//     };
//     Ok(HttpResponse::Ok()
//         .content_type("application/json")
//         .body(injson.dump()))
// }

