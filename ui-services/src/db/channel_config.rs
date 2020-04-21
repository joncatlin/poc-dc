use crate::{app_errors::MyError, models::{ChannelConfig, Channel}};
use deadpool_postgres::Client;
//use tokio_pg_mapper::FromTokioPostgresRow;

// // ***************************** ChannelConfigs ****************************************
pub async fn get_channel_configs(client: &Client, category_mapping_id: i32) -> Result<Vec<ChannelConfig>, MyError> {
    let _stmt = include_str!("../../sql/channel_config/get_channel_config.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    let channel_configs = client
        .query(&stmt, &[&category_mapping_id], )
        .await?
        .iter()
        .map(|row| 
            ChannelConfig {
                channel_config_id: row.get("channel_config_id"),
                category_mapping_id: row.get("category_mapping_id"),
                channel: Channel {
                    channel_id: row.get("channel_id"),
                    channel_name: row.get("channel_name"),
                },
                permitted: row.get("permitted"),
            }
        )
        .collect::<Vec<ChannelConfig>>();

    Ok(channel_configs)
}

pub async fn upsert_channel_configs(
    client: &Client,
    upsert_list: &Vec<ChannelConfig>,
) -> Result<Vec<ChannelConfig>, MyError> {

    // Create the insert and update statements
    let _insert_stmt = include_str!("../../sql/channel_config/insert_channel_config.sql");
    let insert_stmt = client.prepare(&_insert_stmt).await.unwrap();

    let _update_stmt = include_str!("../../sql/channel_config/update_channel_config.sql");
    let update_stmt = client.prepare(&_update_stmt).await.unwrap();

    // TODO. Fix this if possible. Not sure how to do a bulk upsert with this framework so 
    // iterate through individual inserts
    for item in upsert_list {
        if item.channel_config_id == -1 {
            client.query(&insert_stmt, &[&item.category_mapping_id, &item.channel.channel_id, &item.permitted,]).await.unwrap();
        } else {
            client.query(&update_stmt, &[&item.category_mapping_id, &item.channel.channel_id, &item.permitted, &item.channel_config_id]).await.unwrap();
        }
    }

    Ok(get_channel_configs(&client, upsert_list[0].category_mapping_id).await.unwrap())
}

pub async fn delete_channel_configs(
    client: &Client,
    delete_list: &Vec<ChannelConfig>,
) -> Result<Vec<ChannelConfig>, MyError> {

    // Create the delete statement
    let _delete_stmt = include_str!("../../sql/channel_config/delete_channel_config.sql");
    let delete_stmt = client.prepare(&_delete_stmt).await.unwrap();

    // TODO. Fix this if possible. Not sure how to do a bulk delete with this framework so 
    // iterate through individual ones
    for item in delete_list {
        client.query(&delete_stmt, &[&item.channel_config_id,]).await.unwrap();
    }

    Ok(get_channel_configs(&client, delete_list[0].category_mapping_id).await.unwrap())
}




