use actix_web::{error, web, HttpResponse, Error};
use tera::{Tera, Context};

use validator::Validate;

use crate::utils::helper_get_messages;

use crate::models::UserReset;

pub async fn reset_password_form(template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("title", "Reset Password");

    let render = template.render("account/reset.html", &context).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(render))
}


pub async fn reset_password(form: web::Form<UserReset>, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    match form.validate() {
        Ok(_) => Ok(
            HttpResponse::Ok().body("Reset")
        ),
        Err(err) => {

            let mut context = Context::new();

            context.insert("title", "Reset Password");

            let err_resp = helper_get_messages(err);

            context.insert("message_error", &err_resp);

            let render = template.render("account/reset.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().body(render))
        }
    }
}