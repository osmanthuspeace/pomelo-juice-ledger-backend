use crate::db::schema::transactions;
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
}

//插入模型
#[derive(Insertable)]
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
