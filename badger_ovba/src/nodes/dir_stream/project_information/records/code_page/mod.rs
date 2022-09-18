use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct CodePageRecord {
    _id: u16,
    _size: u32,
    code_page: u16,
}

impl CodePageRecord {
    pub fn new() -> Self {
        Self {
            _id: 0x0003,
            _size: 0x00000002,
            code_page: 1252 as u16,
        }
    }

    pub fn value(&self) -> u16 {
        self.code_page
    }
}

impl Parsable for CodePageRecord {
    type Output = CodePageRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size = utils::get_u32(cursor)?;
        let code_page = utils::get_u16(cursor)?;

        Ok(Self {
            _id: id,
            _size: size,
            code_page,
        })
    }
}
