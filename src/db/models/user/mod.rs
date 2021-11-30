use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
    pub enable: bool,
}

impl User {

    pub fn from(id: i64, first_name: String, last_name: String, email: String, password: String, 
                created: chrono::NaiveDateTime, updated: chrono::NaiveDateTime, enable: bool) -> Self {

        User {
            id,
            first_name,
            last_name,
            email,
            password,
            created,
            updated,
            enable,
        }
    }
}