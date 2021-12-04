use sqlx::PgPool;
use actix_web::{error, web, HttpResponse, Error};
use tera::{Context, Tera};

use validator::Validate;

use crate::vars;

use crate::pages::account::actions::{register_new_account_action};
use crate::utils::helper_get_error_messages_validate;

use crate::pages::account::forms::AccountForm;
use crate::pages::account::responses::AccountError;

pub async fn register_new_account_form_handler(template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("title", "Create New Account");
    context.insert("domain_url", &vars::get_app_domain_url());

    let render = template.render("account/register.html", &context).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(render))
}

pub async fn register_new_account_handler(form: web::Form<AccountForm>, pool: web::Data<PgPool>, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("domain_url", &vars::get_app_domain_url());

    match form.validate() {
        Ok(_) => {

            match register_new_account_action(&form, pool.get_ref()).await {
                Ok(account) => {
                    context.insert("title", "Confirm Your Account");
                    context.insert("email", &account.email);
        
                    let render = template.render("account/activate.html", &context).map_err(error::ErrorInternalServerError)?;
        
                    Ok(HttpResponse::Created().body(render))
                },
                Err(err) => {

                    match err {
                        AccountError::UniqueViolation => {

                            context.insert("title", "Create New Account");
                            context.insert("first_name", &form.first_name.trim());
                            context.insert("last_name", &form.last_name.trim());
                            context.insert("email", &form.email.trim());

                            context.insert("message_error", &vec!["This email address has been taken by another account."]);

                            let render = template.render("account/register.html", &context).map_err(error::ErrorInternalServerError)?;

                            Ok(HttpResponse::Ok().body(render))
                        },
                        AccountError::GeneriqueError => {

                            Err(error::ErrorInternalServerError("Internal Server Error"))
                        },
                    }
                }
            }
        },
        Err(err) => {

            let err_resp = helper_get_error_messages_validate(err);

            context.insert("title", "Create New Account");
            context.insert("first_name", &form.first_name.trim());
            context.insert("last_name", &form.last_name.trim());
            context.insert("email", &form.email.trim());

            context.insert("message_error", &err_resp);

            let render = template.render("account/register.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().body(render))
        },
    }     
}