use super::error::{Error, Result};
use serde::{self, de};
use std::io::prelude::*;

pub struct Deserializer<'a>(&'a mut dyn Read);

impl<'a> Deserializer<'a> {
    pub fn new(reader: &'a mut dyn Read) -> Deserializer<'a> {
        return Deserializer(reader);
    }
}

impl<'a: 'de, 'de> de::Deserializer<'de> for Deserializer<'a> {
    type Error = Error;

    fn is_human_readable(&self) -> bool {
        return false;
    }

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!();
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str
        string bytes byte_buf option unit unit_struct newtype_struct seq
        tuple tuple_struct map struct enum identifier
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        return visitor.visit_unit();
    }
}
