use crate::cli::add::handle_add_command;
use crate::cli::daily::handle_daily_command;
use crate::cli::export::handle_export_command;
use crate::cli::import::handle_import_command;
use crate::cli::init::handle_init_command;
use crate::cli::sync::handle_sync_command;
use crate::cli::undo::handle_undo_command;
use clap::{Parser, Subcommand};

/// Ledger CLI
#[derive(Parser, Debug)]
#[command(
    name = "cli",
    version = "1.0",
    about = "记账本命令行工具",
    author = "osmanthuspeace"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 初始化参数
    Init {
        #[arg(num_args = 5)]
        params: Vec<f64>,
    },
    /// 每日惯例：三餐
    Daily {
        #[arg(num_args = 3)]
        daily_params: Vec<String>,
    },
    /// 导出 Excel 文件
    Export,
    /// 导入 Excel 文件
    Import { file: String },
    /// 同步余额
    Sync,
    /// 撤销上一步
    Undo,
    /// 添加交易记录
    Add {
        #[arg(num_args = 1..=3)]
        args: Vec<String>,
        #[arg(short = 'f', long = "food")]
        food: bool,
        #[arg(short = 'l', long = "life")]
        life: bool,
        #[arg(short = 's', long = "study")]
        study: bool,
        #[arg(short = 'r', long = "rest")]
        rest: bool,
        #[arg(long = "salary")]
        salary: bool,
        #[arg(short = 't', long = "transfer")]
        transfer: bool,
        #[arg(short = 'o', long = "other")]
        other: bool,
        #[arg(short = 'a', long = "alipay")]
        alipay: bool,
        #[arg(short = 'w', long = "wechat")]
        wechat: bool,
        #[arg(short = 'b', long = "bankofchina")]
        bankofchina: bool,
        #[arg(short = 'i', long = "icbc")]
        icbc: bool,
    },
}
impl Cli {
    pub fn execute() {
        let cli = Cli::parse();

        match cli.command {
            Commands::Init { params } => handle_init_command(params),
            Commands::Daily { daily_params } => handle_daily_command(daily_params),
            Commands::Export => handle_export_command(),
            Commands::Import { file } => handle_import_command(file),
            Commands::Undo => handle_undo_command(),
            Commands::Sync => handle_sync_command(),
            Commands::Add {
                args,
                food,
                life,
                study,
                rest,
                salary,
                transfer,
                other,
                alipay,
                wechat,
                bankofchina,
                icbc,
            } => handle_add_command(
                args,
                food,
                life,
                study,
                rest,
                salary,
                transfer,
                other,
                alipay,
                wechat,
                bankofchina,
                icbc,
            ),
        }
    }
}
