//! Implements PROJECTLCID Record 2.3.4.2.1.3
//!
//!
use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug)]
pub struct LcidRecord {
    id: u16,
    size: u32,
    lcid: u32,
}

impl LcidRecord {
    pub fn value(&self) -> u32 {
        self.lcid
    }
}

impl Parsable for LcidRecord {
    type Output = LcidRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size = utils::get_u32(cursor)?;
        let lcid = utils::get_u32(cursor)?;

        Ok(Self { id, size, lcid })
    }
}
