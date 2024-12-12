use clap::Parser;
use my_ledger_backend::cli::command::Cli;

fn main() {
    Cli::execute();
}