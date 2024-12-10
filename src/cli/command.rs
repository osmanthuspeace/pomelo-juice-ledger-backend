use crate::db::models::NewTransaction;
use crate::service::create_service::create_transaction;
use crate::service::init_service::init_summary;
use crate::service::undo_service::undo_last_transaction;
use crate::transform::export_to_excel::export_to_excel;
use crate::transform::import_from_excel::import_from_excel;
use crate::util::eval::eval;
use crate::util::parse_daily_params::parse_daily_params;
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
    ArgGroup::new("output")
        .required(false)
        .args(&["food", "life", "study", "rest"])
))]
#[clap(group(
    ArgGroup::new("input")
        .required(false)
        .args(&["transfer", "salary", "other"])
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

    ///是否要导出
    #[arg(long = "export", help = "导出 Excel 文件")]
    pub export: Option<bool>,

    ///是否要导入
    #[arg(long = "import", help = "导入 Excel 文件")]
    pub import: Option<String>,

    ///是否要将余额同步
    #[arg(long = "sync", help = "同步余额")]
    pub sync: Option<bool>,

    ///是否要撤销上一步
    #[arg(long = "undo", help = "撤销最近的一笔交易")]
    pub undo: Option<bool>,

    ///每日惯例，即三餐
    #[arg(long = "da", help = "Daily routine",value_parser,num_args = 1..)]
    pub daily: Option<Vec<f64>>,

    /******************************************************/
    /// 饮食类支出
    #[arg(short = 'f', long = "food", help = "Output: Food")]
    pub food: bool,

    /// 生活类支出
    #[arg(short = 'l', long = "life", help = "Output: Life")]
    pub life: bool,

    /// 学习类支出
    #[arg(short = 's', long = "study", help = "Output: Study")]
    pub study: bool,

    /// 休闲类支出
    #[arg(short = 'r', long = "rest", help = "Output: Rest")]
    pub rest: bool,

    /******************************************************/
    ///工资类收入
    #[arg(long = "salary", help = "Input: Salary")]
    pub salary: bool,

    ///转账类收入
    #[arg(short = 't', long = "transfer", help = "Input: Transfer")]
    pub transfer: bool,

    ///其他类收入
    #[arg(short = 'o', long = "other", help = "Input: Other")]
    pub other: bool,

    /******************************************************/
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

    /******************************************************/
    /// 位置参数：
    /// - 如果有三个参数：date description amount
    /// - 如果有两个参数：description amount
    #[arg(value_parser)]
    pub args: Vec<String>,
}

impl Cli {
    pub fn output(&self) -> String {
        if self.food {
            "food".to_string()
        } else if self.life {
            "life".to_string()
        } else if self.study {
            "study".to_string()
        } else if self.rest {
            "rest".to_string()
        } else {
            "".to_string()
        }
    }
    pub fn input(&self) -> String {
        // self.input.to_string()
        if self.salary {
            "salary".to_string()
        } else if self.transfer {
            "transfer".to_string()
        } else if self.other {
            "other".to_string()
        } else {
            "".to_string()
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
        } else if let Some(params) = &self.daily {
            if params.len() != 3 {
                eprintln!(
                    "Error: `--da` requires at least 3 parameters: breakfast, lunch, dinner, but got {}",
                    params.len()
                );
                return;
            }
            println!("Today's routine: {:?}", params);
            //for example: ml --da a10 w20 a20
            let today = Utc::now().naive_utc().date();
            let (account, amount) = parse_daily_params(params[0].to_string()).unwrap();
            create_transaction(&NewTransaction::new(
                today,
                "food".to_string(),
                "breakfast".to_string(),
                amount,
                account,
            ))
            .expect("Error creating transaction");
            let (account, amount) = parse_daily_params(params[1].to_string()).unwrap();
            create_transaction(&NewTransaction::new(
                today,
                "food".to_string(),
                "lunch".to_string(),
                amount,
                account,
            ))
            .expect("Error creating transaction");
            let (account, amount) = parse_daily_params(params[2].to_string()).unwrap();
            create_transaction(&NewTransaction::new(
                today,
                "food".to_string(),
                "dinner".to_string(),
                amount,
                account,
            ))
            .expect("Error creating transaction");
        } else if let &Some(flag) = &self.export {
            if flag {
                export_to_excel().expect("Error exporting to Excel");
            }
        } else if let Some(file_name) = &self.import {
            import_from_excel(file_name).unwrap()
        } else if let &Some(flag) = &self.undo {
            if flag {
                undo_last_transaction().expect("Error undoing last transaction");
                println!("undo success");
            }
        } else if let &Some(flag) = &self.sync {
            if flag {
                println!("Syncing data...");
            }
        } else {
            if self.args.len() < 2 {
                eprintln!("Error: Not enough arguments provided for transaction creation");
                return;
            }
            let output = self.output();
            let input = self.input();
            let kind = if output.len() > 0 { output } else { input };
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
