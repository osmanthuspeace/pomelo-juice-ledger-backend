use crate::api::router;
use crate::cli::command::Cli;
use crate::db::models::NewTransaction;
use crate::service::create_service::create_transaction;
use clap::Parser;
use std::net::SocketAddr;

mod api;
mod cli;
mod db;
mod service;
mod util;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    cli.execute();

    let app = router::create_router();
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    
}
