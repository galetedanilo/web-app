use actix_web::{error, web, HttpResponse, HttpRequest, Error};
use tera::{Tera, Context};

use uuid::Uuid;
use validator::Validate;

use crate::utils::helper_get_messages;
use crate::utils::helper_create_email_reset;

use crate::models::{PasswordRequestForm, NewPasswordForm};

use crate::services::send_email;

pub async fn password_reset_form(template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("title", "Reset Password");

    let render = template.render("account/password.html", &context).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(render))
}


pub async fn password_reset_request(form: web::Form<PasswordRequestForm>, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    match form.validate() {
        Ok(_) => {

            let uuid = Uuid::new_v4();
            
            let body = helper_create_email_reset(&uuid.to_string() , "Danilo Galete");

            context.insert("title", "Check Inbox");

            let render = template.render("account/check_inbox.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().body(render))
        },
        Err(err) => {

            context.insert("title", "Reset Password");

            let err_resp = helper_get_messages(err);

            context.insert("message_error", &err_resp);

            let render = template.render("account/password.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().body(render))
        }
    }
}

pub async fn password_setting_form(req: HttpRequest, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().body("Vai Brasil"))
}

pub async fn password_setting(form: web::Form<NewPasswordForm>, req: HttpRequest, template: web::Data<Tera>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Deu"))
}