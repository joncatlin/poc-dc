use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
//use tokio_postgres::types::ToSql;

// ***************************** Channels ****************************************
#[derive(Clone, Debug, Deserialize, PostgresMapper, Serialize, ToSql, FromSql)]
#[pg_mapper(table = "channel")] 
pub struct Channel {
    pub channel_id: i32,
    pub channel_name: String,
}


// #[derive(Debug, Deserialize, PostgresMapper, Serialize)]
// #[pg_mapper(table = "channel")] 
// pub struct NewChannel {
//     pub channel_name: String,
// }

// ***************************** Categories ****************************************
#[derive(Clone, Debug, Deserialize, PostgresMapper, Serialize, ToSql, FromSql)]
#[pg_mapper(table = "category")] 
pub struct Category {
    pub category_id: i32,
    pub category_name: String,
}

// #[derive(Debug, Deserialize, PostgresMapper, Serialize)]
// #[pg_mapper(table = "category")] 
// pub struct NewCategory {
//     pub category_name: String,
// }

// ***************************** Languages ****************************************
#[derive(Debug, Deserialize, PostgresMapper, Serialize, ToSql, FromSql)]
#[pg_mapper(table = "language")] 
pub struct Language {
    pub language_id: i32,
    pub language_name: String,
}

// #[derive(Debug, Deserialize, PostgresMapper, Serialize)]
// #[pg_mapper(table = "language")] 
// pub struct NewLanguage {
//     pub language_name: String,
// }

// ***************************** Correspondences ****************************************
#[derive(Clone, Debug, Deserialize, PostgresMapper, Serialize, ToSql, FromSql)]
#[pg_mapper(table = "correspondence")] 
pub struct Correspondence {
    pub correspondence_id: i32,
    pub correspondence_name: String,
}

// #[derive(Debug, Deserialize, PostgresMapper, Serialize)]
// #[pg_mapper(table = "correspondence")] 
// pub struct NewCorrespondence {
//     pub correspondence_name: String,
// }

// ***************************** CategoryMappings ****************************************
#[derive(Debug, Deserialize, PostgresMapper, Serialize, ToSql, FromSql)]
#[pg_mapper(table = "category_mapping")] 
pub struct CategoryMapping {
    pub category_mapping_id: i32,
    pub category: Category,
    pub correspondence: Correspondence,
    pub opt_out: String,
    pub retention_period: i32,
    pub channel_config: Vec<ChannelConfig>,
}

impl CategoryMapping {
    pub fn new() -> CategoryMapping {
        CategoryMapping {
            category_mapping_id: -1,
            category: Category {
                category_id: -1,
                category_name: "".to_string(),
            },
            correspondence: Correspondence {
                correspondence_id: -1,
                correspondence_name: "".to_string(),
            },
            opt_out: "".to_string(),
            retention_period: 1,
            channel_config: Vec::<ChannelConfig>::new(),
        }
    }
}

// #[derive(Debug, Deserialize, PostgresMapper, Serialize)]
// #[pg_mapper(table = "category_mapping")] 
// pub struct NewCategoryMapping {
//     pub category_id: i32,
//     pub correspondence_id: i32,
//     pub opt_out: String,
//     pub retention_period: i32,
// }


// ***************************** Channel Configs ****************************************
#[derive(Clone, Debug, Deserialize, PostgresMapper, Serialize, ToSql, FromSql)]
#[pg_mapper(table = "channel_config")] 
pub struct ChannelConfig {
    pub channel_config_id: i32,
    pub category_mapping_id: i32,
    pub channel: Channel,
    pub permitted: String,
}

// #[derive(Debug, Deserialize, PostgresMapper, Serialize, ToSql, FromSql)]
// #[pg_mapper(table = "channel_config")] 
// pub struct NewChannelConfig {
//     pub category_mapping_id: i32,
//     pub channel_id: i32,
//     pub permitted: String,
// }

// ***************************** Client Preferences ****************************************
#[derive(Clone, Debug, Deserialize, PostgresMapper, Serialize, ToSql, FromSql)]
#[pg_mapper(table = "client_preference")] 
pub struct ClientPreference {
    pub client_preference_id:  i32,
    pub category: Category,
    pub correspondence: Correspondence,
    pub opt_out: String,
    pub selected_opt_out: String,
    pub retention_period: i32,
    pub selected_retention_period: i32,
    pub developer: String,
    pub project: String,
    pub lender: String,
    pub client_pref_channel_config: Vec<ClientPrefChannelConfig>,
}

// This struct is used for calls from the API. It differs from what the database structure is
// bacause it needs the query criteria being used, so that it can return the correct data at
// the end of the API call
#[derive(Debug, Deserialize, Serialize)]
pub struct ClientPreferenceAPI {
    pub client_preferences: Vec<ClientPreference>,
    pub client_pref_query: ClientPreferenceQuery, 
}

// Struct that represents the data passed through the API to query, and update/insert client preferences
// This is needed directly for the query and indirectly for the upsert to return a list of the new or updated elements
// specific to a query. If it was not passed then the upsert would have no idea what the criteria was used to return data
#[derive(Debug, Deserialize, Serialize)]
pub struct ClientPreferenceQuery {
    pub category: Category,
//    pub correspondence: Vec<Correspondence>,
    pub correspondence: Vec<i32>,
    pub dpl: DPL,
}

// {"category":{"category_id":1,"category_name":"Statements"},"correspondence":[2,1],"dpl":{"developer":"100","project":null,"lender":null}}




#[derive(Debug, Deserialize, Serialize)]
pub struct DPL {
    pub developer: String,
    pub project: String,
    pub lender: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientPreferenceDelete {
    pub client_preference_id: i32,
    pub client_pref_query: ClientPreferenceQuery,
}


// ***************************** Client Preference Channel Configs ****************************************
#[derive(Clone, Debug, Deserialize, PostgresMapper, Serialize, ToSql, FromSql)]
#[pg_mapper(table = "client_pref_channel_config")] 
pub struct ClientPrefChannelConfig {
    pub client_pref_channel_config_id: i32,
    pub client_preference_id: i32,
    pub channel: Channel,
    pub permitted: String,
    pub selected_permitted: String,
}


// ***************************** Templates ****************************************
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestTemplate {
    pub template_id: i32,
    pub template_name: String,
    pub document_id: String,
    pub document_content: String,
}





