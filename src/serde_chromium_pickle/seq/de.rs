use super::super::{
    bits::align_up,
    count::ReadCount,
    error::{Error, Result},
};
use serde::de;
use std::io::prelude::*;
use std::mem::size_of;

struct ElDeserializer<'a>(&'a mut dyn Read);

impl<'a, 'de> de::Deserializer<'de> for ElDeserializer<'a> {
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
        bool i128 u128 char str string bytes byte_buf option unit
        unit_struct newtype_struct struct seq tuple tuple_struct map
        enum identifier
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<i8>()] = [0; size_of::<i8>()];
        self.0.read_exact(&mut bytes)?;
        return visitor.visit_i8(i8::from_ne_bytes(bytes));
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<i16>()] = [0; size_of::<i16>()];
        self.0.read_exact(&mut bytes)?;
        return visitor.visit_i16(i16::from_ne_bytes(bytes));
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<i32>()] = [0; size_of::<i32>()];
        self.0.read_exact(&mut bytes)?;
        return visitor.visit_i32(i32::from_ne_bytes(bytes));
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<i64>()] = [0; size_of::<i64>()];
        self.0.read_exact(&mut bytes)?;
        return visitor.visit_i64(i64::from_ne_bytes(bytes));
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<u8>()] = [0; size_of::<u8>()];
        self.0.read_exact(&mut bytes)?;
        return visitor.visit_u8(u8::from_ne_bytes(bytes));
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<u16>()] = [0; size_of::<u16>()];
        self.0.read_exact(&mut bytes)?;
        return visitor.visit_u16(u16::from_ne_bytes(bytes));
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<u32>()] = [0; size_of::<u32>()];
        self.0.read_exact(&mut bytes)?;
        return visitor.visit_u32(u32::from_ne_bytes(bytes));
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<u64>()] = [0; size_of::<u64>()];
        self.0.read_exact(&mut bytes)?;
        return visitor.visit_u64(u64::from_ne_bytes(bytes));
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<f32>()] = [0; size_of::<f32>()];
        self.0.read_exact(&mut bytes)?;
        return visitor.visit_f32(f32::from_ne_bytes(bytes));
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<f64>()] = [0; size_of::<f64>()];
        self.0.read_exact(&mut bytes)?;
        return visitor.visit_f64(f64::from_ne_bytes(bytes));
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        return visitor.visit_unit();
    }
}

pub(crate) struct SeqDeserializer<'a>(usize, ReadCount<'a>);

impl<'a> SeqDeserializer<'a> {
    pub(crate) fn new(len: usize, read: &'a mut dyn Read) -> Self {
        SeqDeserializer(len, read.into())
    }
}

impl<'a, 'de> de::SeqAccess<'de> for SeqDeserializer<'a> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.0 == 0 {
            let read = self.1.count();
            let mut padding = vec![0 as u8; align_up(read, size_of::<u32>()) - read];
            self.1.read_exact(&mut padding)?;
            drop(padding);

            return Ok(None);
        } else {
            self.0 -= 1;

            return seed
                .deserialize(ElDeserializer(&mut self.1))
                .map(Into::into);
        }
    }

    fn size_hint(&self) -> Option<usize> {
        return Some(self.0);
    }
}
