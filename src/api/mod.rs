use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::{http, web};

pub mod error;

mod account;

pub fn routes(cfg: &mut web::ServiceConfig) {

    let erros_handlers = ErrorHandlers::new()
        .handler(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            error::handlers::internal_server_error
        )
        .handler(
            http::StatusCode::NOT_FOUND,
            error::handlers::page_not_found
        )
        .handler(
            http::StatusCode::METHOD_NOT_ALLOWED,
            error::handlers::method_not_allowed
        )
        .handler(
            http::StatusCode::BAD_REQUEST,
            error::handlers::bad_request
        );

    cfg.service(
        web::scope("/api")
        .wrap(erros_handlers)
        .service(
            web::scope("/v1")
            .service(
                web::scope("/account")
                .service(
                    web::resource("/register")
                        .route(web::post().to(account::routes::register_new_account))
                )            
            )   
        )
    );
}
