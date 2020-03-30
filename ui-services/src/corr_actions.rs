use diesel::prelude::*;

//mod models;
use crate::models;


// /// Find all correspondences
// pub fn find_corrs (
//     conn: &PgConnection,
// ) -> Result<Vec<models::Correspondence>, diesel::result::Error> {
//     use crate::schema::corrs::dsl::*;

//     let results = corrs
//         .limit(100)
//         .load::<models::Correspondence>(conn)
//         .expect("Error loading posts");

//     Ok(results)
// }


// /// Run query using Diesel to insert a new database row and return the result.
// pub fn insert_new_corrs(
//     cats: &Vec<models::NewCorrespondence>,
//     conn: &PgConnection,
// ) -> Result<Vec<models::Correspondence>, diesel::result::Error> {
//     use crate::schema::corrs::dsl::*;

//     let results = diesel::insert_into(corrs)
//         .values(cats)
//         .get_results(conn)
//         .expect("Error saving new post");

//     Ok(results)
// }

/// Find all correspondences
pub fn find_correspondences (
    conn: &PgConnection,
) -> Result<Vec<models::Correspondence>, diesel::result::Error> {
    use crate::schema::corrs::dsl::*;

    let results = corrs
        .limit(1000)
        .load::<models::Correspondence>(conn)
        .expect("Error loading posts");

    Ok(results)
}


/// Run query using Diesel to insert a new database row and return the result.
pub fn upsert_new_correspondences(
    upsert_list: &Vec<models::Correspondence>,
    conn: &PgConnection,
) -> Result<Vec<models::Correspondence>, diesel::result::Error> {
    use crate::schema::corrs::dsl::*;

    let mut inserts = Vec::new();

    for item in upsert_list {
        if item.correspondence_id == -1 {
            inserts.push(models::NewCorrespondence{correspondence_name: item.correspondence_name.clone()});
        } else {
            // Update the existing correspondences
            info!("Updating Correspondence with values: {:?}", item);
            match diesel::update(corrs.filter(correspondence_id.eq(item.correspondence_id)))
                .set(correspondence_name.eq(item.correspondence_name.clone()))
                .execute(conn)
            {
                Ok(results) => debug!("Successful update into correspondences. Result: {:?}", results),
                Err(e) => error!("Error updating correspondences, error: {:?}", e),
            }
        }
    }

    // Insert the new correspondences
    match diesel::insert_into(corrs)
        .values(inserts)
        .execute(conn)
    {
        Ok(results) => debug!("Successful insert into correspondences. Result: {:?}", results),
        Err(e) => error!("Error inserting correspondences, error: {:?}", e),
    }

    // Send back a complete list of the items in the table
    let results = corrs
        .limit(1000)
        .load::<models::Correspondence>(conn)
        .expect("Error obtaining list of correspondences");

    Ok(results)
}


/// Run query using Diesel to delete correspondences given their id's
pub fn delete_existing_correspondences(
    delete_list: &Vec<models::Correspondence>,
    conn: &PgConnection,
) -> Result<Vec<models::Correspondence>, diesel::result::Error> {
    use crate::schema::corrs::dsl::*;

    for item in delete_list {
        // Delete the existing correspondences
        info!("Deleteing Correspondence with values: {:?}", item);
        match diesel::delete(corrs.filter(correspondence_id.eq(item.correspondence_id)))
            .execute(conn)
        {
            Ok(results) => debug!("Successful delete from correspondences. Result: {:?}", results),
            Err(e) => error!("Error deleting correspondences, error: {:?}", e),
        }
    }

    // Send back a complete list of the items left in the table after the delete
    let results = corrs
        .limit(1000)
        .load::<models::Correspondence>(conn)
        .expect("Error obtaining list of correspondences");

    Ok(results)
}
