use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub enable: bool,
}

impl User {

    pub fn from(id: i32, first_name: String, last_name: String, email: String, password: String, 
                created_at: chrono::NaiveDateTime, updated_at: chrono::NaiveDateTime, enable: bool) -> Self {

        User {
            id,
            first_name,
            last_name,
            email,
            password,
            created_at,
            updated_at,
            enable,
        }
    }
}