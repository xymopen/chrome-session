use serde::*;
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

    return Ok(());
}
