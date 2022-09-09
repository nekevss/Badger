use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug)]
pub struct ModuleNameUnicodeRecord {
    id: u16,
    size_of_module_name_unicode: u32,
    module_name_unicode: Vec<u8>,
}


impl ModuleNameUnicodeRecord {
    pub fn value(&self) -> String {
        String::from_utf8(self.module_name_unicode.clone()).unwrap()
    }
}

impl Parsable for ModuleNameUnicodeRecord {
    type Output = ModuleNameUnicodeRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size_of_module_name_unicode = utils::get_u32(cursor)?;
        let module_name_unicode = utils::get_n_bytes(cursor, size_of_module_name_unicode as usize)?;

        Ok(Self {
            id,
            size_of_module_name_unicode,
            module_name_unicode,
        })
    }
}
