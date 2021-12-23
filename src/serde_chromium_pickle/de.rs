use super::error::{Error, Result};
use serde::{self, de};
use std::io::prelude::*;
use std::io::Result as IOResult;
use std::mem::size_of;
use std::os::raw::c_int;

fn read_int<'a>(reader: &'a mut dyn Read) -> IOResult<c_int> {
    let mut bytes: [u8; size_of::<c_int>()] = [0; size_of::<c_int>()];
    reader.read_exact(&mut bytes)?;
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
        newtype_struct struct seq tuple tuple_struct map enum identifier
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
