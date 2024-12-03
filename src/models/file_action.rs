use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use sqlx::FromRow;

#[derive(Clone,Debug,serde::Serialize, serde::Deserialize)]
pub struct FileAction {
    pub user_id: i32,
    pub file_id: i32,
    pub from_office_id: i32,
    pub to_office_id: i32,
    pub status: i32,
    pub remarks: Option<String> // Optional field
}

#[derive(FromRow,Clone,Debug,serde::Serialize, serde::Deserialize)]
pub struct Movement {
    pub file_id: i32,
    pub file_number: String,
    pub owner_name: String,
    pub batch_number: i32,
    pub rack_number: i32,
    pub remarks: Option<String>,
    pub previous_location: String,
    pub previous_location_id: i32,
    pub current_location: String,
    pub current_location_id: i32,
    pub created_at: String,  // Adjust according to the correct datetime format
}
