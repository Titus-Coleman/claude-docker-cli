use dotenv::dotenv;
use postgres::{Client, NoTls};
use std::{any::Any, env};

fn main() {
    dotenv().ok();
    let mut client = Client::connect(
        "postgresql://ADMIN:PASS@host.docker.internal:PORT/db",
        NoTls,
    )
    .unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "hello" => println!("Hello from Rust CLI!"),
            command => match client.query(command, &[]) {
                Ok(rows) => {
                    for row in rows {
                        let mut values = Vec::new();
                        for i in 0..row.len() {
                            println!("{}", i);
                            values.push(format!("{}", row.get::<_, String>(i)));
                        }
                        println!("{:?}", values.join(", "));
                    }
                }
                Err(e) => {
                    let error_message = format!("Error: {}", e.to_string());
                    eprintln!("{}", error_message);
                }
            },
        }
    } else {
        println!("No command provided.");
    }
}
