use diesel::prelude::*;
use diesel::sql_types::Integer;

//mod models;
use crate::models;


pub fn find_mapped_category_corrs (
    category_id: i32,
    conn: &PgConnection,
) -> Result<Vec<models::MappedCategories>, diesel::result::Error> {

    use diesel::sql_query;

    let results = sql_query("
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
        .load::<models::MappedCategories>(conn)
        .expect("Error loading category to correspondence mapping");

    Ok(results)
}






// pub fn insert_new_category_mappings (
//     cats: &Vec<models::NewCategoryMapping>,
//     conn: &PgConnection,
// ) -> Result<Vec<models::CategoryMapping>, diesel::result::Error> {
//     use crate::schema::category_mappings::dsl::*;

//     let results = diesel::insert_into(category_mappings)
//         .values(cats)
//         .get_results(conn)
//         .expect("Error saving new category mappings");
//     Ok(results)
// }


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

// pub fn find_corr_mapped (
//     conn: &PgConnection,
// ) -> Result<Vec<models::Correspondence>, diesel::result::Error> {

//     let results = diesel::sql_query("
//         SELECT * FROM corrs WHERE NOT EXISTS ( SELECT correspondence_id FROM category_mappings WHERE correspondence_id = corrs.correspondence_id )"
//     )
//         .load::<models::Correspondence>(conn)
//         .expect("Query failed");
//     Ok(results)
// }




