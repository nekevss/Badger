// page 37 2.3.4.2.1.12

use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct ConstantsRecord {
    id: u16,
    size_of_constants: u32,
    constants: Vec<u8>,
    size_of_constants_unicode: u32,
    constants_unicode: Vec<u8>,
}

impl ConstantsRecord {
    pub fn new() -> Self {
        Self {
            id: 0x000C,
            size_of_constants: 0,
            constants: Vec::new(),
            size_of_constants_unicode: 0,
            constants_unicode: Vec::new(),
        }
    }

    pub fn value(&self) -> String {
        String::from_utf8(self.constants_unicode.clone()).unwrap()
    }
}

impl Parsable for ConstantsRecord {
    type Output = ConstantsRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size_of_constants = utils::get_u32(cursor)?;
        let constants = utils::get_n_bytes(cursor, size_of_constants as usize)?;
        let _reserved = utils::get_u16(cursor)?;
        let size_of_constants_unicode = utils::get_u32(cursor)?;
        let constants_unicode = utils::get_n_bytes(cursor, size_of_constants_unicode as usize)?;

        Ok(Self {
            id,
            size_of_constants,
            constants,
            size_of_constants_unicode,
            constants_unicode,
        })
    }
}
