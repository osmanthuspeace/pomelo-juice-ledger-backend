use crate::db::models::Transaction;
use crate::db::schema::transactions::dsl::*;
use crate::service::connection::establish_connection;
use chrono::NaiveDate;
use diesel::dsl::{max, min};
use diesel::prelude::*;
//导入QueryDsl 和 ExpressionMethods 这两个 trait，它们提供了 filter 和 ge、le 等操作
use diesel::result::Error;

//在日期范围内获取交易记录
pub fn get_transactions(from: NaiveDate, to: NaiveDate) -> Result<Vec<Transaction>, Error> {
    let mut connection = establish_connection();
    let result = transactions
        .filter(date.ge(from)) // date >= from
        .filter(date.le(to)) // date <= to
        .load::<Transaction>(&mut connection)?; // 加载查询结果为 Transaction

    Ok(result)
}

//获取所有交易记录
pub fn get_all_transactions() -> Result<Vec<Transaction>, Error> {
    let mut connection = establish_connection();
    let result = transactions
        .order(date.asc())
        .load::<Transaction>(&mut connection)?;

    Ok(result)
}

pub fn get_default_date() -> Result<(NaiveDate, NaiveDate), Error> {
    let mut conn = establish_connection();

    let result = transactions
        .select((min(date).nullable(), max(date).nullable()))
        .first::<(Option<NaiveDate>, Option<NaiveDate>)>(&mut conn);
    match result {
        Ok((Some(default_from), Some(default_to))) => Ok((default_from, default_to)),
        Ok(_) => {
            eprintln!("No dates found in the database");
            Err(Error::NotFound)
        }
        Err(err) => {
            eprintln!("Error querying default dates: {:?}", err);
            Err(err)
        }
    }
}
