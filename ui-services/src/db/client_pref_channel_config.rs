use crate::{app_errors::MyError, models::{ClientPrefChannelConfig, Channel}};
use deadpool_postgres::Client;

// // ***************************** ClientPrefChannelConfigs ****************************************
pub async fn get_client_pref_channel_configs(client: &Client, client_preference_id: i32) -> Result<Vec<ClientPrefChannelConfig>, MyError> {
    let _stmt = include_str!("../../sql/client_pref_channel_config/get_client_pref_channel_config.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    let client_pref_channel_configs = client
        .query(&stmt, &[&client_preference_id], )
        .await?
        .iter()
        .map(|row| 
            ClientPrefChannelConfig {
                client_pref_channel_config_id: row.get("client_pref_channel_config_id"),
                client_preference_id: row.get("client_preference_id"),
                channel: Channel {
                    channel_id: row.get("channel_id"),
                    channel_name: row.get("channel_name"),
                },
                selected_permitted: row.get("selected_permitted"),
                permitted: row.get("permitted"),
            }
        )
        .collect::<Vec<ClientPrefChannelConfig>>();

    Ok(client_pref_channel_configs)
}

pub async fn upsert_client_pref_channel_configs(
    client: &Client,
    upsert_list: &Vec<ClientPrefChannelConfig>,
) -> Result<Vec<ClientPrefChannelConfig>, MyError> {

//     // Create the insert and update statements
//     let _insert_stmt = include_str!("../../sql/client_pref_channel_config/insert_channel_config.sql");
//     let insert_stmt = client.prepare(&_insert_stmt).await.unwrap();

//     let _update_stmt = include_str!("../../sql/client_pref_channel_config/update_channel_config.sql");
//     let update_stmt = client.prepare(&_update_stmt).await.unwrap();

//     // TODO. Fix this if possible. Not sure how to do a bulk upsert with this framework so 
//     // iterate through individual inserts
//     for item in upsert_list {
//         if item.channel_config_id == -1 {
//             client.query(&insert_stmt, &[&item.client_preference_id, &item.channel.channel_id, &item.permitted,]).await.unwrap();
//         } else {
//             client.query(&update_stmt, &[&item.client_preference_id, &item.channel.channel_id, &item.permitted, &item.channel_config_id]).await.unwrap();
//         }
//     }

    Ok(get_client_pref_channel_configs(&client, upsert_list[0].client_preference_id).await.unwrap())
}

// pub async fn delete_client pref_channel_configs(
//     client: &Client,
//     delete_list: &Vec<ClientPrefChannelConfig>,
// ) -> Result<Vec<ClientPrefChannelConfig>, MyError> {

//     // Create the delete statement
//     let _delete_stmt = include_str!("../../sql/client_pref_channel_config/delete_channel_config.sql");
//     let delete_stmt = client.prepare(&_delete_stmt).await.unwrap();

//     // TODO. Fix this if possible. Not sure how to do a bulk delete with this framework so 
//     // iterate through individual ones
//     for item in delete_list {
//         client.query(&delete_stmt, &[&item.channel_config_id,]).await.unwrap();
//     }

//     Ok(get_client pref_channel_configs(&client, delete_list[0].client_preference_id).await.unwrap())
// }




