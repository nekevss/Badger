use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct ReferenceProject {
    id: u16,
    size_of_libid_absolute: u32,
    libid_absolute: Vec<u8>,
    size_of_libid_relative: u32,
    libid_relative: Vec<u8>,
    major_version: u32,
    minor_version: u16,
}

impl ReferenceProject {
    pub fn new() -> Self {
        Self {
            id: 0x000E,
            size_of_libid_absolute: 0,
            libid_absolute: Vec::<u8>::new(),
            size_of_libid_relative: 0,
            libid_relative: Vec::<u8>::new(),
            major_version: 0 as u32,
            minor_version: 0x0000,
        }
    }

    pub fn reference_type(&self) -> &'static str {
        "project"
    }
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
