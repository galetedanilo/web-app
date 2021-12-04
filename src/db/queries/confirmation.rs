use sqlx::PgPool;

use crate::db::models::Confirmation;

use crate::db::mirrors::ConfirmationNew;

pub struct ConfirmationQuery;

impl ConfirmationQuery {

    pub async fn save(confirmation_new: &ConfirmationNew, pool: &PgPool) -> Result<Confirmation, sqlx::Error> {
        
        let result = sqlx::query_as::<_, Confirmation>(
            r#"
                INSERT INTO tb_confirmations (user_id, token, expires_at) 
                VALUES ($1, $2, $3) RETURNING id, user_id, token, expires_at
            "#,
            )
            .bind(&confirmation_new.user_id)
            .bind(&confirmation_new.token)
            .bind(&confirmation_new.expires_at)
            .fetch_one(pool)
            .await?;
        
        Ok(result)

    }
}