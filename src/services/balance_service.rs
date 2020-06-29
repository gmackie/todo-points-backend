use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        balance::{Balance, BalanceDTO},
    }
};
use actix_web::{web, http::StatusCode};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Balance>, ServiceError> {
    match Balance::find_all(&pool.get().unwrap()) {
        Ok(balance) => Ok(balance),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<Balance, ServiceError> {
    match Balance::find_by_id(id, &pool.get().unwrap()) {
        Ok(balance) => Ok(balance),
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Balance with id {} not found", id))),
    }
}

pub fn query(query: String, pool: &web::Data<Pool>) -> Result<Vec<Balance>, ServiceError> {
    match Balance::query(query, &pool.get().unwrap()) {
        Ok(people) => Ok(people),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}

pub fn insert(new_balance: BalanceDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Balance::insert(new_balance, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string())),
    }
}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Balance::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Balance::delete(id, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string())),
        },
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Balance with id {} not found", id))),
    }
}
