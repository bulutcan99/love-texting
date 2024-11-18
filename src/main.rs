use std::io::{self, Read};

use anyhow::{Error, Ok};
use crossterm::terminal;

fn main() -> Result<(), Error> {
    terminal::enable_raw_mode()?;
    let mut buf = Vec::new();
    dbg!("Hello, World!");
    buf.push(10);
    while io::stdin().read(&mut buf)? > 0 && buf != [b'q'] {}
    Ok(())
}
