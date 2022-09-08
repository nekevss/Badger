
use std::io::Cursor;
use crate::error::Error;
use crate::parser::{Parsable, utils};


pub struct ModuleStream {
    compressed_source_code: Vec<u8>,
}

impl ModuleStream {
    pub fn source_code(&self) -> Vec<u8> {
        self.compressed_source_code.clone()
    }
}

impl ModuleStream {
    // Using a custom parse here since we rely on passing offset into the ModuleStream
    pub fn parse(cursor: &mut Cursor<&[u8]>, offset: u32) -> ModuleStream {
       let _performance_cache = utils::get_n_bytes(cursor, offset as usize);
       let stream_len = cursor.get_ref().len();
       let compressed_source_code = utils::get_n_bytes(cursor, stream_len-offset as usize);
       
       Self {
        compressed_source_code,
       }
    }
}