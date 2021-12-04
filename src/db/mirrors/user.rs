use validator::Validate;

use crate::utils::{
    helper_is_number_validate,
    helper_lower_case_validate,
    helper_no_whitespace_validate,
    helper_upper_case_validate
};

#[derive(Validate)]
pub struct UserNew {

    #[validate(length(min = 2, max = 20, message = "The first name must be 2-20 characters long"))]
    pub first_name: String,

    #[validate(length(min = 2, max = 20, message = "The last name must be 2-20 characters long"))]
    pub last_name: String,

    #[validate(email(message = "Is not a valid email address"))]
    pub email: String,

    #[validate(length(min = 8, max = 12, message = "The password must be 8-12 characters long"))]
    #[validate(custom(function = "helper_is_number_validate", message = "Password must contain at least one number" ))]
    #[validate(custom(function = "helper_lower_case_validate", message = "Password must contain at least one lower character"))]
    #[validate(custom(function = "helper_no_whitespace_validate", message = "Password must not contain whitespaces"))]
    #[validate(custom(function = "helper_upper_case_validate", message = "Password must contain at least one upper character"))]
    pub password: String,
    
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,

    pub enable: bool,
}

impl UserNew {

    pub fn from(first_name: String, last_name: String, email: String, password: String) -> Self {

        UserNew {
            first_name,
            last_name,
            email,
            password,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
            enable: false
        }
    }
}