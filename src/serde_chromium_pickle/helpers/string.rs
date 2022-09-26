use serde::{
    de::{self, Deserialize as _},
    ser::{self, SerializeSeq as _},
};

pub fn serialize<S>(value: &String, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    let buffer: Vec<u16> = value.encode_utf16().collect();
    let mut ser = serializer.serialize_seq(Some(buffer.len()))?;
    for byte in buffer {
        ser.serialize_element(&byte)?;
    }
    ser.end()
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let buffer = Vec::<u16>::deserialize(deserializer)?;

    return String::from_utf16(&buffer).or_else(|_| {
        Err(de::Error::custom(
            "converting from a UTF-16 byte slice failed",
        ))
    });
}
