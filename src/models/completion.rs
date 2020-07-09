use crate::{
    config::db::Connection,
    schema::completed_tasks::{self, dsl::*},
};
use diesel::prelude::*;
use chrono::Utc;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Completion {
    pub id: i32,
    pub task_id: i32,
    pub completed_at: chrono::DateTime<Utc>,
    pub user_id: i32,
    pub points: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "completed_tasks"]
pub struct NewCompletion {
    pub task_id: i32,
    pub completed_at: chrono::DateTime<Utc>,
    pub user_id: i32,
    pub points: i32,
}


#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "completed_tasks"]
pub struct CompletionDTO {
    pub task_id: i32,
    pub user_id: i32,
    pub points: i32,
}

impl Completion {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Completion>> {
        completed_tasks.order(id.asc()).load::<Completion>(conn)
    }

    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Completion> {
        completed_tasks.find(i).get_result::<Completion>(conn)
    }

    pub fn query(_query: String, conn: &Connection) -> QueryResult<Vec<Completion>> {
        completed_tasks.order(id.asc()).load::<Completion>(conn)
    }

    pub fn insert(completion: CompletionDTO, conn: &Connection) -> QueryResult<usize> {
        let new_completion = NewCompletion {
            task_id: completion.task_id,
            user_id: completion.user_id,
            points: completion.points,
            completed_at: Utc::now(),
        };
        diesel::insert_into(completed_tasks)
            .values(&new_completion)
            .execute(conn)
    }

    pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(completed_tasks.find(i)).execute(conn)
    }
}