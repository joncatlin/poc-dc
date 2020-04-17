//use diesel::prelude::*;
//use diesel::sql_types::Integer;

use std::env;
//use postgres::{Client, NoTls};

use crate::models;
use std::vec::Vec;

// pub fn find_mapped_client_preferences (
//     query_prefs: &models::ClientPreferencesQuery, 
//     conn: &PgConnection,
// //) -> Result<Vec<models::ClientPreferencesWithChannelConfig>, diesel::result::Error> {
// ) -> Result<Vec<models::ClientPreferences>, diesel::result::Error> {

//     use diesel::sql_query;

//     let mut statement = "
//         SELECT 
//             cp.client_preferences_id,
//             cat.category_id,
//             cat.category_name,
//             corrs.correspondence_id,
//             corrs.correspondence_name,
//             cp.opt_out AS selected_opt_out,
//             cm.opt_out AS opt_out,
//             cp.retention_period AS selected_retention_period,
//             cp.retention_period AS retention_period,
//             cp.developer,
//             cp.project,
//             cp.lender
//         FROM client_preferences AS cp
//         INNER JOIN categories AS cat ON cp.category_id = cat.category_id
//         INNER JOIN corrs ON cp.correspondence_id = corrs.correspondence_id
//         INNER JOIN category_mappings AS cm ON 
//             cp.category_id = cm.category_id AND
//             cp.correspondence_id = cm.correspondence_id
//         WHERE 
//     ".to_string();

//     // TODO check this is not susceptible to SQL injection etc
//     // Construct the WHERE clause for Hierarchy to get the information needed
//     statement.push_str(&build_sql_where_clause(&query_prefs.hierarchys));
//     debug!("Statement produced is: {}", statement);

//     // Constrcut the IN clause on the stastement if there are categories or correspondencs
//     if query_prefs.categories.len > 0 {
//         statement.push_str(" AND category_id ");
//         statement.push_str(&build_sql_in_clause(&query_prefs.categories));
//     }
//     if query_prefs.correspondences.len > 0 {
//         statement.push_str(" AND correspondence_id ");
//         statement.push_str(&build_sql_in_clause(&query_prefs.correspondences));
//     }
    
//     // Get the mappings without the channel configs because cannot determine how to do that in Diesel. So split the getting of the
//     // structures into two parts, the mappings first and then the channel configs associated with each mapping
//     let client_preferences: Vec<models::ClientPreferences> = sql_query(statement.to_string())
//         .bind::<Integer, _>(query_prefs.category_id)
//         .get_results(conn)
//         .expect("Error loading client preferences");
// /*
//     // Copy the mappings found into the final struct and at the same time get the channel_configs for each client prefs found
//     let mut maps = Vec::<models::ClientPreferencesWithChannelConfig>::new();
//     for client_prefs_map in client_preferences {

//         // Get the channel configs for this category_mapping_id using a SQL SELECT rather than the struct way
//         // Lots of problems with trait errors
//         let configs = sql_query("
//             SELECT 
//                 cfg.channel_config_id,
//                 cfg.client_preferences_id,
//                 chan.channel_id,
//                 chan.channel_name,
//                 cfg.permitted
//             FROM client_pref_channel_configs AS cfg
//             RIGHT OUTER JOIN channels AS chan 
//             ON cfg.channel_id = chan.channel_id
//             WHERE cfg.client_preferences_id = $1
//             ORDER BY chan.channel_name
//         ")
//             .bind::<Integer, _>(client_prefs_map.client_preferences_id)
//             .load::<models::ChannelConfig>(conn)
//             .expect("Error loading channel configs");

//         // Create a new structure from the mappings data
//         let new_struct = models::ClientPreferencesWithChannelConfig {
//             client_preferences_id: client_prefs_map.client_preferences_id, 
//             category: client_prefs_map.category,
//             correspondence: client_prefs_map.correspondence,
//             opt_out: client_prefs_map.opt_out,
//             retention_period: client_prefs_map.retention_period,
//             channel_config: configs,
//             developer: client_prefs_map.developer,
//             project: client_prefs_map.project,
//             lender: client_prefs_map.lender,
//         };

//         // Add the struct to the vectors to be returned
//         maps.push(new_struct);
//     }

//     // Now get the category mappings to get the preferences from there so that they can be added to the structure as the preferences and the client prefs work as the selcted values
//     // // Copy the mappings found into the final struct and at the same time get the channel_configs for each client prefs found
//     // let mut maps = Vec::<models::ClientPreferencesWithChannelConfig>::new();
//     // for client_prefs_map in client_preferences {

