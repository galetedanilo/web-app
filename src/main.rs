use actix_web::{App, web::Data, HttpServer};
use actix_files::Files;
use tera::Tera;

mod api;
mod db;
mod models;
mod pages;
mod services;
mod utils;
mod vars;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    

    let app = move || {

        let templates = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        let db_postgres = Data::new(db::get_connection_pool());

        App::new()
            .data(templates)
            .app_data(db_postgres.clone())
            .service(
                Files::new("/static", "static")
                    .prefer_utf8(true)
                    .use_last_modified(true)
            )
            .configure(pages::routes)
    };
    
    HttpServer::new(app).bind(
        format!("{}:{}", vars::get_app_domain(), vars::get_app_port())
    )?.run().await
}
