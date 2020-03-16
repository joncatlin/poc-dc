use diesel::prelude::*;

//mod models;
use crate::models;


/// Find all correspondences
pub fn find_corrs (
    conn: &PgConnection,
) -> Result<Vec<models::Language>, diesel::result::Error> {
    use crate::schema::corrs::dsl::*;

    let results = corrs
        .limit(100)
        .load::<models::Language>(conn)
        .expect("Error loading posts");

    Ok(results)
}


/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_corrs(
    cats: &Vec<models::NewCorr>,
    conn: &PgConnection,
) -> Result<Vec<models::Corr>, diesel::result::Error> {
    use crate::schema::corrs::dsl::*;

    let results = diesel::insert_into(corrs)
        .values(cats)
        .get_results(conn)
        .expect("Error saving new post");

    Ok(results)
}


