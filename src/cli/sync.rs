use crate::service::sync_service::sync_balance;

pub fn handle_sync_command() {
    println!("Syncing data...");
    sync_balance().expect("Error syncing data");
}
