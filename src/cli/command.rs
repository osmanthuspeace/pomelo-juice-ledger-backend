use crate::db::models::NewTransaction;
use crate::service::create_service::create_transaction;
use crate::service::init_service::init_summary;
use crate::util::eval::eval;
use chrono::{Datelike, NaiveDate, Utc};
use clap::{ArgGroup, Parser};

/// Ledger CLI
#[derive(Parser, Debug)]
#[command(
    name = "ml",
    version = "1.0",
    about = "记账本命令行工具",
    author = "osmanthuspeace"
)]
#[command(about = "A special ledger CLI application", long_about = None)]
#[clap(group(
    ArgGroup::new("kind")
        .required(false)
        .args(&["food", "life", "study", "rest"])
))]
#[clap(group(
    ArgGroup::new("account")
        .required(false)
        .args(&["alipay", "wechat", "bankofchina", "icbc"])
))]
pub struct Cli {
    /// 初始化参数
    #[arg(
        long = "init",
        help = "Initialize summary with parameters",
        value_parser,
        num_args = 1..
    )]
    pub init: Option<Vec<f64>>,
    
    /// 饮食类支出
    #[arg(short = 'f', long = "food", help = "Kind: Food")]
    pub food: bool,

    /// 生活类支出
    #[arg(short = 'l', long = "life", help = "Kind: Life")]
    pub life: bool,

    /// 学习类支出
    #[arg(short = 's', long = "study", help = "Kind: Study")]
    pub study: bool,

    /// 休闲类支出
    #[arg(short = 'r', long = "rest", help = "Kind: Rest")]
    pub rest: bool,

    /// 支付账户: Alipay
    #[arg(short = 'a', long = "alipay", help = "Account: Alipay")]
    pub alipay: bool,

    /// 支付账户: WeChat
    #[arg(short = 'w', long = "wechat", help = "Account: WeChat")]
    pub wechat: bool,

    /// 支付账户: Bank of China
    #[arg(short = 'b', long = "bankofchina", help = "Account: Bank of China")]
    pub bankofchina: bool,

    /// 支付账户: ICBC
    #[arg(short = 'i', long = "icbc", help = "Account: ICBC")]
    pub icbc: bool,

    /// 位置参数：
    /// - 如果有三个参数：date description amount
    /// - 如果有两个参数：description amount
    #[arg(value_parser)]
    pub args: Vec<String>,
}

impl Cli {
    pub fn kind(&self) -> String {
        // self.kind.to_string()
        if self.life {
            "life".to_string()
        } else if self.study {
            "study".to_string()
        } else if self.rest {
            "rest".to_string()
        } else {
            // 默认 food
            "food".to_string()
        }
    }
    pub fn account(&self) -> String {
        // self.account.to_string()
        if self.wechat {
            "wechat".to_string()
        } else if self.bankofchina {
            "bankofchina".to_string()
        } else if self.icbc {
            "icbc".to_string()
        } else {
            // 默认 alipay
            "alipay".to_string()
        }
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
            let arg = &self.args[2];
            Self::parse_amount(arg)
        } else {
            let arg = &self.args[1];
            Self::parse_amount(arg)
        }
    }
    fn parse_amount(arg: &String) -> f64 {
        match arg.chars().nth(0) {
            Some('=') => eval(arg), //返回负数
            Some('+') => arg[1..].parse::<f64>().unwrap_or_else(|_| {
                eprintln!("收入金额解析错误，使用 0");
                return 0.0;
            }),
            _ => {
                let res = arg.parse::<f64>().unwrap_or_else(|_| {
                    eprintln!("支出金额解析错误，使用 0");
                    return 0.0;
                });
                -res
            }
        }
    }
    pub fn execute(&self) {
        if let Some(params) = &self.init {
            if params.len() < 5 {
                eprintln!(
                    "Error: `--init` requires at least 5 parameters, but got {}",
                    params.len()
                );
                return;
            }
            println!("Initializing system with parameters: {:?}", params);
            init_summary(params[0], params[1], params[2], params[3], params[4])
                .expect("Error initializing system");
        } else {
            if self.args.len() < 2 {
                eprintln!("Error: Not enough arguments provided for transaction creation");
                return;
            }
            let kind = self.kind();
            let account = self.account();
            let date = self.date();
            let description = self.description();
            let amount = self.amount();

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
    }
}
