use actix_web::{web, HttpResponse, Error};

mod account;

pub async fn index() -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().body("Hello"))
}

pub fn routes(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/account")
            .service(
                web::resource("/sign_up")
                    .route(web::get().to(account::handlers::sign_up::create_user_form))
                    .route(web::post().to(account::handlers::sign_up::create_user))
            )
            .service(
                web::resource("/sign_in")
                    .route(web::get().to(account::handlers::sign_in::login_user_form))
                    .route(web::post().to(account::handlers::sign_in::login_user))
            )
    );
}