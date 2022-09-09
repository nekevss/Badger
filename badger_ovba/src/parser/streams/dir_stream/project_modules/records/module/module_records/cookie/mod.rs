use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug)]
pub struct ModuleCookieRecord {
    id: u16,
    size: u32,
    cookie: u16,
}

impl Parsable for ModuleCookieRecord {
    type Output = ModuleCookieRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size = utils::get_u32(cursor)?;
        let cookie = utils::get_u16(cursor)?;

        Ok(Self { id, size, cookie })
    }
}
