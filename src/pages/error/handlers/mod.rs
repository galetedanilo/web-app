use actix_web::{dev, web, HttpResponse, Result};
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_files::NamedFile;
use tera::{Context, Tera};

pub fn bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {

    let request = res.request();

    let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());

    let static_error = NamedFile::open("static/error/400.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;

    match tera {
        Some(tera) => {

            let mut context = Context::new();
            
            context.insert("title", "Ooops 400 - Bad Request");
            context.insert("code", "400");
            context.insert("error", "Bad Request!");
            context.insert("message", "The server could not understand the request.");

            let render = tera.render("error/error.html", &context);

            match render {
                Ok(render) => Ok(
                    ErrorHandlerResponse::Response(
                        res.into_response(
                            HttpResponse::BadRequest()
                                .body(render)
                                .into_body(),
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

pub fn page_not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
	
    let request = res.request();

    let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());

    let static_error = NamedFile::open("static/error/404.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;

    match tera {
        Some(tera) => {

            let mut context = Context::new();
            
            context.insert("title", "Ooops 404 - Page Not Found");
            context.insert("code", "404");
            context.insert("error", "Ooops...");
            context.insert("message", "The page you were looking for doesn't exist. You may have mistyped the address or the page may have moved.");

            let render = tera.render("error/error.html", &context);

            match render {
                Ok(render) => Ok(
                    ErrorHandlerResponse::Response(
                        res.into_response(
                            HttpResponse::NotFound()
                                .body(render)
                                .into_body(),
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

pub fn method_not_allowed<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {

    let request = res.request();

    let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());

    let static_error = NamedFile::open("static/error/405.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;

    match tera {
        Some(tera) => {

            let mut context = Context::new();
            
            context.insert("title", "Ooops 405 - Method Not Allowed");
            context.insert("code", "405");
            context.insert("error", "Not Allowed!");
            context.insert("message", "The page you are looking for cannot be displayed because an invalid method is being used");

            let render = tera.render("error/error.html", &context);

            match render {
                Ok(render) => Ok(
                    ErrorHandlerResponse::Response(
                        res.into_response(
                            HttpResponse::MethodNotAllowed()
                                .body(render)
                                .into_body(),
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

pub fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    
    let request = res.request();

    let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());

    let static_error = NamedFile::open("static/error/500.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;

    match tera {
        Some(tera) => {

            let mut context = Context::new();
            
            context.insert("title", "Ooops 500 - Internal Server Error");
            context.insert("code", "500");
            context.insert("error", "How embarrassing!");
            context.insert("message", "Looks like something weird happened while processing your request. Please try again in a few moments.");

            let render = tera.render("error/error.html", &context);

            match render {
                Ok(render) => Ok(
                    ErrorHandlerResponse::Response(
                        res.into_response(
                            HttpResponse::InternalServerError()
                                .body(render)
                                .into_body(),
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