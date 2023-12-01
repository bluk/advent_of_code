use std::{
    error,
    fmt::{self, Debug, Display},
    io,
};

#[derive(Debug)]
pub enum Error {
    IoErr(io::Error),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoErr(e) => Display::fmt(e, f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::IoErr(other)
    }
}
