
use std::ops::Deref;

use crate::users::model::{NewUser, User};
use crate::db::{get_conn, PgPool};


pub fn get_all_users(pool: &PgPool) -> Result<Vec<User>, &'static str> {
    User::all(get_conn(pool)?.deref()).map_err(|_| "Error inserting user")
}

pub fn create_user(new_user: NewUser, pool: &PgPool) -> Result<(), &'static str> {
    User::insert(new_user, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error creating user")
}

pub fn add_points(id: i32, points: i32, pool: &PgPool) -> Result<(), &'static str> {
    User::add_points(id, points, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error adding points to user")
}

pub fn delete_user(id: i32, pool: &PgPool) -> Result<(), &'static str> {
    User::delete_with_id(id, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error deleting user")
}
