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

            let render = tera.render("error/400.html", &context);

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

            let render = tera.render("error/404.html", &context);

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

            let render = tera.render("error/405.html", &context);

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

            context.insert("title", "Titulo");

            let render = tera.render("/error/500.html", &context);

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
                )
            }
        },
        None => Ok(
            ErrorHandlerResponse::Response(
                res.into_response(static_error.into_body()),
            )
        ),
    }
}