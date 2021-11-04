use actix_web::{error, web, HttpResponse, Error};
use tera::{Context, Tera};

use validator::Validate;

use crate::pages::account::forms::register::NewAccountForm;

pub async fn register_new_account_form(template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("page_title", "Create New Account");

    let render = template.render("account/register.html", &context).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(render))
}

pub async fn register_new_account(form: web::Form<NewAccountForm>) -> Result<HttpResponse, Error> {

    match form.validate() {
        Ok(_) => Ok(
            HttpResponse::Ok().body("Cadastrar")
        ),
        Err(err) => {
            Ok(HttpResponse::Ok().body(format!("Error: {}", err)))
        },
    }

       
}