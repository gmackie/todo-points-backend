use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        label::{Label, LabelDTO},
    }
};
use actix_web::{web, http::StatusCode};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Label>, ServiceError> {
    match Label::find_all(&pool.get().unwrap()) {
        Ok(label) => Ok(label),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<Label, ServiceError> {
    match Label::find_by_id(id, &pool.get().unwrap()) {
        Ok(label) => Ok(label),
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Label with id {} not found", id))),
    }
}

pub fn query(query: String, pool: &web::Data<Pool>) -> Result<Vec<Label>, ServiceError> {
    match Label::query(query, &pool.get().unwrap()) {
        Ok(people) => Ok(people),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}

pub fn insert(new_label: LabelDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Label::insert(new_label, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string())),
    }
}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Label::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Label::delete(id, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string())),
        },
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Label with id {} not found", id))),
    }
}
