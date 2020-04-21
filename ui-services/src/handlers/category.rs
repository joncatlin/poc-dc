use crate::{db, app_errors::MyError, models::{Category}};
use actix_web::{get, put, delete, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};


// ***************************** Categorys ****************************************
#[get("/ui-services/v1/categories")]
pub async fn get_categories(
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let categories = db::category::get_categories(&client).await?;

    Ok(HttpResponse::Ok().json(categories))
}

/// Create or update categories given an array of categories
#[put("/ui-services/v1/categories")]
async fn upsert_categories(
    pool: web::Data<Pool>,
    categories: web::Json<Vec<Category>>,
) -> Result<HttpResponse, Error> {

    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    let categories = db::category::upsert_categories(&client, &categories).await?;

    Ok(HttpResponse::Ok().json(categories))
}


/// Delete categories given an array of categories
#[delete("/ui-services/v1/categories")]
async fn delete_categories(
    pool: web::Data<Pool>,
    categories: web::Json<Vec<Category>>,
) -> Result<HttpResponse, Error> {

    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    let categories = db::category::delete_categories(&client, &categories).await?;

    Ok(HttpResponse::Ok().json(categories))
}

