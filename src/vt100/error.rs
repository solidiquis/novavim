use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    CurPosErr,
    NonCanonicalModeRequired
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CurPosErr => write!(f, "Failed to query cursor position."),
            Error::NonCanonicalModeRequired => write!(f, "Noncanonical mode must be enabled to query cursor position.")
        }
    }
}
