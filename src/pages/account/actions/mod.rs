use sqlx::PgPool;

use crate::utils::{
    helper_encode_password,
};

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
        Ok(ok) => println!("{}", ok),
        Err(err) => println!("{}", err),
    };

}