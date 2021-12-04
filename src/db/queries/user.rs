use sqlx::PgPool;

use crate::db::models::User;

use crate::db::mirrors::UserNew;

pub struct UserQuery;

impl UserQuery {

    pub async fn save(user_new: &UserNew, pool: &PgPool) -> Result<User, sqlx::Error> {
    
        let result = sqlx::query_as::<_, User>(
            r#"
                INSERT INTO tb_users (first_name, last_name, email, password, created_at, updated_at, enable) 
                VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, first_name, last_name, email, password, created_at, updated_at, enable
            "#,
            )
            .bind(&user_new.first_name)
            .bind(&user_new.last_name)
            .bind(&user_new.email)
            .bind(&user_new.password)
            .bind(&user_new.created_at)
            .bind(&user_new.updated_at)
            .bind(&user_new.enable)
            .fetch_one(pool)
            .await?;

        Ok(result)

    }

    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<User, sqlx::Error> {
        
        let result = sqlx::query_as::<_, User>("SELECT * FROM tb_users WHERE email = $1")
            .bind(&email)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }
}