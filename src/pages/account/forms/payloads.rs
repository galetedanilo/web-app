use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct NewAccountForm {
    
    #[validate(length(min = 2, max = 20, message = "The first name must be 2-20 characters long"))]
    pub first_name: String,

    #[validate(length(min = 2, max = 20, message = "The last name must be 2-20 characters long"))]
    pub last_name: String,

    #[validate(email(message = "Is not a valid email address"))]
    pub email: String,

    pub password: String,
}