use serde::{Deserialize, Serialize};

use crate::schema::{event, account};

#[derive(Debug, Deserialize, Insertable)]
#[table_name="account"]
pub struct Account {
    pub message_id: String,
    pub channel: String,
    pub account_id: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="event"]
pub struct Event {
    pub message_id: String,
    pub channel: String,
    pub event_status: String,
    pub event_timestamp: chrono::NaiveDateTime,
    pub event_specific_data: String,
}

