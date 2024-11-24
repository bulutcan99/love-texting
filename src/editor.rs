use anyhow::anyhow;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{reader::Reader, screen::Screen};

// All the editoring stuff will process here.
pub struct Editor {
    reader: Reader,
    screen: Screen,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            reader: Reader,
            screen: Screen,
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    pub fn process_key_stroke(&self) -> Result<bool, anyhow::Error> {
        match self.reader.read_key_stroke()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                println!("EXITING!\r");
                Ok(false)
            }
            key => {
                println!("Key Pressed: {:?}\r", key);
                Ok(true)
            }
        }
    }

    pub fn start(&self) -> Result<(), anyhow::Error> {
        self.screen.enable_raw_mode()
    }

    pub fn run(&self) -> Result<bool, anyhow::Error> {
        self.screen.clear_screen()?;
        self.process_key_stroke()
    }
}
