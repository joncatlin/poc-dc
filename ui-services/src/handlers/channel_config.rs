use crate::{db, app_errors::MyError, models::{ChannelConfig}};
use actix_web::{get, put, delete, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};


// ***************************** Channels ****************************************
// #[get("/ui-services/v1/channel_configs")]
// pub async fn get_channel_configs(
//     db_pool: web::Data<Pool>,
// ) -> Result<HttpResponse, Error> {
//     let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

//     let channel_configs = db::channel::get_channel_configs(&client).await?;

//     Ok(HttpResponse::Ok().json(channel_configs))
// }

/// Get a list of all the channel_configs that have been defined.
#[get("/ui-services/v1/channel_configs/{category_mapping_id}")]
async fn get_channel_configs(
    pool: web::Data<Pool>,
    category_mapping_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {

    let category_mapping_id = category_mapping_id.into_inner();
    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    let channel = db::channel_config::get_channel_configs(&client, category_mapping_id).await?;

    Ok(HttpResponse::Ok().json(channel))
}


/// Create or update channel_configs given an array of channel_configs
#[put("/ui-services/v1/channel_configs")]
async fn upsert_channel_configs(
    pool: web::Data<Pool>,
    channel_configs: web::Json<Vec<ChannelConfig>>,
) -> Result<HttpResponse, Error> {

    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    let channel_configs = db::channel_config::upsert_channel_configs(&client, &channel_configs).await?;

    Ok(HttpResponse::Ok().json(channel_configs))
}


/// Delete channel_configs given an array of channel_configs
#[delete("/ui-services/v1/channel_configs")]
async fn delete_channel_configs(
    pool: web::Data<Pool>,
    channel_configs: web::Json<Vec<ChannelConfig>>,
) -> Result<HttpResponse, Error> {

    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    let channel_configs = db::channel_config::delete_channel_configs(&client, &channel_configs).await?;

    Ok(HttpResponse::Ok().json(channel_configs))
}


