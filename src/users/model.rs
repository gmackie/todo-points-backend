use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::{
    users,
    users::dsl::{
        users as all_users, 
        username, 
        first_name, 
        last_name,
        email,
        current_point_total, 
        current_status_level_id,
        admin_level,
        hash,
        created_at
    },
};

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub hash: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub active: bool,
    pub current_point_total: i32,
    pub current_status_level_id: i32,
    pub admin_level: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hash: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub active: bool,
    pub current_point_total: i32,
    pub current_status_level_id: i32,
    pub admin_level: i32,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn all(conn: &PgConnection) -> QueryResult<Vec<User>> {
        all_users.order(users::id.desc()).load::<User>(conn)
    }

    pub fn insert(user: NewUser, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
    }

    pub fn update(id: i32, _username: String, _firstname: String, _lastname: String, _email: String, conn: &PgConnection) -> QueryResult<usize> {
        let user = all_users.find(id).get_result::<User>(conn)?;

        let updated_user = diesel::update(all_users.find(id));
        updated_user
            .set((
                username.eq(_username),
                first_name.eq(_firstname),
                last_name.eq(_lastname),
                email.eq(_email)
            ))
            .execute(conn)
    }

    pub fn add_points(id: i32, points: i32, conn: &PgConnection) -> QueryResult<usize> {
        let user = all_users.find(id).get_result::<User>(conn)?;
        // TODO: Update currentstatus_id if we cross a threshold.
        let new_point_total = user.current_point_total + points;
        let updated_user = diesel::update(all_users.find(id));
        updated_user
            .set(current_point_total.eq(new_point_total))
            .execute(conn)
    }

    pub fn delete_with_id(id: i32, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(all_users.find(id)).execute(conn)
    }
}
