use crate::{
    config::db::Pool,
    constants,
    models::{
        completion::CompletionDTO,
        response::ResponseBody,
    },
    services::completion_service,
};
use actix_web::{web, HttpResponse, Result};

// GET api/completions
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match completion_service::find_all(&pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/completions/{id}
pub async fn find_by_id(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match completion_service::find_by_id(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(completion) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, completion))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/completions/query/{query}
pub async fn query(query: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match completion_service::query(query.into_inner(), &pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/completions
pub async fn insert(new_completion: web::Json<CompletionDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match completion_service::insert(new_completion.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// DELETE api/completions/{id}
pub async fn delete(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match completion_service::delete(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}
