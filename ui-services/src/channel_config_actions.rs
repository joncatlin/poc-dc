// use diesel::prelude::*;

// //mod models;
// use crate::models;


// // /// Find all channel_configs
// // pub fn find_channel_configs (
// //     conn: &PgConnection,
// // ) -> Result<Vec<models::ChannelConfig>, diesel::result::Error> {
// //     use crate::schema::channel_configs::dsl::*;

// //     let results = channel_configs
// //         .limit(1000)
// //         .load::<models::ChannelConfig>(conn)
// //         .expect("Error loading channel_configs");

// //     Ok(results)
// // }


// /// Run query using Diesel to insert a new database row and return the result.
// pub fn upsert_new_channel_configs(
//     upsert_list: &Vec<models::ChannelConfig>,
//     conn: &PgConnection,
// ) -> Result<Vec<models::ChannelConfig>, diesel::result::Error> {
//     use crate::schema::channel_configs::dsl::*;

//     let mut inserts = Vec::new();

//     for item in upsert_list {
//         if item.channel_id == -1 {
//             inserts.push(models::NewChannelConfig{channel_name: item.channel_name.clone()});
//         } else {
//             // Update the existing channel_configs
//             info!("Updating channel with values: {:?}", item);
//             match diesel::update(channel_configs.filter(channel_id.eq(item.channel_id)))
//                 .set(permitted.eq(item.permitted))
//                 .execute(conn)
//             {
//                 Ok(results) => debug!("Successful update into channel_configs. Result: {:?}", results),
//                 Err(e) => error!("Error updating channel_configs, error: {:?}", e),
//             }
//         }
//     }

//     // Insert the new channel_configs
//     match diesel::insert_into(channel_configs)
//         .values(inserts)
//         .execute(conn)
//     {
//         Ok(results) => debug!("Successful insert into channel_configs. Result: {:?}", results),
//         Err(e) => error!("Error inserting channel_configs, error: {:?}", e),
//     }

//     // Send back a complete list of the items in the table
//     let results = channel_configs
//         .limit(1000)
//         .load::<models::ChannelConfig>(conn)
//         .expect("Error obtaining list of channel_configs");

//     Ok(results)
// }


// // /// Run query using Diesel to insert a new database row and return the result.
// // pub fn insert_new_channel_configs(
// //     insert_list: &Vec<models::NewChannelConfig>,
// //     conn: &PgConnection,
// // ) -> Result<Vec<models::ChannelConfig>, diesel::result::Error> {
// //     use crate::schema::channel_configs::dsl::*;

// //     // Insert the new channel_configs
// //     let result = diesel::insert_into(channel_configs)
// //         .values(insert_list)
// //         .execute(conn);

// //     Ok(results)
// // }




