// src/commands/add.rs
use crate::db::models::NewTransaction;
use crate::service::create_service::create_transaction;
use crate::util::eval::eval;
use chrono::{Datelike, NaiveDate, Utc};

pub fn handle_add_command(raw_args: Vec<String>) {
    let split_args = raw_args
        .split(|a| a == "/" || a == "\\")
        .map(|a| a.to_vec())
        .collect::<Vec<Vec<String>>>();

    for sub_args in split_args {
        if sub_args.len() < 3 {
            eprintln!("Error: Not enough arguments provided for transaction creation");
            return;
        }

        let (kind, account, date, description, amount) = parse_sub_args(&sub_args);

        println!(
            "[Debug] Creating transaction: kind={}, account={}, date={}, description={}, amount={}",
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
}

fn parse_sub_args(sub_args: &Vec<String>) -> (String, String, NaiveDate, String, f64) {
    let mut kind = String::new();
    let mut account = String::new();
    let mut sign = -1;

    let mut date = Utc::now().naive_utc().date(); // 如果没有输入日期，就用今天
    let mut description = String::new();
    let mut amount = 0.0;

    let mut idx = 0;
    while idx < sub_args.len() {
        let token = &sub_args[idx];

        // 1) 如果是 -- 开头的参数
        if token.starts_with('-') {
            let (k, a, s) = determine_kind_and_account_with_sign(token);

            if !k.is_empty() {
                kind = k;
            }
            if !a.is_empty() {
                account = a;
            }
            if s == 1 {
                sign = 1;
            }

            idx += 1;
            continue;
        }
        // 2) 如果是日期
        if is_date_format(token) {
            let parsed_date = parse_date(token);
            if let Some(d) = parsed_date {
                date = d;
            }
            idx += 1;
            continue;
        }
        // 3) 如果是金额
        if looks_like_amount(token) {
            let amt = parse_amount(token);
            // 这里根据 sign 来决定最终是正还是负
            amount = sign as f64 * amt;
            idx += 1;
            continue;
        }
        // 4) 否则就认为它是描述
        description += token;
        idx += 1;
    }
    (kind, account, date, description, amount)
}

fn determine_kind_and_account_with_sign(arg: &str) -> (String, String, i32) {
    let mut kind = "";
    let mut account = "";
    let mut sign = -1;

    if arg.starts_with("--") {
        let trimmed = arg.strip_prefix("--").unwrap_or(arg);
        match trimmed {
            "food" => kind = "food",
            "life" => kind = "life",
            "study" => kind = "study",
            "rest" => kind = "rest",
            "salary" => {
                kind = "salary";
                sign = 1
            }
            "transfer" => {
                kind = "transfer";
                sign = 1
            }
            "other" => {
                kind = "other";
                sign = 1
            }
            "alipay" => account = "alipay",
            "wechat" => account = "wechat",
            "bankofchina" => account = "bankofchina",
            "icbc" => account = "icbc",
            _ => {}
        }
    } else {
        for c in arg.chars() {
            match c {
                'f' => kind = "food",
                'l' => kind = "life",
                's' => kind = "study",
                'r' => kind = "rest",
                't' => {
                    kind = "transfer";
                    sign = 1
                }
                'o' => {
                    kind = "other";
                    sign = 1
                }
                'a' => account = "alipay",
                'w' => account = "wechat",
                'b' => account = "bankofchina",
                'i' => account = "icbc",

                _ => {}
            }
        }
    }

    (kind.to_string(), account.to_string(), sign)
}
/// 判断一个字符串是否像日期 (例如 "12-25")
fn is_date_format(s: &str) -> bool {
    let parts: Vec<&str>;
    if s.contains('.') {
        parts = s.split('.').collect();
    } else if s.contains('-') {
        parts = s.split('-').collect();
    } else {
        return false;
    }
    if parts.len() == 2 {
        // 粗略判断两段都是数字
        parts[0].parse::<u32>().is_ok() && parts[1].parse::<u32>().is_ok()
    } else {
        false
    }
}
/// 解析日期字符串，返回 NaiveDate，如果解析失败，返回 None
/// 默认使用当前年份
/// 例如 "12-25" -> 2024-12-25 ; "12.25" -> 2024-12-25
fn parse_date(s: &str) -> Option<NaiveDate> {
    let parts: Vec<&str>;
    if s.contains('.') {
        parts = s.split('.').collect();
    } else if s.contains('-') {
        parts = s.split('-').collect();
    } else {
        return None;
    }
    if parts.len() != 2 {
        return None;
    }
    let current_year = Utc::now().year();
    let month = parts[0].parse::<u32>().ok()?;
    let day = parts[1].parse::<u32>().ok()?;
    NaiveDate::from_ymd_opt(current_year, month, day)
}

/// 判断一个字符串是否为金额：第一位是 数字、+、-、= 就算
fn looks_like_amount(s: &str) -> bool {
    match s.chars().next() {
        Some(ch) if ch.is_ascii_digit() || ch == '+' || ch == '-' || ch == '=' => true,
        _ => false,
    }
}

/// 解析金额字符串，支持 = 开头的表达式
fn parse_amount(s: &str) -> f64 {
    match s.chars().next() {
        Some('=') => eval(s),
        _ => s.parse::<f64>().unwrap_or_else(|_| {
            eprintln!("金额解析错误，使用 0.0");
            0.0
        }),
    }
}
