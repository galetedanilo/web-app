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
    helper_encode_password,
    helper_activate_account_email,
};

pub async fn register_new_account_action(form: &AccountForm, pool: &PgPool) -> Result<AccountResponse, AccountError> {

    match UserQuery::find_by_email(&form.email.trim(), pool).await {
        Ok(_) => Err(AccountError::UniqueViolation),
        Err(err) => {

            match err {
                sqlx::Error::RowNotFound => {

                    let user = UserNew::from(
                        form.first_name.trim().to_string(), 
                        form.last_name.trim().to_string(),
                        form.email.trim().to_string(),
                        helper_encode_password(form.password.as_str())
                    );


                    match UserQuery::save(&user, pool).await {
                        Ok(user) => {

                            let confirmation = ConfirmationNew::from(user.id);

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