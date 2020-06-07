use crate::{
    config::db::Pool,
    constants,
    models::{
        task::TaskDTO,
        response::ResponseBody,
    },
    services::task_service,
};
use actix_web::{web, HttpResponse, Result};

// GET api/tasks
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match task_service::find_all(&pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/tasks/{id}
pub async fn find_by_id(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match task_service::find_by_id(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(task) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, task))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/tasks/query/{query}
pub async fn query(query: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match task_service::query(query.into_inner(), &pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/tasks
pub async fn insert(new_task: web::Json<TaskDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match task_service::insert(new_task.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// PUT api/tasks/{id}
pub async fn update(id: web::Path<String>, updated_task: web::Json<TaskDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match task_service::update(id.into_inner().parse::<i32>().unwrap(), updated_task.0, &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/tasks/{id}/complete
pub async fn complete(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match task_service::complete(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// DELETE api/tasks/{id}
pub async fn delete(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match task_service::delete(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}
