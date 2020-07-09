use crate::{
    config::db::Connection,
    schema::balances::{self, dsl::*}
  };
  use diesel::prelude::*;
  use chrono::Utc;
  
  #[derive(Queryable, Serialize, Deserialize)]
  pub struct Balance {
    pub id: i32,
    pub user_id: i32,
    pub label_id: i32,
    pub points: i32,
    pub updated_at: chrono::DateTime<Utc>,
  }
  
  #[derive(Insertable, Serialize, Deserialize)]
  #[table_name = "balances"]
  pub struct NewBalance {
    pub user_id: i32,
    pub label_id: i32,
    pub points: i32,
    pub updated_at: chrono::DateTime<Utc>,
  }
  
  
  #[derive(AsChangeset, Serialize, Deserialize)]
  #[table_name = "balances"]
  pub struct BalanceDTO {
    pub user_id: i32,
    pub label_id: i32,
    pub points: i32,
  }
  
  impl Balance {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Balance>> {
        balances.order(id.asc()).load::<Balance>(conn)
    }
  
    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Balance> {
        balances.find(i).get_result::<Balance>(conn)
    }
  
    pub fn query(_query: String, conn: &Connection) -> QueryResult<Vec<Balance>> {
        balances.order(id.asc()).load::<Balance>(conn)
    }
  
    pub fn insert(balance: BalanceDTO, conn: &Connection) -> QueryResult<usize> {
        let new_balance = NewBalance {
            user_id: balance.user_id,
            label_id: balance.label_id,
            points: balance.points,
            updated_at: Utc::now(),
        };
        diesel::insert_into(balances)
            .values(&new_balance)
            .execute(conn)
    }
  
    pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(balances.find(i)).execute(conn)
    }
  }
  