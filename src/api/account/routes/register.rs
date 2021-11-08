use actix_web::{error, web, HttpResponse, Error, Result};
use validator::Validate;

use crate::api::account::models::payloads::NewAccount;

pub async fn register_new_account(register: web::Json<NewAccount>) -> Result<HttpResponse, Error> {

    match register.validate() {
        Ok(_) => Ok(HttpResponse::Ok().body("Ok")),
        Err(err) => Err(error::ErrorBadRequest(err)),
    }

}