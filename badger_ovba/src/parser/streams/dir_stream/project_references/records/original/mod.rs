use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug)]
pub struct ReferenceOriginal {
    id: u16,
    size_of_libid_original: u32,
    libid_original: Vec<u8>,
}

impl Parsable for ReferenceOriginal {
    type Output = ReferenceOriginal;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, crate::error::Error> {
        let id = utils::get_u16(cursor);
        let size_of_libid_original = utils::get_u32(cursor);
        let libid_original = utils::get_n_bytes(cursor, size_of_libid_original as usize);

        Ok(Self {
            id,
            size_of_libid_original,
            libid_original,
        })
    }
}
