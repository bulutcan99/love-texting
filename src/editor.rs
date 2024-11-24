use crate::reader::Reader;

pub struct Editor {
    reader: Reader,
}

impl Editor {
    pub fn new(reader: Reader) -> Self {
        Self { reader }
    }

    pub fn process_key_stroke(&self) -> Result<(), anyhow::Error> {}
}
