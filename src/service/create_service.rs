use crate::db::models::{NewTransaction, Transaction};
use crate::db::schema::summary::dsl::summary;
use crate::db::schema::summary::{account, amount};
use crate::db::schema::transactions::dsl::transactions;
use crate::service::connection::establish_connection;
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_transaction(new_transaction: &NewTransaction) -> Result<Transaction, Error> {
    let mut connection = establish_connection();
    //同步更新 summary 表，并获取最新的余额
    let last_balance = update_summary_when_creating(
        &mut connection,
        &new_transaction.account,
        new_transaction.amount,
    )?;

    diesel::insert_into(transactions)
        .values((
            crate::db::schema::transactions::date.eq(new_transaction.date),
            crate::db::schema::transactions::kind.eq(&new_transaction.kind),
            crate::db::schema::transactions::description.eq(&new_transaction.description),
            crate::db::schema::transactions::amount.eq(new_transaction.amount),
            crate::db::schema::transactions::account.eq(&new_transaction.account),
            crate::db::schema::transactions::balance.eq(last_balance),
        ))
        .get_result(&mut connection)
}
fn update_summary_when_creating(
    conn: &mut PgConnection,
    update_account: &str,
    update_amount: f64,
) -> Result<f64, Error> {
    let target_account = match update_account {
        "alipay" => "alipay",
        "wechat" => "wechat",
        "bankofchina" => "bankofchina",
        "icbc" => "icbc",
        _ => {
            eprintln!("未知的账户类型: {}", update_account);
            return Err(Error::NotFound);
        }
    };
    // 更新目标账户的金额
    diesel::update(summary.filter(account.eq(target_account)))
        .set(amount.eq(amount + update_amount))
        .execute(conn)?;
    // 更新总余额
    diesel::update(summary.filter(account.eq("balance")))
        .set(amount.eq(amount + update_amount))
        .execute(conn)?;
    // 获取更新后的总余额，用于更新交易表
    Ok(summary
        .filter(account.eq("balance"))
        .select(amount)
        .first(conn)?)
}
