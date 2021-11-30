use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub id: i64,
    pub user_id: i64,
    pub token: uuid::Uuid,
    pub expires: chrono::NaiveDateTime,
}

impl Token {

    pub fn from(id: i64, user_id: i64, token: uuid::Uuid, expires: chrono::NaiveDateTime) -> Self {
        Token {
            id,
            user_id,
            token,
            expires,
        }
    }
}