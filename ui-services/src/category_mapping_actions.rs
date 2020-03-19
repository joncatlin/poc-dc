use diesel::prelude::*;

//mod models;
use crate::models;


/// Find all category mappings given a category id
pub fn find_category_mappings (
    cat_id: i32,
    conn: &PgConnection,
) -> Result<Vec<models::CategoryMapping>, diesel::result::Error> {
    use crate::schema::category_mappings::dsl::*;
        
    let results = category_mappings
        .filter(category_id.eq(cat_id))
        .load::<models::CategoryMapping>(conn)
        .expect("Error retrieving category mappings ");
     Ok(results)
}


/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_category_mappings (
    cats: &Vec<models::NewCategoryMapping>,
    conn: &PgConnection,
) -> Result<Vec<models::CategoryMapping>, diesel::result::Error> {
    use crate::schema::category_mappings::dsl::*;

    let results = diesel::insert_into(category_mappings)
        .values(cats)
        .get_results(conn)
        .expect("Error saving new category mappings");
    Ok(results)
}


/// Find all of the correspondences that have not been mapped
pub fn find_corr_not_mapped (
    conn: &PgConnection,
) -> Result<Vec<models::Corr>, diesel::result::Error> {

//    let results = diesel::sql_query(r#"SELECT * FROM corrs WHERE id NOT IN (SELECT corr_id FROM category_mappings)"#)
    let results = diesel::sql_query("
        SELECT * FROM corrs WHERE NOT EXISTS (
             SELECT correspondence_id FROM category_mappings WHERE correspondence_id = corrs.correspondence_id
        )"
        .load::<models::Corr>(conn)
        .expect("Query failed");
    Ok(results)
}


/// Find all of the correspondences that have been mapped to a category, along with all of the preferences
pub fn find_corr_mapped (
    conn: &PgConnection,
) -> Result<Vec<models::Corr>, diesel::result::Error> {

    let results = diesel::sql_query("
        SELECT * FROM corrs WHERE NOT EXISTS ( SELECT correspondence_id FROM category_mappings WHERE correspondence_id = corrs.correspondence_id )"
    )
        .load::<models::Corr>(conn)
        .expect("Query failed");
    Ok(results)
}




