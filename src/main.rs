mod command;

use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::io::prelude::*;
pub use command::*;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;

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
type Command<'a> = (u8, &'a[u8]);

lazy_static! {
    static ref FILE_VERSION: std::collections::HashMap<i32, FileFeature> = hashmap! {
        1 => FileFeature {  encrypted: false,   with_marker: false },
        2 => FileFeature {  encrypted: true,    with_marker: false },
        3 => FileFeature {  encrypted: false,   with_marker: true },
        4 => FileFeature {  encrypted: true,    with_marker: true },
    };
}

fn read_type<T>(src: &mut dyn Read, dst: &mut T) -> Result<()>
where
    T: std::marker::Sized,
{
    unsafe {
        let buf = std::slice::from_raw_parts_mut(
            dst as *mut _ as *mut u8,
            std::mem::size_of::<T>()
        );

        src.read_exact(buf)?;

        drop(buf);
    }

    return Ok(());
}

fn main() -> Result<()> {
    let mut file = File::open(SESSION_PATH)?;

    let mut header = FileHeader {
        signature: 0,
        version: 0,
    };

    read_type(&mut file, &mut header)?;

    assert!(header.signature == FILE_SIGNATURE);
    println!("{:#08x?}", header.signature);
    println!("{}", header.version);

    let capability = FILE_VERSION.get(&header.version).unwrap();

    println!("{:?}", capability);

    let mut size: CommandSize = 0;

    read_type(&mut file, &mut size)?;

    if size <= 0 {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid size"));
    }

    let mut command = vec![0; size as usize];

    file.read_exact(&mut command)?;

    let command: Command = (command[0], &command[1..]);

    println!("Command size: {:?}", size);
    if let Some(kind) = command::CommandKind::from(command.0) {
        println!("Command type: {:?}", kind);
    } else {
        println!("Command type: {:?}", command.0);
    }
    println!("Command body: {:?}", command.1);
    println!();

    return Ok(());
}
