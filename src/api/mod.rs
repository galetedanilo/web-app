use actix_web::{web};
pub mod error;

mod account;

pub fn routes(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("api/v1")
           .service(
                web::resource("/account")
                    .route(web::post().to(account::routes::register_new_account))
           )
    );
}
