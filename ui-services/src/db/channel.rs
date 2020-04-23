use crate::{app_errors::MyError, models::{Channel}};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

// ***************************** Channels ****************************************
pub async fn get_channels(client: &Client) -> Result<Vec<Channel>, MyError> {
    let _stmt = include_str!("../../sql/channel/get_channel.sql");
    let _stmt = _stmt.replace("$table_fields", &Channel::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let channels = client
        .query(&stmt, &[], )
        .await?
        .iter()
        .map(|row| Channel::from_row_ref(row).unwrap())
        .collect::<Vec<Channel>>();

        Ok(channels)
}

pub async fn get_channel_by_name(client: &Client, name: &String) -> Result<Channel, MyError> {
    let _stmt = include_str!("../../sql/channel/get_channel_by_name.sql");
    let _stmt = _stmt.replace("$table_fields", &Channel::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let channel = client
        .query(&stmt, &[name], )
        .await?
        .iter()
        .map(|row| Channel::from_row_ref(row).unwrap())
        .collect::<Vec<Channel>>()
        .pop()
        .unwrap();

        Ok(channel)
}

pub async fn upsert_channels(
    client: &Client,
    upsert_list: &Vec<Channel>,
) -> Result<Vec<Channel>, MyError> {

    // Create the insert and update statements
    let _insert_stmt = include_str!("../../sql/channel/insert_channel.sql");
    let insert_stmt = client.prepare(&_insert_stmt).await.unwrap();

    let _update_stmt = include_str!("../../sql/channel/update_channel.sql");
    let update_stmt = client.prepare(&_update_stmt).await.unwrap();

    let _insert_channel_config_stmt = include_str!("../../sql/channel_config/insert_channel_config_from_new_channel.sql");
    let insert_channel_config_stmt = client.prepare(&_insert_channel_config_stmt).await.unwrap();

    let _insert_cp_channel_config_stmt = include_str!("../../sql/client_pref_channel_config/insert_cp_channel_config_from_new_channel.sql");
    let insert_cp_channel_config_stmt = client.prepare(&_insert_cp_channel_config_stmt).await.unwrap();

    // TODO. Fix this if possible. Not sure how to do a bulk upsert with this framework so 
    // iterate through individual inserts
    for item in upsert_list {
        if item.channel_id == -1 {
            let new_id: i32 = client.query(&insert_stmt, &[&item.channel_name,]).await.unwrap()[0].get("channel_id");

            // When inserting a new channel, all existing category mappings and client preferences need a 
            // channel_config created for them, that references the new channel
            client.query(&insert_channel_config_stmt, &[&new_id,]).await.unwrap();
            client.query(&insert_cp_channel_config_stmt, &[&new_id,]).await.unwrap();

        } else {
            client.query(&update_stmt, &[&item.channel_name, &item.channel_id]).await.unwrap();
        }
    }

    Ok(get_channels(&client).await.unwrap())
}

pub async fn delete_channels(
    client: &Client,
    delete_list: &Vec<Channel>,
) -> Result<Vec<Channel>, MyError> {

    // Create the delete statement
    let _delete_stmt = include_str!("../../sql/channel/delete_channel.sql");
    let delete_stmt = client.prepare(&_delete_stmt).await.unwrap();

    // TODO. Fix this if possible. Not sure how to do a bulk delete with this framework so 
    // iterate through individual ones
    for item in delete_list {
        client.query(&delete_stmt, &[&item.channel_id,]).await.unwrap();
    }

    Ok(get_channels(&client).await.unwrap())
}




