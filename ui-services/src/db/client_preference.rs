use crate::{
    app_errors::MyError, 
    models::{ClientPreference, ClientPreferenceQuery, 
        Correspondence, ClientPrefChannelConfig, Category, Channel, },
    db::client_pref_channel_config,
};
use deadpool_postgres::Client;

//***************************** Client Preferences ****************************************
// Get a list of client preferences for category to correspondence mappings given a category id
pub async fn get_client_preferences(client: &Client, 
    client_prefs_query: &ClientPreferenceQuery,
) -> Result<Vec<ClientPreference>, MyError> {
    
    // Create the variable where clause depending on inputs
    let where_clause: String = build_sql_where_clause (&client_prefs_query.dpl.developer, 
        &client_prefs_query.dpl.project, &client_prefs_query.dpl.lender,
        client_prefs_query.category.category_id, 
//        client_prefs_query.correspondence.iter().map(|x| x.correspondence_id).collect(),
        &client_prefs_query.correspondence,
    );

    // Create a temporary table that holds the DPL for the main query
    let _stmt = include_str!("../../sql/client_preference/get_client_preference_mapped.sql");
    let _stmt = _stmt.replace("$where_clause", &*where_clause);

    // Prepare the statement for execution
    let stmt = client.prepare(&_stmt).await.unwrap();

    // Retrieve the data and build the return structure
    let mut new_client_prefs = Vec::<ClientPreference>::new();

    let mut cur_cp_id = -2;

    let mut first_time = true;
    let mut index = 0;

    // TODO fix this. There should be a better way of doing this
    // Initialize an empty struct which is not going to be used but keeps the compiler from complaining
//    let mut cur_cp_struct = &empty_client_pref();
    
    let rows = client.query(&stmt, &[], ).await.unwrap();
    for row in rows {

        // See if the row is for the existing client preference struct or is a new one needed
        let row_cp_id = row.get("client_preference_id");
        if row_cp_id != cur_cp_id {

            // Create a new cp struct and add it to the vector
            let tmp = ClientPreference {
                client_preference_id: row_cp_id,
                category: Category {
                    category_id: row.get("category_id"),
                    category_name: row.get("category_name"),
                },
                correspondence: Correspondence {
                    correspondence_name: row.get("correspondence_name"),
                    correspondence_id: row.get("correspondence_id"),
                },
                opt_out: row.get("opt_out"),
                selected_opt_out: row.get("selected_opt_out"),
                retention_period: row.get("retention_period"),
                selected_retention_period: row.get("selected_retention_period"),
                developer: row.get("developer"),
                project: row.get("project"),
                lender: row.get("lender"),
                client_pref_channel_config: Vec::<ClientPrefChannelConfig>::new(),
            };
            cur_cp_id = row_cp_id;

            // Add the new one to the list to return
            new_client_prefs.push(tmp);

            if first_time { 
                first_time = false;
            } else {
                index += 1;
            }

        } 
        
        // Add client pref channel config details to existing cp struct
        new_client_prefs[index].client_pref_channel_config.push(ClientPrefChannelConfig {
            client_pref_channel_config_id: row.get("client_pref_channel_config_id"),
            client_preference_id: row.get("client_preference_id"),
            channel: Channel {
                channel_id: row.get("channel_id"),
                channel_name: row.get("channel_name"),
            },
            permitted: row.get("permitted"),
            selected_permitted: row.get("selected_permitted"),
        });
    }

    Ok(new_client_prefs)
}

fn build_sql_where_clause (developer: &String, project: &String, lender: &String,
    category_id: i32, correspondence_ids: &Vec<i32>) -> String {

    let mut clause = String::new();

    clause.push_str(&*format!(" WHERE cp.developer = '{}' ", developer));
    if *project != "".to_string() {
        clause.push_str(&*format!(" AND cp.project = '{}' ", project));
    }

    if *lender != "".to_string() {
        clause.push_str(&*format!(" AND cp.lender = '{}' ", lender));
    }

    clause.push_str(&*format!(" AND cm.category_id = {} ", category_id));

    if correspondence_ids.len() != 0 {

//        let mut in_clause = correspondence_ids.into_iter().map(|x| format!("{},", x)).collect::<String>();
        let mut in_clause = correspondence_ids.into_iter().map(|x| format!("{},", x)).collect::<String>();
        in_clause.truncate(in_clause.len()-1);
        clause.push_str(&*format!(" AND cm.correspondence_id IN ({}) ", in_clause.to_string()));
    }

    clause
}





