//! Implements the SysKindRecord 2.3.4.2.1.1
//!
//!
use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct SysKindRecord {
    id: u16,
    size: u32,
    sys_kind: u32,
}

impl SysKindRecord {
    pub fn new() -> Self {
        Self {
            id: 0x0001,
            size: 0x00000004,
            sys_kind: 0x00000003,
        }
    }

    pub fn value(&self) -> String {
        match self.sys_kind {
            0x00000000 => "Win16".into(),
            0x00000001 => "Win32".into(),
            0x00000002 => "MacOS".into(),
            0x00000003 => "Win64".into(),
            _ => "".into(),
        }
    }
}

impl Parsable for SysKindRecord {
    type Output = SysKindRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size = utils::get_u32(cursor)?;
        let sys_kind = utils::get_u32(cursor)?;

        Ok(Self { id, size, sys_kind })
    }
}
