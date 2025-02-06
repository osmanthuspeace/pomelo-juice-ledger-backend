use std::env;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;

pub(crate) fn establish_connection() -> PgConnection {
    dotenv().ok();
    load_embedded_env();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
fn load_embedded_env() {
    let env_content = include_str!("../../.env"); 
    for line in env_content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            env::set_var(key.trim(), value.trim());
        }
    }
}