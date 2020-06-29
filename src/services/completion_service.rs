use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        completion::{Completion, CompletionDTO},
    }
};
use actix_web::{web, http::StatusCode};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Completion>, ServiceError> {
    match Completion::find_all(&pool.get().unwrap()) {
        Ok(completion) => Ok(completion),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<Completion, ServiceError> {
    match Completion::find_by_id(id, &pool.get().unwrap()) {
        Ok(completion) => Ok(completion),
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Completion with id {} not found", id))),
    }
}

pub fn query(query: String, pool: &web::Data<Pool>) -> Result<Vec<Completion>, ServiceError> {
    match Completion::query(query, &pool.get().unwrap()) {
        Ok(people) => Ok(people),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}

pub fn insert(new_completion: CompletionDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Completion::insert(new_completion, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string())),
    }
}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Completion::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Completion::delete(id, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string())),
        },
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Completion with id {} not found", id))),
    }
}
