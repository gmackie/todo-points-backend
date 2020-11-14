use crate::{
  config::db::Connection,
  schema::groups::{self, dsl::*}
};
use diesel::prelude::*;
use chrono::Utc;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Group {
  pub id: i32,
  pub group_name: String,
  pub created_by: i32,
  pub created_at: chrono::DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "groups"]
pub struct NewGroup {
  pub group_name: String,
  pub created_by: i32,
  pub created_at: chrono::DateTime<Utc>,
}


#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "groups"]
pub struct GroupDTO {
  pub group_name: String,
  pub created_by: i32,
}

impl Group {
  pub fn find_all(conn: &Connection) -> QueryResult<Vec<Group>> {
      groups.order(id.asc()).load::<Group>(conn)
  }

  pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Group> {
      groups.find(i).get_result::<Group>(conn)
  }

  pub fn query(_query: String, conn: &Connection) -> QueryResult<Vec<Group>> {
      groups.order(id.asc()).load::<Group>(conn)
  }

  pub fn insert(group: GroupDTO, conn: &Connection) -> QueryResult<usize> {
      let new_group = NewGroup {
          group_name: group.group_name,
          created_by: group.created_by,
          created_at: Utc::now(),
      };
      diesel::insert_into(groups)
          .values(&new_group)
          .execute(conn)
  }

  pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
      diesel::delete(groups.find(i)).execute(conn)
  }
}
