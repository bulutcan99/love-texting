use std::{
    io::{self, Read},
    time::Duration,
};

use anyhow::{Error, Ok};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};
use text::{
    editor::{self, Editor},
    reader::Reader,
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
    let editor = Editor::default();
    while editor.run()? {}
    Ok(())
}
