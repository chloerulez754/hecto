use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::io::{self, Read};

fn main() {
    enable_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{c}");

        if c == 'q' {
            break;
        }
    }

    disable_raw_mode().unwrap();
}
