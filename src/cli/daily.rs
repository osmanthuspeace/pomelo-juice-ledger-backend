use crate::db::models::NewTransaction;
use crate::service::create_service::create_transaction;
use crate::util::parse_daily_params::parse_daily_params;
use chrono::Utc;

pub fn handle_daily_command(daily_params: Vec<String>) {
    if daily_params.len() != 3 {
        eprintln!("Error: `daily` requires 3 parameters: breakfast, lunch, dinner");
        return;
    }
    println!("Today's routine: {:?}", daily_params);
    let today = Utc::now().naive_utc().date();

    let (account, amount) = parse_daily_params(daily_params[0].to_string()).unwrap();
    create_transaction(&NewTransaction::new(
        today,
        "food".to_string(),
        "breakfast".to_string(),
        amount,
        account,
    ))
    .expect("Error creating breakfast transaction");
    let (account, amount) = parse_daily_params(daily_params[1].to_string()).unwrap();
    create_transaction(&NewTransaction::new(
        today,
        "food".to_string(),
        "lunch".to_string(),
        amount,
        account,
    ))
    .expect("Error creating lunch transaction");
    let (account, amount) = parse_daily_params(daily_params[2].to_string()).unwrap();
    create_transaction(&NewTransaction::new(
        today,
        "food".to_string(),
        "dinner".to_string(),
        amount,
        account,
    ))
    .expect("Error creating dinner transaction");
}
