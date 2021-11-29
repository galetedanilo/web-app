use sqlx::PgPool;

use crate::utils::{
    helper_encode_password,
    helper_activate_account_email,
};

use crate::models::user::Token;
use crate::models::user::UserNew;
use crate::pages::account::forms::NewAccountForm;

pub async fn register_new_account_action(form: NewAccountForm, pool: &PgPool) {

    let user = UserNew::from(
        form.first_name, 
        form.last_name,
        form.email,
        helper_encode_password(form.password.as_str())
    );

    match user.insert(pool).await {
        Ok(id) => {

            let token = Token::from(id);

            match token.insert(pool).await {
                Ok(ok) => {
                    let message = helper_activate_account_email(&format!("{} {}", user.first_name, user.last_name), &ok);

                    println!("{}", message.into_string());
                },
                Err(err) => println!("{}", err),
            };
        },
        Err(err) => println!("{}", err),
    };

}