pub async fn upsert_client_preferences(
    client: &Client,
    cps: &Vec<ClientPreference>,
) -> Result<(), MyError> {

    // Create the insert and update statements
    let _insert_stmt = include_str!("../../sql/client_preference/insert_client_preference.sql");
    let insert_stmt = client.prepare(&_insert_stmt).await.unwrap();

    let _update_stmt = include_str!("../../sql/client_preference/update_client_preference.sql");
    let update_stmt = client.prepare(&_update_stmt).await.unwrap();

    // TODO. Fix this if possible. Not sure how to do a bulk upsert with this framework so 
    // iterate through individual inserts
    for item in cps {

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

    Ok(())
}


pub async fn delete_client_preferences(
    client: &Client,
    cp: &ClientPreference,
) -> Result<(), MyError> {

    // Create the delete statement
    let _delete_stmt = include_str!("../../sql/client_preference/delete_client_preference.sql");
    let delete_stmt = client.prepare(&_delete_stmt).await.unwrap();

//    let cat_id = client.query(&delete_stmt, &[&cp_delete.client_preference_id,]).await.unwrap()[0].get("category_id");
    client.query(&delete_stmt, &[&cp.client_preference_id,]).await.unwrap();

    Ok(())
}































// Saved before modification to inputs and outputs to these functions
// pub async fn upsert_client_preferences(
//     client: &Client,
//     client_pref_api: &ClientPreferenceAPI,
// ) -> Result<Vec<ClientPreference>, MyError> {

//     // Create the insert and update statements
//     let _insert_stmt = include_str!("../../sql/client_preference/insert_client_preference.sql");
//     let insert_stmt = client.prepare(&_insert_stmt).await.unwrap();

//     let _update_stmt = include_str!("../../sql/client_preference/update_client_preference.sql");
//     let update_stmt = client.prepare(&_update_stmt).await.unwrap();

//     // TODO. Fix this if possible. Not sure how to do a bulk upsert with this framework so 
//     // iterate through individual inserts
//     for item in &client_pref_api.client_preferences {

//         if item.client_preference_id == -1 {
//             // Insert client preference
//             let new_id = client.query(&insert_stmt, &[&item.category.category_id, 
//                 &item.correspondence.correspondence_id, &item.selected_opt_out, &item.selected_retention_period,
//                 &item.developer, &item.project, &item.lender],).await.unwrap()[0].get("client_preference_id");
            
//             // Build a vec of channel_config and insert the client_preference_id of the new entry
//             let mut new_channel_configs = Vec::<ClientPrefChannelConfig>::new();
//             for cfg in &item.client_pref_channel_config {
//                 let mut new_cfg = cfg.clone();
//                 new_cfg.client_preference_id = new_id;
//                 new_channel_configs.push(new_cfg)
//             }

//             // Upsert client pref channel configs 
//             client_pref_channel_config::upsert_client_pref_channel_configs(client, &new_channel_configs).await.unwrap();
//         } else {
//             client.query(&update_stmt, &[&item.category.category_id, 
//                 &item.correspondence.correspondence_id, &item.selected_opt_out, &item.selected_retention_period,
//                 &item.developer, &item.project, &item.lender, &item.client_preference_id]).await.unwrap();

//             // Upsert client pref channel configs 
//             client_pref_channel_config::upsert_client_pref_channel_configs(client, &item.client_pref_channel_config).await.unwrap();
//         }
//     }

//     // Return the list of client preferences with all of the upserts included
//     Ok(get_client_preferences(&client, &client_pref_api.client_pref_query).await.unwrap())
// }


// pub async fn delete_client_preferences(
//     client: &Client,
//     cp_delete: &ClientPreferenceDelete,
// ) -> Result<Vec<ClientPreference>, MyError> {

//     // Create the delete statement
//     let _delete_stmt = include_str!("../../sql/client_preference/delete_client_preference.sql");
//     let delete_stmt = client.prepare(&_delete_stmt).await.unwrap();

// //    let cat_id = client.query(&delete_stmt, &[&cp_delete.client_preference_id,]).await.unwrap()[0].get("category_id");
//     client.query(&delete_stmt, &[&cp_delete.client_preference_id,]).await.unwrap();

//     Ok(get_client_preferences(&client, &cp_delete.client_pref_query).await.unwrap())
// }
