use crate::error::Error;
use crate::parser::streams::dir_stream::project_information::records::name;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug)]
pub struct ModuleStreamNameRecord {
    id: u16,
    size_of_stream_name: u32,
    stream_name: Vec<u8>,
    size_of_stream_name_unicode: u32,
    stream_name_unicode: Vec<u8>,
}

impl ModuleStreamNameRecord {
    pub fn value(&self) -> String {
        let name_u16 = utils::convert_le_u16(&self.stream_name_unicode).unwrap();
        String::from_utf16(&name_u16).unwrap()
    }
}

impl Parsable for ModuleStreamNameRecord {
    type Output = ModuleStreamNameRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor);
        let size_of_stream_name = utils::get_u32(cursor);
        let stream_name = utils::get_n_bytes(cursor, size_of_stream_name as usize);
        let _reserved = utils::get_u16(cursor);
        let size_of_stream_name_unicode = utils::get_u32(cursor);
        let stream_name_unicode = utils::get_n_bytes(cursor, size_of_stream_name_unicode as usize);

        Ok(Self {
            id,
            size_of_stream_name,
            stream_name,
            size_of_stream_name_unicode,
            stream_name_unicode,
        })
    }
}
