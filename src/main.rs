use std::io;
use std::io::BufRead;

use ansi_parser::AnsiSequence;
use ansi_parser::{AnsiParser, Output};

const RESET_CODE: &str = "\x1b[0m";

struct State {
    fgcolor: String,
    bgcolor: String,
    bold: bool,
    italic: bool,
    underline: bool,
    inverse: bool,
}

impl State {
    fn new() -> Self {
        State {
            fgcolor: String::new(),
            bgcolor: String::new(),
            bold: false,
            italic: false,
            underline: false,
            inverse: false,
        }
    }

    fn current_state(&self) -> String {
        format!("")
    }

    fn wrap_line(&self, line: &str) -> String {
        format!("{}{}{}", self.current_state(), line, RESET_CODE)
    }
}

fn main() {
    let mut state = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        print!(
            "{}",
            state
                .iter()
                .map(|code: &AnsiSequence| code.to_string())
                .collect::<Vec<_>>()
                .join("")
        );

        let line = line.unwrap();
        let parsed = line.ansi_parse();
        for block in parsed.into_iter() {
            match block {
                Output::TextBlock(text) => print!("{}", text),
                Output::Escape(code) => {
                    print!("{}", code);
                    state.push(code);
                }
            }
        }
        // Write reset code at the end of line
        println!("{}", RESET_CODE);
    }
}
