use crate::{app_errors::MyError, models::{Category}};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

// ***************************** Categorys ****************************************
pub async fn get_categories(client: &Client) -> Result<Vec<Category>, MyError> {
    let _stmt = include_str!("../../sql/category/get_category.sql");
    let _stmt = _stmt.replace("$table_fields", &Category::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let categories = client
        .query(&stmt, &[], )
        .await?
        .iter()
        .map(|row| Category::from_row_ref(row).unwrap())
        .collect::<Vec<Category>>();

        Ok(categories)
}

pub async fn upsert_categories(
    client: &Client,
    upsert_list: &Vec<Category>,
) -> Result<Vec<Category>, MyError> {

    // Create the insert and update statements
    let _insert_stmt = include_str!("../../sql/category/insert_category.sql");
    // let _insert_stmt = _insert_stmt.replace("$table_fields", &Category::sql_table_fields());
    let insert_stmt = client.prepare(&_insert_stmt).await.unwrap();

    let _update_stmt = include_str!("../../sql/category/update_category.sql");
    // let _update_stmt = _update_stmt.replace("$table_fields", &Category::sql_table_fields());
    let update_stmt = client.prepare(&_update_stmt).await.unwrap();

    // TODO. Fix this if possible. Not sure how to do a bulk upsert with this framework so 
    // iterate through individual inserts
    for item in upsert_list {
        if item.category_id == -1 {
            client.query(&insert_stmt, &[&item.category_name,]).await.unwrap();
        } else {
            client.query(&update_stmt, &[&item.category_name, &item.category_id]).await.unwrap();
        }
    }

    Ok(get_categories(&client).await.unwrap())
}

pub async fn delete_categories(
    client: &Client,
    delete_list: &Vec<Category>,
) -> Result<Vec<Category>, MyError> {

    // Create the delete statement
    let _delete_stmt = include_str!("../../sql/category/delete_category.sql");
    let delete_stmt = client.prepare(&_delete_stmt).await.unwrap();

    // TODO. Fix this if possible. Not sure how to do a bulk delete with this framework so 
    // iterate through individual ones
    for item in delete_list {
        client.query(&delete_stmt, &[&item.category_id,]).await.unwrap();
    }

    Ok(get_categories(&client).await.unwrap())
}




