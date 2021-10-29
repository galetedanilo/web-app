use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct SignUpForm {
    
    #[validate(length(min = 4, max = 30))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    pub password: String,
}