use crate::{db, app_errors::MyError, models::{Channel}};
use actix_web::{get, put, delete, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};


// ***************************** Channels ****************************************
#[get("/ui-services/v1/channels")]
pub async fn get_channels(
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let channels = db::channel::get_channels(&client).await?;

    Ok(HttpResponse::Ok().json(channels))
}

/// Get a list of all the channels that have been defined.
#[get("/ui-services/v1/channels/{chan_name}")]
async fn get_channel_by_name(
    pool: web::Data<Pool>,
    name: web::Path<String>,
) -> Result<HttpResponse, Error> {

    let name = name.into_inner();
    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    let channel = db::channel::get_channel_by_name(&client, &name).await?;

    Ok(HttpResponse::Ok().json(channel))
}


/// Create or update channels given an array of channels
#[put("/ui-services/v1/channels")]
async fn upsert_channels(
    pool: web::Data<Pool>,
    channels: web::Json<Vec<Channel>>,
) -> Result<HttpResponse, Error> {

    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    let channels = db::channel::upsert_channels(&client, &channels).await?;

    Ok(HttpResponse::Ok().json(channels))
}


/// Delete channels given an array of channels
#[delete("/ui-services/v1/channels")]
async fn delete_channels(
    pool: web::Data<Pool>,
    channels: web::Json<Vec<Channel>>,
) -> Result<HttpResponse, Error> {

    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    let channels = db::channel::delete_channels(&client, &channels).await?;

    Ok(HttpResponse::Ok().json(channels))
}


// /// Delete a list of channels
// #[delete("/ui-services/v1/channels")]
// async fn delete_channels(
//     pool: web::Data<DbPool>,
//     cats: web::Json<Vec<models::Channel>>,
// ) -> Result<HttpResponse, Error> {
//     let conn = pool.get().expect("couldn't get db connection from pool");

//     // use web::block to offload blocking Diesel code without blocking server thread
//     let results = web::block(move || channel_actions::delete_existing_channels(&cats, &conn))
//         .await
//         .map_err(|e| {
//             eprintln!("{}", e);
//             HttpResponse::InternalServerError().finish()
//         })?;

//     Ok(HttpResponse::Ok().json(results))
// }


