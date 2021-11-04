use actix_web::{HttpResponse, Error};

pub async fn login_user_form() -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().body("Formulario"))
}

pub async fn login_user() -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().body("Logar"))
}