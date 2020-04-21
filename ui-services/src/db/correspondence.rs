use crate::{app_errors::MyError, models::{Correspondence}};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

// ***************************** Correspondences ****************************************
pub async fn get_correspondences(client: &Client) -> Result<Vec<Correspondence>, MyError> {
    let _stmt = include_str!("../../sql/correspondence/get_correspondence.sql");
    let _stmt = _stmt.replace("$table_fields", &Correspondence::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let correspondences = client
        .query(&stmt, &[], )
        .await?
        .iter()
        .map(|row| Correspondence::from_row_ref(row).unwrap())
        .collect::<Vec<Correspondence>>();

        Ok(correspondences)
}

pub async fn upsert_correspondences(
    client: &Client,
    upsert_list: &Vec<Correspondence>,
) -> Result<Vec<Correspondence>, MyError> {

    // Create the insert and update statements
    let _insert_stmt = include_str!("../../sql/correspondence/insert_correspondence.sql");
    // let _insert_stmt = _insert_stmt.replace("$table_fields", &Correspondence::sql_table_fields());
    let insert_stmt = client.prepare(&_insert_stmt).await.unwrap();

    let _update_stmt = include_str!("../../sql/correspondence/update_correspondence.sql");
    // let _update_stmt = _update_stmt.replace("$table_fields", &Correspondence::sql_table_fields());
    let update_stmt = client.prepare(&_update_stmt).await.unwrap();

    // TODO. Fix this if possible. Not sure how to do a bulk upsert with this framework so 
    // iterate through individual inserts
    for item in upsert_list {
        if item.correspondence_id == -1 {
            client.query(&insert_stmt, &[&item.correspondence_name,]).await.unwrap();
        } else {
            client.query(&update_stmt, &[&item.correspondence_name, &item.correspondence_id]).await.unwrap();
        }
    }

    Ok(get_correspondences(&client).await.unwrap())
}

pub async fn delete_correspondences(
    client: &Client,
    delete_list: &Vec<Correspondence>,
) -> Result<Vec<Correspondence>, MyError> {

    // Create the delete statement
    let _delete_stmt = include_str!("../../sql/correspondence/delete_correspondence.sql");
    let delete_stmt = client.prepare(&_delete_stmt).await.unwrap();

    // TODO. Fix this if possible. Not sure how to do a bulk delete with this framework so 
    // iterate through individual ones
    for item in delete_list {
        client.query(&delete_stmt, &[&item.correspondence_id,]).await.unwrap();
    }

    Ok(get_correspondences(&client).await.unwrap())
}




