use crate::parser::{utils, Parsable};
use crate::utils::convert_mbcs_value;
use std::{cell::Ref, io::Cursor};

use super::{ReferenceName, ReferenceOriginal};

#[derive(Debug, Clone)]
pub struct ReferenceControl {
    reference_original: Option<ReferenceOriginal>,
    id: u16,
    size_of_libid_twiddled: u32,
    libid_twiddled: Vec<u8>,
    reference_name: Option<ReferenceName>,
    size_of_libid_extended: u32,
    libid_extended: Vec<u8>,
    original_type_lib: Vec<u8>,
    cookie: u32,
}

impl ReferenceControl {
    pub fn new() -> Self {
        Self {
            reference_original: None,
            id: 0x002F,
            size_of_libid_twiddled: 0 as u32,
            libid_twiddled: Vec::<u8>::new(),
            reference_name: None,
            size_of_libid_extended: 0 as u32,
            libid_extended: Vec::<u8>::new(),
            original_type_lib: Vec::<u8>::new(),
            cookie: 0 as u32,
        }
    }

    pub fn reference_type(&self) -> &'static str {
        "control"
    }

    // Libid Twiddled is encoded depending on the Code Page Value
    pub fn get_libid_twiddled(&self) -> String {
        let utf8_converted = convert_mbcs_value(&self.libid_twiddled);
        String::from_utf8(utf8_converted).unwrap()
    }

    // Libid Extended is encoded according to the Code Page Value
    pub fn get_libid_extended(&self) -> String {
        let utf8_converted = convert_mbcs_value(&self.libid_extended);
        String::from_utf8(utf8_converted).unwrap()
    }
}

impl Parsable for ReferenceControl {
    type Output = ReferenceControl;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, crate::error::Error> {
        let mut peek_id = utils::peek_u16(cursor)?;

        let reference_original = if peek_id == 0x0033 {
            let ref_original = ReferenceOriginal::parse(cursor)?;
            Some(ref_original)
        } else {
            None
        };

        let id = utils::get_u16(cursor)?;
        let _size_twiddled = utils::get_u32(cursor)?;
        let size_of_libid_twiddled = utils::get_u32(cursor)?;
        let libid_twiddled = utils::get_n_bytes(cursor, size_of_libid_twiddled as usize)?;

        let _reserved1 = utils::get_u32(cursor)?;
        let _reserved2 = utils::get_u16(cursor)?;

        peek_id = utils::peek_u16(cursor)?;

        let reference_name = if peek_id == 0x0016 {
            let ref_name = ReferenceName::parse(cursor)?;
            Some(ref_name)
        } else {
            None
        };

        let _reserved3 = utils::get_u16(cursor)?;
        let _size_extended = utils::get_u32(cursor)?;
        let size_of_libid_extended = utils::get_u32(cursor)?;
        let libid_extended = utils::get_n_bytes(cursor, size_of_libid_extended as usize)?;

        let _reserved4 = utils::get_u32(cursor)?;
        let _reserved5 = utils::get_u16(cursor)?;

        let original_type_lib = utils::get_n_bytes(cursor, 16 as usize)?;
        let cookie = utils::get_u32(cursor)?;

        Ok(Self {
            reference_original,
            id,
            size_of_libid_twiddled,
            libid_twiddled,
            reference_name,
            size_of_libid_extended,
            libid_extended,
            original_type_lib,
            cookie,
        })
    }
}
