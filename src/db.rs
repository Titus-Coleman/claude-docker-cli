use dotenv::dotenv;
use postgres::{Client, NoTls};
use std::env;

pub fn establish_db_connection() -> Client {
    dotenv().ok();
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "".to_string());
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| "".to_string());
    let db_user = env::var("DB_USER").unwrap_or_else(|_| "".to_string());
    let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "".to_string());

    let db_connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );
    println!("{}", db_connection_string.to_string());

    return Client::connect(&db_connection_string, NoTls).unwrap();
}
