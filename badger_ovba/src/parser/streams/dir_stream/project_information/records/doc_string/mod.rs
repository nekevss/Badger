use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug)]
pub struct DocStringRecord {
    id: u16,
    size_of_doc_string: u32,
    doc_string: Vec<u8>,
    size_of_doc_string_unicode: u32,
    doc_string_unicode: Vec<u8>,
}

impl DocStringRecord {
    pub fn value(&self) -> String {
        String::from_utf8(self.doc_string_unicode.clone()).unwrap()
    }
}

impl Parsable for DocStringRecord {
    type Output = DocStringRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size_of_doc_string = utils::get_u32(cursor)?;
        let doc_string = utils::get_n_bytes(cursor, size_of_doc_string as usize)?;
        let _reserved = utils::get_u16(cursor)?;
        let size_of_doc_string_unicode = utils::get_u32(cursor)?;
        let doc_string_unicode = utils::get_n_bytes(cursor, size_of_doc_string_unicode as usize)?;

        Ok(Self {
            id,
            size_of_doc_string,
            doc_string,
            size_of_doc_string_unicode,
            doc_string_unicode,
        })
    }
}
