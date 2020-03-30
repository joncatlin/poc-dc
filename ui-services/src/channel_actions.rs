use diesel::prelude::*;

//mod models;
use crate::models;


// /// Find all channels
// pub fn find_channels (
//     conn: &PgConnection,
// ) -> Result<Vec<models::Channel>, diesel::result::Error> {
//     use crate::schema::channels::dsl::*;

//     let results = channels
//         .limit(100)
//         .load::<models::Channel>(conn)
//         .expect("Error loading posts");

//     Ok(results)
// }


// /// Run query using Diesel to insert a new database row and return the result.
// pub fn insert_new_channels(
//     cats: &Vec<models::NewChannel>,
//     conn: &PgConnection,
// ) -> Result<Vec<models::Channel>, diesel::result::Error> {
//     use crate::schema::channels::dsl::*;

//     let results = diesel::insert_into(channels)
//         .values(cats)
//         .get_results(conn)
//         .expect("Error saving new post");

//     Ok(results)
// }


/// Find all channels
pub fn find_channels (
    conn: &PgConnection,
) -> Result<Vec<models::Channel>, diesel::result::Error> {
    use crate::schema::channels::dsl::*;

    let results = channels
        .limit(1000)
        .load::<models::Channel>(conn)
        .expect("Error loading channels");

    Ok(results)
}


/// Run query using Diesel to insert a new database row and return the result.
pub fn upsert_new_channels(
    upsert_list: &Vec<models::Channel>,
    conn: &PgConnection,
) -> Result<Vec<models::Channel>, diesel::result::Error> {
    use crate::schema::channels::dsl::*;

    let mut inserts = Vec::new();

    for item in upsert_list {
        if item.channel_id == -1 {
            inserts.push(models::NewChannel{channel_name: item.channel_name.clone()});
        } else {
            // Update the existing channels
            info!("Updating channel with values: {:?}", item);
            match diesel::update(channels.filter(channel_id.eq(item.channel_id)))
                .set(channel_name.eq(item.channel_name.clone()))
                .execute(conn)
            {
                Ok(results) => debug!("Successful update into channels. Result: {:?}", results),
                Err(e) => error!("Error updating channels, error: {:?}", e),
            }
        }
    }

    // Insert the new channels
    match diesel::insert_into(channels)
        .values(inserts)
        .execute(conn)
    {
        Ok(results) => debug!("Successful insert into channels. Result: {:?}", results),
        Err(e) => error!("Error inserting channels, error: {:?}", e),
    }

    // Send back a complete list of the items in the table
    let results = channels
        .limit(1000)
        .load::<models::Channel>(conn)
        .expect("Error obtaining list of channels");

    Ok(results)
}


/// Run query using Diesel to delete channels given their id's
pub fn delete_existing_channels(
    delete_list: &Vec<models::Channel>,
    conn: &PgConnection,
) -> Result<Vec<models::Channel>, diesel::result::Error> {
    use crate::schema::channels::dsl::*;

    for item in delete_list {
        // Delete the existing channels
        info!("Deleteing channel with values: {:?}", item);
        match diesel::delete(channels.filter(channel_id.eq(item.channel_id)))
            .execute(conn)
        {
            Ok(results) => debug!("Successful delete from channels. Result: {:?}", results),
            Err(e) => error!("Error deleting channels, error: {:?}", e),
        }
    }

    // Send back a complete list of the items left in the table after the delete
    let results = channels
        .limit(1000)
        .load::<models::Channel>(conn)
        .expect("Error obtaining list of channels");

    Ok(results)
}


