pub struct NewAccountTransfer {
    pub full_name: String,
    pub email: String,
}

impl NewAccountTransfer {

    pub fn from(full_name: String, email: String) -> Self {
        NewAccountTransfer {
            full_name,
            email,
        }
    }
}