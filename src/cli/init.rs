use crate::service::init_service::init_summary;

pub fn handle_init_command(params: Vec<f64>) {
    if params.len() < 5 {
        eprintln!("Error: `init` requires at least 5 parameters");
        return;
    }
    println!("Initializing system with parameters: {:?}", params);
    init_summary(params[0], params[1], params[2], params[3], params[4])
        .expect("Error initializing system");
}
