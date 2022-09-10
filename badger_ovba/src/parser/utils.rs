use crate::error::Error;
use std::{convert::TryInto, io::Cursor};

pub fn peek_u16(cursor: &mut Cursor<&[u8]>) -> Result<u16, Error> {
    let cursor_pos = cursor.position() as usize;
    let internal_ref = &cursor.get_ref();
    if cursor_pos < internal_ref.len() - 1 {
        let peek_val = u16::from_le_bytes(
            internal_ref[cursor_pos..cursor_pos + 2]
                .try_into()
                .expect("Try into should not fail unless stream is corrupted"),
        );
        Ok(peek_val)
    } else {
        Err(Error::Parser(
            "Parser Error: Index to peek exceeds the length of the slice".into(),
            cursor_pos as u64,
        ))
    }
}

/// Returns a u16 value from the cursor and increments cursor by 2 bytes
pub fn get_u16(cursor: &mut Cursor<&[u8]>) -> Result<u16, Error> {
    let cursor_pos = cursor.position() as usize;
    let internal_ref = &cursor.get_ref();
    if cursor_pos < internal_ref.len() - 1 {
        let value = u16::from_le_bytes(
            (internal_ref[cursor_pos..cursor_pos + 2])
                .try_into()
                .expect("Try_into() for get_u16 should not fail"),
        );
        cursor.set_position(cursor_pos as u64 + 2);
        Ok(value)
    } else {
        Err(Error::Parser(
            "Parser Error: Index get_u16 exceeds the length of the slice".into(),
            cursor_pos as u64,
        ))
    }
}

/// Returns a u32 value from the cursor and increments cursor by 4 bytes
pub fn get_u32(cursor: &mut Cursor<&[u8]>) -> Result<u32, Error> {
    let cursor_pos = cursor.position() as usize;
    let internal_ref = &cursor.get_ref();
    if cursor_pos < internal_ref.len() - 3 {
        let value = u32::from_le_bytes(
            (&cursor.get_ref()[cursor_pos..cursor_pos + 4])
                .try_into()
                .expect("Try_into() for get_u32 should not fail."),
        );
        cursor.set_position(cursor_pos as u64 + 4);
        Ok(value)
    } else {
        Err(Error::Parser(
            "Parser Error: Index for get_u32 exceeds the length of the slice".into(),
            cursor_pos as u64,
        ))
    }
}

/// Returns a Vec<u8> of n bytes and increments cursor by n
pub fn get_n_bytes(cursor: &mut Cursor<&[u8]>, n: usize) -> Result<Vec<u8>, Error> {
    let cursor_pos = cursor.position() as usize;
    let internal_ref = cursor.get_ref();
    if cursor_pos < internal_ref.len() + n - 1 {
        let value = cursor.get_ref()[cursor_pos..cursor_pos + n].to_vec();
        let new_cursor_pos = (cursor_pos + n) as u64;
        cursor.set_position(new_cursor_pos);
        Ok(value)
    } else {
        Err(Error::Parser(
            "Parser Error: Index for get_n_bytes() exceeds the lenght of the slice".into(),
            cursor_pos as u64,
        ))
    }
}

pub fn convert_le_u16(vector: &[u8]) -> Result<Vec<u16>, Error> {
    if vector.len() % 2 == 0 {
        let mut vec_u16: Vec<u16> = Vec::new();
        for index in (0..vector.len()).step_by(2) {
            let new_u16 = ((vector[index + 1] as u16) << 8) | vector[index] as u16;
            vec_u16.push(new_u16);
        }
        Ok(vec_u16)
    } else {
        Err(Error::Decompresssion(
            "&[u8] could not be converted to &[u16]".into(),
        ))
    }
}
