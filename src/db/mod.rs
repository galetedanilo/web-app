use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use sqlx::postgres::PgConnectOptions;

pub mod mirrors;
pub mod models;
pub mod queries;

use crate::vars;

pub fn get_postgres_pool() -> PgPool {

    println!("Open PostgreSQL Connection");
    PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configure_postgres_connection())
}

fn configure_postgres_connection() -> PgConnectOptions {

    PgConnectOptions::new()
        .host(&vars::get_database_host())
        .port(vars::get_database_port())
        .username(&vars::get_database_username())
        .password(&vars::get_database_password())
        .database(&vars::get_database_name())
}
