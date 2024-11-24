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

fn main() -> Result<(), Error> {
    let editor = Editor::default();
    editor.start()?;
    while editor.run()? {}
    Ok(())
}
