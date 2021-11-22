use crate::utils::helper_encode_password;

use crate::models::{NewUser, NewAccountForm};

pub fn register_new_user_action(form: NewAccountForm) {
   
    let user = NewUser::from(
        form.email,
        form.first_name, 
        form.last_name,
        helper_encode_password(form.password.as_str())
    );

}