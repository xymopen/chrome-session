mod bits;
mod de;
mod error;
mod seq;
mod ser;
#[cfg(test)]
mod test;
#[macro_use]
mod macros;

pub use de::from_reader;
pub(crate) use de::Deserializer;
pub use error::{Error, Result};
pub use ser::into_writer;
pub(crate) use ser::Serializer;
