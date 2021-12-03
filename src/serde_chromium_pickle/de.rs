use serde::{self, de};
use super::error::{Error, Result};

pub struct Deserializer;

impl<'de> de::Deserializer<'de> for Deserializer {
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
