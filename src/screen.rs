use std::{any, io::stdout, thread, time::Duration};

use anyhow::anyhow;
use crossterm::{execute, terminal};

pub struct Screen;

impl Drop for Screen {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode!");
    }
}

impl Screen {
    pub fn enable_raw_mode(&self) -> Result<(), anyhow::Error> {
        terminal::enable_raw_mode()
            .map_err(|e| anyhow::anyhow!("Error while enabling raw mode: {}", e))
    }

    pub fn clear_screen(&self) -> Result<(), anyhow::Error> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))
            .map_err(|e| anyhow::anyhow!("Error while clearing screen: {}", e))
    }

    pub fn refresh_screen(&self) -> Result<(), anyhow::Error> {
        self.clear_screen()
    }
}
