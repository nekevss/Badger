//! Implementation of section 2.3.4.2.1 PROJECTINFORMATIONRECORD
//!

use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

pub(crate) mod records;
use records::{
    CodePageRecord, CompatVersionRecord, ConstantsRecord, DocStringRecord, HelpContextRecord,
    HelpFilePathRecord, LcidInvokeRecord, LcidRecord, LibFlagsRecord, NameRecord, SysKindRecord,
    VersionRecord,
};

#[derive(Debug, Clone)]
pub struct ProjectInformation {
    sys_kind: SysKindRecord,
    compat_version: Option<CompatVersionRecord>,
    lcid: LcidRecord,
    lcid_invoke: LcidInvokeRecord,
    code_page: CodePageRecord,
    name: NameRecord,
    doc_string: DocStringRecord,
    help_file_path: HelpFilePathRecord,
    help_context: HelpContextRecord,
    lib_flags: LibFlagsRecord,
    version: VersionRecord,
    constants: Option<ConstantsRecord>,
}

impl ProjectInformation {
    pub fn new() -> Self {
        let sys_kind = SysKindRecord::new();
        let lcid = LcidRecord::new();
        let lcid_invoke = LcidInvokeRecord::new();
        let code_page = CodePageRecord::new();
        let name = NameRecord::new();
        let doc_string = DocStringRecord::new();
        let help_file_path = HelpFilePathRecord::new();
        let help_context = HelpContextRecord::new();
        let lib_flags = LibFlagsRecord::new();
        let version = VersionRecord::new();
        let constants = Some(ConstantsRecord::new());

        Self {
            sys_kind,
            compat_version: None,
            lcid,
            lcid_invoke,
            code_page,
            name,
            doc_string,
            help_file_path,
            help_context,
            lib_flags,
            version,
            constants,
        }
    }

    pub fn sys_kind(&self) -> String {
        self.sys_kind.value()
    }

    pub fn compat_version(&self) -> u32 {
        match &self.compat_version {
            Some(record) => record.value(),
            None => 0 as u32,
        }
    }

    pub fn lcid(&self) -> u32 {
        self.lcid.value()
    }

    pub fn lcid_invoke(&self) -> u32 {
        self.lcid_invoke.value()
    }

    pub fn code_page(&self) -> u16 {
        self.code_page.value()
    }

    pub fn name(&self) -> String {
        self.name.value()
    }

    pub fn doc_string(&self) -> String {
        self.doc_string.value()
    }

    pub fn help_file1_path(&self) -> String {
        self.help_file_path.value1()
    }

    pub fn help_file2_path(&self) -> String {
        self.help_file_path.value2()
    }

    pub fn help_context(&self) -> u32 {
        self.help_context.value()
    }

    pub fn major_version(&self) -> u32 {
        self.version.major_version()
    }

    pub fn minor_version(&self) -> u16 {
        self.version.minor_version()
    }

    pub fn constants(&self) -> String {
        match &self.constants {
            Some(record) => record.value(),
            None => "".into(),
        }
    }
}

// trait may be the way to go here
impl Parsable for ProjectInformation {
    type Output = ProjectInformation;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let sys_kind = SysKindRecord::parse(cursor)?;
        let compat_version = if utils::peek_u16(cursor)? == 0x004A {
            let compat_value = CompatVersionRecord::parse(cursor)?;
            Some(compat_value)
        } else {
            None
        };
        let lcid = LcidRecord::parse(cursor)?;
        let lcid_invoke = LcidInvokeRecord::parse(cursor)?;
        let code_page = CodePageRecord::parse(cursor)?;
        let name = NameRecord::parse(cursor)?;
        let doc_string = DocStringRecord::parse(cursor)?;
        let help_file_path = HelpFilePathRecord::parse(cursor)?;
        let help_context = HelpContextRecord::parse(cursor)?;
        let lib_flags = LibFlagsRecord::parse(cursor)?;
        let version = VersionRecord::parse(cursor)?;
        let constants = if utils::peek_u16(cursor)? == 0x000C {
            let constants_record = ConstantsRecord::parse(cursor)?;
            Some(constants_record)
        } else {
            None
        };

        //ConstantsRecord::parse(cursor)?;

        Ok(ProjectInformation {
            sys_kind,
            compat_version,
            lcid,
            lcid_invoke,
            code_page,
            name,
            doc_string,
            help_file_path,
            help_context,
            lib_flags,
            version,
            constants,
        })
    }
}
