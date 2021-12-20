use actix_web::{error, web, HttpResponse, HttpRequest, Error};
use tera::{Tera, Context};

use validator::Validate;

use crate::vars;

use crate::utils::helper_get_error_messages_validate;

use crate::pages::account::forms::{EmailForm, NewPasswordForm};

pub async fn password_reset_form_handler(template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("title", "Reset Password");
    context.insert("domain_url", &vars::get_app_domain_url());

    let render = template.render("account/password.html", &context).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(render))
}


pub async fn password_reset_handler(form: web::Form<EmailForm>, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("domain_url", &vars::get_app_domain_url());
    context.insert("title", "Reset Password");

    match form.validate() {
        Ok(_) => {

            let render = template.render("account/messages/reset.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().body(render))
        },
        Err(err) => {

            let err_resp = helper_get_error_messages_validate(err);

            context.insert("message_error", &err_resp);

            let render = template.render("account/password.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().body(render))
        }
    }
}

pub async fn password_change_form_handler(req: HttpRequest, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().body("Vai Brasil"))
}

pub async fn password_change_handler(form: web::Form<NewPasswordForm>, req: HttpRequest, template: web::Data<Tera>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Deu"))
}