pub enum AccountError {
    UniqueViolation,
    NotFound,
    ExpiredValue,
    GeneriqueError,
}

pub struct AccountResponse {
    pub full_name: String,
    pub email: String,
}

impl AccountResponse {

    pub fn from(full_name: String, email: String) -> Self {
        AccountResponse {
            full_name,
            email,
        }
    }
}