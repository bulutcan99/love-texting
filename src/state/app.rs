use druid::{Data, Lens};

#[derive(Debug, Clone, Data, Lens)]
pub struct State {
    pub content: String,
    pub font_size: f64,
    pub file_name: Option<String>,
    pub is_dirty: bool,
}
