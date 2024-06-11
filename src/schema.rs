use std::env;
use std::fs;
use std::io::Read;

pub fn schema() -> String {
    let schema_file = env::var("SCHEMA_PATH").unwrap_or_else(|_| "".to_string());
    println!("Schema location: {}", schema_file);
    let mut file = fs::File::open(schema_file).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    contents
}
