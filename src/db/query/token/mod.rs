use crate::db::models::Token;
use sqlx::PgPool;

pub struct TokenNew {
    pub user_id: i64,
    pub token: uuid::Uuid,
    pub expires: chrono::NaiveDateTime,
}

impl TokenNew {

    pub fn from(user_id: i64) -> Self {
        TokenNew {
            user_id,
            token: uuid::Uuid::new_v4(),
            expires: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
        }
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<Token, sqlx::Error> {
        
        let row: (i64, ) = sqlx::query_as("INSERT INTO tb_tokens (user_id, token, expires) VALUES ($1, $2, $3) returning id")
            .bind(&self.user_id)
            .bind(&self.token)
            .bind(&self.expires)
            .fetch_one(pool)
            .await?;
        
        Ok(
            Token {
                id: row.0,
                user_id: self.user_id,
                token: self.token,
                expires: self.expires,
            }
        )
    }
}