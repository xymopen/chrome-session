use serde::{
    de::{self, Deserialize as _},
    ser::{self, SerializeSeq as _},
};
use std::ffi::CString;

pub fn serialize<S>(value: &CString, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    let bytes = value.as_bytes();
    let mut ser = serializer.serialize_seq(Some(bytes.len()))?;
    for byte in bytes.iter() {
        ser.serialize_element(byte)?;
    }
    ser.end()
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<CString, D::Error>
where
    D: de::Deserializer<'de>,
{
    let bytes = Vec::<u8>::deserialize(deserializer)?;

    return CString::new(bytes).or_else(|_| {
        Err(de::Error::custom(
            "an interior nul byte was found in the string",
        ))
    });
}
