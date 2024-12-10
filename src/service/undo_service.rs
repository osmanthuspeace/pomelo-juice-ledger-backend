use crate::db::models::Transaction;
use crate::db::schema::summary;
use crate::db::schema::transactions;
use crate::service::connection::establish_connection;
use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::result::Error;

/// 撤销最近的一笔交易
pub fn undo_last_transaction() -> Result<(), Error> {
    let connection = &mut establish_connection();

    // 开始事务
    connection.batch_execute("BEGIN")?;

    // 查找最新的一笔交易记录，按 id 降序
    let last_tx_result: Result<Transaction, Error> = transactions::dsl::transactions
        .order(transactions::id.desc())
        .first(connection);

    let last_tx = match last_tx_result {
        Ok(tx) => tx,
        Err(e) => {
            // 发生错误，回滚事务并返回错误
            connection.batch_execute("ROLLBACK")?;
            return Err(e);
        }
    };

    // 删除这笔交易记录
    if let Err(e) =
        diesel::delete(transactions::dsl::transactions.filter(transactions::id.eq(last_tx.id)))
            .execute(connection)
    {
        connection.batch_execute("ROLLBACK")?;
        return Err(e);
    }

    // 更新 summary 表，减少对应账户的金额
    if let Err(e) = diesel::update(summary::dsl::summary.filter(summary::account.eq(&last_tx.account)))
        .set(summary::amount.eq(summary::amount - last_tx.amount))
        .execute(connection)
    {
        connection.batch_execute("ROLLBACK")?;
        return Err(e);
    }

    // 更新总余额
    if let Err(e) = diesel::update(summary::dsl::summary.filter(summary::account.eq("balance")))
        .set(summary::amount.eq(summary::amount - last_tx.amount))
        .execute(connection)
    {
        connection.batch_execute("ROLLBACK")?;
        return Err(e);
    }

    // 提交事务
    connection.batch_execute("COMMIT")?;

    Ok(())
}
