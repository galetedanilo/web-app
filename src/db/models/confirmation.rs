use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Confirmation {
    pub id: i32,
    pub user_id: i32,
    pub token: uuid::Uuid,
    pub expires_at: chrono::NaiveDateTime,
}