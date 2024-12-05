use crate::db::models::{NewTransaction, Transaction};
use crate::db::schema::transactions::dsl::transactions;
use crate::service::connection::establish_connection;
use diesel::result::Error;
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl};

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
    use crate::db::schema::summary::dsl::*;

    match update_account {
        "alipay" => {
            // 假设你要更新金额或其他字段
            diesel::update(summary)
                .set(alipay.eq(alipay + update_amount))
                .execute(conn)
                .expect("Error updating summary for Alipay");
        }
        "wechat" => {
            diesel::update(summary)
                .set(wechat.eq(wechat + update_amount)) // 假设更新金额
                .execute(conn)
                .expect("Error updating summary for WeChat");
        }
        "bankofchina" => {
            diesel::update(summary)
                .set(bankofchina.eq(bankofchina + update_amount)) // 假设更新金额
                .execute(conn)
                .expect("Error updating summary for Bank of China");
        }
        "icbc" => {
            diesel::update(summary)
                .set(icbc.eq(icbc + update_amount)) // 假设更新金额
                .execute(conn)
                .expect("Error updating summary for ICBC");
        }
        _ => {
            eprintln!("未知的账户类型: {}", update_account);
            return Err(Error::NotFound);
        }
    };
    Ok(())
}
