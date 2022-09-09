use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

use super::Reference;

#[derive(Debug)]
pub struct ReferenceProject {
    id: u16,
    size_of_libid_absolute: u32,
    libid_absolute: Vec<u8>,
    size_of_libid_relative: u32,
    libid_relative: Vec<u8>,
    major_version: u32,
    minor_version: u16,
}

impl Parsable for ReferenceProject {
    type Output = ReferenceProject;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let _size = utils::get_u32(cursor)?;

        let size_of_libid_absolute = utils::get_u32(cursor)?;
        let libid_absolute = utils::get_n_bytes(cursor, size_of_libid_absolute as usize)?;

        let size_of_libid_relative = utils::get_u32(cursor)?;
        let libid_relative = utils::get_n_bytes(cursor, size_of_libid_relative as usize)?;

        let major_version = utils::get_u32(cursor)?;
        let minor_version = utils::get_u16(cursor)?;

        Ok(Self {
            id,
            size_of_libid_absolute,
            libid_absolute,
            size_of_libid_relative,
            libid_relative,
            major_version,
            minor_version,
        })
    }
}
