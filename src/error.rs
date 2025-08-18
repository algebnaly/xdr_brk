use std;
use std::fmt::{self, Display};
use std::str::Utf8Error;

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    EndOfFile,
    SequenceWithoutLength,
    TrailingBytes,
    Utf8Error(String),
    NonZeroPadding,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Error(format!("{}", value))
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::EndOfFile => formatter.write_str("unexpected end of input"),
            Error::SequenceWithoutLength => {
                formatter.write_str("failed to serialise sequence with no length")
            }
            Error::TrailingBytes => formatter.write_str("not all input bytes are comsumed"),
            Error::Utf8Error(msg) => formatter.write_str(msg),
            Error::NonZeroPadding => formatter.write_str("padding data is not zero"),
        }
    }
}

impl std::error::Error for Error {}
