use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum UserRoles {
    Customer,
    Driver,
}

#[derive(FromRow,Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub staff_id: String,
    pub office_id: i32,
    pub role: String,
}

#[derive(FromRow,Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdateUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub staff_id: String,
    pub office_id: i32,
    pub role: String,
}

#[derive(FromRow,Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub staff_id: String,
    pub office_id: i32,
    pub role: String,
}

#[derive(FromRow,Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NewUpdateUser {
    #[serde(default)]
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    #[serde(default)] 
    pub password: Option<String>,
    pub staff_id: String,
    pub office_id: i32,
    pub role: String,
}

#[derive(FromRow,Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String
}
