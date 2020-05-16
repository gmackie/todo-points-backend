use std::ops::Deref;

use crate::tasks::model::{NewTask, Task};
use crate::db::{get_conn, PgPool};

pub fn get_all_tasks(pool: &PgPool) -> Result<Vec<Task>, &'static str> {
    Task::all(get_conn(pool)?.deref()).map_err(|_| "Error retreiving tasks")
}

pub fn create_task(todo: String, pts: i32, u_id: i32, pool: &PgPool) -> Result<(), &'static str> {
    let new_task = NewTask { 
        description: todo,
        points: pts,
        user_id: u_id,
        created_at: chrono::Local::now().naive_local(),
        due_by: None,
    };
    Task::insert(new_task, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error inserting task")
}

pub fn toggle_task(id: i32, pool: &PgPool) -> Result<(), &'static str> {
    Task::toggle_with_id(id, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error updating task")
}

pub fn delete_task(id: i32, pool: &PgPool) -> Result<(), &'static str> {
    Task::delete_with_id(id, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error deleting task")
}
