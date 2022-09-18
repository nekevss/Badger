
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

use crate::parser::Parser;
use crate::nodes::{DirStream, VbaProjectStream, ProjectLkStream, ProjectWmStream, ProjectStream};
use crate::{OvbaProjectStorage, OvbaModule};
use crate::Error;


pub struct Ovba {
    _vba_project: VbaProjectStream,
    dir: DirStream,
    modules: Vec<OvbaModule>,
    _srp_streams: Vec<u8>,
    _forms: Vec<u8>,
    project_lk: ProjectLkStream,
    project_wm: ProjectWmStream,
    project: ProjectStream,
}

impl Ovba {
    pub fn new() -> Self {
        todo!()
    }

    pub fn from_compound_file(compound_file: CompoundFile<Cursor<Vec<u8>>>) -> Result<Self, Error> {
        let project_storage = OvbaProjectStorage::new(compound_file);
        let mut parser = Parser::new(project_storage);

        let _vba_project = parser.parse_vba_project_stream()?;
        let dir = parser.parse_dir_stream()?;
        let project_lk = parser.parse_project_lk_stream()?;
        let project_wm = parser.parse_project_wm_stream()?;
        let project = parser.parse_project_stream()?;

        let modules = parser.parse_modules(&dir)?;

        Ok(Self{
            _vba_project,
            dir,
            modules,
            _srp_streams: Vec::<u8>::new(),
            _forms: Vec::<u8>::new(),
            project_lk,
            project_wm,
            project,
        })
    }

}