use crate::{
    app_errors::MyError, 
    models::{ClientPreference, ClientPreferenceAPI, ClientPreferenceQuery, Correspondence, ClientPrefChannelConfig, Category},
    db::client_pref_channel_config,
};
use deadpool_postgres::Client;

//***************************** Client Preferences ****************************************
// Get a list of client preferences for category to correspondence mappings given a category id
pub async fn get_client_preferences(client: &Client, client_prefs_query: ClientPreferenceQuery,) -> Result<Vec<ClientPreference>, MyError> {
    
    let new_client_prefs = Vec::<ClientPreference>::new();

//     let _stmt = include_str!("../../sql/client_preference/get_client_preference_mapped.sql");
//     let stmt = client.prepare(&_stmt).await.unwrap();

//     let category_maps = client
//         .query(&stmt, &[&category_id], )
//         .await?
//         .iter()
//         .map(|row| 
//             // TODO look into creating the trait from_row_ref, on the struct
//             ClientPreference {
//                 client_preference_id: row.get("client_preference_id"),
//                 category: Category {
//                     category_id: row.get("category_id"),
//                     category_name: row.get("category_name"),
//                 },
//                 correspondence: Correspondence {
//                     correspondence_id: row.get("correspondence_id"),
//                     correspondence_name: row.get("correspondence_name")
//                 },
//                 opt_out: row.get("opt_out"),
//                 retention_period: row.get("retention_period"),
//                 channel_config: Vec::<ChannelConfig>::new(),
//             }
//         )
//         .collect::<Vec<ClientPreference>>();

//     // Get the channel configs for each client preference
//     let mut new_maps = Vec::<ClientPreference>::new();
//     for mut item in category_maps {
//         item.channel_config = channel_config::get_channel_configs(&client, item.client_preference_id.clone()).await?;
//         new_maps.push(item);
//     }

    Ok(new_client_prefs)
}


pub async fn upsert_client_preferences(
    client: &Client,
    client_pref_api: ClientPreferenceAPI,
) -> Result<Vec<ClientPreference>, MyError> {

    // Create the insert and update statements
    let _insert_stmt = include_str!("../../sql/client_preference/insert_client_preference.sql");
    let insert_stmt = client.prepare(&_insert_stmt).await.unwrap();

    let _update_stmt = include_str!("../../sql/client_preference/update_client_preference.sql");
    let update_stmt = client.prepare(&_update_stmt).await.unwrap();

    // TODO. Fix this if possible. Not sure how to do a bulk upsert with this framework so 
    // iterate through individual inserts
    for item in client_pref_api.client_preferences {

        if item.client_preference_id == -1 {
            // Insert client preference
            let new_id = client.query(&insert_stmt, &[&item.category.category_id, 
                &item.correspondence.correspondence_id, &item.selected_opt_out, &item.selected_retention_period,
                &item.developer, &item.project, &item.lender],).await.unwrap()[0].get("client_preference_id");
            
            // Build a vec of channel_config and insert the client_preference_id of the new entry
            let mut new_channel_configs = Vec::<ClientPrefChannelConfig>::new();
            for cfg in &item.client_pref_channel_config {
                let mut new_cfg = cfg.clone();
                new_cfg.client_preference_id = new_id;
                new_channel_configs.push(new_cfg)
            }

            // Upsert client pref channel configs 
            client_pref_channel_config::upsert_client_pref_channel_configs(client, &new_channel_configs).await.unwrap();
        } else {
            client.query(&update_stmt, &[&item.category.category_id, 
                &item.correspondence.correspondence_id, &item.selected_opt_out, &item.selected_retention_period,
                &item.developer, &item.project, &item.lender, &item.client_preference_id]).await.unwrap();

            // Upsert client pref channel configs 
            client_pref_channel_config::upsert_client_pref_channel_configs(client, &item.client_pref_channel_config).await.unwrap();
        }
    }

    // Return the list of client preferences with all of the upserts included
    Ok(get_client_preferences(&client, client_pref_api.client_pref_query).await.unwrap())
}


// pub async fn delete_client_preferences(
//     client: &Client,
//     cat_map_id: i32,
// ) -> Result<Vec<ClientPreference>, MyError> {

//     // Create the delete statement
//     let _delete_stmt = include_str!("../../sql/client_preference/delete_client_preference.sql");
//     let delete_stmt = client.prepare(&_delete_stmt).await.unwrap();

//     // TODO. Fix this if possible. Not sure how to do a bulk delete with this framework so 
//     // iterate through individual ones
//     let cat_id = client.query(&delete_stmt, &[&cat_map_id,]).await.unwrap()[0].get("category_id");

//     Ok(get_mapped_categories(&client, cat_id).await.unwrap())
// }

