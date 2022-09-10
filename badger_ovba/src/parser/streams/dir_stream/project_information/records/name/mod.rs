use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug)]
pub struct NameRecord {
    id: u16,
    size_of_project_name: u32,
    name: Vec<u8>,
}

impl NameRecord {
    pub fn value(&self) -> String {
        String::from_utf8(self.name.clone()).unwrap()
    }
}

impl Parsable for NameRecord {
    type Output = NameRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size_of_project_name = utils::get_u32(cursor)?;
        let name = utils::get_n_bytes(cursor, size_of_project_name as usize)?;

        Ok(Self {
            id,
            size_of_project_name,
            name,
        })
    }
}
