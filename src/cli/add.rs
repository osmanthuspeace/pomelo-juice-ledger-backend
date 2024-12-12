// src/commands/add.rs
use crate::db::models::NewTransaction;
use crate::service::create_service::create_transaction;
use crate::util::eval::eval;
use chrono::{Datelike, NaiveDate, Utc};

pub fn handle_add_command(
    args: Vec<String>,
    food: bool,
    life: bool,
    study: bool,
    rest: bool,
    salary: bool,
    transfer: bool,
    other: bool,
    alipay: bool,
    wechat: bool,
    bankofchina: bool,
    icbc: bool,
) {
    if args.len() < 2 {
        eprintln!("Error: Not enough arguments provided for transaction creation");
        return;
    }
    let kind = determine_kind(food, life, study, rest, salary, transfer, other);
    let account = determine_account(alipay, wechat, bankofchina, icbc);
    let date = parse_date(&args);
    let description = parse_description(&args);
    let amount = parse_amount(&args);

    println!(
        "Creating transaction: kind={}, account={}, date={}, description={}, amount={}",
        kind, account, date, description, amount
    );
    create_transaction(&NewTransaction::new(
        date,
        kind,
        description.clone(),
        amount,
        account,
    ))
    .expect("Error creating transaction");
}

fn determine_kind(
    food: bool,
    life: bool,
    study: bool,
    rest: bool,
    salary: bool,
    transfer: bool,
    other: bool,
) -> String {
    if food {
        "food".into()
    } else if life {
        "life".into()
    } else if study {
        "study".into()
    } else if rest {
        "rest".into()
    }
    //收入需要显式的添加+号
    else if salary {
        "salary".into()
    } else if transfer {
        "transfer".into()
    } else if other {
        "other".into()
    } else {
        "".into()
    }
}

fn determine_account(alipay: bool, wechat: bool, bankofchina: bool, icbc: bool) -> String {
    if alipay {
        "alipay".to_string()
    } else if wechat {
        "wechat".to_string()
    } else if bankofchina {
        "bankofchina".to_string()
    } else if icbc {
        "icbc".to_string()
    } else {
        "other".to_string()
    }
}

fn parse_date(args: &Vec<String>) -> NaiveDate {
    let current_year = Utc::now().year();
    if args.len() == 3 {
        let date_str = &args[0];
        let parts: Vec<&str> = date_str.split('.').collect();
        if parts.len() != 2 {
            eprintln!("日期格式错误，应为 MM.DD，使用当前日期");
            return Utc::now().naive_utc().date();
        }
        let month = parts[0].parse::<u32>().unwrap_or_else(|_| {
            eprintln!("月份解析错误，使用当前月份");
            Utc::now().month()
        });
        let day = parts[1].parse::<u32>().unwrap_or_else(|_| {
            eprintln!("日期解析错误，使用当前日期");
            Utc::now().day()
        });
        NaiveDate::from_ymd_opt(current_year, month, day)
            .unwrap_or_else(|| Utc::now().naive_utc().date())
    } else {
        Utc::now().naive_utc().date()
    }
}

fn parse_description(args: &Vec<String>) -> String {
    if args.len() == 3 {
        args[1].clone()
    } else {
        args[0].clone()
    }
}

fn parse_amount(args: &Vec<String>) -> f64 {
    let arg = if args.len() == 3 { &args[2] } else { &args[1] };
    match arg.chars().nth(0) {
        Some('=') => eval(arg),
        Some('+') => arg[1..].parse::<f64>().unwrap_or_else(|_| {
            eprintln!("收入金额解析错误，使用 0");
            0.0
        }),
        _ => {
            let res = arg.parse::<f64>().unwrap_or_else(|_| {
                eprintln!("支出金额解析错误，使用 0");
                0.0
            });
            -res
        }
    }
}
