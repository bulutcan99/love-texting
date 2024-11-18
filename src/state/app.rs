use core::fmt;
use std::fmt::{Debug, Formatter};

use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct State {
    pub content: String,
    pub size: f64,
    pub mono: bool,
    pub file_name: Option<String>,
    pub is_dirty: bool,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Size {:.1}{}: {}",
            self.size,
            if self.mono { " mono" } else { "" },
            self.content
        )
    }
}
