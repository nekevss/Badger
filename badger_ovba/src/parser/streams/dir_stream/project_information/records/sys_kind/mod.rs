//! Implements the SysKindRecord 2.3.4.2.1.1
//!
//!
use crate::error::Error;
use crate::parser::{Parsable, utils};
use std::io::Cursor;

#[derive(Debug)]
pub enum SysKind {
    Win16,
    Win32,
    Win64,
    MacOS,
}

#[derive(Debug)]
pub struct SysKindRecord {
    id: u16,
    size: u32,
    sys_kind: SysKind,
}

impl SysKindRecord {
    pub fn value(&self) -> String {
        match self.sys_kind {
            SysKind::Win16 => "Win16".into(),
            SysKind::Win32 => "Win32".into(),
            SysKind::Win64 => "Win64".into(),
            SysKind::MacOS => "MacOS".into(),
        }
    }
}

impl Parsable for SysKindRecord {
    type Output = SysKindRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor);
        let size = utils::get_u32(cursor);
        let sys_kind_id = utils::get_u32(cursor);

        let sys_kind = match sys_kind_id {
            0x00000000 => SysKind::Win16,
            0x00000001 => SysKind::Win32,
            0x00000002 => SysKind::MacOS,
            0x00000003 => SysKind::Win64,
            _ => {
                return Err(Error::Parser(
                    "Invalid SysKind Value provided".into(),
                    cursor.position(),
                ))
            }
        };

        Ok(Self { id, size, sys_kind })
    }
}
