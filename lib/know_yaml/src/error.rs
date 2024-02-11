// This is free and unencumbered software released into the public domain.

use std::fmt::{self, Display};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    Eof,
    Message(String),
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::Message(err.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Eof => formatter.write_str("unexpected end of input"),
            Error::Message(msg) => formatter.write_str(msg),
        }
    }
}
