use std::{
    error,
    fmt::{self, Debug, Display},
    io, num,
};

#[derive(Debug)]
pub enum Error {
    IoErr(io::Error),
    TryFromIntError(num::TryFromIntError),
    ParseIntError(num::ParseIntError),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IoErr(e) => Some(e),
            Error::TryFromIntError(e) => Some(e),
            Error::ParseIntError(e) => Some(e),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoErr(e) => Display::fmt(&*e, f),
            Error::TryFromIntError(e) => Display::fmt(&*e, f),
            Error::ParseIntError(e) => Display::fmt(&*e, f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::IoErr(other)
    }
}

impl From<num::TryFromIntError> for Error {
    fn from(other: num::TryFromIntError) -> Self {
        Error::TryFromIntError(other)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(other: num::ParseIntError) -> Self {
        Error::ParseIntError(other)
    }
}
