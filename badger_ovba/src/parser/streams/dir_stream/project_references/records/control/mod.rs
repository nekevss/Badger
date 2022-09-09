use crate::parser::{utils, Parsable};
use std::{cell::Ref, io::Cursor};

use super::{ReferenceName, ReferenceOriginal};

#[derive(Debug)]
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
