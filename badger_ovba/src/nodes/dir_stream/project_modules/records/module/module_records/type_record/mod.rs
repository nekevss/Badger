use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct ModuleTypeRecord {
    id: u16,
}

impl ModuleTypeRecord {
    pub fn new() -> Self {
        Self { id: 0x0021 }
    }

    pub fn set_as_procedural_type(&mut self) {
        self.id = 0x0021;
    }

    pub fn set_as_non_procedural_type(&mut self) {
        self.id = 0x0022;
    }
}

impl Parsable for ModuleTypeRecord {
    type Output = ModuleTypeRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let _reserved = utils::get_u32(cursor)?;

        Ok(Self { id })
    }
}
