use actix_web::{error, web, HttpResponse, HttpRequest, Error};
use tera::{Tera, Context};

use validator::Validate;

use crate::vars;

use crate::utils::helper_get_error_messages_validate;

use crate::models::account::{PasswordForm, NewPasswordForm};

pub async fn password_reset_form_handler(template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("title", "Reset Password");
    context.insert("domain_url", &vars::get_domain_url());

    let render = template.render("account/reset.html", &context).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(render))
}


pub async fn password_reset_handler(form: web::Form<PasswordForm>, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("domain_url", &vars::get_domain_url());

    match form.validate() {
        Ok(_) => {

            context.insert("title", "Check Inbox");

            let render = template.render("account/check_inbox.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().body(render))
        },
        Err(err) => {

            context.insert("title", "Reset Password");

            let err_resp = helper_get_error_messages_validate(err);

            context.insert("message_error", &err_resp);

            let render = template.render("account/reset.html", &context).map_err(error::ErrorInternalServerError)?;

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