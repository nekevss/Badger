use crate::parser::{utils, Parsable};
use std::io::Cursor;
use crate::utils::convert_mbcs_value;

#[derive(Debug, Clone)]
pub struct ReferenceOriginal {
    id: u16,
    size_of_libid_original: u32,
    libid_original: Vec<u8>,
}

impl ReferenceOriginal {
    pub fn new() -> Self {
        Self {
            id: 0x0033,
            size_of_libid_original: 0,
            libid_original: Vec::<u8>::new(),
        }
    }

    pub fn reference_type(&self) -> &'static str {
        "original"
    }

    pub fn libid_original(&self) -> String {
        let converted_utf8 = convert_mbcs_value(&self.libid_original);
        String::from_utf8(converted_utf8).expect("Converted value should exist")
    }
}

impl Parsable for ReferenceOriginal {
    type Output = ReferenceOriginal;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, crate::error::Error> {
        let id = utils::get_u16(cursor)?;
        let size_of_libid_original = utils::get_u32(cursor)?;
        let libid_original = utils::get_n_bytes(cursor, size_of_libid_original as usize)?;

        Ok(Self {
            id,
            size_of_libid_original,
            libid_original,
        })
    }
}
