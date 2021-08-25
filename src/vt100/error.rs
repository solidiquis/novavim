use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    CurPosErr,
    RawModeRequired
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CurPosErr => write!(f, "Failed to query cursor position."),
            Error::RawModeRequired => write!(f, "Canonical mode and echo must be disabled to query cursor position.")
        }
    }
}
