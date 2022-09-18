use crate::parser::{Parsable, utils};
use std::io::Cursor;
use crate::Error;

pub struct LicenseInfo {
    class_id: Vec<u8>,
    size_of_license_key: u32,
    license_key: Vec<u8>,
    license_required: u32
}

impl LicenseInfo {
    pub fn new() -> Self {
        Self {
            class_id: Vec::<u8>::new(),
            size_of_license_key: 0 as u32,
            license_key: Vec::<u8>::new(),
            license_required: 0x00000000,
        }
    }
}

impl Parsable for LicenseInfo {
    type Output = LicenseInfo;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let class_id = utils::get_n_bytes(cursor, 16 as usize)?;
        let size_of_license_key = utils::get_u32(cursor)?;
        let license_key = utils::get_n_bytes(cursor, size_of_license_key as usize)?;
        let license_required = utils::get_u32(cursor)?;

        Ok(Self {
            class_id,
            size_of_license_key,
            license_key,
            license_required,
        })
    }
}