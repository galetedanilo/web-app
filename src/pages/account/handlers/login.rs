use actix_web::{error, web, HttpResponse, Error};
use tera::{Context, Tera};

use validator::Validate;

use crate::vars;

use crate::utils::helper_get_error_messages_validate;

use crate::models::account::LoginForm;

pub async fn login_user_form_handler(template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("title", "Log In Your Account");
    context.insert("domain_url", &vars::get_domain_url());

    let render = template.render("account/login.html", &context).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(render))
}

pub async fn login_user_handler(form: web::Form<LoginForm>, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("domain_url", &vars::get_domain_url());

    match form.validate() {
        Ok(_) => Ok(
            HttpResponse::Ok().body("Login")
        ),
        Err(err) => {

            context.insert("title", "Log In Your Acount");
            context.insert("email", &form.email);

            let err_resp = helper_get_error_messages_validate(err);

            context.insert("message_error", &err_resp);

            let render = template.render("account/login.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().body(render))
        },
    }
}