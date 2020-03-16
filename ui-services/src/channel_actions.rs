use diesel::prelude::*;

//mod models;
use crate::models;


/// Find all channels
pub fn find_channels (
    conn: &PgConnection,
) -> Result<Vec<models::Channel>, diesel::result::Error> {
    use crate::schema::channels::dsl::*;

    let results = channels
        .limit(100)
        .load::<models::Channel>(conn)
        .expect("Error loading posts");

    Ok(results)
}


/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_channels(
    cats: &Vec<models::NewChannel>,
    conn: &PgConnection,
) -> Result<Vec<models::Channel>, diesel::result::Error> {
    use crate::schema::channels::dsl::*;

    let results = diesel::insert_into(channels)
        .values(cats)
        .get_results(conn)
        .expect("Error saving new post");

    Ok(results)
}


