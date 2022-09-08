use std::{io::Cursor, convert::TryInto};
use crate::error::Error;

pub fn peek_u16(cursor: &mut Cursor<&[u8]>) -> u16 {
    let cursor_pos = cursor.position() as usize;
    u16::from_le_bytes(
        (&cursor.get_ref()[cursor_pos..cursor_pos + 2])
            .try_into()
            .unwrap(),
    )
}

/// Returns a u16 value from the cursor and increments cursor by 2 bytes
pub fn get_u16(cursor: &mut Cursor<&[u8]>) -> u16 {
    let cursor_pos = cursor.position() as usize;
    let value = u16::from_le_bytes(
        (&cursor.get_ref()[cursor_pos..cursor_pos + 2])
            .try_into()
            .unwrap(),
    );
    cursor.set_position(cursor_pos as u64 + 2);
    value
}

/// Returns a u32 value from the cursor and increments cursor by 4 bytes
pub fn get_u32(cursor: &mut Cursor<&[u8]>) -> u32 {
    let cursor_pos = cursor.position() as usize;
    let value = u32::from_le_bytes(
        (&cursor.get_ref()[cursor_pos..cursor_pos + 4])
            .try_into()
            .unwrap(),
    );
    cursor.set_position(cursor_pos as u64 + 4);
    value
}

/// Returns a Vec<u8> of n bytes and increments cursor by n
pub fn get_n_bytes(cursor: &mut Cursor<&[u8]>, n: usize) -> Vec<u8> {
    let cursor_pos = cursor.position() as usize;
    let value = cursor.get_ref()[cursor_pos..cursor_pos + n].to_vec();
    let new_cursor_pos = (cursor_pos + n) as u64;
    cursor.set_position(new_cursor_pos);
    value
}

pub fn convert_le_u16(vector: &[u8]) -> Result<Vec<u16>, Error> {
    if vector.len() % 2 == 0 {
        let mut vec_u16:Vec<u16> = Vec::new();
        for index in (0..vector.len()).step_by(2) {
            vec_u16.push(u16::from_le_bytes(vector[index..index+2].try_into().unwrap()));
        }
        Ok(vec_u16)
    } else {
        Err(Error::Decompresssion("&[u8] could not be converted to &[u16]".into()))
    }
}