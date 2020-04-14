#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

#[macro_use]
extern crate diesel;

use actix_web::{get, middleware, post, put, delete, web, App, Error, HttpResponse, HttpServer, HttpRequest, FromRequest, error, };
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;
mod category_actions;
mod language_actions;
mod channel_actions;
mod template_actions;
mod corr_actions;
mod category_mapping_actions;
mod channel_config_actions;
mod schema;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use std::io::Write;

use actix_multipart::Multipart;
//use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::{StreamExt, TryStreamExt};


/*
************************* CATEGORIES ***********************************************************
*/
/// Get a list of all the categories that have been defined.
#[get("/ui-services/v1/categories")]
async fn get_categories(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || category_actions::find_categories(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Create categories given an array of categories
#[put("/ui-services/v1/categories")]
async fn upsert_categories(
    pool: web::Data<DbPool>,
    cats: web::Json<Vec<models::Category>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || category_actions::upsert_new_categories(&cats, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Delete a list of categories
#[delete("/ui-services/v1/categories")]
async fn delete_categories(
    pool: web::Data<DbPool>,
    cats: web::Json<Vec<models::Category>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || category_actions::delete_existing_categories(&cats, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/*
************************* CORRESPONDENCES ***********************************************************
*/
/// Get a list of all the correspondences that have been defined.
#[get("/ui-services/v1/correspondences")]
async fn get_correspondences(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || corr_actions::find_correspondences(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Create correspondences given an array of correspondences
#[put("/ui-services/v1/correspondences")]
async fn upsert_correspondences(
    pool: web::Data<DbPool>,
    cats: web::Json<Vec<models::Correspondence>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || corr_actions::upsert_new_correspondences(&cats, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Delete a list of correspondences
#[delete("/ui-services/v1/correspondences")]
async fn delete_correspondences(
    pool: web::Data<DbPool>,
    cats: web::Json<Vec<models::Correspondence>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || corr_actions::delete_existing_correspondences(&cats, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}

//RUBBISH


/*
************************* CHANNELS ***********************************************************
*/
/// Get a list of all the channels that have been defined.
#[get("/ui-services/v1/channels")]
async fn get_channels(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || channel_actions::find_channels(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Get a list of all the channels that have been defined.
#[get("/ui-services/v1/channels/{chan_name}")]
async fn get_channel_by_name(
    pool: web::Data<DbPool>,
    chan_name: web::Path<String>,
) -> Result<HttpResponse, Error> {

    let chan_name = chan_name.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || channel_actions::find_channel_by_name(chan_name, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Create channels given an array of channels
#[put("/ui-services/v1/channels")]
async fn upsert_channels(
    pool: web::Data<DbPool>,
    cats: web::Json<Vec<models::Channel>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || channel_actions::upsert_new_channels(&cats, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Delete a list of channels
#[delete("/ui-services/v1/channels")]
async fn delete_channels(
    pool: web::Data<DbPool>,
    cats: web::Json<Vec<models::Channel>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || channel_actions::delete_existing_channels(&cats, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}



/*
************************* LANGUAGES ***********************************************************
*/
/// Get a list of all the languages that have been defined.
#[get("/ui-services/v1/languages")]
async fn get_languages(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || language_actions::find_languages(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Create languages given an array of languages
#[put("/ui-services/v1/languages")]
async fn upsert_languages(
    pool: web::Data<DbPool>,
    cats: web::Json<Vec<models::Language>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || language_actions::upsert_new_languages(&cats, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}

/// Delete a list of languages
#[delete("/ui-services/v1/languages")]
async fn delete_languages(
    pool: web::Data<DbPool>,
    cats: web::Json<Vec<models::Language>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || language_actions::delete_existing_languages(&cats, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/*
************************* CATEGORY to CORRESPONDENCE MAPPING ***********************************************************
*/
/// Get a list of all the correspondences that have been mapped to a category.
#[get("/ui-services/v1/category-correspondence-mappings/mapped/{cat_id}")]
async fn get_mapped_category_corr (
    pool: web::Data<DbPool>,
    cat_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {

    let cat_id = cat_id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || category_mapping_actions::find_mapped_category_corrs(cat_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Get a list of all the correspondences that are not mapped
#[get("/ui-services/v1/category-correspondence-mappings/unmapped")]
async fn get_unmapped_category_corr(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || category_mapping_actions::find_unmapped_category_corrs(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Delete category correspondence mappings given a category id
#[delete("/ui-services/v1/category-correspondence-mappings/mapped/{cat_id}")]
async fn delete_mapped_category_corr(
    pool: web::Data<DbPool>,
    cat_maps: web::Json<Vec<i32>>,
    cat_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let cat_id = cat_id.into_inner();
    let results = web::block(move || category_mapping_actions::delete_existing_category_mappings(cat_id, &cat_maps, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Upsert category correspondence mappings given a category id
#[put("/ui-services/v1/category-correspondence-mappings/mapped")]
async fn upsert_category_correspondence_mappings(
    pool: web::Data<DbPool>,
    cat_maps: web::Json<Vec<models::CategoryMappingsWithChannelConfig>>,
) -> Result<HttpResponse, Error> {

    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || category_mapping_actions::upsert_new_category_mappings(&cat_maps, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
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















/// Get a list of all the correspondences that have been mapped to a category.
// #[get("/ui-services/v1/category-correspondence-mappings/mapped/{cat_id}")]
// async fn get_category_corr_mappings (
//     pool: web::Data<DbPool>,
//     cat_id: web::Path<i32>,
// ) -> Result<HttpResponse, Error> {

//     let cat_id = cat_id.into_inner();
//     let conn = pool.get().expect("couldn't get db connection from pool");

//     // use web::block to offload blocking Diesel code without blocking server thread
//     let results = web::block(move || category_mapping_actions::find_category_mappings(cat_id, &conn))
//         .await
//         .map_err(|e| {
//             eprintln!("{}", e);
//             HttpResponse::InternalServerError().finish()
//         })?;

//     Ok(HttpResponse::Ok().json(results))
// }


/// Create correspondences given an array of corrs
// #[post("/ui-services/v1/category-correspondence-mappings/mapped")]
// async fn add_category_corr_mappings(
//     pool: web::Data<DbPool>,
//     cats: web::Json<Vec<models::NewCategoryMapping>>,
// ) -> Result<HttpResponse, Error> {
//     let conn = pool.get().expect("couldn't get db connection from pool");


//     // use web::block to offload blocking Diesel code without blocking server thread
//     let results = web::block(move || category_mapping_actions::insert_new_category_mappings(&cats, &conn))
//         .await
//         .map_err(|e| {
//             eprintln!("{}", e);
//             HttpResponse::InternalServerError().finish()
//         })?;

//     Ok(HttpResponse::Ok().json(results))
// }







/// Get a list of templates
// #[get("/ui-services/v1/templates")]
// async fn get_templates (
//     pool: web::Data<DbPool>,
// ) -> Result<HttpResponse, Error> {

//     let conn = pool.get().expect("couldn't get db connection from pool");

//     // use web::block to offload blocking Diesel code without blocking server thread
//     let results = web::block(move || template_actions::find_templates(&conn))
//         .await
//         .map_err(|e| {
//             eprintln!("{}", e);
//             HttpResponse::InternalServerError().finish()
//         })?;

//     Ok(HttpResponse::Ok().json(results))
// }


/// Get a template by its id
// #[get("/ui-services/v1/templates/{obj_id}")]
// async fn get_template (
//     pool: web::Data<DbPool>,
//     obj_id: web::Path<i32>,
// ) -> Result<HttpResponse, Error> {

//     let obj_id = obj_id.into_inner();
//     let conn = pool.get().expect("couldn't get db connection from pool");

//     // use web::block to offload blocking Diesel code without blocking server thread
//     let results = web::block(move || template_actions::find_template(obj_id, &conn))
//         .await
//         .map_err(|e| {
//             eprintln!("{}", e);
//             HttpResponse::InternalServerError().finish()
//         })?;

//     Ok(HttpResponse::Ok().json(results))
// }


/// Create correspondences given an array of corrs
#[post("/ui-services/v1/templates")]
async fn add_templates(
    pool: web::Data<DbPool>,
    objs: web::Json<Vec<models::NewTemplate>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");


    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || template_actions::insert_templates(&objs, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}






//***********************************************************************************
fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
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
    error::InternalError::from_response(err, resp).into()
}



//***********************************************************************************
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "0.0.0.0:8080";

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())

            .service(get_categories)
            .service(upsert_categories)
            .service(delete_categories)

            .service(get_correspondences)
            .service(upsert_correspondences)
            .service(delete_correspondences)

            .service(get_channels)
            .service(get_channel_by_name)
            .service(upsert_channels)
            .service(delete_channels)

            .service(get_languages)
            .service(upsert_languages)
            .service(delete_languages)

            .service(get_mapped_category_corr)
            .service(get_unmapped_category_corr)
            .service(delete_mapped_category_corr)
            .service(upsert_category_correspondence_mappings)

            .service(upload_template)
            .service(save_template)

            // .service(
            //     web::resource("/ui-services/v1/categories")
            //         // change json extractor configuration
            //         .app_data(web::Json::<Vec<models::Category>>::configure(|cfg| {
            //             cfg.limit(4096).error_handler(|err, _req| {cfg.error_handler(json_error_handler)})
            //         }))
            //         .route(web::put().to(upsert_categories))
            // )

            // .service(
            //     web::resource("/ui-services/v1/categories")
            //         .data(web::JsonConfig::default().limit(1024).error_handler(json_error_handler)) // <- limit size of the payload (resource level)
            //         .route(web::put().to(upsert_categories)),
            // )

            // .service(
            //     web::resource("/ui-services/v1/categories")
            //         .data(web::JsonConfig::default().limit(1024).error_handler(json_error_handler)) // <- limit size of the payload (resource level)
            //         .route(web::put().to(delete_categories)),
            // )

            // ************** Register Error Handler for all  the models used

            .app_data(web::Json::<Vec<models::Category>>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))

            .app_data(web::Json::<Vec<models::Correspondence>>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))

            .app_data(web::Json::<Vec<models::Channel>>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))

            .app_data(web::Json::<Vec<models::Language>>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))

            .app_data(web::Json::<Vec<i32>>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))

            .app_data(web::Json::<Vec<models::CategoryMappingsWithChannelConfig>>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))
    })
    .bind(&bind)?
    .run()
    .await
}




















// TODO. Upload the file not using the filenme in the form, but use a different mechanism
// that cannot be guessed by the uploader
// TODO. Ensure the file permissions are correct
// TODO. Once complete the file should be moved from the directory to its final location with the correct name
#[post("/ui-services/v1/templates/file-upload")]
async fn save_template(mut payload: Multipart) -> Result<HttpResponse, Error> {
    debug!("In save_template");
    
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./template_temp/{}", filename);
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}


#[get("/ui-services/v1/templates/file-upload")]
fn upload_template() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/ui-services/v1/templates/file-upload" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
//     std::fs::create_dir_all("./tmp").unwrap();

//     let ip = "0.0.0.0:3000";

//     HttpServer::new(|| {
//         App::new().wrap(middleware::Logger::default()).service(
//             web::resource("/")
//                 .route(web::get().to(index))
//                 .route(web::post().to(save_file)),
//         )
//     })
//     .bind(ip)?
//     .run()
//     .await
// }
