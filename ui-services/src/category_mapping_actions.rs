use diesel::prelude::*;

//mod models;
use crate::models;


// pub fn find_category_mappings (
//     cat_id: i32,
//     conn: &PgConnection,
// ) -> Result<Vec<models::CategoryMapping>, diesel::result::Error> {
//     use crate::schema::category_mappings::dsl::*;
        
//     let results = category_mappings
//         .filter(category_id.eq(cat_id))
//         .load::<models::CategoryMapping>(conn)
//         .expect("Error retrieving category mappings ");
//      Ok(results)
// }


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


// pub fn find_corr_not_mapped (
//     conn: &PgConnection,
// ) -> Result<Vec<models::Correspondence>, diesel::result::Error> {

// //    let results = diesel::sql_query(r#"SELECT * FROM corrs WHERE id NOT IN (SELECT corr_id FROM category_mappings)"#)
//     let results = diesel::sql_query("
//         SELECT * FROM corrs WHERE NOT EXISTS (
//              SELECT correspondence_id FROM category_mappings WHERE correspondence_id = corrs.correspondence_id
//         )"
//     )
//         .load::<models::Correspondence>(conn)
//         .expect("Query failed");
//     Ok(results)
// }


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




