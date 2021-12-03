use serde::{self, ser};
use super::error::{Error, Result};

pub struct Serializer;

impl ser::Serializer for Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn is_human_readable(&self) -> bool {
        return false;
    }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok> {
        unimplemented!();
    }

    serde::serde_if_integer128! {
        fn serialize_i128(self, _v: i128) -> Result<Self::Ok> {
            unimplemented!();
        }
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        unimplemented!();
    }

    serde::serde_if_integer128! {
        fn serialize_u128(self, _v: u128) -> Result<Self::Ok> {
            unimplemented!();
        }
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok>
    where
        T: ser::Serialize,
    {
        unimplemented!();
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: ser::Serialize,
    {
        unimplemented!();
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok> {
        unimplemented!();
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!();
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!();
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!();
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!();
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!();
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        unimplemented!();
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!();
    }
}
