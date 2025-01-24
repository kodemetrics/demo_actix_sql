use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow,Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Report {
    pub from: String,
    pub to: String
}
