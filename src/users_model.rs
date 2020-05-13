use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Serialize;

use crate::schema::{
    users,
    users::dsl::{users as all_users},
};

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub description: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}

impl User {
    pub fn all(conn: &PgConnection) -> QueryResult<Vec<User>> {
        all_users.order(users::id.desc()).load::<User>(conn)
    }

    pub fn insert(user: NewUser, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(users::table)
            .values(&todo)
            .execute(conn)
    }

    pub fn delete_with_id(id: i32, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(all_users.find(id)).execute(conn)
    }
}
