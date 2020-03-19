use diesel::prelude::*;

//mod models;
use crate::models;


/// Find all categories
pub fn find_categories (
    conn: &PgConnection,
) -> Result<Vec<models::Category>, diesel::result::Error> {
    use crate::schema::categories::dsl::*;

    let results = categories
        .limit(1000)
        .load::<models::Category>(conn)
        .expect("Error loading posts");

    Ok(results)
}


/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_categories(
    cats: &Vec<models::NewCategory>,
    conn: &PgConnection,
) -> Result<Vec<models::Category>, diesel::result::Error> {
    use crate::schema::categories::dsl::*;

    let results = diesel::insert_into(categories)
        .values(cats)
        .get_results(conn)
        .expect("Error saving new post");

    Ok(results)
}


