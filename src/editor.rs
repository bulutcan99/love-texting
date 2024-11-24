use anyhow::anyhow;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::reader::Reader;

pub struct Editor {
    reader: Reader,
}

impl Editor {
    pub fn new() -> Self {
        Self { reader: Reader }
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

    pub fn run(&self) -> Result<bool, anyhow::Error> {
        self.process_key_stroke()
    }
}
