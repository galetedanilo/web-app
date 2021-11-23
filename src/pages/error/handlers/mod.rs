use actix_web::{dev, web, HttpResponse, Result};
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_files::NamedFile;
use tera::{Context, Tera};

use crate::vars;

pub fn bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {

    render_page_error(
        res, 400,
        "Ooops 400 - Bad Request", 
        "Bad Request!",
        "The server could not understand the request."
    )
}

pub fn page_not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
	
    render_page_error(
        res, 404,
        "Ooops 404 - Page Not Found", 
        "Ooops...",
        "The page you were looking for doesn't exist. You may have mistyped the address or the page may have moved."
    )
}

pub fn method_not_allowed<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {

    render_page_error(
        res, 405,
        "Ooops 405 - Method Not Allowed", 
        "Not Allowed!",
        "The page you are looking for cannot be displayed because an invalid method is being used"
    )
}

pub fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    
    render_page_error(
        res, 500,
        "Ooops 500 - Internal Server Error", 
        "How embarrassing!",
        "Looks like something weird happened while processing your request. Please try again in a few moments."
    )
}

fn render_page_error<B>(res: dev::ServiceResponse<B>, code: u16, title: &str, error: &str, message: &str) -> Result<ErrorHandlerResponse<B>> {

    let request = res.request();

    let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());

    let static_error = NamedFile::open(format!("static/error/{}.html", res.status().as_str()))?
        .set_status_code(res.status())
        .into_response(res.request())?;
        
    match tera {
        Some(tera) => {
    
            let mut context = Context::new();
                
            context.insert("title", title);
            context.insert("domain_url", &vars::get_domain_url());
            context.insert("code", res.status().as_str());
            context.insert("error", error);
            context.insert("message", message);
    
            let render = tera.render("error/error.html", &context);

            match render {
                Ok(render) => Ok(
                    ErrorHandlerResponse::Response(
                        res.into_response(

                            match code {
                                400 => HttpResponse::BadRequest().body(render).into_body(),
                                404 => HttpResponse::NotFound().body(render).into_body(),
                                405 => HttpResponse::MethodNotAllowed().body(render).into_body(),
                                _ => HttpResponse::InternalServerError().body(render).into_body(),
                            }
 
                        )
                    )
                ),
                Err(_) => Ok(
                    ErrorHandlerResponse::Response(
                        res.into_response(static_error.into_body()),
                    )
                ),
            }
        },
        None => Ok(      
            ErrorHandlerResponse::Response(
                res.into_response(static_error.into_body()),
            )
        ),
    }
}