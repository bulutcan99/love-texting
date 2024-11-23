use std::io::{self, Read};

use anyhow::{Error, Ok};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};

struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode!");
    }
}

fn main() -> Result<(), Error> {
    let _ = CleanUp;
    terminal::enable_raw_mode()?;
    loop {
        if let Event::Key(event) = event::read()? {
            match event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => break,
                _ => {
                    //todo
                }
            }
            println!("{:?}\r", event)
        }
    }
    Ok(())
}
