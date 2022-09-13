
// VBA Project File Structure 2.2
//
// Project Root Storage
// -- /VBA
// ---- /_VBA_Project (Dependent Project Data)
// ---- /dir (Independent Project Data)
// ---- /<Module Streams>
// ---- /<SRP Streams> (designated _SRP_<n> where n is the number of SRP streams)
// -- /<VBA Form Storages>
// -- /PROJECTlk
// -- /PROJECTwm
// -- /PROJECT.
use cfb::CompoundFile;
use std::io::Cursor;

use crate::parser::{Parser, DirStream};
use crate::{OvbaProjectStorage, OvbaModule};
use crate::Error;


pub struct Ovba {
    _vba_project: u8,
    dir: DirStream,
    modules: Vec<OvbaModule>,
    _srp_streams: Vec<u8>,
    _forms: Vec<u8>,
    project_lk: u8,
    project_wm: u8,
    project: u8,
}

impl Ovba {
    pub fn new() -> Self {
        todo!()
    }

    pub fn from_compound_file(compound_file: CompoundFile<Cursor<Vec<u8>>>) -> Result<Self, Error> {
        let project_storage = OvbaProjectStorage::new(compound_file);
        let mut parser = Parser::new(project_storage);

        let dir = parser.parse_dir_stream()?;
        let modules = parser.parse_modules(&dir)?;
        

        Ok(Self{
            _vba_project: 0,
            dir,
            modules,
            _srp_streams: Vec::<u8>::new(),
            _forms: Vec::<u8>::new(),
            project_lk: 0,
            project_wm: 0,
            project: 0,
        })
    }

}