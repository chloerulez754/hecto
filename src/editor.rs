use crossterm::event::{self, Event::Key, KeyCode::Char};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub struct Editor;

impl Editor {
    pub const fn new() -> Self {
        Self
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();

        loop {
            match event::read() {
                Ok(Key(event)) => {
                    println!("{event:?}\r");

                    if let Char('q') = event.code {
                        break;
                    }
                }
                Err(err) => println!("Error: {err}"),
                _ => (),
            }
        }

        disable_raw_mode().unwrap();
    }
}
