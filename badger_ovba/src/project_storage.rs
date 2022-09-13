use std::io::{Cursor, Read};
use cfb::CompoundFile;
use crate::Error;
pub struct OvbaProjectStorage {
    inner: CompoundFile<Cursor<Vec<u8>>>,
}

impl OvbaProjectStorage {
    pub fn new(ovba_compound_file: CompoundFile<Cursor<Vec<u8>>>) -> Self {
        Self {
            inner: ovba_compound_file,
        }
    }

    fn read_stream(&mut self, stream_path: String) -> Result<Vec<u8>, Error> {
        let mut container = Vec::<u8>::new();
        self.inner.open_stream(stream_path)?.read_to_end(&mut container)?;
        Ok(container)
    }

    pub fn _vba_project(&mut self) -> Result<Vec<u8>, Error> {
        let path: &'static str = "/VBA/_VBA_PROJECT";
        self.read_stream(path.to_owned())
    }

    pub fn dir_stream(&mut self) -> Result<Vec<u8>, Error> {
        let path: &'static str = "/VBA/dir";
        self.read_stream(path.to_owned())
    }

    pub fn module_stream(&mut self, stream_name: String) -> Result<Vec<u8>, Error> {
        let path = format!("/VBA/{}", stream_name);
        self.read_stream(path)
    }
}