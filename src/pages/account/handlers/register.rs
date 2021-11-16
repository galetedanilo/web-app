use actix_web::{error, web, HttpResponse, Error};
use tera::{Context, Tera};

use validator::Validate;

use crate::utils::helper_get_messages;

use crate::models::NewAccountForm;

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
            
            let mut context = Context::new();

            context.insert("title", "Create New Account");
            context.insert("first_name", &form.first_name);
            context.insert("last_name", &form.last_name);
            context.insert("email", &form.email);

            let err_resp = helper_get_messages(err);

            context.insert("message_error", &err_resp);

            let render = template.render("account/register.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().body(render))
        },
    }

       
}