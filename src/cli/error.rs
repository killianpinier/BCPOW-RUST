use std::fmt::Display;


pub enum Error {
    IoError(std::io::Error),
    ParseFloatError(std::num::ParseFloatError),
    UnvalidFlagError(String),
    NoCommandError,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "{}", e),
            Error::ParseFloatError(e) => write!(f, "{}", e),
            Error::UnvalidFlagError(e) => writeln!(f, "Flag not recognized: {}", e),
            Error::NoCommandError => writeln!(f, "")
        }
    }
}