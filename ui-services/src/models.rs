use serde::{Deserialize, Serialize};

use crate::schema::categories;
use crate::schema::channels;
use crate::schema::languages;
use crate::schema::templates;
use crate::schema::corrs;
use crate::schema::category_mappings;
use crate::schema::channel_configs;

// Categories
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName, Insertable, Identifiable, AsChangeset)]
#[table_name="categories"]
#[primary_key(category_id)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[table_name="languages"]
#[primary_key(language_id)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[table_name="channels"]
#[primary_key(channel_id)]
pub struct Channel {
    pub channel_id: i32,
    pub channel_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[table_name="channels"]
pub struct NewChannel {
    pub channel_name: String,
}


// Correspondences
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName, Insertable, Identifiable, AsChangeset)]
#[table_name="corrs"]
#[primary_key(correspondence_id)]
pub struct Correspondence {
    pub correspondence_id: i32,
    pub correspondence_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="corrs"]
pub struct NewCorrespondence {
    pub correspondence_name: String,
}






// Correspondences Embedded
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name="corrs"]
#[primary_key(correspondence_id)]
pub struct EmbedCorrespondence {
    pub correspondence_id: i32,
    pub correspondence_name: String,
}


// Category Mappings
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Identifiable)]
#[table_name="category_mappings"]
#[primary_key(category_mappings_id)]
pub struct CategoryMapping {
    pub category_mappings_id: i32,
    pub category_id: i32,
    #[diesel(embed)]
    pub correspondence: EmbedCorrespondence,
    pub opt_out: String,
    pub retention_period: i32,
    // #[diesel(embed)]
    // pub channel_config: ChannelConfig,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="category_mappings"]
pub struct NewCategoryMapping {
    pub category_id: i32,
    pub correspondence_id: i32,
    pub opt_out: String,
    pub retention_period: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="category_mappings"]
pub struct CategoryMappingQuery {
    pub category_mappings_id: i32,
    pub category_id: i32,
    pub correspondence_id: i32,
    pub opt_out: String,
    pub retention_period: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Identifiable)]
#[table_name="category_mappings"]
#[primary_key(category_mappings_id)]
pub struct MappedCategories {
        pub category_mappings_id: i32,
    
    #[diesel(embed)]
    pub category: Category,

    #[diesel(embed)]
    pub correspondence: Correspondence,
    
    pub opt_out: String,
    pub retention_period: i32,

    // #[diesel(embed)]
    // pub channel_config: Vec<ChannelConfig>,
}

// ******************************* TEST *********************************
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Identifiable)]
#[table_name="category_mappings"]
#[primary_key(category_mappings_id)]
pub struct CategoryMappings {
    pub category_mappings_id: i32,
    
    #[diesel(embed)]
    pub category: Category,

    #[diesel(embed)]
    pub correspondence: Correspondence,
    
    pub opt_out: String,
    pub retention_period: i32,

}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName, Identifiable)]
#[table_name="category_mappings"]
#[primary_key(category_mappings_id)]
pub struct ExistingCategoryMappings {
    pub category_mappings_id: i32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryMappingsWithChannelConfig {
    pub category_mappings_id: i32,
    
    pub category: Category,

    pub correspondence: Correspondence,
    
    pub opt_out: String,
    pub retention_period: i32,

    pub channel_config: Vec<ChannelConfig>,

}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName, Identifiable)]
#[table_name="channels"]
#[primary_key(channel_id)]
pub struct EmbedChannel {
    pub channel_id: i32,
    pub channel_name: String,
}


// Channel configurations
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName, Identifiable)]
//#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName, Identifiable)]
#[table_name="channel_configs"]
#[primary_key(channel_config_id)]
pub struct ChannelConfig {
    pub channel_config_id: i32,
    pub category_mappings_id: i32,
 
    #[diesel(embed)]
    pub channel: EmbedChannel,

    pub permitted: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="channel_configs"]
pub struct NewChannelConfig {
    pub category_mappings_id: i32,
    pub channel_id: i32,
    pub permitted: String,
}



// table! {
//     channel_configs (channel_config_id) {
//         channel_config_id -> Int4,
//         category_mappings_id -> Int4,
//         channel_id -> Int4,
//         permitted -> Int4,
//     }
// }














// Templates
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[table_name="templates"]
#[primary_key(template_id)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[table_name="templates"]
#[primary_key(template_id)]
pub struct TemplateWithLanguage {
    pub template_id: i32,
    pub template_name: String,
    // #[diesel(embed)]
    // pub language: Language,
}


// // ******************************* CLIENT PREFERENCES *********************************
// #[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Identifiable)]
// #[table_name="client_preferences"]
// #[primary_key(client_preferences_id)]
// pub struct ClientPreferences {

//     pub client_preferences_id:  i32,

//     #[diesel(embed)]
//     pub category: Category,

//     #[diesel(embed)]
//     pub correspondence: Correspondence,
    
//     pub opt_out: String,
//     pub retention_period: i32,

//     pub developer: String,
//     pub project: String,
//     pub lender: String,
// }


#[derive(Debug, Serialize, Deserialize)]
pub struct ClientPreferencesQuery {

    pub category_id: i32,
    pub hierarchys: Vec<Hierarchy>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Hierarchy {
    pub developer: String,
    pub project: String,
    pub lender: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ClientPreferencesWithChannelConfig {
    pub client_preferences_id: i32,
    
    pub category: Category,

    pub correspondence: Correspondence,
    
    pub opt_out: String,
    pub retention_period: i32,

    pub channel_config: Vec<ChannelConfig>,

    pub developer: String,
    pub project: String,
    pub lender: String,
}


