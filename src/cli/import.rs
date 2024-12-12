use crate::transform::import_from_excel::import_from_excel;

pub fn handle_import_command(file: String) {
    import_from_excel(&file).expect("Error importing from Excel");
    println!("Imported from Excel successfully.");
}