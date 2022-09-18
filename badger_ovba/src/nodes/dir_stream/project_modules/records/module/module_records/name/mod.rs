use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct ModuleNameRecord {
    id: u16,
    size_of_module_name: u32,
    module_name: Vec<u8>,
}

impl ModuleNameRecord {
    pub fn new() -> Self {
        Self {
            id: 0x0019,
            size_of_module_name: 0,
            module_name: Vec::<u8>::new(),
        }
    }
}

impl Parsable for ModuleNameRecord {
    type Output = ModuleNameRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size_of_module_name = utils::get_u32(cursor)?;
        let module_name = utils::get_n_bytes(cursor, size_of_module_name as usize)?;

        Ok(Self {
            id,
            size_of_module_name,
            module_name,
        })
    }
}
