use crate::db::schema::summary::dsl::summary;
use crate::db::schema::summary::{account, amount};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;
pub fn update_summary_when_creating(
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
