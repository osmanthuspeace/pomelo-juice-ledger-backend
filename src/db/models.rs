use crate::db::schema::*;
use chrono::NaiveDate;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

//查询模型
#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "transactions"]
pub struct Transaction {
    pub id: i32,
    pub date: NaiveDate,
    pub kind: String,
    pub description: String,
    pub amount: f64,
    pub account: String,
    pub balance: f64,
}
#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "summary"]
pub struct Summary {
    pub id: i32,

    ///余额应该与交易表中的最新的余额字段保持一致
    pub account: String,
    pub amount: f64,
}

// #[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
// #[table_name = "user"]
// pub struct User {
//     pub id: i32,
//     
//     pub username: String,
//     pub password: String,
//     pub is_modified: bool,
// }

//插入模型
#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub date: NaiveDate,
    pub kind: String,
    pub description: String,
    pub amount: f64,
    pub account: String,
}
impl NewTransaction {
    pub fn new(
        date: NaiveDate,
        kind: String,
        description: String,
        amount: f64,
        account: String,
    ) -> Self {
        NewTransaction {
            date,
            kind,
            description,
            amount,
            account,
        }
    }
}
