use diesel::prelude::*;
use diesel::sql_types::Integer;

//mod models;
use crate::models;


pub fn find_mapped_category_corrs (
    category_id: i32,
    conn: &PgConnection,
) -> Result<Vec<models::CategoryMappingsWithChannelConfig>, diesel::result::Error> {

    use diesel::sql_query;
    use crate::schema::channel_configs::dsl::*;

    // Get the mappings without the channel configs because cannot determine how to do that in Diesel. So spliut the getting of the
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

        // Get the channel configs for this category_mapping_id
        let configs = channel_configs
            .filter(category_mappings_id.eq(cat_map.category_mappings_id))
            .load::<models::ChannelConfig>(conn)
            .expect("Error loading channel_configs");

        // Create a new structure from the mappings data
        let new_struct = models::CategoryMappingsTest {
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
        .load::<models::Correspondence>(conn)
        .expect("Query failed");
    Ok(results)
}

