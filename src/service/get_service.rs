use crate::db::models::Transaction;
use crate::db::schema::transactions::dsl::*;
use crate::service::connection::establish_connection;
use chrono::NaiveDate;
use diesel::prelude::*; //导入QueryDsl 和 ExpressionMethods 这两个 trait，它们提供了 filter 和 ge、le 等操作

pub fn get_transactions(
    from: NaiveDate,
    to: NaiveDate,
) -> Result<Vec<Transaction>, diesel::result::Error> {
    let mut connection = establish_connection();
    let result = transactions
        .filter(date.ge(from)) // date >= from
        .filter(date.le(to)) // date <= to
        .load::<Transaction>(&mut connection); // 加载查询结果为 Transaction

    result
}
