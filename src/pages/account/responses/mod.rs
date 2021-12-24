pub enum AccountError {
    UniqueViolation,
    NotFound,
    AccountNotFound,
    ExpiredValue,
    GeneriqueError,
    WrongPassword,
}

pub struct AccountResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl AccountResponse {

    pub fn from(first_name: String, last_name: String, email: String) -> Self {
        AccountResponse {
            first_name,
            last_name,
            email,
        }
    }
}