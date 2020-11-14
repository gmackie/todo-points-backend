use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        task::{Task, TaskDTO, NewTask},
    }
};
use actix_web::{web, http::StatusCode};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Task>, ServiceError> {
    match Task::find_all(&pool.get().unwrap()) {
        Ok(task) => Ok(task),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<Task, ServiceError> {
    match Task::find_by_id(id, &pool.get().unwrap()) {
        Ok(task) => Ok(task),
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Task with id {} not found", id))),
    }
}

pub fn query(query: String, pool: &web::Data<Pool>) -> Result<Vec<Task>, ServiceError> {
    match Task::query(query, &pool.get().unwrap()) {
        Ok(people) => Ok(people),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}

pub fn insert(new_task: NewTask, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Task::insert(new_task, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string())),
    }
}

pub fn update(id: i32, updated_task: TaskDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Task::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Task::update(id, updated_task, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string())),
        },
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Task with id {} not found", id))),
    }
}

// pub fn complete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
//     match Task::find_by_id(id, &pool.get().unwrap()) {
//         Ok(_) => match Task::complete(id, &pool.get().unwrap()) {
//             Ok(_) => Ok(()),
//             Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string())),
//         },
//         Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Task with id {} not found", id))),
//     }
// }

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Task::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Task::delete(id, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string())),
        },
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Task with id {} not found", id))),
    }
}
