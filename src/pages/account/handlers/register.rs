use actix_web::{error, web, HttpResponse, Error};
use tera::{Context, Tera};

use validator::Validate;

use crate::pages::account::forms::NewAccountForm;

pub async fn register_new_account_form(template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("title", "Create New Account");

    let render = template.render("account/register.html", &context).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(render))
}

pub async fn register_new_account(form: web::Form<NewAccountForm>, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    match form.validate() {
        Ok(_) => Ok(
            HttpResponse::Ok().body("Cadastrar")
        ),
        Err(err) => {
            
            let mut err_resp: Vec<String> = Vec::new();
            let mut context = Context::new();

            context.insert("title", "Create New Account");

            for (_key, value) in &err.field_errors() {
                for ex in value.into_iter() {
                    err_resp.push(ex.to_string());
                }
            }

            context.insert("message_error", &err_resp);

            let render = template.render("account/register.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().body(render))
        },
    }

       
}