use crate::{
    config::db::Connection,
    schema::completed_tasks::{self, dsl::*},
    schema::completed_todos::{self, dsl::*},
  };
  use diesel::prelude::*;
  use chrono::Utc;
  
  #[derive(Queryable, Serialize, Deserialize)]
  pub struct Completion {
    pub id: i32,
    pub name: String,
    pub color: String,
    pub user_id: i32,
    pub created_at: chrono::DateTime<Utc>,
  }
  
  #[derive(Insertable, Serialize, Deserialize)]
  #[table_name = "completions"]
  pub struct NewCompletion {
    pub name: String,
    pub color: String,
    pub created_by: i32,
    pub created_at: chrono::DateTime<Utc>,
  }
  
  
  #[derive(AsChangeset, Serialize, Deserialize)]
  #[table_name = "completions"]
  pub struct CompletionDTO {
    pub name: String,
    pub color: String,
    pub created_by: i32,
  }
  
  impl Completion {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Completion>> {
        completions.order(id.asc()).load::<Completion>(conn)
    }
  
    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Completion> {
        completions.find(i).get_result::<Completion>(conn)
    }
  
    pub fn query(_query: String, conn: &Connection) -> QueryResult<Vec<Completion>> {
        completions.order(id.asc()).load::<Completion>(conn)
    }
  
    pub fn insert(completion: CompletionDTO, conn: &Connection) -> QueryResult<usize> {
        let new_completion = NewCompletion {
            name: completion.name,
            color: completion.color,
            created_by: completion.created_by,
            created_at: Utc::now(),
        };
        diesel::insert_into(completions)
            .values(&new_completion)
            .execute(conn)
    }
  
    pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(completions.find(i)).execute(conn)
    }
  }
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
  