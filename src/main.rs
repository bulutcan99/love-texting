use std::io::{self, Read};

use anyhow::{Error, Ok};
use crossterm::terminal;

struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode!");
    }
}

fn main() -> Result<(), Error> {
    let _ = CleanUp;
    terminal::enable_raw_mode()?;
    let mut buf = Vec::with_capacity(1);
    while io::stdin().read(&mut buf)? == 1 && buf != [b'q'] {
        let character = buf[0] as char;
        println!("You pressed: {}", character);
    }
    Ok(())
}
