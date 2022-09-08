use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug)]
pub struct LcidInvokeRecord {
    id: u16,
    size: u32,
    lcid_invoke: u32,
}

impl LcidInvokeRecord {
    pub fn value(&self) -> u32 {
        self.lcid_invoke
    }
}

impl Parsable for LcidInvokeRecord {
    type Output = LcidInvokeRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor);
        let size = utils::get_u32(cursor);
        let lcid_invoke = utils::get_u32(cursor);

        Ok(Self {
            id,
            size,
            lcid_invoke,
        })
    }
}
