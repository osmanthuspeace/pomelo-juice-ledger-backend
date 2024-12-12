use crate::transform::export_to_excel::export_to_excel;

pub fn handle_export_command() {
    export_to_excel().expect("Error exporting to Excel");
    println!("Exported to Excel successfully.");
}
