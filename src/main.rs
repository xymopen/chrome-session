use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

const SESSION_PATH: &str = "./Session";

// The signature at the beginning of the file = SSNS (Sessions).
const FILE_SIGNATURE: i32 = 0x53534E53;

#[repr(C)]
struct FileHeader {
    signature: i32,
    version: i32,
}

fn main() -> Result<()> {
    let mut file = File::open(SESSION_PATH)?;

    let mut header = FileHeader {
        signature: 0,
        version: 0,
    };

    unsafe {
        let buf = std::slice::from_raw_parts_mut(
            &mut header as *mut _ as *mut u8,
            std::mem::size_of::<FileHeader>()
        );

        file.read_exact(buf)?;

        drop(buf);
    }

    assert!(header.signature == FILE_SIGNATURE);
    println!("{:#08x?}", header.signature);
    println!("{}", header.version);

    return Ok(());
}
