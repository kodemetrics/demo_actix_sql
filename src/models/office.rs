use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};

#[derive(Clone,Debug,serde::Serialize, serde::Deserialize)]
pub struct Office {
    id: i32,
    name: String,
}





