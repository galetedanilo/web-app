use actix_web::{error, web, HttpResponse, Error};
use tera::{Context, Tera};

use validator::Validate;

use crate::pages::account::forms::new_account::NewAccountForm;

pub async fn create_user_form(template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("title", "Titulo");

    let render = template.render("account/account.html", &context).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(render))
}

pub async fn create_user(form: web::Form<NewAccountForm>) -> Result<HttpResponse, Error> {

    form.validate().map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::Ok().body("Cadastrar"))    
}