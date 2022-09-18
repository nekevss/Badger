
use crate::parser::{Parsable, utils};
use std::io::Cursor;
use crate::Error;

pub mod records;
pub(crate) use records::LicenseInfo;

pub struct ProjectLkStream {
    version: u16,
    count: u32,
    license_info: Vec<LicenseInfo>,
}

impl ProjectLkStream {
    pub fn new() -> Self {
        Self {
            version: 0x0001,
            count: 0 as u32,
            license_info: Vec::new(),
        }
    }
}

impl Parsable for ProjectLkStream {
    type Output = ProjectLkStream;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let version = utils::get_u16(cursor)?;
        let count = utils::get_u32(cursor)?;
        let license_info = if count != 0 {
            let mut licenses = Vec::new();
            for _item in 0..count as usize {
                let license_info_record = LicenseInfo::parse(cursor)?;
                licenses.push(license_info_record);
            }
            licenses
        } else {
            Vec::new()
        };

        Ok(Self {
            version,
            count,
            license_info,
        })
    }
}

