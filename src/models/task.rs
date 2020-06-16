use crate::{
    config::db::Connection,
    schema::tasks::{self, dsl::*}
};
use diesel::prelude::*;
use chrono::Utc;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub points: i32,
    pub user_id: i32,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
    pub description: String,
    pub points: i32,
    pub user_id: i32,
    pub created_at: chrono::DateTime<Utc>,
}


#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct TaskDTO {
    pub description: String,
    pub points: i32,
    pub user_id: i32,
}

impl Task {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Task>> {
        tasks.order(id.asc()).load::<Task>(conn)
    }

    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Task> {
        tasks.find(i).get_result::<Task>(conn)
    }

    pub fn query(_query: String, conn: &Connection) -> QueryResult<Vec<Task>> {
        tasks.order(id.asc()).load::<Task>(conn)
    }

    pub fn insert(task: TaskDTO, conn: &Connection) -> QueryResult<usize> {
        let new_task = NewTask {
            description: task.description,
            points: task.points,
            user_id: task.user_id,
            created_at: Utc::now(),
        };
        diesel::insert_into(tasks)
            .values(&new_task)
            .execute(conn)
    }

    pub fn update(i: i32, updated_task: TaskDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::update(tasks.find(i))
            .set(&updated_task)
            .execute(conn)
    }

    pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(tasks.find(i)).execute(conn)
    }
}
