use std::env;
use std::io;
use std::io::BufRead;

fn main() {
    let mut args = env::args().skip(1);
    let prefix = args.next().unwrap_or_else(|| "".to_string());
    let postfix = args.next().unwrap_or_else(|| "".to_string());

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}{}{}", prefix, line.unwrap(), postfix);
    }
}
