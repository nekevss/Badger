use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct ModuleOffsetRecord {
    id: u16,
    size: u32,
    text_offset: u32,
}

impl ModuleOffsetRecord {
    pub fn new() -> Self {
        Self {
            id: 0x0031,
            size: 0x00000004,
            text_offset: 0 as u32,
        }
    }

    pub fn value(&self) -> u32 {
        self.text_offset
    }
}

impl Parsable for ModuleOffsetRecord {
    type Output = ModuleOffsetRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size = utils::get_u32(cursor)?;
        let text_offset = utils::get_u32(cursor)?;

        Ok(Self {
            id,
            size,
            text_offset,
        })
    }
}
