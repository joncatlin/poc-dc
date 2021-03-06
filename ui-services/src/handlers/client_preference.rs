use crate::{db, app_errors::MyError, models::{ClientPreferenceQuery, ClientPreference}};
use actix_web::{post, put, delete, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

// ***************************** Client Preferences ****************************************
#[post("/ui-services/v1/client-preference-mapping/mapped")]
pub async fn get_client_preferences(
    db_pool: web::Data<Pool>,
    client_pref_query: web::Json<ClientPreferenceQuery>,
) -> Result<HttpResponse, Error> {

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let client_prefs = db::client_preference::get_client_preferences(&client, &client_pref_query).await?;

    Ok(HttpResponse::Ok().json(client_prefs))
}


/// Create or update category_mappings given an array of category_mappings
#[put("/ui-services/v1/client-preference-mapping/mapped")]
async fn upsert_client_preferences(
    pool: web::Data<Pool>,
    client_pref_api: web::Json<Vec<ClientPreference>>,
) -> Result<HttpResponse, Error> {

    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    db::client_preference::upsert_client_preferences(&client, &client_pref_api).await?;

    Ok(HttpResponse::Ok().into())
}


/// Delete category_mappings given an array of category_mappings
#[delete("/ui-services/v1/client-preference-mapping/mapped")]
async fn delete_client_preferences(
    pool: web::Data<Pool>,
    client_pref_delete: web::Json<ClientPreference>,
) -> Result<HttpResponse, Error> {

    let client: Client = pool.get().await.map_err(MyError::PoolError)?;

    db::client_preference::delete_client_preferences(&client, &client_pref_delete).await?;

    Ok(HttpResponse::Ok().into())
}