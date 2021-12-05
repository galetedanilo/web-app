use sqlx::PgPool;
use actix_web::{error, web, HttpResponse, Error};
use tera::{Context, Tera};

use validator::Validate;

use crate::vars;

use crate::pages::account::actions::{
    account_confirmation_action,
    account_register_action
};

use crate::pages::account::forms::AccountForm;
use crate::pages::account::responses::AccountError;

use crate::utils::helper_get_error_messages_validate;

pub async fn account_confirmation_handler(uuid: web::Path<uuid::Uuid>, pool: web::Data<PgPool>, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("domain_url", &vars::get_app_domain_url());
    
    match account_confirmation_action(&uuid, pool.get_ref()).await {
        Ok(account) => {
            Ok(HttpResponse::Ok().body("Conf"))
        },
        Err(err) => {
            match err {
                AccountError::ExpiredValue => {

                    context.insert("title", "Account Activation Update");

                    let render = template.render("account/expired_activation.html", &context).map_err(error::ErrorInternalServerError)?;

                    Ok(HttpResponse::Ok().body(render))
                },
                _ => Err(error::ErrorBadRequest("Bad Request"))
            }
        }
    }
}

pub async fn account_register_form_handler(template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("title", "Create New Account");
    context.insert("domain_url", &vars::get_app_domain_url());

    let render = template.render("account/register.html", &context).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(render))
}

pub async fn account_register_handler(form: web::Form<AccountForm>, pool: web::Data<PgPool>, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("domain_url", &vars::get_app_domain_url());

    match form.validate() {
        Ok(_) => {

            match account_register_action(&form, pool.get_ref()).await {
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
                        _ => {

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