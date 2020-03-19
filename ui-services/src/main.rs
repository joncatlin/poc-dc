#[macro_use]
extern crate diesel;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
//use uuid::Uuid;

mod models;
mod category_actions;
mod language_actions;
mod channel_actions;
mod template_actions;
mod corr_actions;
mod category_mapping_actions;
mod schema;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


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
#[post("/ui-services/v1/categories")]
async fn add_categories(
    pool: web::Data<DbPool>,
    cats: web::Json<Vec<models::NewCategory>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || category_actions::insert_new_categories(&cats, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}



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


/// Create channels given an array of channels
#[post("/ui-services/v1/channels")]
async fn add_channels(
    pool: web::Data<DbPool>,
    cats: web::Json<Vec<models::NewChannel>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || channel_actions::insert_new_channels(&cats, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}





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
#[post("/ui-services/v1/languages")]
async fn add_languages(
    pool: web::Data<DbPool>,
    cats: web::Json<Vec<models::NewLanguage>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || language_actions::insert_new_languages(&cats, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}




/// Get a list of all the correspondences that have been defined.
#[get("/ui-services/v1/correspondences")]
async fn get_corrs(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || corr_actions::find_corrs(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Create correspondences given an array of corrs
#[post("/ui-services/v1/correspondences")]
async fn add_corrs(
    pool: web::Data<DbPool>,
    cats: web::Json<Vec<models::NewCorr>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || corr_actions::insert_new_corrs(&cats, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}




/// Get a list of all the correspondences that have been mapped to a category.
#[get("/ui-services/v1/category-correspondence-mappings/mapped/{cat_id}")]
async fn get_category_corr_mappings (
    pool: web::Data<DbPool>,
    cat_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {

    let cat_id = cat_id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || category_mapping_actions::find_category_mappings(cat_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Get a list of all the correspondences that are not mapped
#[get("/ui-services/v1/category-correspondence-mappings/not_mapped")]
async fn get_unmapped_category_corr_mappings (
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || category_mapping_actions::find_corr_not_mapped(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Create correspondences given an array of corrs
#[post("/ui-services/v1/category-correspondence-mappings/mapped")]
async fn add_category_corr_mappings(
    pool: web::Data<DbPool>,
    cats: web::Json<Vec<models::NewCategoryMapping>>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || category_mapping_actions::insert_new_category_mappings(&cats, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}







/// Get a list of templates
#[get("/ui-services/v1/templates")]
async fn get_templates (
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || template_actions::find_templates(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


/// Get a template by its id
#[get("/ui-services/v1/templates/{obj_id}")]
async fn get_template (
    pool: web::Data<DbPool>,
    obj_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {

    let obj_id = obj_id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || template_actions::find_template(obj_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(results))
}


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
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

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
            .service(add_categories)
            .service(get_channels)
            .service(add_channels)
            .service(get_languages)
            .service(add_languages)
            .service(get_corrs)
            .service(add_corrs)
            .service(get_category_corr_mappings)
            .service(add_category_corr_mappings)
            .service(get_unmapped_category_corr_mappings)
            .service(get_template)
            .service(get_templates)
            .service(add_templates)
    })
    .bind(&bind)?
    .run()
    .await
}
