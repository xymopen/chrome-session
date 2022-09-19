use super::bits::align_up;
use super::error::{Error, Result};
use super::seq::SeqDeserializer;
use serde::{self, de};
use std::io::prelude::*;
use std::io::Result as IOResult;
use std::mem::size_of;
use std::os::raw::c_int;

fn read_aligned<'a>(reader: &'a mut dyn Read, buf: &mut [u8]) -> IOResult<()> {
    let aligned_size = align_up(buf.len(), size_of::<u32>());
    let mut padding = vec![0 as u8; aligned_size - buf.len()];
    reader.read_exact(buf)?;
    reader.read_exact(&mut padding)?;
    return Ok(());
}

fn read_int<'a>(reader: &'a mut dyn Read) -> IOResult<c_int> {
    let mut bytes: [u8; size_of::<c_int>()] = [0; size_of::<c_int>()];
    read_aligned(reader, &mut bytes)?;
    return Ok(c_int::from_ne_bytes(bytes));
}

pub struct Deserializer<'a>(pub &'a mut dyn Read);

impl<'a, 'de> de::Deserializer<'de> for Deserializer<'a> {
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
        i128 u128 char str string bytes byte_buf option unit unit_struct
        newtype_struct map enum identifier
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let v = read_int(self.0)?;
        return visitor.visit_bool(v != 0);
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<i8>()] = [0; size_of::<i8>()];
        read_aligned(self.0, &mut bytes)?;
        return visitor.visit_i8(i8::from_ne_bytes(bytes));
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<i16>()] = [0; size_of::<i16>()];
        read_aligned(self.0, &mut bytes)?;
        return visitor.visit_i16(i16::from_ne_bytes(bytes));
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<i32>()] = [0; size_of::<i32>()];
        read_aligned(self.0, &mut bytes)?;
        return visitor.visit_i32(i32::from_ne_bytes(bytes));
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<i64>()] = [0; size_of::<i64>()];
        read_aligned(self.0, &mut bytes)?;
        return visitor.visit_i64(i64::from_ne_bytes(bytes));
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<u8>()] = [0; size_of::<u8>()];
        read_aligned(self.0, &mut bytes)?;
        return visitor.visit_u8(u8::from_ne_bytes(bytes));
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<u16>()] = [0; size_of::<u16>()];
        read_aligned(self.0, &mut bytes)?;
        return visitor.visit_u16(u16::from_ne_bytes(bytes));
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<u32>()] = [0; size_of::<u32>()];
        read_aligned(self.0, &mut bytes)?;
        return visitor.visit_u32(u32::from_ne_bytes(bytes));
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<u64>()] = [0; size_of::<u64>()];
        read_aligned(self.0, &mut bytes)?;
        return visitor.visit_u64(u64::from_ne_bytes(bytes));
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<f32>()] = [0; size_of::<f32>()];
        read_aligned(self.0, &mut bytes)?;
        return visitor.visit_f32(f32::from_ne_bytes(bytes));
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut bytes: [u8; size_of::<f64>()] = [0; size_of::<f64>()];
        read_aligned(self.0, &mut bytes)?;
        return visitor.visit_f64(f64::from_ne_bytes(bytes));
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let len = read_int(self.0)? as usize;
        return visitor.visit_seq(SeqDeserializer(len, self.0));
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        return visitor.visit_seq(SeqAccess(len, self.0));
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'de str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        return visitor.visit_seq(SeqAccess(len, self.0));
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        return visitor.visit_seq(SeqAccess(fields.len(), self.0));
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        return visitor.visit_unit();
    }
}

struct SeqAccess<'a>(usize, &'a mut dyn Read);

impl<'a, 'de> de::SeqAccess<'de> for SeqAccess<'a> {
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
