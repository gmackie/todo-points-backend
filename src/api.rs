use actix_files::NamedFile;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, error, http, web, Error, HttpResponse, Result};
use serde::Deserialize;

use crate::db;

pub async fn index(
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    let tasks = web::block(move || db::get_all_tasks(&pool)).await?;
    Ok(HttpResponse::Ok().json(tasks))
}

#[derive(Deserialize)]
pub struct CreateForm {
    description: String,
}

pub async fn create(
    params: web::Form<CreateForm>,
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    if params.description.is_empty() {
        Ok(HttpResponse::Ok().json("ERROR"))
    } else {
        const points:i32 = 100;
        const user_id:i32 = 1;
        web::block(move || db::create_task(params.into_inner().description, points, user_id, &pool))
            .await?;
        Ok(HttpResponse::Ok().json("ok."))
    }
}

#[derive(Deserialize)]
pub struct UpdateParams {
    id: i32,
}

#[derive(Deserialize)]
pub struct UpdateForm {
    _method: String,
}

pub async fn update(
    db: web::Data<db::PgPool>,
    params: web::Path<UpdateParams>,
    form: web::Form<UpdateForm>,
) -> Result<HttpResponse, Error> {
    match form._method.as_ref() {
        "put" => toggle(db, params).await,
        "delete" => delete(db, params).await,
        unsupported_method => {
            let msg = format!("Unsupported HTTP method: {}", unsupported_method);
            Err(error::ErrorBadRequest(msg))
        }
    }
}

async fn toggle(
    pool: web::Data<db::PgPool>,
    params: web::Path<UpdateParams>,
) -> Result<HttpResponse, Error> {
    web::block(move || db::toggle_task(params.id, &pool)).await?;
    Ok(redirect_to("/"))
}

async fn delete(
    pool: web::Data<db::PgPool>,
    params: web::Path<UpdateParams>,
) -> Result<HttpResponse, Error> {
    web::block(move || db::delete_task(params.id, &pool)).await?;
    Ok(redirect_to("/"))
}

fn redirect_to(location: &str) -> HttpResponse {
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
