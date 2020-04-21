use crate::{db, app_errors::MyError, models::{CategoryMapping}};
use actix_web::{get, put, delete, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

// #[get("/ui-services/v1/category-correspondence-mappings/unmapped")]
// #[put("/ui-services/v1/category-correspondence-mappings/mapped")]










// ***************************** Categorys ****************************************
#[get("/ui-services/v1/category-correspondence-mappings/mapped/{cat_id}")]
pub async fn get_mapped_categories(db_pool: web::Data<Pool>, cat_id: web::Path<i32>,) 
    -> Result<HttpResponse, Error> {

    let cat_id = cat_id.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let category_maps = db::category_mapping::get_mapped_categories(&client, cat_id).await?;

    Ok(HttpResponse::Ok().json(category_maps))
}

#[get("/ui-services/v1/category-correspondence-mappings/unmapped")]
pub async fn get_unmapped_correspondences(db_pool: web::Data<Pool>,) 
    -> Result<HttpResponse, Error> {

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let correspondences = db::category_mapping::get_unmapped_correspondences(&client).await?;

    Ok(HttpResponse::Ok().json(correspondences))
}

/// Create or update category_mappings given an array of category_mappings
#[put("/ui-services/v1/category-correspondence-mappings/mapped")]
async fn upsert_category_mappings(
    pool: web::Data<Pool>,
    category_mappings: web::Json<Vec<CategoryMapping>>,
) -> Result<HttpResponse, Error> {

    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    let category_mappings = db::category_mapping::upsert_category_mappings(&client, &category_mappings).await?;

    Ok(HttpResponse::Ok().json(category_mappings))
}


/// Delete category_mappings given an array of category_mappings
#[delete("/ui-services/v1/category-correspondence-mappings/mapped/{cat_map_id}")]
async fn delete_category_mappings(
    pool: web::Data<Pool>,
    cat_map_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {

    let cat_map_id = cat_map_id.into_inner();
    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    let category_mappings = db::category_mapping::delete_category_mappings(&client, cat_map_id).await?;

    Ok(HttpResponse::Ok().json(category_mappings))
}