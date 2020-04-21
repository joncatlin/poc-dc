use crate::{app_errors::MyError, models::{Language}};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

// ***************************** Languages ****************************************
pub async fn get_languages(client: &Client) -> Result<Vec<Language>, MyError> {
    let _stmt = include_str!("../../sql/language/get_language.sql");
    let _stmt = _stmt.replace("$table_fields", &Language::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let languages = client
        .query(&stmt, &[], )
        .await?
        .iter()
        .map(|row| Language::from_row_ref(row).unwrap())
        .collect::<Vec<Language>>();

        Ok(languages)
}

pub async fn upsert_languages(
    client: &Client,
    upsert_list: &Vec<Language>,
) -> Result<Vec<Language>, MyError> {

    // Create the insert and update statements
    let _insert_stmt = include_str!("../../sql/language/insert_language.sql");
    let insert_stmt = client.prepare(&_insert_stmt).await.unwrap();

    let _update_stmt = include_str!("../../sql/language/update_language.sql");
    let update_stmt = client.prepare(&_update_stmt).await.unwrap();

    // TODO. Fix this if possible. Not sure how to do a bulk upsert with this framework so 
    // iterate through individual inserts
    for item in upsert_list {
        if item.language_id == -1 {
            client.query(&insert_stmt, &[&item.language_name,]).await.unwrap();
        } else {
            client.query(&update_stmt, &[&item.language_name, &item.language_id]).await.unwrap();
        }
    }

    Ok(get_languages(&client).await.unwrap())
}

pub async fn delete_languages(
    client: &Client,
    delete_list: &Vec<Language>,
) -> Result<Vec<Language>, MyError> {

    // Create the delete statement
    let _delete_stmt = include_str!("../../sql/language/delete_language.sql");
    let delete_stmt = client.prepare(&_delete_stmt).await.unwrap();

    // TODO. Fix this if possible. Not sure how to do a bulk delete with this framework so 
    // iterate through individual ones
    for item in delete_list {
        client.query(&delete_stmt, &[&item.language_id,]).await.unwrap();
    }

    Ok(get_languages(&client).await.unwrap())
}




