use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug)]
pub struct ModuleTypeRecord {
    id: u16,
}

impl Parsable for ModuleTypeRecord {
    type Output = ModuleTypeRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let _reserved = utils::get_u32(cursor)?;

        Ok(Self { id })
    }
}
