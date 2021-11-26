use uuid::Uuid;

use crate::vars;

use crate::utils::{
    helper_encode_password,
    helper_activate_account_email
};

use crate::models::account::{NewAccount, NewAccountForm};

pub fn register_new_user_action(form: NewAccountForm) {
   
    let user = NewAccount::from(
        form.email,
        form.first_name, 
        form.last_name,
        helper_encode_password(form.password.as_str())
    );

    let token = Uuid::new_v4();
    
    let email = helper_activate_account_email(
        &vars::get_app_name(),
        &vars::get_domain_url(),
        &"User Full Name Here",
        &token.to_string()
    );

    println!("{}", email.into_string());
    println!("{:?}", user);

}