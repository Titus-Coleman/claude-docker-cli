use dotenv::dotenv;
use postgres::{Client, NoTls};
use std::io::{stdin, stdout, Write};
use std::os::unix::process;
use std::{any::Any, env};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
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

    // let mut client = Client::connect(&db_connection_string, NoTls).unwrap();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "So what are you looking for? (press Esc to finish):\r\n"
    )
    .unwrap();
    stdout.flush().unwrap();

    let mut text = String::new();

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Esc => break,
            Key::Char('\n') => {
                text.push('\n');
                write!(stdout, "\r\n").unwrap();
            }
            Key::Char(c) => {
                text.push(c);
                write!(stdout, "{}", c).unwrap();
            }
            Key::Backspace => {
                if !text.is_empty() {
                    text.pop();
                    write!(stdout, "\x08 \x08").unwrap();
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "\r\nYou entered:\r\n{}\r\n", text).unwrap();
    stdout.flush().unwrap();
}
