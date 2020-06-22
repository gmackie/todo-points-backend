use crate::{
    config::db::Pool,
    constants,
    models::{
        label::LabelDTO,
        response::ResponseBody,
    },
    services::label_service,
};
use actix_web::{web, HttpResponse, Result};

// GET api/labels
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match label_service::find_all(&pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/labels/{id}
pub async fn find_by_id(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match label_service::find_by_id(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(label) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, label))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/labels/query/{query}
pub async fn query(query: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match label_service::query(query.into_inner(), &pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/labels
pub async fn insert(new_label: web::Json<LabelDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match label_service::insert(new_label.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// DELETE api/labels/{id}
pub async fn delete(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match label_service::delete(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}
