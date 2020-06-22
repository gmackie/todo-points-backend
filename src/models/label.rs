use crate::{
  config::db::Connection,
  schema::labels::{self, dsl::*}
};
use diesel::prelude::*;
use chrono::Utc;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Label {
  pub id: i32,
  pub name: String,
  pub color: String,
  pub user_id: i32,
  pub created_at: chrono::DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "labels"]
pub struct NewLabel {
  pub name: String,
  pub color: String,
  pub created_by: i32,
  pub created_at: chrono::DateTime<Utc>,
}


#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "labels"]
pub struct LabelDTO {
  pub name: String,
  pub color: String,
  pub created_by: i32,
}

impl Label {
  pub fn find_all(conn: &Connection) -> QueryResult<Vec<Label>> {
      labels.order(id.asc()).load::<Label>(conn)
  }

  pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Label> {
      labels.find(i).get_result::<Label>(conn)
  }

  pub fn query(_query: String, conn: &Connection) -> QueryResult<Vec<Label>> {
      labels.order(id.asc()).load::<Label>(conn)
  }

  pub fn insert(label: LabelDTO, conn: &Connection) -> QueryResult<usize> {
      let new_label = NewLabel {
          name: label.name,
          color: label.color,
          created_by: label.created_by,
          created_at: Utc::now(),
      };
      diesel::insert_into(labels)
          .values(&new_label)
          .execute(conn)
  }

  pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
      diesel::delete(labels.find(i)).execute(conn)
  }
}
