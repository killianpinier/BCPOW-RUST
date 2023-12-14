use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseFloatError(std::num::ParseFloatError),
    ParseUsizeError(std::num::ParseIntError),
    NotEnoughArgsError(String),
    Base58Error(bs58::decode::Error),
    UnvalidAddressError(String),
    UnvalidCheckSumError,

    UnrecognizedFlag(String),
    UnvalidArgsCountError(String, usize)
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(e)                        => write!(f, "{}", e),
            Error::ParseFloatError(e)      => write!(f, "Amount entered is not well formated: {}\nUse -h for more information", e),
            Error::ParseUsizeError(e)        => write!(f, "Parse error: {}", e),
            Error::NotEnoughArgsError(e)            => write!(f, "Not enough arguments: {}\nUse -h for more information", e),
            Error::Base58Error(e)                    => write!(f, "Cannot convert address to public key hash: {}", e),
            Error::UnvalidAddressError(e)           => write!(f, "Unvalid address: {}", e),
            Error::UnvalidCheckSumError                      => write!(f, "Unvalid checksum"),
            Error::UnrecognizedFlag(flag)           => write!(f, "Unrecognized flag: {}", flag) ,
            Error::UnvalidArgsCountError(command, count, )          => write!(f, "{} only takes {} arguments", command, count)
        }
    }
}