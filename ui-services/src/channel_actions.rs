use diesel::prelude::*;
use diesel::sql_types::Integer;

use crate::models;

/// Find all channels
pub fn find_channels (
    conn: &PgConnection,
) -> Result<Vec<models::Channel>, diesel::result::Error> {
    use crate::schema::channels::dsl::*;

    let results = channels
        .order(channel_name.asc())
        .load::<models::Channel>(conn)
        .expect("Error loading channels");

    Ok(results)
}


/// Find channel by name
pub fn find_channel_by_name (
    chan_name: String,
    conn: &PgConnection,
) -> Result<models::Channel, diesel::result::Error> {

    // use diesel::sql_query;

    // let results = sql_query("
    //     SELECT 
    //         channel_id
    //     FROM channels
    //     WHERE channel_name = $1
    // ")
    //     .bind::<String, _>(chan_name)
    //     .load::<models::NewChannel>(conn)
    //     .expect("Error loading category to correspondence mapping");

    use crate::schema::channels::dsl::*;

    let result = channels
        .filter(channel_name.eq(chan_name))
        .first::<models::Channel>(conn)
        .expect("Error loading channels");

    // Return the first result. There should only be one due to the unique key on the table
    Ok(result)
}


/// Run query using Diesel to insert a new database row and return the result.
pub fn upsert_new_channels(
    upsert_list: &Vec<models::Channel>,
    conn: &PgConnection,
) -> Result<Vec<models::Channel>, diesel::result::Error> {
    use crate::schema::channels::dsl::*;

    let mut new_channel_names = Vec::new();
    let mut new_channel_names_to_pass = Vec::new();

    for item in upsert_list {
        if item.channel_id == -1 {
            new_channel_names.push(channel_name.eq(item.channel_name.clone()));
            new_channel_names_to_pass.push(item.channel_name.clone());
            // inserts.push(models::NewChannel{channel_name: item.channel_name.clone()});
        } else {
            // Update the existing channels
            info!("Updating channel with values: {:?}", item);
            match diesel::update(channels.filter(channel_id.eq(item.channel_id)))
                .set(channel_name.eq(item.channel_name.clone()))
                .execute(conn)
            {
                Ok(results) => debug!("Successful update into channels. Result: {:?}", results),
                Err(e) => error!("Error updating channels, error: {:?}", e),
            }
        }
    }

    match diesel::insert_into(channels)
        .values(new_channel_names)
        .execute(conn)
    {
        Ok(_) => {
            // Create a channel_config for each new channel created
            create_new_channel_configs(&new_channel_names_to_pass, &conn);
        },
        Err(e) => error!("Error inserting channels, error: {:?}", e),
    };

    // Send back a complete list of the items in the table
    let results = channels
        .limit(1000)
        .load::<models::Channel>(conn)
        .expect("Error obtaining list of channels");

    Ok(results)
}


/// Run query using Diesel to delete channels given their id's
pub fn delete_existing_channels(
    delete_list: &Vec<models::Channel>,
    conn: &PgConnection,
) -> Result<Vec<models::Channel>, diesel::result::Error> {
    use crate::schema::channels::dsl::*;

    let mut channel_ids = Vec::<i32>::new();

    for item in delete_list {
        // Save the id for later use in deleting the channel_configs
        channel_ids.push(item.channel_id);

        // Delete the existing channels
        info!("Deleteing channel with values: {:?}", item);
        match diesel::delete(channels.filter(channel_id.eq(item.channel_id)))
            .execute(conn)
        {
            Ok(results) => debug!("Successful delete from channels. Result: {:?}", results),
            Err(e) => error!("Error deleting channels, error: {:?}", e),
        }
    }

    // Delete all channel_configs that contain any of the channel ids
    delete_existing_channel_configs(&channel_ids, &conn);

    // Send back a complete list of the items left in the table after the delete
    let results = channels
        .limit(1000)
        .load::<models::Channel>(conn)
        .expect("Error obtaining list of channels");

    Ok(results)
}


// Create new channel_confids for any new channel added
pub fn create_new_channel_configs(
    channel_names: &Vec<String>,
    conn: &PgConnection
) {
    use crate::schema::channel_configs::dsl::*;

    // Get the ids of the new channels
    let chan_ids = find_channel_ids(channel_names, &conn);

    // Get the existing category_mappings
    let map_ids = find_category_mapping_ids(&conn);

    // For each new channel create a channel_config struct for every category mapping and then insert them all into the DB
    let mut new_channel_configs = Vec::<models::NewChannelConfig>::new();
    for map in map_ids {
        for chan in &chan_ids {
            let new_channel_config = models::NewChannelConfig {
                category_mappings_id: map.category_mappings_id,
                channel_id: *chan,
                permitted: None,
            };
            new_channel_configs.push(new_channel_config);
        } 
    }

    // Insert the new channel configs
    match diesel::insert_into(channel_configs)
        .values(new_channel_configs)
        .execute(conn)
    {
        Ok(results) => {
            debug!("Successful insert into channel_configs. Result: {:?}", results);
        },
        Err(e) => error!("Error inserting channel_configs, error: {:?}", e),
    }
}


// Find the channel ids given the names
fn find_channel_ids (
    channel_names: &Vec<String>,
    conn: &PgConnection
) -> Vec<i32>
{
    use crate::schema::channels::dsl::*;

    // This is a HACK! There must be a way to filter on a set of names rather than to do this in a loop!
    let mut ids = Vec::new();
    for name in channel_names {
        ids.push(channels.select(channel_id).filter(channel_name.eq(name)).first(conn).unwrap());
    }

    ids
}


// Find the channel ids given the names
fn find_category_mapping_ids (
    conn: &PgConnection
) -> Vec<models::ExistingCategoryMappings>
{
    use diesel::sql_query;

    // Get a list of all the category correspondence mappings that a new channel_config needs to be created for
    let category_mappings = sql_query("
        SELECT 
            category_mappings_id
        FROM category_mappings
    ")
        .load::<models::ExistingCategoryMappings>(conn)
        .expect("Error loading category to correspondence mapping");

    category_mappings
}


/// Delete all channel_configs that contain a channel_id
pub fn delete_existing_channel_configs(
    delete_list: &Vec<i32>,
    conn: &PgConnection,
) 
{
    use diesel::sql_query;

    for channel_id in delete_list {
        sql_query("DELETE FROM channel_configs WHERE channel_id = $1")
            .bind::<Integer, _>(channel_id)
            .execute(conn)
            .expect("Error deleteing channel_configs");
    }
}





