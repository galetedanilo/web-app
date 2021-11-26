use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::{http, web, HttpResponse, Error};

mod account;
mod error;

pub fn routes(cfg: &mut web::ServiceConfig) {

    let error_handlers = ErrorHandlers::new()
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
        web::scope("")
        .wrap(error_handlers)
        .route("/", web::get().to(index))
        .service(
            web::scope("/account")
            .service(
                web::resource("/register")
                    .route(web::get().to(account::handlers::register_new_account_form_handler))
                    .route(web::post().to(account::handlers::register_new_account_handler))
            )
            .service(
                web::resource("/login")
                    .route(web::get().to(account::handlers::login_user_form_handler))
                    .route(web::post().to(account::handlers::login_user_handler))
            )
            .service(
                web::resource("/password/reset")
                    .route(web::get().to(account::handlers::password_reset_form_handler))
                    .route(web::post().to(account::handlers::password_reset_handler))
            )
            .service(
                web::resource("/password/reset/{token}")
                    .route(web::get().to(account::handlers::password_change_form_handler))
                    .route(web::post().to(account::handlers::password_change_handler))
            )
        )
    );
}

async fn index() -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().body("Hello"))
}