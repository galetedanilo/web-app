use sqlx::PgPool;

use crate::db::queries::UserQuery;

use crate::utils::helper_hash_password;

use crate::pages::account::forms::LoginForm;

use crate::pages::account::responses::AccountResponse;
use crate::pages::account::responses::AccountError;

pub async fn login_verify_credentials_handler(form: &LoginForm, pool: &PgPool) -> Result<AccountResponse, AccountError> {
    
    match UserQuery::find_by_email(form.email.trim(), pool).await {
        Ok(user) => {
            if user.password == helper_hash_password(form.password.trim()) {
                Ok(AccountResponse::from(user.first_name, user.last_name, user.email))
            }else {
                Err(AccountError::WrongPassword)
            }
        },
        Err(err) => {
            match err {
                sqlx::Error::RowNotFound => Err(AccountError::AccountNotFound),
                _ => Err(AccountError::GeneriqueError),
            }
        }
    }
}