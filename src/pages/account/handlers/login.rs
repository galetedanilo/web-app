use actix_web::{error, web, HttpResponse, Error};
use tera::{Context, Tera};

use validator::Validate;

use crate::utils::helper_get_messages;

use crate::models::LoginForm;

pub async fn login_user_form(template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("title", "Log In Your Account");

    let render = template.render("account/login.html", &context).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(render))
}

pub async fn login_user(form: web::Form<LoginForm>, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    match form.validate() {
        Ok(_) => Ok(
            HttpResponse::Ok().body("Login")
        ),
        Err(err) => {

            let mut context = Context::new();
            
            context.insert("title", "Log In Your Acount");
            context.insert("email", &form.email);

            let err_resp = helper_get_messages(err);

            context.insert("message_error", &err_resp);

            let render = template.render("account/login.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().body(render))
        },
    }
}