use std::{error, fmt};

#[derive(Debug)]
pub enum Error<'a> {
    CmdMissing(&'a str)
}

impl error::Error for Error<'_> {}
impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CmdMissing(e) => write!(f, "Not an editor command: {}", e),
        }
    }
}
