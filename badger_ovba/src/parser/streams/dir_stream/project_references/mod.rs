//! 2.3.4.2.2
use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

pub mod records;

use records::Reference;

#[derive(Debug, Clone)]
pub struct ProjectReferences {
    reference_array: Vec<Reference>,
}

impl ProjectReferences {
    pub fn new() -> Self {
        Self {
            reference_array: Vec::new(),
        }
    }
}

impl Parsable for ProjectReferences {
    type Output = ProjectReferences;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let mut reference_array = Vec::new();

        let mut peek_id = utils::peek_u16(cursor)?;
        // The ProjectReferences Record is terminated by a 2 byte id == 0x000F, which
        // signals the beginning of the ProjectModules
        while peek_id != 0x000F {
            let reference = Reference::parse(cursor)?;
            reference_array.push(reference);

            peek_id = utils::peek_u16(cursor)?;
        }

        Ok(Self { reference_array })
    }
}
