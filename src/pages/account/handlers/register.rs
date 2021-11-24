use actix_web::{error, web, HttpResponse, Error};
use tera::{Context, Tera};

use validator::Validate;

use crate::vars;

use crate::pages::account::actions::{register_new_user_action};
use crate::utils::helper_get_error_messages_validate;

use crate::models::NewAccountForm;

pub async fn register_new_account_form(template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("title", "Create New Account");
    context.insert("domain_url", &vars::get_domain_url());

    let render = template.render("account/register.html", &context).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(render))
}

pub async fn register_new_account(form: web::Form<NewAccountForm>, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("domain_url", &vars::get_domain_url());

    match form.validate() {
        Ok(_) => {
            register_new_user_action(form.into_inner());

            context.insert("title", "Confirm Your Account");
            context.insert("email", "your_account@email.com");

            let render = template.render("account/activate.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Created().body(render))
        },
        Err(err) => {

            context.insert("title", "Create New Account");
            context.insert("first_name", &form.first_name);
            context.insert("last_name", &form.last_name);
            context.insert("email", &form.email);

            let err_resp = helper_get_error_messages_validate(err);

            context.insert("message_error", &err_resp);

            let render = template.render("account/register.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().body(render))
        },
    }

       
}