use super::super::{
    bits::align_up,
    count::WriteCount,
    error::{Error, Result},
};
use serde::ser;
use std::io::prelude::*;
use std::mem::size_of;

struct ElSerializer<'a>(&'a mut dyn Write);

type Ok = ();

impl<'a> ElSerializer<'a> {
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

impl<'a> ser::Serializer for ElSerializer<'a> {
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
        bool i128 u128 char str bytes none some unit unit_struct
        unit_variant newtype_struct newtype_variant // seq tuple
        // tuple_struct tuple_variant map struct struct_variant
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        return Ok(self.0.write_all(&v.to_ne_bytes())?);
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        return Ok(self.0.write_all(&v.to_ne_bytes())?);
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        return Ok(self.0.write_all(&v.to_ne_bytes())?);
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        return Ok(self.0.write_all(&v.to_ne_bytes())?);
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        return Ok(self.0.write_all(&v.to_ne_bytes())?);
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        return Ok(self.0.write_all(&v.to_ne_bytes())?);
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        return Ok(self.0.write_all(&v.to_ne_bytes())?);
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        return Ok(self.0.write_all(&v.to_ne_bytes())?);
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        return Ok(self.0.write_all(&v.to_ne_bytes())?);
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        return Ok(self.0.write_all(&v.to_ne_bytes())?);
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

pub(crate) struct SeqSerializer<'a>(WriteCount<'a>);

impl<'a> SeqSerializer<'a> {
    pub(crate) fn new(write: &'a mut dyn Write) -> Self {
        SeqSerializer(write.into())
    }
}

impl<'a> ser::SerializeSeq for SeqSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(ElSerializer(&mut self.0))
    }

    fn end(mut self) -> Result<Self::Ok> {
        let written = self.0.count();
        let mut padding = vec![0 as u8; align_up(written, size_of::<u32>()) - written];
        self.0.write_all(&mut padding)?;
        drop(padding);

        return Ok(());
    }
}
