use serde::de;

pub type Result<T> = std::result::Result<T, Error>;

pub use de::value::Error;
