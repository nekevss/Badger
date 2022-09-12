//! 2.3.4.2 dir Stream
//!
//!
//! NOTES:
//!
//! - This stream must be compressed as specified according to Compression (2.4.1)
//! - This is a version independent description
//!
use crate::error::Error;
use crate::parser::Parsable;
use std::io::Cursor;

pub(crate) mod project_information;
pub(crate) mod project_modules;
pub(crate) mod project_references;

use project_information::ProjectInformation;
use project_modules::ProjectModules;
use project_references::ProjectReferences;

#[derive(Debug, Clone)]
pub struct IndependentVbaProject {
    project_information: ProjectInformation,
    project_references: ProjectReferences,
    pub(crate) project_modules: ProjectModules,
}

impl IndependentVbaProject {
    pub fn new() -> Self {
        Self {
            project_information: ProjectInformation::new(),
            project_references: ProjectReferences::new(),
            project_modules: ProjectModules::new(),
        }
    }

    pub fn project_information(&self) -> ProjectInformation {
        self.project_information.clone()
    }

    pub fn project_references(&self) -> ProjectReferences {
        self.project_references.clone()
    }

    pub fn project_modules(&self) -> ProjectModules {
        self.project_modules.clone()
    }
}

// Note: Experimental layout.
impl Parsable for IndependentVbaProject {
    type Output = IndependentVbaProject;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let project_information = ProjectInformation::parse(cursor)?;
        let project_references = ProjectReferences::parse(cursor)?;
        let project_modules = ProjectModules::parse(cursor)?;
        // let terminator = parse_terminator
        // let reserved = parse_reserved

        Ok(IndependentVbaProject {
            project_information,
            project_references,
            project_modules,
        })
    }

    // fn parse_terminator() -> Result<u16> {}

    // fn parse_reserved() -> Result<u32> {}
}
