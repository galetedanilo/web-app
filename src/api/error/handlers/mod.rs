use actix_web::{dev, HttpResponse, Result};
use actix_web::middleware::errhandlers::ErrorHandlerResponse;

pub fn bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
 
    Ok(
        ErrorHandlerResponse::Response(
            res.into_response(
                HttpResponse::BadRequest()
                    .body("Bad Request")
                    .into_body(),
            )
        )
    )
   
}

pub fn page_not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {

    Ok(
        ErrorHandlerResponse::Response(
            res.into_response(
                HttpResponse::NotFound()
                    .body("Found")
                    .into_body(),
            )
        )
    )
}

pub fn method_not_allowed<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {

    Ok(
        ErrorHandlerResponse::Response(
            res.into_response(
                HttpResponse::MethodNotAllowed()
                    .json("Method")
                    .into_body(),
            )
        )
    )
}

pub fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {

    Ok(
        ErrorHandlerResponse::Response(
            res.into_response(
                HttpResponse::InternalServerError()
                    .body("Internal")
                    .into_body(),
            )
        )
    )
}