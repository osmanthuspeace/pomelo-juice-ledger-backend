use crate::db::schema::summary::dsl::*;
use diesel::dsl::sum;
use diesel::prelude::*;
///修改summary时，同步更新交易表中的余额字段
pub fn sync_balance() -> QueryResult<()> {
    let conn = &mut crate::service::connection::establish_connection();

    let total_sum: f64 = summary
        .filter(account.ne("balance"))
        .select(sum(amount))
        .first::<Option<f64>>(conn)?
        .unwrap_or(0.0);

    println!("total_sum: {}", total_sum);

    diesel::update(summary.filter(account.eq("balance")))
        .set(amount.eq(total_sum))
        .execute(conn)?;

    Ok(())
}

///转账
pub fn transfer() {
    todo!()
}
