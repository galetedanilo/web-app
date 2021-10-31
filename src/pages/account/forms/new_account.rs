use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct NewAccountForm {
    
    #[validate(length(min = 4, max = 30, message = "Username must be 4-30 characters long"))]
    pub username: String,

    #[validate(email(message = "Is not a valid email address"))]
    pub email: String,

    pub password: String,
}