use crate::{db, app_errors::MyError, models::{Correspondence}};
use actix_web::{get, put, delete, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};


// ***************************** Correspondences ****************************************
#[get("/ui-services/v1/correspondences")]
pub async fn get_correspondences(
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let correspondences = db::correspondence::get_correspondences(&client).await?;

    Ok(HttpResponse::Ok().json(correspondences))
}

/// Create or update correspondences given an array of correspondences
#[put("/ui-services/v1/correspondences")]
async fn upsert_correspondences(
    pool: web::Data<Pool>,
    correspondences: web::Json<Vec<Correspondence>>,
) -> Result<HttpResponse, Error> {

    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    let correspondences = db::correspondence::upsert_correspondences(&client, &correspondences).await?;

    Ok(HttpResponse::Ok().json(correspondences))
}


/// Delete correspondences given an array of correspondences
#[delete("/ui-services/v1/correspondences")]
async fn delete_correspondences(
    pool: web::Data<Pool>,
    correspondences: web::Json<Vec<Correspondence>>,
) -> Result<HttpResponse, Error> {

    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    let correspondences = db::correspondence::delete_correspondences(&client, &correspondences).await?;

    Ok(HttpResponse::Ok().json(correspondences))
}

