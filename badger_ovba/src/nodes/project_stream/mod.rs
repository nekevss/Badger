use std::io::Cursor;
use crate::Error;
use crate::parser::{Parsable, utils};

pub struct ProjectStream {
    inner: Vec<u8>,
}


impl ProjectStream {
    pub fn new() -> Self {
        Self {
            inner:Vec::<u8>::new()
        }
    }
}

impl Parsable for ProjectStream {
    type Output = ProjectStream;

    fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self::Output, Error> {
        let vector_length = cursor.get_ref().len();
        let inner = utils::get_n_bytes(cursor, vector_length)?;

        Ok(Self {
            inner,
        })
    }
}
