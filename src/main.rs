use std::io;
use std::io::BufRead;

use ansi_parser::AnsiSequence;
use ansi_parser::{AnsiParser, Output};

const RESET_CODE: &str = "\x1b[0m";

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

        let line = line?;
        let parsed = line.ansi_parse();
        for block in parsed.into_iter() {
            match block {
                Output::TextBlock(text) => print!("{}", text),
                Output::Escape(code) => {
                    print!("{}", code);

                    // Clear state vector if a reset sequence occurs
                    if code.to_string() == RESET_CODE {
                        state.clear()
                    } else {
                        state.push(code);
                    }
                }
            }
        }
        // Write reset code at the end of line
        println!("{}", RESET_CODE);
    }

    Ok(())
}
