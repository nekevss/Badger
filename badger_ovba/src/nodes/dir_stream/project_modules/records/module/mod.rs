use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

pub(crate) mod module_records;

pub(crate) use module_records::{
    ModuleCookieRecord, ModuleDocStringRecord, ModuleHelpContextRecord, ModuleNameRecord,
    ModuleNameUnicodeRecord, ModuleOffsetRecord, ModulePrivateRecord, ModuleReadOnlyRecord,
    ModuleStreamNameRecord, ModuleTypeRecord,
};

#[derive(Debug, Clone)]
pub struct ModuleRecord {
    name: ModuleNameRecord,
    name_unicode: ModuleNameUnicodeRecord,
    stream_name: ModuleStreamNameRecord,
    doc_string: ModuleDocStringRecord,
    offset: ModuleOffsetRecord,
    help_context: ModuleHelpContextRecord,
    cookie: ModuleCookieRecord,
    type_record: ModuleTypeRecord,
    read_only: Option<ModuleReadOnlyRecord>,
    private: Option<ModulePrivateRecord>,
    terminator: u16,
}

impl ModuleRecord {
    pub fn new() -> Self {
        Self {
            name: ModuleNameRecord::new(),
            name_unicode: ModuleNameUnicodeRecord::new(),
            stream_name: ModuleStreamNameRecord::new(),
            doc_string: ModuleDocStringRecord::new(),
            offset: ModuleOffsetRecord::new(),
            help_context: ModuleHelpContextRecord::new(),
            cookie: ModuleCookieRecord::new(),
            type_record: ModuleTypeRecord::new(),
            read_only: None,
            private: None,
            terminator: 0x002B,
        }
    }

    pub fn stream_name(&self) -> String {
        self.stream_name.value()
    }

    pub fn name_unicode(&self) -> String {
        self.name_unicode.value()
    }

    pub fn offset(&self) -> u32 {
        self.offset.value()
    }
}

impl Parsable for ModuleRecord {
    type Output = ModuleRecord;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let name = ModuleNameRecord::parse(cursor)?;
        let name_unicode = ModuleNameUnicodeRecord::parse(cursor)?;
        let stream_name = ModuleStreamNameRecord::parse(cursor)?;
        let doc_string = ModuleDocStringRecord::parse(cursor)?;
        let offset = ModuleOffsetRecord::parse(cursor)?;
        let help_context = ModuleHelpContextRecord::parse(cursor)?;
        let cookie = ModuleCookieRecord::parse(cursor)?;
        let type_record = ModuleTypeRecord::parse(cursor)?;
        let read_only = if utils::peek_u16(cursor)? == 0x0025 {
            let record_value = ModuleReadOnlyRecord::parse(cursor)?;
            Some(record_value)
        } else {
            None
        };
        let private = if utils::peek_u16(cursor)? == 0x0028 {
            let private_value = ModulePrivateRecord::parse(cursor)?;
            Some(private_value)
        } else {
            None
        };

        let terminator = utils::get_u16(cursor)?;
        let _reserved = utils::get_u32(cursor)?;

        Ok(Self {
            name,
            name_unicode,
            stream_name,
            doc_string,
            offset,
            help_context,
            cookie,
            type_record,
            read_only,
            private,
            terminator,
        })
    }
}
