use crate::service::undo_service::undo_last_transaction;

pub fn handle_undo_command() {
    undo_last_transaction().expect("Error undoing last transaction");
    println!("Undo success");
}