//     //     // Get the channel configs for this category_mapping_id using a SQL SELECT rather than the struct way
//     //     // Lots of problems with trait errors
//     //     let configs = sql_query("
//     //         SELECT 
//     //             cfg.channel_config_id,
//     //             cfg.category_mappings_id,
//     //             chan.channel_id,
//     //             chan.channel_name,
//     //             cfg.permitted
//     //         FROM channel_configs AS cfg
//     //         RIGHT OUTER JOIN channels AS chan 
//     //         ON cfg.channel_id = chan.channel_id
//     //         WHERE cfg.category_mappings_id = $1
//     //         ORDER BY chan.channel_name
//     //     ")
//     //         .bind::<Integer, _>(cat_map.category_mappings_id)
//     //         .load::<models::ChannelConfig>(conn)
//     //         .expect("Error loading channel configs");

//     //     // Create a new structure from the mappings data
//     //     let new_struct = models::CategoryMappingsWithChannelConfig {
//     //         category_mappings_id: cat_map.category_mappings_id, 
//     //         category: cat_map.category,
//     //         correspondence: cat_map.correspondence,
//     //         opt_out: cat_map.opt_out,
//     //         retention_period: cat_map.retention_period,
//     //         channel_config: configs,
//     //     };

//     //     // Add the struct to the vectors to be returned
//     //     maps.push(new_struct);
//     // }

//     Ok(maps)
// */
//     Ok(client_preferences)
// }


fn build_sql_where_clause (hierarchys: &Vec<models::Hierarchy>) -> String {

    let mut clause = String::new();
    let mut first = true;

    for hierarchy in hierarchys {

        if !first {
            // Put an OR at the beginning
            clause.push_str(" OR ");
        } else {
            first = false;
        }

        let d_term = match &hierarchy.developer[..] {
            "" => "(developer LIKE '%' OR developer IS NULL)".to_string(),
            d =>  format!("(developer LIKE '{}')", d),
        };
        let p_term = match &hierarchy.project[..] {
            "" => "(project LIKE '%' OR project IS NULL)".to_string(),
            p =>  format!("(project LIKE '{}')", p),
        };
        let l_term = match &hierarchy.lender[..] {
            "" => "(lender LIKE '%' OR lender IS NULL)".to_string(),
            l =>  format!("(lender LIKE '{}')", l),
        };
        clause.push_str(&*format!("( {} AND {} AND {} )", d_term, p_term, l_term));
    }

    clause
}



fn build_sql_in_clause (in_list: &Vec<i32>) -> String {

    let mut clause = String::new();
    let mut first = true;

    clause.push_str(&" IN ( ".to_string());
    
    for item in in_list {

        if first {
            // Put an , at the beginning as we are not the first part of the clause
            first = false;
        } else {
            clause.push_str(&",".to_string());
        }
        clause.push_str(&item.to_string());
    }

    clause.push_str(&" ) ".to_string());
    clause
}


// pub fn find_unmapped_client_preferences (
//     conn: &PgConnection,
// ) -> Result<Vec<models::Correspondence>, diesel::result::Error> {

// //    use crate::schema::corrs::dsl::*;
//     //use diesel::sql_query;

//     let results = diesel::sql_query("
//         SELECT * FROM corrs WHERE NOT EXISTS (
//             SELECT correspondence_id FROM category_mappings WHERE correspondence_id = corrs.correspondence_id
//         ) ORDER BY correspondence_name"
//     )
// //        .order(correspondence_name.asc())
// //        .load::<models::Correspondence>(conn)
//         .get_results(conn)
//         .expect("Query failed");
    
//     Ok(results)
// }



// /// Run query using Diesel to delete channels given their id's
// pub fn delete_existing_category_mappings(
//     cat_id: i32,
//     delete_list: &Vec<i32>,
//     conn: &PgConnection,
// ) -> Result<Vec<models::CategoryMappingsWithChannelConfig>, diesel::result::Error> {
//     use crate::schema::category_mappings::dsl::*;

//     // Delete the existing channel_cofigs for this mapping
//     delete_channel_configs_using_category_mapping_id(&delete_list, &conn);

//     for id in delete_list {
        
//         // Delete the existing category mappings
//         debug!("Deleteing category mappings with id: {:?}", id);
//         match diesel::delete(category_mappings.filter(category_mappings_id.eq(id)))
//             .execute(conn)
//         {
//             Ok(results) => debug!("Successful delete from category mappings. Result: {:?}", results),
//             Err(e) => error!("Error deleting category_mappings, error: {:?}", e),
//         }
//     }

//     find_mapped_client_preferences(cat_id, &conn)
// }


