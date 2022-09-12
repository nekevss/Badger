//! Implements PROJECTCOMPATVERSION Record 2.3.4.2.1.2
//!

use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct CompatVersionRecord {
    id: u16,
    size: u32,
    compat_version: u32,
}

impl CompatVersionRecord {
    pub fn new(compat_version: u32) -> Self {
        Self {
            id: 0x004A,
            size: 0x00000004,
            compat_version,
        }
    }

    pub fn value(&self) -> u32 {
        self.compat_version
    }
}

impl Parsable for CompatVersionRecord {
    type Output = CompatVersionRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size = utils::get_u32(cursor)?;
        let compat_version = utils::get_u32(cursor)?;

        Ok(Self {
            id,
            size,
            compat_version,
        })
    }
}
