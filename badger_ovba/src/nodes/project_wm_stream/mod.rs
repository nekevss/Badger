use std::io::Cursor;
use crate::Error;
use crate::parser::{Parsable, utils};

pub mod records;
use records::NameMapRecord;

pub struct ProjectWmStream {
    name_map: NameMapRecord,
    terminator: u16,
}

impl ProjectWmStream {
    pub fn new() -> Self {
        let name_map = NameMapRecord::new();
        let terminator = 0x0000;
        
        Self {
            name_map,
            terminator,
        }
    }
}

impl Parsable for ProjectWmStream {
    type Output = ProjectWmStream;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let name_map = NameMapRecord::parse(cursor)?;
        let terminator = utils::get_u16(cursor)?;

        Ok(Self {
            name_map,
            terminator,
        })
    }
}