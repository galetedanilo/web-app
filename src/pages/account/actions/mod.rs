use sqlx::PgPool;

use crate::db::query::TokenNew;
use crate::db::query::UserNew;

use crate::vars;

use crate::pages::account::transfers::NewAccountTransfer;
use crate::pages::error::enums::ActionDatabaseError;

use crate::utils::{
    helper_encode_password,
    helper_activate_account_email,
};

use crate::pages::account::forms::NewAccountForm;

pub async fn register_new_account_action(form: &NewAccountForm, pool: &PgPool) -> Result<NewAccountTransfer, ActionDatabaseError> {

    let user_new = UserNew::from(
        form.first_name.trim().to_string(), 
        form.last_name.trim().to_string(),
        form.email.trim().to_string(),
        helper_encode_password(form.password.as_str())
    );

    match user_new.insert(pool).await {
        Ok(user) => {

            let token_new = TokenNew::from(user.id);

            match token_new.insert(pool).await {
                Ok(token) => {

                    let url = format!("{}/account/activate/{}", vars::get_app_domain_url(), token.token);

                    let full_name = format!("{} {}", user.first_name, user.last_name);

                    let email_message = helper_activate_account_email(&full_name, &url);

                    Ok(NewAccountTransfer::from(full_name, user.email))
                },
                Err(err) => {
                    Err(ActionDatabaseError::UniqueColumn)
                },
            }
        },
        Err(err) => {
            
            match err {
                sqlx::Error::PoolTimedOut => Err(ActionDatabaseError::PoolTimedOut),
                sqlx::Error::Database(_) => Err(ActionDatabaseError::UniqueColumn),
                _ => Err(ActionDatabaseError::PoolTimedOut),
            }
            
        },
    }
}