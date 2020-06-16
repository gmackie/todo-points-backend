use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        todo::{Todo, TodoDTO},
    }
};
use actix_web::{web, http::StatusCode};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Todo>, ServiceError> {
    match Todo::find_all(&pool.get().unwrap()) {
        Ok(todo) => Ok(todo),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<Todo, ServiceError> {
    match Todo::find_by_id(id, &pool.get().unwrap()) {
        Ok(todo) => Ok(todo),
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Todo with id {} not found", id))),
    }
}

pub fn query(query: String, pool: &web::Data<Pool>) -> Result<Vec<Todo>, ServiceError> {
    match Todo::query(query, &pool.get().unwrap()) {
        Ok(people) => Ok(people),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}

pub fn insert(new_todo: TodoDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Todo::insert(new_todo, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string())),
    }
}

pub fn update(id: i32, updated_todo: TodoDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Todo::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Todo::update(id, updated_todo, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string())),
        },
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Todo with id {} not found", id))),
    }
}

// pub fn complete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
//     match Todo::find_by_id(id, &pool.get().unwrap()) {
//         Ok(_) => match Todo::complete(id, &pool.get().unwrap()) {
//             Ok(_) => Ok(()),
//             Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string())),
//         },
//         Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Todo with id {} not found", id))),
//     }
// }

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Todo::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Todo::delete(id, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string())),
        },
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Todo with id {} not found", id))),
    }
}
