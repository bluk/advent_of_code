use std::{
    error,
    fmt::{self, Debug, Display},
    io, num,
};

#[derive(Debug)]
pub enum Error {
    IoErr(io::Error),
    ParseNumError(num::ParseIntError),
    TryFromIntError(num::TryFromIntError),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoErr(e) => Display::fmt(e, f),
            Error::ParseNumError(e) => Display::fmt(e, f),
            Error::TryFromIntError(e) => Display::fmt(e, f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::IoErr(other)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(other: num::ParseIntError) -> Self {
        Error::ParseNumError(other)
    }
}

impl From<num::TryFromIntError> for Error {
    fn from(other: num::TryFromIntError) -> Self {
        Error::TryFromIntError(other)
    }
}
