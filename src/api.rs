use actix_files::NamedFile;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, http, HttpResponse, Result};

pub fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}

pub fn bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/400.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

pub fn not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

pub fn internal_server_error<B>(
    res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/500.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}
