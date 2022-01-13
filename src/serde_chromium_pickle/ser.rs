use serde::{self, ser};
use super::error::{Error, Result};

pub struct Serializer;

type Ok = ();

impl Serializer {
    fn serialize_any_unit(self) -> Result<Ok> {
        unimplemented!();
    }
    fn serialize_any<T: Sized>(self, _v: T) -> Result<Ok> {
        unimplemented!();
    }
    fn serialize_any_ref<T: ?Sized>(self, _v: &T) -> Result<Ok> {
        unimplemented!();
    }
}

impl ser::Serializer for Serializer {
    type Ok = Ok;
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

    crate::forward_to_serialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str bytes none
        some unit unit_struct unit_variant newtype_struct newtype_variant
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
