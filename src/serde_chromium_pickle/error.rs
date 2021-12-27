use serde::{
    de::{self, Error as _},
    ser,
};
use std::fmt;
use std::io::Error as IOError;
use std::ops::{Deref, DerefMut};

pub type Result<T> = std::result::Result<T, Error>;

pub struct Error(de::value::Error);

impl Deref for Error {
    type Target = de::value::Error;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Error {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.0.fmt(f);
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.0.fmt(f);
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        return Error(ser::Error::custom(msg));
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        return Error(de::Error::custom(msg));
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        return Error(de::value::Error::custom(err));
    }
}
