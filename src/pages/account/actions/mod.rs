use sqlx::PgPool;

use crate::db::query::TokenNew;
use crate::db::query::UserNew;

use crate::vars;

use crate::utils::{
    helper_encode_password,
    helper_activate_account_email,
};

use crate::pages::account::forms::NewAccountForm;

pub async fn register_new_account_action(form: NewAccountForm, pool: &PgPool) {

    let user_new = UserNew::from(
        form.first_name, 
        form.last_name,
        form.email,
        helper_encode_password(form.password.as_str())
    );

    match user_new.insert(pool).await {
        Ok(user) => {

            let token_new = TokenNew::from(user.id);

            match token_new.insert(pool).await {
                Ok(token) => {

                    let url = format!("{}/account/activate/{}", vars::get_app_domain_url(), token.token);

                    let email_message = helper_activate_account_email(&format!("{} {}", user.first_name, user.last_name), &url);

                    println!("{}", email_message.into_string());
                },
                Err(err) => println!("{}", err),
            };
        },
        Err(err) => println!("{}", err),
    };

}