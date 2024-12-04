use serde::{Deserialize, Serialize};
use diesel::prelude::*;
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "transactions"]
pub struct Transaction {
    pub id: i32,
    pub amount: f64,
    pub description: String,
}