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


/// Run query using Diesel to insert a new database row and return the result.
pub fn upsert_new_categories(
    upsert_list: &Vec<models::Category>,
    conn: &PgConnection,
) -> Result<Vec<models::Category>, diesel::result::Error> {
    use crate::schema::categories::dsl::*;

    let mut inserts = Vec::new();

    for item in upsert_list {
        if item.category_id == -1 {
            inserts.push(models::NewCategory{category_name: item.category_name.clone()});
        } else {
            // Update the existing categories
            info!("Updating category with values: {:?}", item);
            match diesel::update(categories.filter(category_id.eq(item.category_id)))
                .set(category_name.eq(item.category_name.clone()))
                .execute(conn)
//                .expect("Error updating category")
            {
                Ok(results) => debug!("Successful update into categories. Result: {:?}", results),
                Err(e) => error!("Error updating categories, error: {:?}", e),
            }
        }
    }

    // Insert the new categories
    match diesel::insert_into(categories)
        .values(inserts)
        // .on_conflict(category_id)
        // .do_update()
        // .set(category_name.eq(excluded(category_name)))
        .execute(conn)
//        .expect("Error inserting categories") 
    {
        Ok(results) => debug!("Successful insert into categories. Result: {:?}", results),
        Err(e) => error!("Error inserting categories, error: {:?}", e),
    }

    // Send back a complete list of the items in the table
    let results = categories
        .limit(1000)
        .load::<models::Category>(conn)
        .expect("Error obtaining list of categories");

    Ok(results)
}


/// Run query using Diesel to delete categories given their id's
pub fn delete_existing_categories(
    delete_list: &Vec<models::Category>,
    conn: &PgConnection,
) -> Result<Vec<models::Category>, diesel::result::Error> {
    use crate::schema::categories::dsl::*;

    for item in delete_list {
        // Delete the existing categories
        info!("Deleteing category with values: {:?}", item);
        match diesel::delete(categories.filter(category_id.eq(item.category_id)))
            .execute(conn)
        {
            Ok(results) => debug!("Successful delete from categories. Result: {:?}", results),
            Err(e) => error!("Error deleting categories, error: {:?}", e),
        }
    }

    // Send back a complete list of the items left in the table after the delete
    let results = categories
        .limit(1000)
        .load::<models::Category>(conn)
        .expect("Error obtaining list of categories");

    Ok(results)
}


