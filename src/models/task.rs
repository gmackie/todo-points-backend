use crate::{
    config::db::Connection,
    schema::tasks::{self, dsl::*}
};
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    pub points: i32,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub due_by: Option<chrono::NaiveDateTime>,
    pub completed_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
    pub description: String,
    pub completed: bool,
    pub points: i32,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub due_by: Option<chrono::NaiveDateTime>,
    pub completed_at: Option<chrono::NaiveDateTime>,
}


#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct TaskDTO {
    pub description: String,
    pub completed: bool,
    pub points: i32,
    pub user_id: i32,
    pub due_by: Option<chrono::NaiveDateTime>,
    pub completed_at: Option<chrono::NaiveDateTime>,
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
            completed: task.completed,
            points: task.points,
            user_id: task.user_id,
            created_at: chrono::Local::now().naive_local(),
            due_by: task.due_by,
            completed_at: task.completed_at,
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
