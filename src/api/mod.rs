use actix_web::{web};
pub mod error;
pub mod models;

mod account;

pub fn routes(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("api/v1")
           .service(
                web::resource("/account")
                    .route(web::post().to(account::actions::register_new_account))
           )
    );
}
