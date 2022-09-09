use crate::error::Error;
use crate::parser::utils;
use crate::parser::Parsable;
use std::io::Cursor;

#[derive(Debug)]
pub struct ReferenceName {
    id: u16,
    size_of_name: u32,
    name: Vec<u8>,
    reserved: u16,
    size_of_name_unicode: u32,
    name_unicode: Vec<u8>,
}

impl Parsable for ReferenceName {
    type Output = ReferenceName;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size_of_name = utils::get_u32(cursor)?;
        let name = utils::get_n_bytes(cursor, size_of_name as usize)?;
        let reserved = utils::get_u16(cursor)?;
        let size_of_name_unicode = utils::get_u32(cursor)?;
        let name_unicode = utils::get_n_bytes(cursor, size_of_name_unicode as usize)?;

        Ok(Self {
            id,
            size_of_name,
            name,
            reserved,
            size_of_name_unicode,
            name_unicode,
        })
    }
}
