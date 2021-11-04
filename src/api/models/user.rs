use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize)]
pub struct Register {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    email: String,
    password: String,
}