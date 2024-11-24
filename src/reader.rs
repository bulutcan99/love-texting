use std::time::Duration;

use anyhow::{self, Result};

use crossterm::event::{self, Event, KeyEvent};

pub struct Reader;
impl Reader {
    fn read_key_stroke(&self) -> Result<KeyEvent, anyhow::Error> {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                return Ok(event);
            }
        }
        Err(anyhow::anyhow!(
            "There is an error while listening the key stroke!"
        ))
    }
}
