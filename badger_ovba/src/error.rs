use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Parser(Box<str>, u64),
    Decompresssion(Box<str>),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl Error {
    pub(crate) fn parser<M, P>(err: M, position: P) -> Self
    where
        M: Into<Box<str>>,
        P: Into<u64>,
    {
        Self::Parser(err.into(), position.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(err) => write!(f, "I/O error: {}", err),
            Self::Parser(err, position) => {
                write!(f, "Parser Error: {} at position: {}", err, position)
            }
            Self::Decompresssion(err) => write!(f, "Decompression Error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::IO(err) => Some(err),
            Self::Parser(_, _) => None,
            Self::Decompresssion(_) => None,
        }
    }
}
