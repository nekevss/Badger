use crate::error::Error;
use crate::parser::{utils, Parsable};
use std::io::Cursor;

pub(crate) mod records;

use records::{ModuleRecord, ProjectCookieRecord};

#[derive(Debug)]
pub struct ProjectModules {
    id: u16,
    size: u32,
    count: u16,
    project_cookie: ProjectCookieRecord,
    pub(crate) modules: Vec<ModuleRecord>,
}

impl Parsable for ProjectModules {
    type Output = ProjectModules;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let id = utils::get_u16(cursor)?;
        let size = utils::get_u32(cursor)?;
        let count = utils::get_u16(cursor)?;
        let project_cookie = ProjectCookieRecord::parse(cursor)?;
        let mut modules = Vec::new();

        for _module_count in 0..count as usize {
            let module_record = ModuleRecord::parse(cursor)?;
            modules.push(module_record);
        }

        Ok(Self {
            id,
            size,
            count,
            project_cookie,
            modules,
        })
    }
}
