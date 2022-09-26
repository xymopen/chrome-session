use super::super::error::{Error, Result};
use super::super::Serializer;
use serde::ser;
use std::io::prelude::*;

pub struct SeqSerializer<'a>(pub(crate) &'a mut dyn Write);

impl<'a> ser::SerializeSeq for SeqSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(Serializer(self.0))
    }

    fn end(self) -> Result<Self::Ok> {
        return Ok(());
    }
}
