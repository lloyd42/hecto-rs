use std::io::{self, Read};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    enable_raw_mode();
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
        if c == 'q' {
            disable_raw_mode();
            break;
        }
    }
}
