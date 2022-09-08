use crate::error::Error;
use crate::parser::{Parsable, utils};
use std::io::Cursor;

#[derive(Debug)]
pub struct VersionRecord {
    id: u16,
    reserved: u32,
    major_version: u32,
    minor_version: u16,
}

impl VersionRecord {
    pub fn major_version(&self) -> u32 {
        self.major_version
    }

    pub fn minor_version(&self) -> u16 {
        self.minor_version
    }
}

impl Parsable for VersionRecord {
    type Output = VersionRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor);
        let reserved = utils::get_u32(cursor);
        let major_version = utils::get_u32(cursor);
        let minor_version = utils::get_u16(cursor);

        Ok(Self {
            id,
            reserved,
            major_version,
            minor_version,
        })
    }
}
