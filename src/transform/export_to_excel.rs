use crate::db::models::Transaction;
use crate::service::get_service;
use chrono::Datelike;
use rust_xlsxwriter::{Format, FormatAlign, Workbook};
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
    let header_format = Format::new().set_bold().set_align(FormatAlign::Center);
    let number_format = Format::new().set_num_format("0.00");
    for (month, transactions) in transactions_by_month {
        let sheet = workbook.add_worksheet();
        sheet.set_name(&month).expect("Error setting sheet name");
        let headers = ["月", "日", "摘要", "类型", "账户", "收入", "支出", "结余"];
        for (col, &header) in headers.iter().enumerate() {
            sheet
                .write_string_with_format(0, col as u16, header, &header_format)
                .expect("Error writing header");
        }
        sheet
            .set_column_width(0, 4)
            .expect("Error setting column width");
        sheet
            .set_column_width(1, 4)
            .expect("Error setting column width");
        sheet
            .set_column_width(2, 20)
            .expect("Error setting column width");
        sheet
            .set_column_width(3, 10)
            .expect("Error setting column width");
        sheet
            .set_column_width(4, 12)
            .expect("Error setting column width");
        sheet
            .set_column_width(5, 10)
            .expect("Error setting column width");
        sheet
            .set_column_width(6, 10)
            .expect("Error setting column width");
        for (i, tr) in transactions.iter().enumerate() {
            let row = (i + 1) as u32;
            let month = tr.date.month();
            let day = tr.date.day();
            let description = &tr.description;
            let kind = &tr.kind;
            let account = &tr.account;

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
            sheet
                .write_string(row, 3, kind)
                .expect("Error writing account");
            sheet
                .write_string(row, 4, account)
                .expect("Error writing account");
            if amount > 0.0 {
                sheet
                    .write_number_with_format(row, 5, amount, &number_format)
                    .expect("Error writing income");
            } else {
                sheet
                    .write_number_with_format(row, 6, -amount, &number_format)
                    .expect("Error writing expense");
            }
            sheet
                .write_number_with_format(row, 7, balance, &number_format)
                .expect("Error writing balance");
        }
    }
    workbook.save("ledger.xlsx")?;
    println!("数据已导出到 ledger.xlsx");
    Ok(())
}
