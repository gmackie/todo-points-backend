use crate::{
    config::db::Pool,
    constants,
    models::{
        balance::BalanceDTO,
        response::ResponseBody,
    },
    services::balance_service,
};
use actix_web::{web, HttpResponse, Result};

// GET api/balances
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match balance_service::find_all(&pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/balances/{id}
pub async fn find_by_id(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match balance_service::find_by_id(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(balance) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, balance))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/balances/query/{query}
pub async fn query(query: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match balance_service::query(query.into_inner(), &pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/balances
pub async fn insert(new_balance: web::Json<BalanceDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match balance_service::insert(new_balance.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// DELETE api/balances/{id}
pub async fn delete(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match balance_service::delete(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}
