use std::net::SocketAddr;
use crate::cli::command::Cli;
use crate::db::models::NewTransaction;
use crate::service::create_service::create_transaction;
use chrono::NaiveDate;
use clap::Parser;
use crate::api::router;

mod api;
mod cli;
mod db;
mod service;
mod util;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let kind = cli.kind();
    let account = cli.account();
    let date: NaiveDate = cli.date();
    let description = cli.description().to_string();
    let amount = cli.amount();


    let app = router::create_router();
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
  
    
    let new_transaction = NewTransaction::new(date, kind, description, amount, account);
    create_transaction(&new_transaction).expect("创建账目时发生错误");
}
