use actix_web::{web, HttpResponse, Error};

mod account;

pub async fn index() -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().body("Hello"))
}

pub fn routes(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/account")
            .service(
                web::resource("/register")
                    .route(web::get().to(account::handlers::register_new_account_form))
                    .route(web::post().to(account::handlers::register_new_account))
            )
            .service(
                web::resource("/login")
                    .route(web::get().to(account::handlers::login_user_form))
                    .route(web::post().to(account::handlers::login_user))
            )
    );
}