// /// Run query using Diesel to delete channels given their id's
// pub fn delete_channel_configs_using_category_mapping_id (
//     delete_list: &Vec<i32>,
//     conn: &PgConnection,
// ) 
// {
//     use crate::schema::channel_configs::dsl::*;

//     for id in delete_list {
//         // Delete the existing channel_configs

//         debug!("Deleteing channel_configs with id: {:?}", id);
//         match diesel::delete(channel_configs.filter(category_mappings_id.eq(id)))
//             .execute(conn)
//         {
//             Ok(results) => debug!("Successful delete from channel_configs. Result: {:?}", results),
//             Err(e) => error!("Error deleting channel_configs, error: {:?}", e),
//         }
//     }
// }
































// /// Run query using Diesel to insert a new database row and return the result.
// pub fn upsert_new_category_mappings(
//     upsert_list: &Vec<models::CategoryMappingsWithChannelConfig>,
//     conn: &PgConnection,
// ) -> Result<Vec<models::CategoryMappingsWithChannelConfig>, diesel::result::Error> {
//     use crate::schema::category_mappings::dsl::*;
// //    use crate::schema::channel_configs::dsl::*;


//     // // Output the raw data in the body of the request
//     // debug!("upsert_new_category_mappings Bytes={:?}", body);


//     // // Convert it to json for use in the rest of the code
//     // let upsert_list: &Vec<models::CategoryMappingsWithChannelConfig> = serde_json::from_str(
//     //     str::from_utf8(&body.to_ascii_lowercase()).expect("Failed to convert to utf8")
//     // ).expect("Failed to convert to Json");







//     // What is an insert and what is an update?
//     // Inserts for category_mappings are defined as a structure that has a -1 for category_mappings_id
//     // Inserts for channel_configs are defined as a structure that has a -1 for channel_config_id

//     // Loop through the data picking out the updates verses the inserts
//     for cat_map in upsert_list {
//         if cat_map.category_mappings_id == -1 {
//             // Insert for everything
//             debug!("Inserting category_mappings: {:?}", cat_map);

//             let new_cat_map = models::NewCategoryMapping {
//                 category_id: cat_map.category.category_id,
//                 correspondence_id: cat_map.correspondence.correspondence_id,
//                 opt_out: cat_map.opt_out.clone(),
//                 retention_period: cat_map.retention_period,
//             };

//             // Create the category mappings entry
//             let inserted_cat_map: Vec<models::CategoryMappingQuery> = diesel::insert_into(category_mappings)
//                 .values(&new_cat_map)
//                 .get_results(conn)
// //                .execute(conn)
//                 .expect("Error saving new post");

//             // Create all of the channel configs for the category mapping
//             create_channel_configs (inserted_cat_map[0].category_mappings_id, &cat_map.channel_config, &conn);

//         } else {
//             // Update for category_mappings
//             debug!("Updating category_mappings: {:?}", cat_map);
//             match diesel::update(category_mappings.filter(category_mappings_id.eq(cat_map.category_mappings_id)))
//                 .set((opt_out.eq(cat_map.opt_out.clone()), retention_period.eq(cat_map.retention_period)))
//                 .execute(conn)
//             {
//                 Ok(results) => debug!("Successful update into category_mappings. Result: {:?}", results),
//                 Err(e) => error!("Error updating category_mappings, error: {:?}", e),
//             }

//             // Put something in here for the call
//             update_channel_configs (&cat_map.channel_config, &conn);
//         }
//     }

//     // Send back a complete list of the items in the table
//     find_mapped_client_preferences(upsert_list[0].category.category_id, &conn)
// }


// /// Update the information in the DB for all the channel configs in the Vector
// fn update_channel_configs ( 
//     cfgs: &Vec<models::ChannelConfig>,     
//     conn: &PgConnection
// ) 
// {
//     use crate::schema::channel_configs::dsl::*;

//     // Update for channel_configs
//     for chan_cfg in cfgs {
//         debug!("Updating channel_configs: {:?}", chan_cfg);
//         match diesel::update(channel_configs.filter(channel_config_id.eq(chan_cfg.channel_config_id)))
// //        .set(permitted.eq(chan_cfg.permitted.as_ref()))
//             .set(permitted.eq(chan_cfg.permitted.clone()))
//             .execute(conn)
//         {
//             Ok(results) => debug!("Successful update into channel_configs. Result: {:?}", results),
//             Err(e) => error!("Error updating channel_configs, error: {:?}", e),
//         }
//     }
// }

// /// Create a chennel config for each existing channel, for the category mapping id passed in
// fn create_channel_configs (
//     cat_map_id: i32,
//     channel_cfgs: &Vec<models::ChannelConfig>,
//     conn: &PgConnection
// ) 
// {
//     use crate::schema::channel_configs::dsl::*;

