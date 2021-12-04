pub struct ConfirmationNew {
    pub user_id: i32,
    pub token: uuid::Uuid,
    pub expires_at: chrono::NaiveDateTime,
}

impl ConfirmationNew {

    pub fn from(user_id: i32) -> Self {
        
        ConfirmationNew {
            user_id,
            token: uuid::Uuid::new_v4(),
            expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
        }
    }
}