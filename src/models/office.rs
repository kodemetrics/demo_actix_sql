use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use sqlx::FromRow;

#[derive(FromRow,Clone,Debug,serde::Serialize, serde::Deserialize)]
pub struct Office {
    pub id: Option<i32>,
    pub name: String,
    pub unit_count: i32
}
