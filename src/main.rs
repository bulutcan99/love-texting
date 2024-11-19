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
    buf.push(10);
    while io::stdin().read(&mut buf)? > 0 && buf != [b'q'] {}
    panic!("User quit the program");
}
