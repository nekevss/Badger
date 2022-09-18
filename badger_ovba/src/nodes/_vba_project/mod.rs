use std::io::Cursor;
use crate::Error;
use crate::parser::{Parsable, utils};

pub struct VbaProjectStream {
    version: u16,
    performance_cache: Option<Vec<u8>>,
}


// Important Note regarding performance cache: this value must be ignored on read, and
// absent on write. But omitting the value from the stream feels like it would be
// misrepresenting the stream.
impl VbaProjectStream {
    pub(crate) fn new() -> Self {
        Self {
            version: 0xFFFF,
            performance_cache: None,
        }
    }
}

impl Parsable for VbaProjectStream {
    type Output = VbaProjectStream;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let _reserved1 = utils::get_u16(cursor)?;
        let version = utils::get_u16(cursor)?;
        let _reserved2 = utils::get_u8(cursor)?;
        let _reserved3 = utils::get_u16(cursor)?;

        Ok(Self {
            version,
            performance_cache: None
        })
    }
}