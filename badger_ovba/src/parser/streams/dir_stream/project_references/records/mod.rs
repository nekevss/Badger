use crate::error::Error;
use crate::parser::utils;
use crate::parser::Parsable;
use std::io::Cursor;

pub mod control;
pub mod name;
pub mod original;
pub mod project;
pub mod registered;

pub use control::ReferenceControl;
pub use name::ReferenceName;
pub use original::ReferenceOriginal;
pub use project::ReferenceProject;
pub use registered::ReferenceRegistered;

#[derive(Debug)]
pub(crate) enum ReferenceRecord {
    Control(ReferenceControl),
    Original(ReferenceOriginal),
    Registered(ReferenceRegistered),
    Project(ReferenceProject),
}

#[derive(Debug)]
pub struct Reference {
    name: Option<ReferenceName>,
    reference_record: ReferenceRecord,
}

impl Parsable for Reference {
    type Output = Reference;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let name = if utils::peek_u16(cursor)? == 0x0016 {
            let name_value = ReferenceName::parse(cursor)?;
            Some(name_value)
        } else {
            None
        };

        let peek_id = utils::peek_u16(cursor)?;

        let reference_record = match peek_id {
            0x002F => {
                let ref_control = ReferenceControl::parse(cursor)?;
                ReferenceRecord::Control(ref_control)
            }
            0x0033 => {
                let ref_original = ReferenceOriginal::parse(cursor)?;
                ReferenceRecord::Original(ref_original)
            }
            0x000D => {
                let ref_registered = ReferenceRegistered::parse(cursor)?;
                ReferenceRecord::Registered(ref_registered)
            }
            0x000E => {
                let ref_project = ReferenceProject::parse(cursor)?;
                ReferenceRecord::Project(ref_project)
            }
            _ => {
                return Err(Error::Parser(
                    "Unexpected ReferenceRecord ID Value".into(),
                    cursor.position(),
                ))
            }
        };

        Ok(Self {
            name,
            reference_record,
        })
    }
}
