use std::io;
use std::io::BufRead;

use ansi_parser::AnsiSequence;
use ansi_parser::{AnsiParser, Output};

const RESET_CODE: &str = "\x1b[0m";

struct EscapeCodeState {
    foreground: Option<AnsiSequence>,
    background: Option<AnsiSequence>,
    other: Vec<AnsiSequence>,
}

impl EscapeCodeState {
    fn new() -> Self {
        EscapeCodeState {
            foreground: None,
            background: None,
            other: Vec::new(),
        }
    }

    fn handle_ansi_sequence(&mut self, code: AnsiSequence) {
        // Clear state vector if a reset sequence occurs
        match code {
            AnsiSequence::SetGraphicsMode(ref vec) => match vec.len() {
                // Reset code
                1 if vec[0] == 0 => {
                    self.foreground = None;
                    self.background = None;
                    self.other.clear();
                }
                // Simple foreground or background color
                2 if vec[0] == 1 => match vec[1] {
                    x if (30..=37).contains(&x) => {
                        self.foreground = Some(code);
                    }
                    x if (40..=47).contains(&x) => {
                        self.background = Some(code);
                    }
                    _ => self.other.push(code),
                },
                _ => self.other.push(code),
            },
            // Ignore sequences not related to colors/reseting. 
            _ => ()
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = EscapeCodeState::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        print!(
            "{}{}{}",
            if let Some(ref foreground) = state.foreground {
                foreground.to_string()
            } else {
                "".to_owned()
            },
            if let Some(ref background) = state.background {
                background.to_string()
            } else {
                "".to_owned()
            },
            state
                .other
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
                    state.handle_ansi_sequence(code);
                }
            }
        }
        // Write reset code at the end of line
        println!("{}", RESET_CODE);
    }

    Ok(())
}
