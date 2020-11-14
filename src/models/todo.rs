use crate::{
    config::db::Connection,
    schema::todos::{self, dsl::*}
};
use diesel::prelude::*;
use chrono::Utc;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub points: i32,
    pub user_id: i32,
    pub created_at: chrono::DateTime<Utc>,
    pub completed: bool,
    pub completed_at: Option<chrono::DateTime<Utc>>,
    pub due_by: Option<chrono::DateTime<Utc>>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "todos"]
pub struct NewDbTodo {
    pub description: String,
    pub points: i32,
    pub user_id: i32,
    pub created_at: chrono::DateTime<Utc>,
    pub completed: bool,
    pub due_by: Option<chrono::DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct NewTodo {
    pub description: String,
    pub points: i32,
    pub user_id: i32,
    pub due_by: Option<chrono::DateTime<Utc>>,
}


#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "todos"]
pub struct TodoDTO {
    pub description: String,
    pub points: i32,
    pub user_id: i32,
    pub due_by: Option<chrono::DateTime<Utc>>,
    pub completed: bool,
    pub completed_at: Option<chrono::DateTime<Utc>>,
}

impl Todo {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Todo>> {
        todos.order(id.asc()).load::<Todo>(conn)
    }

    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Todo> {
        todos.find(i).get_result::<Todo>(conn)
    }

    pub fn query(_query: String, conn: &Connection) -> QueryResult<Vec<Todo>> {
        todos.order(id.asc()).load::<Todo>(conn)
    }

    pub fn insert(todo: NewTodo, conn: &Connection) -> QueryResult<usize> {
        let new_todo = NewDbTodo {
            description: todo.description,
            points: todo.points,
            user_id: todo.user_id,
            created_at: Utc::now(),
            due_by: todo.due_by,
            completed: false,
        };
        diesel::insert_into(todos)
            .values(&new_todo)
            .execute(conn)
    }

    pub fn update(i: i32, updated_todo: TodoDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::update(todos.find(i))
            .set(&updated_todo)
            .execute(conn)
    }

    pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(todos.find(i)).execute(conn)
    }
}
