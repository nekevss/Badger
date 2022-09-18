use crate::parser::{Parsable};
use crate::nodes::dir_stream::project_modules::records::module::{ModuleNameRecord, ModuleNameUnicodeRecord};
use std::io::Cursor;
use crate::Error;


pub struct NameMapRecord {
    module_name: ModuleNameRecord,
    module_name_unicode: ModuleNameUnicodeRecord,
}

impl NameMapRecord {
    pub fn new() -> Self {
        let module_name = ModuleNameRecord::new();
        let module_name_unicode = ModuleNameUnicodeRecord::new();

        Self {
            module_name,
            module_name_unicode,
        }
    }
}

impl Parsable for NameMapRecord {
    type Output = NameMapRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let module_name = ModuleNameRecord::parse(cursor)?;
        let module_name_unicode = ModuleNameUnicodeRecord::parse(cursor)?;

        Ok(Self {
            module_name,
            module_name_unicode,
        })

    }
}