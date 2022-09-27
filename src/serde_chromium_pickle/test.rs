use serde::*;
use std::ffi::CString;
use std::os::raw::{c_int, c_long};

use super::*;

/// checks that the results can be read correctly from the Pickle
#[test]
fn encode_decode() -> Result<()> {
    #[derive(Serialize, Deserialize)]
    struct Result(
        bool,
        bool,
        c_int,
        c_long,
        u16,
        u32,
        i64,
        u64,
        f32,
        f64,
        #[serde(with = "helpers::cstring")] CString,
        #[serde(with = "helpers::string")] String,
        Vec<u8>,
        Vec<u16>,
        Vec<u8>,
    );

    let mut vec = Vec::new();

    into_writer(
        &Result(
            false,
            true,
            2_093_847_192,
            1_093_847_192,
            32123,
            1593847192,
            -0x7E8CA925_3104BDFC,
            0xCE8CA925_3104BDF7,
            3.1415926935,
            2.71828182845904523,
            // note non-aligned string length
            CString::new("Hello world").unwrap(),
            ("Hello, world").into(),
            // Test raw string writing
            ("Hello new world").into(),
            vec![
                'A' as u16,
                'l' as u16,
                'o' as u16,
                'h' as u16,
                'a' as u16,
                '\0' as u16,
            ],
            ("AAA\0BBB\0").into(),
        ),
        &mut vec,
    )?;

    let result: Result = from_reader(&mut &vec[..])?;

    assert_eq!(result.0, false);
    assert_eq!(result.1, true);
    assert_eq!(result.2, 2_093_847_192);
    assert_eq!(result.3, 1_093_847_192);
    assert_eq!(result.4, 32123);
    assert_eq!(result.5, 1593847192);
    assert_eq!(result.6, -0x7E8CA925_3104BDFC);
    assert_eq!(result.7, 0xCE8CA925_3104BDF7);
    assert_eq!(result.8, 3.1415926935);
    assert_eq!(result.9, 2.71828182845904523);
    assert_eq!(result.10, CString::new("Hello world").unwrap());
    assert_eq!(result.11, String::from("Hello, world"));
    assert_eq!(result.12, Vec::from("Hello new world"));
    assert_eq!(
        result.13,
        vec![
            'A' as u16,
            'l' as u16,
            'o' as u16,
            'h' as u16,
            'a' as u16,
            '\0' as u16,
        ]
    );
    assert_eq!(result.14, Vec::from("AAA\0BBB\0"));

    return Ok(());
}

#[test]
fn unaligned_size() -> Result<()> {
    let buffer = [10 as u32, 25, 40];
    let vec: Vec<u8> = buffer.iter().flat_map(|e| e.to_ne_bytes()).collect();

    assert!(
        from_reader::<(u32, u32)>(&mut vec.as_slice())
            .unwrap_err()
            .to_string()
            == "payload is not aligned"
    );

    return Ok(());
}

#[test]
fn zero_len_cstr() -> Result<()> {
    let mut vec = Vec::new();
    into_writer(&[0 as u8; 0], &mut vec)?;

    assert_eq!(from_reader::<[u8; 0]>(&mut vec.as_slice())?, [0 as u8; 0]);

    return Ok(());
}

#[test]
fn zero_len_str() -> Result<()> {
    let mut vec = Vec::new();
    into_writer(&[0 as u8; 0], &mut vec)?;

    assert_eq!(from_reader::<[u16; 0]>(&mut vec.as_slice())?, [0 as u16; 0]);

    return Ok(());
}

#[test]
fn bad_len_cstr() -> Result<()> {
    let mut vec = Vec::new();
    into_writer(&(-2,), &mut vec)?;

    assert!(
        from_reader::<Vec<u8>>(&mut vec.as_slice())
            .unwrap_err()
            .to_string()
            == "invalid sequence length"
    );

    return Ok(());
}

#[test]
fn bad_len_str() -> Result<()> {
    let mut vec = Vec::new();
    into_writer(&(-2,), &mut vec)?;

    assert!(
        from_reader::<Vec<u16>>(&mut vec.as_slice())
            .unwrap_err()
            .to_string()
            == "invalid sequence length"
    );

    return Ok(());
}
