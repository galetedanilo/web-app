use actix_web::{http, web, App, HttpServer};
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_files::Files;
use tera::Tera;

mod pages;
mod api;

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

        let error_handlers = ErrorHandlers::new()
            .handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                api::error::handlers::internal_server_error
            )
            .handler(
                http::StatusCode::NOT_FOUND,
                api::error::handlers::page_not_found
            )
            .handler(
                http::StatusCode::METHOD_NOT_ALLOWED,
                api::error::handlers::method_not_allowed
            )
            .handler(
                http::StatusCode::BAD_REQUEST, 
                api::error::handlers::bad_request
            );

        App::new()
            .data(templates)
            .wrap(error_handlers)
            .route("/", web::get().to(pages::index))
            .configure(pages::routes)
            .service(Files::new("/static", "static")
                        .prefer_utf8(true)
                        .use_last_modified(true))
    };
    
    HttpServer::new(app).bind("127.0.0.1:8000")?.run().await
}
