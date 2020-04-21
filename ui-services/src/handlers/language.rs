use crate::{db, app_errors::MyError, models::{Language}};
use actix_web::{get, put, delete, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};


// ***************************** Languages ****************************************
#[get("/ui-services/v1/languages")]
pub async fn get_languages(
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let languages = db::language::get_languages(&client).await?;

    Ok(HttpResponse::Ok().json(languages))
}

/// Create or update languages given an array of languages
#[put("/ui-services/v1/languages")]
async fn upsert_languages(
    pool: web::Data<Pool>,
    languages: web::Json<Vec<Language>>,
) -> Result<HttpResponse, Error> {

    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    let languages = db::language::upsert_languages(&client, &languages).await?;

    Ok(HttpResponse::Ok().json(languages))
}


/// Delete languages given an array of languages
#[delete("/ui-services/v1/languages")]
async fn delete_languages(
    pool: web::Data<Pool>,
    languages: web::Json<Vec<Language>>,
) -> Result<HttpResponse, Error> {

    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    let languages = db::language::delete_languages(&client, &languages).await?;

    Ok(HttpResponse::Ok().json(languages))
}

