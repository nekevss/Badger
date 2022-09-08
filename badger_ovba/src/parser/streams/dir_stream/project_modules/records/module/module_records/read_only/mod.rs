use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug)]
pub struct ModuleReadOnlyRecord {
    id: u16,
}

impl Parsable for ModuleReadOnlyRecord {
    type Output = ModuleReadOnlyRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor);
        let _reserved = utils::get_u32(cursor);

        Ok(Self { id })
    }
}
