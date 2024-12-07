use crate::db::models::{NewTransaction, Transaction};
use crate::db::schema::summary::{account, amount};
use crate::db::schema::summary::dsl::summary;
use crate::db::schema::transactions::dsl::transactions;
use crate::service::connection::establish_connection;
use diesel::result::Error;
use diesel::prelude::*;

pub fn create_transaction(new_transaction: &NewTransaction) -> Result<Transaction, Error> {
    let mut connection = establish_connection();
    update_summary_when_creating(
        &mut connection,
        &new_transaction.account,
        new_transaction.amount,
    )
    .expect("Error updating summary");
    diesel::insert_into(transactions)
        .values(new_transaction)
        .get_result(&mut connection)
}
fn update_summary_when_creating(
    conn: &mut PgConnection,
    update_account: &str,
    update_amount: f64,
) -> Result<(), Error> {
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

    diesel::update(summary.filter(account.eq(target_account)))
        .set(amount.eq(amount + update_amount))
        .execute(conn)?;

    Ok(())
}
