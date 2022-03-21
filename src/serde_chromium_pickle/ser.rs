use super::error::{Error, Result};
use super::seq::SeqSerializer;
use serde::{self, ser};
use std::io::prelude::*;
use std::io::Result as IOResult;
use std::os::raw::c_int;

fn write_int<'a>(writer: &'a mut dyn Write, v: c_int) -> IOResult<()> {
    return Ok(writer.write_all(&v.to_ne_bytes())?);
}

pub struct Serializer<'a>(pub &'a mut dyn Write);

type Ok = ();

impl<'a> Serializer<'a> {
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

impl<'a> ser::Serializer for Serializer<'a> {
    type Ok = Ok;
    type Error = Error;
    type SerializeSeq = SeqSerializer<'a>;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn is_human_readable(&self) -> bool {
        return false;
    }

    crate::forward_to_serialize_any! {
        i128 u128 char str bytes none some unit unit_struct unit_variant
        newtype_struct newtype_variant
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        if v {
            return Ok(write_int(self.0, 1)?);
        } else {
            return Ok(write_int(self.0, 0)?);
        }
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

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if let Some(len) = len {
            write_int(self.0, len as c_int)?;
            return Ok(SeqSerializer(self.0));
        } else {
            return Err(ser::Error::custom("sequence length is not known"));
        }
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        return Ok(self);
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        return Ok(self);
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        return Ok(self);
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!();
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        return Ok(self);
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        return Ok(self);
    }
}

impl<'a> ser::SerializeSeq for Serializer<'a> {
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

impl<'a> ser::SerializeTuple for Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleStruct for Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleVariant for Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeStruct for Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeStructVariant for Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}
