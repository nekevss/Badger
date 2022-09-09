//! Implement a stripped parser for the [MS-OVBA] spec
//!
//! Goal: clarity - std libraries only
//!

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
use std::io::{Cursor, Read};
use std::fs::File;
use cfb::CompoundFile;

pub mod error;
pub mod parser;
pub mod utils;
pub mod ovba_module;

pub use crate::ovba_module::OvbaModule;

use parser::{IndependentVbaProject as DirStream};
use error::Error;

use crate::parser::Ovba;


pub struct BadgerOvba {
    independent_info: DirStream,
    modules: Vec<OvbaModule>,
}

impl BadgerOvba {
    /// Creates a [`BadgerOvba`] from a file buffer.
    /// 
    /// Note: this file uses an external dependency 
    pub fn from_file(mut file: File) -> Result<Self, Error> {
        let mut buffer:Vec<u8>= Vec::new();
        file.read_to_end(&mut buffer)?;

        let cursor = Cursor::new(buffer);

        let compound_file = CompoundFile::open(cursor)?;

        let mut ovba = Ovba::new(compound_file);

        let independent_info = ovba.parse_independent_info()?;

        let modules = ovba.parse_modules(&independent_info)?;

        Ok(Self{
            independent_info,
            modules,
        })
    }

    pub fn from_compound_file(compound_file: CompoundFile<Cursor<Vec<u8>>>) -> Result<Self, Error> {
        let mut ovba = Ovba::new(compound_file);
        
        let independent_info = ovba.parse_independent_info()?;
        let modules = ovba.parse_modules(&independent_info)?;

        Ok(Self{
            independent_info,
            modules,
        })
    }

    pub fn display_module(&self) {
        for module in self.modules.iter() {
            module.print_module()
        }
    }
}
