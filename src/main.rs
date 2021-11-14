use actix_web::{App, HttpServer};
use actix_files::Files;
use tera::Tera;

mod api;
mod models;
mod pages;
mod utils;

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


        App::new()
            .data(templates)
            .service(
                Files::new("/static", "static")
                    .prefer_utf8(true)
                    .use_last_modified(true)
            )
            .configure(api::routes)
            .configure(pages::routes)
    };
    
    HttpServer::new(app).bind("127.0.0.1:8000")?.run().await
}
