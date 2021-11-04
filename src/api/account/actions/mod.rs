use actix_web::{web, HttpResponse, Error, Result};

use crate::api::models::user::Register;

pub async fn register_new_account(register: web::Json<Register>) -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().Json(register))
}