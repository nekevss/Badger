use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct ModulePrivateRecord {
    id: u16,
}

impl ModulePrivateRecord {
    pub fn new() -> Self {
        Self { id: 0x0028 }
    }
}

impl Parsable for ModulePrivateRecord {
    type Output = ModulePrivateRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let _reserved = utils::get_u32(cursor)?;

        Ok(Self { id })
    }
}
