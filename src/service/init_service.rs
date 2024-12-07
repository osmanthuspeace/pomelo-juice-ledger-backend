use diesel::insert_into;
use diesel::prelude::*;
use diesel::PgConnection;
pub fn init_summary(
    conn: &mut PgConnection,
    alipay_amount: f64,
    wechat_amount: f64,
    bankofchina_amount: f64,
    icbc_amount: f64,
    storage_amount: f64,
) -> Result<(), diesel::result::Error> {
    use crate::db::schema::summary::dsl::*;

    // 计算可以花的余额总数
    let balance_amount = alipay_amount + wechat_amount + bankofchina_amount + icbc_amount;

    // 创建或更新
    let mut upsert = |account_name: &str, amount_value: f64| -> Result<(), diesel::result::Error> {
        let exists = summary
            .filter(account.eq(account_name))
            .select(id)
            .first::<i32>(conn)
            .optional()?; // 判断记录是否存在

        match exists {
            Some(_) => {
                // 如果记录存在，执行更新
                diesel::update(summary.filter(account.eq(account_name)))
                    .set(amount.eq(amount_value))
                    .execute(conn)?;
            }
            None => {
                // 如果记录不存在，执行插入
                insert_into(summary)
                    .values((account.eq(account_name), amount.eq(amount_value)))
                    .execute(conn)?;
            }
        }

        Ok(())
    };

    // 更新或插入每个字段
    upsert("alipay", alipay_amount)?;
    upsert("wechat", wechat_amount)?;
    upsert("bankofchina", bankofchina_amount)?;
    upsert("icbc", icbc_amount)?;
    upsert("balance", balance_amount)?;
    upsert("storage", storage_amount)?;

    Ok(())
}
