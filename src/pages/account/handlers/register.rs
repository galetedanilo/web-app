use sqlx::PgPool;
use actix_web::{error, web, HttpResponse, Error};
use tera::{Context, Tera};

use validator::Validate;

use crate::vars;

use crate::pages::account::forms::EmailForm;
use crate::pages::account::actions::{
    account_activate_expired_action,
    account_activate_action,
    account_register_action
};

use crate::pages::account::forms::AccountForm;
use crate::pages::account::responses::AccountError;

use crate::utils::helper_get_error_messages_validate;

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
                    context.insert("first_name", account.first_name.trim());
                    context.insert("last_name", account.last_name.trim());
                    context.insert("email", account.email.trim());
        
                    let render = template.render("account/create_new_account.html", &context).map_err(error::ErrorInternalServerError)?;
        
                    Ok(HttpResponse::Created().body(render))
                },
                Err(err) => {

                    match err {
                        AccountError::UniqueViolation => {

                            context.insert("title", "Create New Account");
                            context.insert("first_name", form.first_name.trim());
                            context.insert("last_name", form.last_name.trim());
                            context.insert("email", form.email.trim());

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
            context.insert("first_name", form.first_name.trim());
            context.insert("last_name", form.last_name.trim());
            context.insert("email", form.email.trim());

            context.insert("message_error", &err_resp);

            let render = template.render("account/register.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().body(render))
        },
    }     
}

pub async fn account_activate_expired_form_handler(template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("domain_url", &vars::get_app_domain_url());
    context.insert("title", "Account Activation Update");

    let render = template.render("account/activate_account_expired.html", &context).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(render))
}

pub async fn account_activate_expired_handler(form: web::Form<EmailForm>, pool: web::Data<PgPool>, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("domain_url", &vars::get_app_domain_url());

    match form.validate() {
        Ok(_) => {

            match account_activate_expired_action(&form, pool.get_ref()).await {
                Ok(account) => {

                    context.insert("title", "Confirm Your Account");
                    context.insert("first_name", account.first_name.trim());
                    context.insert("last_name", account.last_name.trim());
                    context.insert("email", account.email.trim());
        
                    let render = template.render("account/create_new_account.html", &context).map_err(error::ErrorInternalServerError)?;
        
                    Ok(HttpResponse::Ok().body(render))
                },
                Err(err) => {
                    match err {
                        AccountError::NotFound => {
                            context.insert("title", "Account Activation Update");
                            context.insert("message_error", &vec!["There is no account for the email entered"]);

                            let render = template.render("account/activate_account_expired.html", &context).map_err(error::ErrorInternalServerError)?;

                            Ok(HttpResponse::Ok().body(render))
                        },
                        _ => Err(error::ErrorInternalServerError("Internal Server Error"))
                    }
                }
            }
        },
        Err(err) => {
            let err_resp = helper_get_error_messages_validate(err);

            context.insert("title", "Account Activation Update");
            context.insert("message_error", &err_resp);

            let render = template.render("account/activate_expired.html", &context).map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().body(render))
        }
    }
}

pub async fn account_activate_handler(uuid: web::Path<uuid::Uuid>, pool: web::Data<PgPool>, template: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    context.insert("domain_url", &vars::get_app_domain_url());
    
    match account_activate_action(&uuid, pool.get_ref()).await {
        Ok(account) => {
            
            context.insert("title", "Confirm Your Account");
            context.insert("first_name", account.first_name.trim());
            context.insert("last_name", account.last_name.trim());
            context.insert("web_app", &vars::get_app_name());
        
            let render = template.render("account/account_activated.html", &context).map_err(error::ErrorInternalServerError)?;
            
            Ok(HttpResponse::Ok().body(render))
        },
        Err(err) => {
            match err {
                AccountError::ExpiredValue => {

                    context.insert("title", "Account Activation Update");

                    let render = template.render("account/activate_account_expired.html", &context).map_err(error::ErrorInternalServerError)?;

                    Ok(HttpResponse::Ok().body(render))
                },
                _ => Err(error::ErrorBadRequest("Bad Request"))
            }
        }
    }
}