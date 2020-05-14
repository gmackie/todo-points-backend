use actix_web::{error, web, Error, HttpResponse, Result};
use serde::Deserialize;

use crate::tasks::db;
use crate::tasks::model::{NewTask};
use crate::api::{redirect_to};

pub async fn index(
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    let tasks = web::block(move || db::get_all_tasks(&pool)).await?;
    Ok(HttpResponse::Ok().json(tasks))
}

pub async fn create(
    params: web::Form<NewTask>,
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    if params.description.is_empty() {
        Ok(HttpResponse::Ok().json("ERROR"))
    } else {
        let inner_params = params.into_inner();
        let description:String = inner_params.description;
        let points:i32 = inner_params.points;
        let user_id:i32 = inner_params.user_id;
        web::block(move || db::create_task(description, points, user_id, &pool))
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
