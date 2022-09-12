use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct HelpContextRecord {
    id: u16,
    size: u32,
    help_context: u32,
}

impl HelpContextRecord {
    pub fn new() -> Self {
        Self {
            id: 0x0007,
            size: 0x00000004,
            help_context: 0x00000000,
        }
    }

    pub fn value(&self) -> u32 {
        self.help_context
    }
}

impl Parsable for HelpContextRecord {
    type Output = HelpContextRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size = utils::get_u32(cursor)?;
        let help_context = utils::get_u32(cursor)?;

        Ok(Self {
            id,
            size,
            help_context,
        })
    }
}
