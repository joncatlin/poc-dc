use serde::{Deserialize, Serialize};

use crate::schema::event_trans;

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct Event_Trans {
    pub id: u32,
    pub event_id: String,
    pub event_type: String,
    pub event_status: String,
    pub event_timestamp: i64,
    pub event_specific_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEvent_Trans {
    pub event_id: String,
    pub event_type: String,
    pub event_status: String,
    pub event_timestamp: i64,
    pub event_specific_data: String,
}