//     // let chans = channel_actions::find_channels(&conn).expect("Failed to get a list of the channels");


//     // let mut new_channel_configs = Vec::<models::NewChannelConfig>::new();
//     // for chan in chans {
//     //     let new_channel_config = models::NewChannelConfig {
//     //         category_mappings_id: cat_map_id,
//     //         channel_id: chan.channel_id,
//     //         permitted: None,
//     //     };
//     //     new_channel_configs.push(new_channel_config);
//     // }


//     let mut new_channel_configs = Vec::<models::NewChannelConfig>::new();
//     for chan in channel_cfgs {
//         let new_channel_config = models::NewChannelConfig {
//             category_mappings_id: cat_map_id,
//             channel_id: chan.channel.channel_id,
//             permitted: chan.permitted.clone(),
//         };
//         new_channel_configs.push(new_channel_config);
//     }

//     // Insert the new channel configs
//     match diesel::insert_into(channel_configs)
//         .values(new_channel_configs)
//         .execute(conn)
//     {
//         Ok(results) => {
//             debug!("Successful insert into channel_configs. Result: {:?}", results);
//         },
//         Err(e) => error!("Error inserting channel_configs, error: {:?}", e),
//     }
// }













































pub fn find_mapped_client_preferences (
    query_prefs: &models::ClientPreferencesQuery, 
    conn: &PgConnection,
//) -> Result<Vec<models::ClientPreferencesWithChannelConfig>, diesel::result::Error> {
) -> Result<Vec<models::ClientPreferences>, diesel::result::Error> {

//    use diesel::sql_query;

    let mut statement = "
        SELECT 
            cp.client_preferences_id,
            cat.category_id,
            corrs.correspondence_id,
            cm.opt_out AS opt_out,
            cp.opt_out AS selected_opt_out,
            cp.retention_period AS retention_period,
            cp.retention_period AS selected_retention_period,
            cp.developer,
            cp.project,
            cp.lender
        FROM client_preferences AS cp
        INNER JOIN categories AS cat ON cp.category_id = cat.category_id
        INNER JOIN corrs ON cp.correspondence_id = corrs.correspondence_id
        INNER JOIN category_mappings AS cm ON 
            cp.category_id = cm.category_id AND
            cp.correspondence_id = cm.correspondence_id
        WHERE 
    ".to_string();

    // TODO check this is not susceptible to SQL injection etc
    // Construct the WHERE clause for Hierarchy to get the information needed
    statement.push_str(&build_sql_where_clause(&query_prefs.hierarchys));
    debug!("Statement produced is: {}", statement);

    // Constrcut the IN clause on the stastement if there are categories or correspondencs
    if query_prefs.categories.len() > 0 {
        statement.push_str(" AND category_id ");
        statement.push_str(&build_sql_in_clause(&query_prefs.categories));
    }
    if query_prefs.correspondences.len() > 0 {
        statement.push_str(" AND correspondence_id ");
        statement.push_str(&build_sql_in_clause(&query_prefs.correspondences));
    }
    
    // Get the mappings without the channel configs because cannot determine how to do that in Diesel. So split the getting of the
    // structures into two parts, the mappings first and then the channel configs associated with each mapping
    // let client_preferences: Vec<models::ClientPreferences> = sql_query(statement.to_string())
    //     .get_results(conn)
    //     .expect("Error loading client preferences");
    
//    let db_url = env::var("DATABASE_URL").expect("Failed to get the env variable DATABASE_URL");
//    let pg_client = conn::connect(&db_url, NoTls).expect("Failed to get a postgres DB connection");


    let mut client_preferences = Vec::<models::ClientPreferences>::new();
    
//    for pref in pg_client.query(&*statement, &[]).expect("Query failed") {
    for pref in conn.query(&*statement, &[]).expect("Query failed") {
            let cp = models::ClientPreferences {

            client_preferences_id: pref.get("client_preferences_id"),
            category_id: pref.get("category_id"),
          
            correspondence_id: pref.get("correspondence_id"),
            
            opt_out: pref.get("opt_out"),
            selected_opt_out: pref.get("selected_opt_out"),
            retention_period: pref.get("retention_period"),
            selected_retention_period: pref.get("selected_retention_period"),
         
            developer: pref.get("developer"),
            project: pref.get("project"),
            lender: pref.get("lender"),
        };
        client_preferences.push(cp);
    }

    debug!("Retrieved client preferences: {:?}", client_preferences);

    Ok(client_preferences)
}


