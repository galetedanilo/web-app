use validator::Validate;

use crate::utils::{
    helper_contains_lower_case_validate,
    helper_contains_number_validate,
    helper_contains_upper_case_validate,
    helper_contains_whitespace_validate
};

#[derive(Validate)]
pub struct UserNew {

    #[validate(length(min = 2, max = 20, message = "The first name must be 2-20 characters long"))]
    pub first_name: String,

    #[validate(length(min = 2, max = 20, message = "The last name must be 2-20 characters long"))]
    pub last_name: String,

    
    pub birth_date: chrono::NaiveDate,

    #[validate(length(min = 5, max = 30, message = "The username must be 5-30 characters long"))]
    pub username: String,

    #[validate(email(message = "Is not a valid email address"))]
    pub email: String,

    #[validate(length(min = 8, max = 12, message = "The password must be 8-12 characters long"))]
    #[validate(custom(function = "helper_contains_number_validate", message = "Password must contain at least one number" ))]
    #[validate(custom(function = "helper_contains_lower_case_validate", message = "Password must contain at least one lower character"))]
    #[validate(custom(function = "helper_contains_whitespace_validate", message = "Password must not contain whitespaces"))]
    #[validate(custom(function = "helper_contains_upper_case_validate", message = "Password must contain at least one upper character"))]
    pub password: String,
    
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,

    pub enable: bool,
}

impl UserNew {

    pub fn from(first_name: String, last_name: String, birth_date: chrono::NaiveDate, 
                username: String, email: String, password: String) -> Self {

        UserNew {
            first_name,
            last_name,
            birth_date,
            username,
            email,
            password,
            created_at: chrono::Local::now().naive_utc(),
            updated_at: chrono::Local::now().naive_utc(),
            enable: false
        }
    }
}