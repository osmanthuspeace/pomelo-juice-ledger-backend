use chrono::{Datelike, NaiveDate, Utc};
use clap::Parser;
use clap::ValueEnum;
use std::fmt::Display;

#[derive(ValueEnum, Clone, Debug)]
pub enum Kind {
    #[clap(alias = "f")]
    Food, //饮食

    #[clap(alias = "l")]
    Life, //生活

    #[clap(alias = "s")]
    Study, //学习

    #[clap(alias = "r")]
    Rest, //休闲
}
#[derive(ValueEnum, Clone, Debug)]
pub enum Account {
    #[clap(alias = "a")]
    Alipay, //支付宝

    #[clap(alias = "w")]
    Wechat, //微信，校园卡

    #[clap(alias = "b")]
    BankOfChina, //中国银行

    #[clap(alias = "i")]
    ICBC, //工商银行
}
impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Kind::Food => "food".to_string(),
            Kind::Life => "life".to_string(),
            Kind::Study => "study".to_string(),
            Kind::Rest => "rest".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Account::Alipay => "alipay".to_string(),
            Account::Wechat => "wechat".to_string(),
            Account::BankOfChina => "bankofchina".to_string(),
            Account::ICBC => "icbc".to_string(),
        };
        write!(f, "{}", str)
    }
}

/// Ledger CLI
#[derive(Parser, Debug)]
#[command(
    name = "ml",
    version = "1.0",
    about = "记账本命令行工具",
    author = "osmanthuspeace"
)]
#[command(about = "A special ledger CLI application", long_about = None)]
pub struct Cli {
    #[arg(short, long, value_enum, default_value_t = Kind::Food)]
    pub kind: Kind,

    #[arg(short, long, value_enum, default_value_t = Account::Alipay)]
    pub account: Account,

    /// 位置参数：
    /// - 如果有三个参数：date description amount
    /// - 如果有两个参数：description amount
    #[arg(value_parser)]
    pub args: Vec<String>,
}

impl Cli {
    pub fn kind(&self) -> String {
        self.kind.to_string()
    }
    pub fn account(&self) -> String {
        self.account.to_string()
    }
    pub fn date(&self) -> NaiveDate {
        let current_year = Utc::now().year();
        if self.args.len() == 3 {
            let date_str = &self.args[0];
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
            // 默认使用当前日期
            Utc::now().naive_utc().date()
        }
    }
    /// 获取描述
    pub fn description(&self) -> &String {
        if self.args.len() == 3 {
            &self.args[1]
        } else {
            &self.args[0]
        }
    }

    /// 获取金额
    pub fn amount(&self) -> f64 {
        if self.args.len() == 3 {
            self.args[2].parse::<f64>().unwrap_or_else(|_| {
                eprintln!("金额解析错误，使用 0");
                0.0
            })
        } else {
            self.args[1].parse::<f64>().unwrap_or_else(|_| {
                eprintln!("金额解析错误，使用 0");
                0.0
            })
        }
    }
}
