use std::io::{self, Read};

use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::event::{KeyCode, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
    pub kind: KeyEventKind,
    pub state: KeyEventState,
}

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }
    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{:?} \r", event);
                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                }
                Err(err) => print!("Error: {}", err),
                _ => (),
            }
        }
        for b in io::stdin().bytes() {
            match b {
                Ok(b) => {
                    let c = b as char;
                    if c.is_control() {
                        println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
                    } else {
                        println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
                    }
                    if c == 'q' {
                        break;
                    }
                }
                Err(err) => println!("Error: {err}"),
            }
        }
        disable_raw_mode().unwrap();
    }
}
