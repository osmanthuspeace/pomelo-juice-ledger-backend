use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/bin/cli.rs");
    let exe_path = "target/debug/cli";

    let status = Command::new("install_name_tool")
        .arg("-add_rpath")
        .arg("/opt/homebrew/opt/libiconv/lib")
        .arg(exe_path)
        .status()
        .expect("Failed to execute install_name_tool");

    if !status.success() {
        eprintln!("Failed to add rpath to the executable");
    } else {
        println!("Successfully added rpath to the executable");
    }
    let status = Command::new("install_name_tool")
        .arg("-add_rpath")
        .arg("/opt/homebrew/opt/libpq/lib")
        .arg(exe_path)
        .status()
        .expect("Failed to execute install_name_tool");
    if !status.success() {
        eprintln!("Failed to add rpath to the executable");
    } else {
        println!("Successfully added rpath to the executable");
    }
}
