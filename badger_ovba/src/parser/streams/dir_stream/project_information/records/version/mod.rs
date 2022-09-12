use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct VersionRecord {
    id: u16,
    major_version: u32,
    minor_version: u16,
}

// VersionRecord will probably never be cleaning determined for write. It is not entirely clear where the information
// can be found.
impl VersionRecord {
    pub fn new() -> Self {
        Self {
            id: 0x0009,
            major_version: 1656542101 as u32,
            minor_version: 0x0006,
        }
    }

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
        let id = utils::get_u16(cursor)?;
        let _reserved = utils::get_u32(cursor)?;
        let major_version = utils::get_u32(cursor)?;
        let minor_version = utils::get_u16(cursor)?;

        Ok(Self {
            id,
            major_version,
            minor_version,
        })
    }
}
