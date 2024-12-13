use crate::db::models::{NewTransaction, Transaction};
use crate::db::schema::transactions::dsl::transactions;
use crate::service::connection::establish_connection;
use diesel::prelude::*;
use diesel::result::Error;
use crate::service::update_service::update_summary_when_creating;

pub fn create_transaction(new_transaction: &NewTransaction) -> Result<Transaction, Error> {
    let mut connection = establish_connection();
    let round_amount = (new_transaction.amount * 100.0).round() / 100.0;
    //同步更新 summary 表，并获取最新的余额
    let last_balance =
        update_summary_when_creating(&mut connection, &new_transaction.account, round_amount)?;

    diesel::insert_into(transactions)
        .values((
            crate::db::schema::transactions::date.eq(new_transaction.date),
            crate::db::schema::transactions::kind.eq(&new_transaction.kind),
            crate::db::schema::transactions::description.eq(&new_transaction.description),
            crate::db::schema::transactions::amount.eq(round_amount),
            crate::db::schema::transactions::account.eq(&new_transaction.account),
            crate::db::schema::transactions::balance.eq(last_balance),
        ))
        .get_result(&mut connection)
}
