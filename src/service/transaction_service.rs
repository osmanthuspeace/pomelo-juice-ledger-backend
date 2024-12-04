use crate::db::models::{NewTransaction, Transaction};
use crate::db::schema::transactions::dsl::*;
use diesel::result::Error;
use diesel::{Connection, PgConnection, RunQueryDsl};
use dotenv::dotenv;
use std::env;

pub fn create_transaction(new_transaction: &NewTransaction) -> Result<Transaction, Error> {
    let mut connection = establish_connection();
    diesel::insert_into(transactions)
        .values(new_transaction)
        .get_result(&mut connection)
}
fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
