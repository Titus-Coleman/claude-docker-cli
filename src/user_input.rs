use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn user_input() -> String {
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
    text
}
