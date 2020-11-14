use crate::{
    config::db::Pool,
    constants,
    models::{
        todo::TodoDTO,
        todo::NewTodo,
        response::ResponseBody,
    },
    services::todo_service,
};
use actix_web::{web, HttpResponse, Result};

// GET api/todos
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match todo_service::find_all(&pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/todos/{id}
pub async fn find_by_id(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match todo_service::find_by_id(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(todo) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, todo))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/todos/query/{query}
pub async fn query(query: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match todo_service::query(query.into_inner(), &pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/todos
pub async fn insert(new_todo: web::Json<NewTodo>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match todo_service::insert(new_todo.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// PUT api/todos/{id}
pub async fn update(id: web::Path<String>, updated_todo: web::Json<TodoDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match todo_service::update(id.into_inner().parse::<i32>().unwrap(), updated_todo.0, &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/todos/{id}/complete
pub async fn complete(_id: web::Path<String>, _pool: web::Data<Pool>) -> Result<HttpResponse> {
    // match todo_service::complete(id.into_inner().parse::<i32>().unwrap(), &pool) {
    //     Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
    //     Err(err) => Ok(err.response()),
    // }
    Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
}

// DELETE api/todos/{id}
pub async fn delete(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match todo_service::delete(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}
