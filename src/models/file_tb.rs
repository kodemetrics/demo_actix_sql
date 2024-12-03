use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::FromRow;
// use chrono::{NaiveDateTime};

#[derive(FromRow,Clone,Debug,serde::Serialize, serde::Deserialize)]
pub struct FileRecord {
    pub user_id: i32,
    pub file_number: String,
    pub owner_name: String,
    pub batch_number: i32,
    pub rack_number: i32,
    pub lga: String,
    pub land_application_exists: i32, // 0 or 1
    pub r_of_o_letter_exists: i32,    // 0 or 1
    pub c_of_o_letter_exists: i32,    // 0 or 1
    pub lan_number: String,
    pub phone_number: String,
    pub remark: Option<String>, // Optional field
    pub file_condition: String, // This will be one of 'new', 'pending', 'approved', or 'rejected'
    pub number_of_pages: i32,
    pub location: String,
    pub application_date: String,
    pub coo_date: String,
    pub roo_date: String,

}


#[derive(FromRow,Clone,Debug,serde::Serialize, serde::Deserialize)]
pub struct GetFileRecord {
    pub id: i32,
    pub user_id: i32,
    pub file_number: String,
    pub owner_name: String,
    pub batch_number: i32,
    pub rack_number: i32,
    pub lga: String,
    pub land_application_exists: i32, // 0 or 1
    pub r_of_o_letter_exists: i32,    // 0 or 1
    pub c_of_o_letter_exists: i32,    // 0 or 1
    pub lan_number: String,
    pub phone_number: String,
    pub remark: Option<String>, // Optional field
    pub file_condition: String, // This will be one of 'new', 'pending', 'approved', or 'rejected'
    pub number_of_pages: i32,
    pub location: String
}
