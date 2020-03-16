use serde::{Deserialize, Serialize};

use crate::schema::categories;
use crate::schema::channels;
use crate::schema::languages;
use crate::schema::corrs;

// Categories
#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
#[table_name="categories"]
pub struct Category {
    pub id: i32,
    pub category_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="categories"]
pub struct NewCategory {
    pub category_name: String,
}

// Languages
#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
#[table_name="languages"]
pub struct Language {
    pub id: i32,
    pub language_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="languages"]
pub struct NewLanguage {
    pub language_name: String,
}


// Channels
#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
#[table_name="channels"]
pub struct Channel {
    pub id: i32,
    pub channel_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="channels"]
pub struct NewChannel {
    pub channel_name: String,
}


// Correspondences
#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
#[table_name="corrs"]
pub struct Corr {
    pub id: i32,
    pub corr_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="corrs"]
pub struct NewCorr {
    pub corr_name: String,
}
