use std::ops::Deref;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use crate::tasks::model::{NewTask, Task};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}

pub fn get_all_tasks(pool: &PgPool) -> Result<Vec<Task>, &'static str> {
    Task::all(get_conn(pool)?.deref()).map_err(|_| "Error retreiving tasks")
}

pub fn create_task(todo: String, pts: i32, u_id: i32, pool: &PgPool) -> Result<(), &'static str> {
    let new_task = NewTask { description: todo, points: pts, user_id: u_id };
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