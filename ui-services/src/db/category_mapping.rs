use crate::{
    app_errors::MyError, 
    models::{CategoryMapping, Correspondence, ChannelConfig, Category},
    db::channel_config,
};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

//***************************** Category Mappings ****************************************
// Get a list of correspondences that are not mapped to any category
pub async fn get_unmapped_correspondences(client: &Client) -> Result<Vec<Correspondence>, MyError> {
    let _stmt = include_str!("../../sql/category_mapping/get_category_mapping_unmapped.sql");
    let _stmt = _stmt.replace("$table_fields", &Correspondence::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let categories = client
        .query(&stmt, &[], )
        .await?
        .iter()
        .map(|row| Correspondence::from_row_ref(row).unwrap())
        .collect::<Vec<Correspondence>>();

        Ok(categories)
}

// Get a list of category to correspondence mappings given a category id
pub async fn get_mapped_categories(client: &Client, category_id: i32,) -> Result<Vec<CategoryMapping>, MyError> {

    let _stmt = include_str!("../../sql/category_mapping/get_category_mapping_mapped.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    let category_maps = client
        .query(&stmt, &[&category_id], )
        .await?
        .iter()
        .map(|row| 
            // TODO look into creating the trait from_row_ref, on the struct
            CategoryMapping {
                category_mapping_id: row.get("category_mapping_id"),
                category: Category {
                    category_id: row.get("category_id"),
                    category_name: row.get("category_name"),
                },
                correspondence: Correspondence {
                    correspondence_id: row.get("correspondence_id"),
                    correspondence_name: row.get("correspondence_name")
                },
                opt_out: row.get("opt_out"),
                retention_period: row.get("retention_period"),
                channel_config: Vec::<ChannelConfig>::new(),
            }
        )
        .collect::<Vec<CategoryMapping>>();

    // Get the channel configs for each category mapping
    let mut new_maps = Vec::<CategoryMapping>::new();
    for mut item in category_maps {
        item.channel_config = channel_config::get_channel_configs(&client, item.category_mapping_id.clone()).await?;
        new_maps.push(item);
    }

    Ok(new_maps)
}

pub async fn upsert_category_mappings(
    client: &Client,
    upsert_list: &Vec<CategoryMapping>,
) -> Result<Vec<CategoryMapping>, MyError> {

    // Create the insert and update statements
    let _insert_stmt = include_str!("../../sql/category_mapping/insert_category_mapping.sql");
    // let _insert_stmt = _insert_stmt.replace("$table_fields", &Category::sql_table_fields());
    let insert_stmt = client.prepare(&_insert_stmt).await.unwrap();

    let _update_stmt = include_str!("../../sql/category_mapping/update_category_mapping.sql");
    // let _update_stmt = _update_stmt.replace("$table_fields", &Category::sql_table_fields());
    let update_stmt = client.prepare(&_update_stmt).await.unwrap();

    // TODO. Fix this if possible. Not sure how to do a bulk upsert with this framework so 
    // iterate through individual inserts
    for item in upsert_list {

        if item.category_mapping_id == -1 {
            // Insert category mapping
            let new_id = client.query(&insert_stmt, &[&item.category.category_id, 
                &item.correspondence.correspondence_id, &item.opt_out, &item.retention_period]).await.unwrap()[0].get("category_mapping_id");
            
            // Build a vec of channel_config and insert the category_mapping_id of the new entry
            let mut new_channel_configs = Vec::<ChannelConfig>::new();
            for cfg in &item.channel_config {
                let mut new_cfg = cfg.clone();
                new_cfg.category_mapping_id = new_id;
                new_channel_configs.push(new_cfg)
            }

            // Upsert channel configs 
            channel_config::upsert_channel_configs(client, &new_channel_configs).await.unwrap();
        } else {
            client.query(&update_stmt, &[&item.category.category_id, 
                &item.correspondence.correspondence_id, &item.opt_out, &item.retention_period,
                &item.category_mapping_id]).await.unwrap();

            // Upsert channel configs 
            channel_config::upsert_channel_configs(client, &item.channel_config).await.unwrap();
        }

    }

    // Return the list of category mappings with all of the upserts
    Ok(get_mapped_categories(&client, upsert_list[0].category.category_id).await.unwrap())
}

pub async fn delete_category_mappings(
    client: &Client,
    cat_map_id: i32,
) -> Result<Vec<CategoryMapping>, MyError> {

    // Create the delete statement
    let _delete_stmt = include_str!("../../sql/category_mapping/delete_category_mapping.sql");
    let delete_stmt = client.prepare(&_delete_stmt).await.unwrap();

    // TODO. Fix this if possible. Not sure how to do a bulk delete with this framework so 
    // iterate through individual ones
    let cat_id = client.query(&delete_stmt, &[&cat_map_id,]).await.unwrap()[0].get("category_id");

    Ok(get_mapped_categories(&client, cat_id).await.unwrap())
}

