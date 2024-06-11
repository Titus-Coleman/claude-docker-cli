use dotenv::dotenv;
use regex::Regex;
use std::env;

pub async fn sql_response_validator(response: String) -> String {
    dotenv().ok();
    let regex_pattern = env::var("SQL_REGEX").unwrap_or_else(|_| r"(?i)\b(INSERT|UPDATE|DELETE|ALTER\s+TABLE|DROP\s+TABLE|TRUNCATE\s+TABLE|CREATE\s+TABLE|CREATE\s+INDEX|ALTER\s+INDEX|DROP\s+INDEX|CREATE\s+SCHEMA|DROP\s+SCHEMA|CREATE\s+VIEW|ALTER\s+VIEW|DROP\s+VIEW|CREATE\s+FUNCTION|ALTER\s+FUNCTION|DROP\s+FUNCTION|CREATE\s+PROCEDURE|ALTER\s+PROCEDURE|DROP\s+PROCEDURE|CREATE\s+TRIGGER|ALTER\s+TRIGGER|DROP\s+TRIGGER|CREATE\s+DOMAIN|ALTER\s+DOMAIN|DROP\s+DOMAIN|CREATE\s+SEQUENCE|ALTER\s+SEQUENCE|DROP\s+SEQUENCE|CREATE\s+TYPE|ALTER\s+TYPE|DROP\s+TYPE|CREATE\s+EXTENSION|ALTER\s+EXTENSION|DROP\s+EXTENSION|RENAME|REINDEX|CLUSTER|COPY|VACUUM|REFRESH\s+MATERIALIZED\s+VIEW)\b".to_string());
    println!("Regex pattern: {}", regex_pattern);
    let regex = Regex::new(&regex_pattern).unwrap();

    if regex.is_match(&response) {
        println!("The SQL query contains a destructive command.");
        "Error: Destructive command found or response is blocked".to_string()
    } else {
        println!("The SQL query does not contain any destructive commands.");
        response
    }
}
