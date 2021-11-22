use dotenv::dotenv;

use std::env::var;

pub fn get_app_name() -> String {
    dotenv().ok();

    var("APP_NAME").unwrap_or_else(|_|"My App".to_string())
}

pub fn get_domain_url() -> String {
    dotenv().ok();

    var("DOMAIN_URL").unwrap_or_else(|_| "http://127.0.0.1:8000".to_string())
}

pub fn get_domain() -> String {
    dotenv().ok();

    var("DOMAIN").unwrap_or_else(|_| "127.0.0.1".to_string())
}

pub fn get_port() -> String {
    dotenv().ok();

    var("PORT").unwrap_or_else(|_| "8000".to_string())
}
