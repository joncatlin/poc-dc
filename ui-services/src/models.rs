use serde::{Deserialize, Serialize};

use crate::schema::categories;
use crate::schema::channels;
use crate::schema::languages;
use crate::schema::templates;
use crate::schema::corrs;
use crate::schema::category_mappings;

// Categories
#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
#[table_name="categories"]
pub struct Category {
    pub category_id: i32,
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
    pub language_id: i32,
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
    pub channel_id: i32,
    pub channel_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="channels"]
pub struct NewChannel {
    pub channel_name: String,
}


// Correspondences
#[derive(Debug, Clone, Serialize, Queryable, Insertable, QueryableByName)]
#[table_name="corrs"]
pub struct Correspondence {
    pub correspondence_id: i32,
    pub correspondence_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="corrs"]
pub struct NewCorrespondence {
    pub correspondence_name: String,
}


// Category Mappings
#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
#[table_name="category_mappings"]
pub struct CategoryMapping {
    pub category_mappings_id: i32,
    pub category_id: i32,
    #[diesel(embed)]
    pub correspondence: Correspondece,
    pub opt_out: bool,
    pub retention_period: i32,
    #[diesel(embed)]
    pub channel_config: ChannelConfig,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="category_mappings"]
pub struct NewCategoryMapping {
    pub category_id: i32,
    pub correspondence_id: i32,
    pub opt_out: bool,
    pub retention_period: i32,
}


// Templates
#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
#[table_name="templates"]
pub struct Template {
    pub template_id: i32,
    pub template_name: String,
    pub language_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="templates"]
pub struct NewTemplate {
    pub template_name: String,
    pub language_id: i32,
//    pub language_name: String,
}


// Template List which includes the language name from the languages table
#[derive(Debug, Clone, Serialize, QueryableByName)]
#[table_name="templates"]
pub struct TemplateWithLanguage {
    pub template_id: i32,
    pub template_name: String,
    #[diesel(embed)]
    pub language: Language,
}


// // EMBEDDED structs. These are structs referenced in other models
// // Languages struct for quesrying templates
// #[derive(Debug, Clone, Serialize, QueryableByName)]
// #[table_name="languages"]
// pub struct EmbeddedLanguage {
//     pub language_id: i32,
//     pub language_name: String,
// }


// Template List which includes the language name from the languages table
#[derive(Debug, Clone, Serialize, QueryableByName)]
#[table_name="channel_configs"]
pub struct ChannelConfig {
    pub channel_config_id: i32,
    #[diesel(embed)]
    pub channel: Channel,
    pub permitted: bool,
    pub selected: language: Language,
}


