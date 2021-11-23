// mess around the in-memory representation of a type
use std::io::prelude::*;
use std::io::Result;
use std::marker::Sized;
use std::mem;
use std::slice;

pub unsafe fn as_ref<'a, T: Sized>(bytes: &'a [u8]) -> Option<&'a T> {
    if bytes.len() != mem::size_of::<T>() {
        return None;
    } else {
        return Some(mem::transmute::<&'a u8, &'a T>(&bytes[0]));
    }
}

pub unsafe fn as_mut<'a, T: Sized>(bytes: &'a mut [u8]) -> Option<&'a mut T> {
    if bytes.len() != mem::size_of::<T>() {
        return None;
    } else {
        return Some(mem::transmute::<&'a mut u8, &'a mut T>(&mut bytes[0]));
    }
}

pub unsafe fn as_bytes_ref<'a, T: Sized>(value: &'a T) -> &'a [u8] {
    return slice::from_raw_parts(value as *const T as *const u8, mem::size_of::<T>());
}

pub unsafe fn as_bytes_mut<'a, T: Sized>(value: &'a mut T) -> &'a mut [u8] {
    return slice::from_raw_parts_mut(value as *mut T as *mut u8, mem::size_of::<T>());
}

pub unsafe fn from_reader<T: Sized>(reader: &mut dyn Read) -> Result<T> {
    let mut value = mem::MaybeUninit::<T>::uninit();
    reader.read_exact(as_bytes_mut(&mut value))?;
    return Ok(value.assume_init());
}

pub unsafe fn into_writer<'a, T: Sized>(value: &'a T, writer: &mut dyn Write) -> Result<()> {
    writer.write_all(as_bytes_ref(value))?;
    return Ok(());
}
