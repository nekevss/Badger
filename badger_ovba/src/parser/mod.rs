use crate::error::Error;
use crate::utils::decompress;
use crate::{OvbaModule, OvbaProjectStorage};
use std::io::Cursor;

pub(crate) mod utils;

pub use crate::nodes::{DirStream, ModuleStream, ProjectStream, ProjectLkStream, ProjectWmStream, VbaProjectStream};

pub(crate) trait Parsable {
    type Output;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error>;
}

pub struct Parser {
    inner: OvbaProjectStorage,
}

impl Parser {
    pub fn new(project_storage: OvbaProjectStorage) -> Self {
        Self {
            inner: project_storage,
        }
    }

    pub fn parse_dir_stream(&mut self) -> Result<DirStream, Error> {
        let compressed_buffer = self.inner.dir_stream()?;
        let decompressed_buffer = decompress(&compressed_buffer)?;
        let mut cursor: Cursor<&[u8]> = Cursor::new(&decompressed_buffer);

        let data = DirStream::parse(&mut cursor)?;

        Ok(data)
    }

    pub fn parse_modules(
        &mut self,
        data: &DirStream,
    ) -> Result<Vec<OvbaModule>, Error> {
        let mut module_storage = Vec::new();

        for item in data.project_modules.modules.iter() {
            let stream_buffer = self.inner.module_stream(item.stream_name())?;
            let offset = item.offset();

            let mut module_cursor = Cursor::new(&stream_buffer[..]);

            let module_stream = ModuleStream::parse(&mut module_cursor, offset)?;
            let source_code = decompress(&module_stream.source_code())?;

            module_storage.push(OvbaModule::new(item.stream_name(), source_code))
        }

        Ok(module_storage)
    }
    
    pub fn parse_vba_project_stream(&mut self) -> Result<VbaProjectStream, Error> {
        let stream_buffer = self.inner._vba_project()?;
        let mut cursor = Cursor::new(&stream_buffer[..]);
        let project_stream = VbaProjectStream::parse(&mut cursor)?;

        Ok(project_stream)
    }

    pub fn parse_project_stream(&mut self) -> Result<ProjectStream, Error> {
        let stream_buffer = self.inner.project_stream()?;
        let mut cursor = Cursor::new(&stream_buffer[..]);
        let project_stream = ProjectStream::parse(&mut cursor)?;

        Ok(project_stream)
    }

    pub fn parse_project_lk_stream(&mut self) -> Result<ProjectLkStream, Error> {
        let stream_buffer = self.inner.project_lk_stream()?;
        let mut cursor =  Cursor::new(&stream_buffer[..]);
        let project_lk = ProjectLkStream::parse(&mut cursor)?;

        Ok(project_lk)
    }

    pub fn parse_project_wm_stream(&mut self) -> Result<ProjectWmStream, Error> {
        let stream_buffer = self.inner.project_wm_stream()?;
        let mut cursor = Cursor::new(&stream_buffer[..]);
        let project_wm = ProjectWmStream::parse(&mut cursor)?;

        Ok(project_wm)
    }
}
