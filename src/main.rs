mod command;
mod transmute;

pub use command::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

const SESSION_PATH: &str = "./Session";

// The signature at the beginning of the file = SSNS (Sessions).
const FILE_SIGNATURE: i32 = 0x53534E53;

#[repr(C)]
struct FileHeader {
    signature: i32,
    version: i32,
}

#[derive(Debug)]
struct FileFeature {
    encrypted: bool,
    with_marker: bool,
}

type CommandSize = u16;

lazy_static! {
    static ref FILE_VERSION: std::collections::HashMap<i32, FileFeature> = hashmap! {
        1 => FileFeature {  encrypted: false,   with_marker: false },
        2 => FileFeature {  encrypted: true,    with_marker: false },
        3 => FileFeature {  encrypted: false,   with_marker: true },
        4 => FileFeature {  encrypted: true,    with_marker: true },
    };
}

fn main() -> Result<()> {
    let mut file = File::open(SESSION_PATH)?;

    let header = unsafe { transmute::from_reader::<FileHeader>(&mut file)? };

    assert!(header.signature == FILE_SIGNATURE);
    println!("{:#08x?}", header.signature);
    println!("{}", header.version);

    let capability = FILE_VERSION.get(&header.version).unwrap();

    println!("{:?}", capability);

    let size = unsafe { transmute::from_reader::<CommandSize>(&mut file)? };

    if size <= 0 {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid size"));
    }

    let mut command = vec![0; size as usize];

    file.read_exact(&mut command)?;

    let command = Command::from(command);

    println!("Command size: {:?}", size);
    if let Some(kind) = command.kind() {
        println!("Command type: {:?}", kind);
    } else {
        println!("Command type: {:?}", command.id());
    }
    println!("Command body: {:?}", command.payload());
    println!();

    return Ok(());
}
