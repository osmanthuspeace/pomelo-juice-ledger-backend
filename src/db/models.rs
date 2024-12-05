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
}
#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "summary"]
pub struct Summary {
    pub id: i32,
    pub balance: f64,
    pub storage: f64,

    pub alipay: f64,
    pub wechat: f64,
    pub bankofchina: f64,
    pub icbc: f64,
}

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
