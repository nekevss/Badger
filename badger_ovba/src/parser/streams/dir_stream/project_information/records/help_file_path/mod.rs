use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug)]
pub struct HelpFilePathRecord {
    id: u16,
    size_of_help_file1: u32,
    help_file1: Vec<u8>,
    size_of_help_file2: u32,
    help_file2: Vec<u8>,
}

impl HelpFilePathRecord {
    pub fn value1(&self) -> String {
        String::from_utf8(self.help_file1.clone()).unwrap()
    }

    pub fn value2(&self) -> String {
        String::from_utf8(self.help_file2.clone()).unwrap()
    }
}

impl Parsable for HelpFilePathRecord {
    type Output = HelpFilePathRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor);
        let size_of_help_file1 = utils::get_u32(cursor);
        let help_file1 = utils::get_n_bytes(cursor, size_of_help_file1 as usize);
        let _reserved = utils::get_u16(cursor);
        let size_of_help_file2 = utils::get_u32(cursor);
        let help_file2 = utils::get_n_bytes(cursor, size_of_help_file2 as usize);

        Ok(Self {
            id,
            size_of_help_file1,
            help_file1,
            size_of_help_file2,
            help_file2,
        })
    }
}
