use actix_web::{error, web, Error, HttpResponse, Result};
use serde::Deserialize;

use crate::api::redirect_to;

use crate::db;
use crate::users::db as users_db;
use crate::users::model::NewUser;

pub async fn index(
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    let users = web::block(move || users_db::get_all_users(&pool)).await?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn create(
    params: web::Form<NewUser>,
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    if params.username.is_empty() {
        Ok(HttpResponse::Ok().json("ERROR"))
    } else {
        let new_user: NewUser = params.into_inner();
        web::block(move || users_db::create_user(new_user, &pool))
            .await?;
        Ok(HttpResponse::Ok().json("ok."))
    }
}

#[derive(Deserialize)]
pub struct UpdateParams {
    id: i32,
    points: i32,
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
        "put" => add_points(db, params).await,
        "delete" => delete(db, params).await,
        unsupported_method => {
            let msg = format!("Unsupported HTTP method: {}", unsupported_method);
            Err(error::ErrorBadRequest(msg))
        }
    }
}

async fn add_points(
    pool: web::Data<db::PgPool>,
    params: web::Path<UpdateParams>,
) -> Result<HttpResponse, Error> {
    web::block(move || users_db::add_points(params.id, params.points, &pool)).await?;
    Ok(redirect_to("/"))
}

async fn delete(
    pool: web::Data<db::PgPool>,
    params: web::Path<UpdateParams>,
) -> Result<HttpResponse, Error> {
    web::block(move || users_db::delete_user(params.id, &pool)).await?;
    Ok(redirect_to("/"))
}
