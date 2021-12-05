use sqlx::PgPool;

use crate::vars;

use crate::db::queries::ConfirmationQuery;
use crate::db::queries::UserQuery;

use crate::db::mirrors::ConfirmationNew;
use crate::db::mirrors::UserNew;

use crate::pages::account::forms::AccountForm;
use crate::pages::account::responses::AccountResponse;
use crate::pages::account::responses::AccountError;

use crate::utils::{
    helper_activate_account_email,
    helper_hash_password,
};

pub async fn account_confirmation_action(uuid: &uuid::Uuid, pool: &PgPool) -> Result<AccountResponse, AccountError> {

    match ConfirmationQuery::find_by_token(&uuid, pool).await {
        Ok(confirmation) => {

            if confirmation.expires_at > chrono::Local::now().naive_local() {
                match UserQuery::enable_account(confirmation.user_id, true, pool).await {
                    Ok(user) => {
                        
                        match ConfirmationQuery::delete(confirmation.id, pool).await {
                            Ok(_) => Ok(
                                        AccountResponse::from(
                                            format!("{} {}", user.first_name, user.last_name),
                                            user.email,
                                        )
                                    ),
                            Err(_) => Err(AccountError::GeneriqueError)
                        }
                    },
                    Err(err) => {
                        match err {
                            sqlx::Error::RowNotFound => Err(AccountError::NotFound),
                            _ => Err(AccountError::GeneriqueError)
                        }
                    }
                }
            } else {
                Err(AccountError::ExpiredValue)
            }            
        },
        Err(err) => {
            match err {
                sqlx::Error::RowNotFound => Err(AccountError::NotFound),
                _ => Err(AccountError::GeneriqueError)
            }
            
        }
    }    
}

pub async fn account_register_action(form: &AccountForm, pool: &PgPool) -> Result<AccountResponse, AccountError> {

    match UserQuery::find_by_email(&form.email.trim(), pool).await {
        Ok(_) => Err(AccountError::UniqueViolation),
        Err(err) => {

            match err {
                sqlx::Error::RowNotFound => {

                    let user = UserNew::from(
                        form.first_name.trim().to_string(), 
                        form.last_name.trim().to_string(),
                        form.email.trim().to_string(),
                        helper_hash_password(form.password.as_str())
                    );

                    //Save a new account in database
                    match UserQuery::save(&user, pool).await {
                        Ok(user) => {

                            //Create a confirnation token
                            let confirmation = ConfirmationNew::from(user.id);

                            //Save a confirmation token for the new account
                            match ConfirmationQuery::save(&confirmation, pool).await {
                                Ok(confirmation) => {

                                    let url = format!("{}/account/activate/{}", vars::get_app_domain_url(), confirmation.token);

                                    let full_name = format!("{} {}", user.first_name, user.last_name);

                                    let email = helper_activate_account_email(&full_name, &url);

                                    Ok(
                                        AccountResponse::from(
                                            full_name,
                                            user.email
                                        )
                                    )
                                },
                                Err(_) => Err(AccountError::GeneriqueError)
                            }
                        },
                        Err(_) => Err(AccountError::GeneriqueError)
                    }
                }
                _ => Err(AccountError::GeneriqueError)
            }
        }
    }
}