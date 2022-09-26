use super::super::error::{Error, Result};
use super::super::Deserializer;
use serde::de;
use std::io::prelude::*;

pub(crate) struct SeqDeserializer<'a>(pub(crate) usize, pub(crate) &'a mut dyn Read);

impl<'a, 'de> de::SeqAccess<'de> for SeqDeserializer<'a> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.0 == 0 {
            return Ok(None);
        } else {
            self.0 -= 1;

            return seed.deserialize(Deserializer(self.1)).map(Into::into);
        }
    }

    fn size_hint(&self) -> Option<usize> {
        return Some(self.0);
    }
}
