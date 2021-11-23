// mess around the in-memory representation of a type
use std::io::prelude::*;
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

unsafe fn from_bytes<T: Sized, E, Write>(write: Write) -> Result<T, E>
where
    Write: FnOnce(&mut [u8]) -> Result<(), E>,
{
    let mut value = mem::MaybeUninit::<T>::uninit();
    write(as_bytes_mut(&mut value))?;
    return Ok(value.assume_init());
}

pub unsafe fn from_reader<T: Sized>(reader: &mut dyn Read) -> std::io::Result<T> {
    return from_bytes(|dst| reader.read_exact(dst));
}

pub unsafe fn into_writer<'a, T: Sized>(
    value: &'a T,
    writer: &mut dyn Write,
) -> std::io::Result<()> {
    writer.write_all(as_bytes_ref(value))?;
    return Ok(());
}
