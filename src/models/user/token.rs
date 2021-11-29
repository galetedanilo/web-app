use chrono::NaiveDateTime;
use uuid::Uuid;
use sqlx::PgPool;

use crate::vars;

pub struct Token {
    pub id: i64,
    pub token: Uuid,
    pub expires: NaiveDateTime,
}

impl Token {

    pub fn from(id: i64) -> Self {
        Token {
            id,
            token:  uuid::Uuid::new_v4(),
            expires: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
        }
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<String, sqlx::Error> {
        
        let _row: (i32, ) = sqlx::query_as("INSERT INTO tb_tokens (id, token, expires) VALUES ($1, $2, $3) returning id")
            .bind(&self.id)
            .bind(&self.token)
            .bind(&self.expires)
            .fetch_one(pool)
            .await?;
        
        Ok(format!("{}/account/activate/{}", vars::get_app_domain_url(), self.token))
    }
}