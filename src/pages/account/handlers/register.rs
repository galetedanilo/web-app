use actix_web::{error, web, HttpResponse, Error};
use tera::{Context, Tera};

use validator::Validate;

use crate::vars;

use crate::pages::account::actions::{register_new_user_action};
use crate::utils::helper_get_messages;

use crate::models::NewAccountForm;

pub async fn register_new_account_form(template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("title", "Create New Account");
    context.insert("domain_url", &vars::get_domain_url());

    let render = template.render("account/register.html", &context).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(render))
}

pub async fn register_new_account(form: web::Form<NewAccountForm>, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    match form.validate() {
        Ok(_) => {
            register_new_user_action(form.into_inner());

            Ok(HttpResponse::Ok().body("Cadastrar"))
        },
        Err(err) => {
            
            let mut context = Context::new();

            context.insert("title", "Create New Account");
            context.insert("domain_url", &vars::get_domain_url());
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