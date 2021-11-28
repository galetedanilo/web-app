use dotenv::dotenv;

use std::env::var;

pub fn get_app_name() -> String {
    dotenv().ok();

    var("APP_NAME").unwrap_or_else(|_|"My App".to_string())
}

pub fn get_app_domain_url() -> String {
    dotenv().ok();

    var("APP_DOMAIN_URL").unwrap_or_else(|_| "http://127.0.0.1:8000".to_string())
}

pub fn get_app_domain() -> String {
    dotenv().ok();

    var("APP_DOMAIN").unwrap_or_else(|_| "127.0.0.1".to_string())
}

pub fn get_app_port() -> String {
    dotenv().ok();

    var("APP_PORT").unwrap_or_else(|_| "8000".to_string())
}

pub fn get_database_host() -> String {
    dotenv().ok();

    var("DATABASE_HOST").unwrap_or_else(|_| "172.17.0.2".to_string())
}

pub fn get_database_port() -> u16 {
    dotenv().ok();

    var("DATABASE_PORT").unwrap_or_else(|_| "5432".to_string()).parse().unwrap()

}

pub fn get_database_username() -> String {
    dotenv().ok();

    var("DATABASE_USERNAME").unwrap_or_else(|_| "postgres".to_string())
}

pub fn get_database_password() -> String {
    dotenv().ok();

    var("DATABASE_PASSWORD").unwrap_or_else(|_| "123456".to_string())
}

pub fn get_database_name() -> String {
    dotenv().ok();

    var("DATABASE_NAME").unwrap_or_else(|_| "ead".to_string())
}