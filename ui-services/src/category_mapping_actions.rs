use diesel::prelude::*;
use diesel::sql_types::Integer;

//mod models;
use crate::models;
use crate::channel_actions;


pub fn find_mapped_category_corrs (
    category_id: i32,
    conn: &PgConnection,
) -> Result<Vec<models::CategoryMappingsWithChannelConfig>, diesel::result::Error> {

    use diesel::sql_query;

    // Get the mappings without the channel configs because cannot determine how to do that in Diesel. So split the getting of the
    // structures into two parts, the mappings first and then the channel configs associated with each mapping
    let category_mappings = sql_query("
        SELECT 
            cm.category_mappings_id,
            cat.category_id,
            cat.category_name,
            corrs.correspondence_id,
            corrs.correspondence_name,
            cm.opt_out,
            cm.retention_period
        FROM category_mappings AS cm
        INNER JOIN categories AS cat ON cm.category_id = cat.category_id
        INNER JOIN corrs ON cm.correspondence_id = corrs.correspondence_id
        WHERE cm.category_id = $1
    ")
        .bind::<Integer, _>(category_id)
        .load::<models::CategoryMappings>(conn)
        .expect("Error loading category to correspondence mapping");

    // Copy the mappings found into the final struct and at the same time get the channel_configs for each category mapping found
    let mut maps = Vec::<models::CategoryMappingsWithChannelConfig>::new();
    for cat_map in category_mappings {

        // Get the channel configs for this category_mapping_id using a SQL SELECT rather than the struct way
        // Lots of problems with trait errors
        let configs = sql_query("
            SELECT 
                cfg.channel_config_id,
                cfg.category_mappings_id,
                chan.channel_id,
                chan.channel_name,
                cfg.permitted
            FROM channel_configs AS cfg
            RIGHT OUTER JOIN channels AS chan 
            ON cfg.channel_id = chan.channel_id
            WHERE cfg.category_mappings_id = $1
            ORDER BY chan.channel_name
        ")
            .bind::<Integer, _>(cat_map.category_mappings_id)
            .load::<models::ChannelConfig>(conn)
            .expect("Error loading channel configs");

        // Create a new structure from the mappings data
        let new_struct = models::CategoryMappingsWithChannelConfig {
            category_mappings_id: cat_map.category_mappings_id, 
            category: cat_map.category,
            correspondence: cat_map.correspondence,
            opt_out: cat_map.opt_out,
            retention_period: cat_map.retention_period,
            channel_config: configs,
        };

        // Add the struct to the vectors to be returned
        maps.push(new_struct);
    }

    Ok(maps)
}


pub fn find_unmapped_category_corrs (
    conn: &PgConnection,
) -> Result<Vec<models::Correspondence>, diesel::result::Error> {

    let results = diesel::sql_query("
        SELECT * FROM corrs WHERE NOT EXISTS (
             SELECT correspondence_id FROM category_mappings WHERE correspondence_id = corrs.correspondence_id
        )"
    )
        .order(correspondence_name.asc())
        .load::<models::Correspondence>(conn)
        .expect("Query failed");
    Ok(results)
}



/// Run query using Diesel to delete channels given their id's
pub fn delete_existing_category_mappings(
    cat_id: i32,
    delete_list: &Vec<i32>,
    conn: &PgConnection,
) -> Result<Vec<models::CategoryMappingsWithChannelConfig>, diesel::result::Error> {
    use crate::schema::category_mappings::dsl::*;

    // Delete the existing channel_cofigs for this mapping
    delete_channel_configs_using_category_mapping_id(&delete_list, &conn);

    for id in delete_list {
        
        // Delete the existing category mappings
        debug!("Deleteing category mappings with id: {:?}", id);
        match diesel::delete(category_mappings.filter(category_mappings_id.eq(id)))
            .execute(conn)
        {
            Ok(results) => debug!("Successful delete from category mappings. Result: {:?}", results),
            Err(e) => error!("Error deleting category_mappings, error: {:?}", e),
        }
    }

    find_mapped_category_corrs(cat_id, &conn)
}


/// Run query using Diesel to delete channels given their id's
pub fn delete_channel_configs_using_category_mapping_id (
    delete_list: &Vec<i32>,
    conn: &PgConnection,
) 
{
    use crate::schema::channel_configs::dsl::*;

    for id in delete_list {
        // Delete the existing channel_configs

        debug!("Deleteing channel_configs with id: {:?}", id);
        match diesel::delete(channel_configs.filter(category_mappings_id.eq(id)))
            .execute(conn)
        {
            Ok(results) => debug!("Successful delete from channel_configs. Result: {:?}", results),
            Err(e) => error!("Error deleting channel_configs, error: {:?}", e),
        }
    }
}




































/// Run query using Diesel to insert a new database row and return the result.
pub fn upsert_new_category_mappings(
    upsert_list: &Vec<models::CategoryMappingsWithChannelConfig>,
    conn: &PgConnection,
) -> Result<Vec<models::CategoryMappingsWithChannelConfig>, diesel::result::Error> {
    use crate::schema::category_mappings::dsl::*;
//    use crate::schema::channel_configs::dsl::*;

    // What is an insert and what is an update?
    // Inserts for category_mappings are defined as a structure that has a -1 for category_mappings_id
    // Inserts for channel_configs are defined as a structure that has a -1 for channel_config_id

    // Loop through the data picking out the updates verses the inserts
    for cat_map in upsert_list {
        if cat_map.category_mappings_id == -1 {
            // Insert for everything
            debug!("Inserting category_mappings: {:?}", cat_map);

            let new_cat_map = models::NewCategoryMapping {
                category_id: cat_map.category.category_id,
                correspondence_id: cat_map.correspondence.correspondence_id,
                opt_out: cat_map.opt_out.clone(),
                retention_period: cat_map.retention_period,
            };

            // Create the category mappings entry
            let new_map: models::CategoryMappingQuery = diesel::insert_into(category_mappings)
                .values(&new_cat_map)
                .get_result(conn)
                .expect("Error saving new post");

            // Create all of the channel configs for the category mapping
            create_channel_configs (new_map.category_mappings_id, &conn);

        } else {
            // Update for category_mappings
            debug!("Updating category_mappings: {:?}", cat_map);
            match diesel::update(category_mappings.filter(category_mappings_id.eq(cat_map.category_mappings_id)))
                .set((opt_out.eq(cat_map.opt_out.clone()), retention_period.eq(cat_map.retention_period)))
                .execute(conn)
            {
                Ok(results) => debug!("Successful update into category_mappings. Result: {:?}", results),
                Err(e) => error!("Error updating category_mappings, error: {:?}", e),
            }

            // Put something in here for the call
            update_channel_configs (&cat_map.channel_config, &conn);
        }
    }

    // Send back a complete list of the items in the table
    find_mapped_category_corrs(upsert_list[0].category.category_id, &conn)
}


/// Update the information in the DB for all the channel configs in the Vector
fn update_channel_configs ( 
    cfgs: &Vec<models::ChannelConfig>,     
    conn: &PgConnection
) 
{
    use crate::schema::channel_configs::dsl::*;

    // Update for channel_configs
    for chan_cfg in cfgs {
        debug!("Updating channel_configs: {:?}", chan_cfg);
        match diesel::update(channel_configs.filter(channel_config_id.eq(chan_cfg.channel_config_id)))
            .set(permitted.eq(chan_cfg.permitted.as_ref()))
            .execute(conn)
        {
            Ok(results) => debug!("Successful update into channel_configs. Result: {:?}", results),
            Err(e) => error!("Error updating channel_configs, error: {:?}", e),
        }
    }
}

/// Create a chennel config for each existing channel, for the category mapping id passed in
fn create_channel_configs (
    cat_map_id: i32,
    conn: &PgConnection
) 
{
    use crate::schema::channel_configs::dsl::*;

    let chans = channel_actions::find_channels(&conn).expect("Failed to get a list of the channels");


    let mut new_channel_configs = Vec::<models::NewChannelConfig>::new();
    for chan in chans {
        let new_channel_config = models::NewChannelConfig {
            category_mappings_id: cat_map_id,
            channel_id: chan.channel_id,
            permitted: None,
        };
        new_channel_configs.push(new_channel_config);
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