use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug)]
pub struct ReferenceRegistered {
    id: u16,
    size_of_libid: u32,
    libid: Vec<u8>,
}

impl Parsable for ReferenceRegistered {
    type Output = ReferenceRegistered;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let _size = utils::get_u32(cursor)?;

        let size_of_libid = utils::get_u32(cursor)?;
        let libid = utils::get_n_bytes(cursor, size_of_libid as usize)?;

        let _reserved1 = utils::get_u32(cursor)?;
        let _reserved2 = utils::get_u16(cursor)?;

        Ok(Self {
            id,
            size_of_libid,
            libid,
        })
    }
}
