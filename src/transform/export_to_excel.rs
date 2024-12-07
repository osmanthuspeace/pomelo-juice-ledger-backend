use crate::db::models::Transaction;
use crate::service::get_service;
use chrono::Datelike;
use rust_xlsxwriter::{Format, FormatAlign, FormatPattern, Workbook};
use std::collections::HashMap;

pub fn export_to_excel() -> Result<(), Box<dyn std::error::Error>> {
    let all_transactions = get_service::get_all_transactions()?;
    if all_transactions.is_empty() {
        return Err("没有交易记录可导出。".into());
    }
    // 按月份分组
    let mut transactions_by_month: HashMap<String, Vec<Transaction>> = HashMap::new();
    for tr in all_transactions {
        let year = tr.date.year();
        let mouth = tr.date.month();
        let key = format!("{}-{:02}", year, mouth);
        transactions_by_month.entry(key).or_insert(vec![]).push(tr);
    }
    let mut workbook = Workbook::new();
    // 定义表头格式
    let header_format = Format::new()
        .set_bold()
        .set_pattern(FormatPattern::Solid)
        .set_align(FormatAlign::Center);
    let number_format = Format::new().set_num_format("0.00");
    for (month, transactions) in transactions_by_month {
        let sheet = workbook.add_worksheet();
        sheet.set_name(&month).expect("Error setting sheet name");
        sheet
            .write_string(0, 0, "月")
            .expect("Error writing header");
        sheet
            .write_string(0, 1, "日")
            .expect("Error writing header");
        sheet
            .write_string(0, 2, "摘要")
            .expect("Error writing header");
        sheet
            .write_string(0, 3, "收入")
            .expect("Error writing header");
        sheet
            .write_string(0, 4, "支出")
            .expect("Error writing header");
        sheet
            .write_string(0, 5, "结余")
            .expect("Error writing header");
        for (i, tr) in transactions.iter().enumerate() {
            let row = (i + 1) as u32;
            let month = tr.date.month();
            let day = tr.date.day();
            let description = &tr.description;
            let amount = tr.amount;
            let balance = tr.balance;
            sheet
                .write_number(row, 0, month as f64)
                .expect("Error writing month");
            sheet
                .write_number(row, 1, day as f64)
                .expect("Error writing day");
            sheet
                .write_string(row, 2, description)
                .expect("Error writing description");
            if amount > 0.0 {
                sheet
                    .write_number(row, 3, amount)
                    .expect("Error writing income");
            } else {
                sheet
                    .write_number(row, 4, -amount)
                    .expect("Error writing expense");
            }
            sheet
                .write_number(row, 5, balance)
                .expect("Error writing balance");
        }
    }
    workbook.save("ledger.xlsx")?;
    println!("数据已导出到 ledger.xlsx");
    Ok(())
}
