use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: u64,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    locked: bool,
    enabled: bool,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct NewAccount {

    #[validate(length(min = 2, max = 20, message = "First name must be 2-20 characters long"))]
    #[serde(rename = "firstName")]
    first_name: String,

    #[validate(length(min = 2, max = 20, message = "Last name must be 2-20 characters long"))]
    #[serde(rename = "lastName")]
    last_name: String,

    #[validate(email(message = "Is not a valid email address"))]
    email: String,
    
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    email: String,
    password: String,
}