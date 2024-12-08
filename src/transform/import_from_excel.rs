use crate::db::models::NewTransaction;
use crate::service::create_service::create_transaction;
use calamine::{open_workbook_auto, RangeDeserializerBuilder, Reader};
use chrono::NaiveDate;

pub fn import_from_excel(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = open_workbook_auto(file_path)?;
    // 获取所有sheet名称
    let sheet_names = workbook.sheet_names().to_owned();

    for sheet_name in sheet_names {
        println!("正在读取工作表: {}", sheet_name);
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            // 创建行解析器
            let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;

            // 跳过第一行（表头）
            if let Some(_) = iter.next() {}

            // 解析工作表名称为 "年-月"
            let parts: Vec<&str> = sheet_name.split('-').collect();
            if parts.len() != 2 {
                eprintln!("无效的工作表名称: {}", sheet_name);
                continue;
            }
            println!("{:?}", parts);
            let year: i32 = match parts[0].parse() {
                Ok(y) => y,
                Err(_) => {
                    eprintln!("无效的年份: {}", parts[0]);
                    continue;
                }
            };
            let month: u32 = match parts[1].parse() {
                Ok(m) => m,
                Err(_) => {
                    eprintln!("无效的月份: {}", parts[1]);
                    continue;
                }
            };

            // 遍历每一行
            for result in iter {
                println!("{:?}", result);
                // 定义每行的预期格式
                let row: (
                    i32,
                    i32,
                    String,
                    String,
                    String,
                    Option<f64>,
                    Option<f64>,
                    f64,
                ) = match result {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("解析row错误: {}", e);
                        continue;
                    }
                };

                let (
                    _month_cell,
                    day_cell,
                    description,
                    kind_cell,
                    account_cell,
                    income,
                    expense,
                    _balance_cell,
                ) = row;

                let day = day_cell as u32;
                let date = match NaiveDate::from_ymd_opt(year, month, day) {
                    Some(d) => d,
                    None => {
                        eprintln!("无效的日期: {}-{:02}-{}", year, month, day);
                        continue;
                    }
                };

                // 确定收入和支出
                let income = income.unwrap_or(0.0);
                let expense = expense.unwrap_or(0.0);
                let amount = if income > 0.0 { income } else { -expense };

                let kind = match kind_cell {
                    String { .. } => "food".to_string(),
                };
                let account = match account_cell {
                    String { .. } => "alipay".to_string(),
                };

                let new_transaction = NewTransaction {
                    date,
                    kind,
                    description,
                    amount,
                    account,
                };

                match create_transaction(&new_transaction) {
                    Ok(tx) => println!("插入交易记录成功，ID: {}", tx.id),
                    Err(e) => eprintln!("插入交易记录失败: {}", e),
                }
            }
        } else {
            eprintln!("无法读取工作表: {}", sheet_name);
        }
    }
    Ok(())
}
