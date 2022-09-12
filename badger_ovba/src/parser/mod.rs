use crate::error::Error;
use crate::utils::decompress;
use crate::OvbaModule;
use cfb::CompoundFile;
use std::io::{Cursor, Read};

pub mod streams;
pub(crate) mod utils;

pub use crate::parser::streams::{IndependentVbaProject, ModuleStream};

pub(crate) trait Parsable {
    type Output;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error>;
}

pub struct Ovba<'a> {
    inner: CompoundFile<Cursor<Vec<u8>>>,
    vba_storage: &'a str,
    dir_stream: &'a str,
}

impl<'a> Ovba<'a> {
    pub fn new(compound_file: CompoundFile<Cursor<Vec<u8>>>) -> Self {
        let vba_storage = "/VBA";
        let dir_stream: &str = "/dir";

        Self {
            inner: compound_file,
            vba_storage,
            dir_stream,
        }
    }

    pub fn parse_independent_info(&mut self) -> Result<IndependentVbaProject, Error> {
        let mut compressed_buffer: Vec<u8> = Vec::new();
        let stream_name = format!("{}{}", self.vba_storage, self.dir_stream);
        self.inner
            .open_stream(stream_name)?
            .read_to_end(&mut compressed_buffer)?;
        let decompressed_buffer = decompress(&compressed_buffer)?;
        let mut cursor: Cursor<&[u8]> = Cursor::new(&decompressed_buffer);

        let data = IndependentVbaProject::parse(&mut cursor)?;

        Ok(data)
    }

    pub fn parse_modules(
        &mut self,
        data: &IndependentVbaProject,
    ) -> Result<Vec<OvbaModule>, Error> {
        let mut module_storage = Vec::new();

        for item in data.project_modules.modules.iter() {
            let mut stream_buffer: Vec<u8> = Vec::new();
            let module_stream = format!("{}/{}", self.vba_storage, item.stream_name());
            let offset = item.offset();
            let module_name = item.name_unicode();

            self.inner
                .open_stream(module_stream)?
                .read_to_end(&mut stream_buffer)?;
            let mut module_cursor: Cursor<&[u8]> = Cursor::new(&stream_buffer);

            let module_stream = ModuleStream::parse(&mut module_cursor, offset)?;
            let source_code = decompress(&module_stream.source_code())?;

            module_storage.push(OvbaModule::new(module_name, source_code))
        }

        Ok(module_storage)
    }
}
