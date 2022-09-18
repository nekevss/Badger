use crate::error::Error;
use crate::parser::utils;
use crate::parser::Parsable;
use crate::utils::convert_mbcs_value;
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct ReferenceName {
    id: u16,
    size_of_name: u32,
    name: Vec<u8>,
    size_of_name_unicode: u32,
    name_unicode: Vec<u8>,
}

impl ReferenceName {
    pub fn new() -> Self {
        Self {
            id: 0x0016,
            size_of_name: 0 as u32,
            name: Vec::<u8>::new(),
            size_of_name_unicode: 0 as u32,
            name_unicode: Vec::<u8>::new(),
        }
    }

    pub fn reference_type(&self) -> &'static str {
        "name"
    }

    pub fn value(&self) -> String {
        String::from_utf8(self.name_unicode.clone())
            .expect("name_unicode must always have a valid value present")
    }
}

impl Parsable for ReferenceName {
    type Output = ReferenceName;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size_of_name = utils::get_u32(cursor)?;
        let mbcs_name = utils::get_n_bytes(cursor, size_of_name as usize)?;
        let name = convert_mbcs_value(&mbcs_name);
        let name_from_mbcs = String::from_utf8(name).unwrap();
        println!("{}", name_from_mbcs);
        let _reserved = utils::get_u16(cursor)?;
        let size_of_name_unicode = utils::get_u32(cursor)?;
        let name_unicode = utils::get_n_bytes(cursor, size_of_name_unicode as usize)?;

        Ok(Self {
            id,
            size_of_name,
            name: Vec::<u8>::new(),
            size_of_name_unicode,
            name_unicode,
        })
    }
}
