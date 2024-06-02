use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "hello" => println!("Hello from Rust CLI!"),
            _ => println!("Unknown command."),
        }
    } else {
        println!("No command provided.");
    }
}
