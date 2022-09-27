mod command;
mod serde_chromium_pickle;
mod transmute;

pub use command::*;
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::mem::size_of;

use crate::serde_chromium_pickle::from_reader;

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

#[derive(Debug)]
struct InvalidFileVersion;

impl TryFrom<i32> for FileFeature {
    type Error = InvalidFileVersion;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(FileFeature {
                encrypted: false,
                with_marker: false,
            }),
            2 => Ok(FileFeature {
                encrypted: true,
                with_marker: false,
            }),
            3 => Ok(FileFeature {
                encrypted: false,
                with_marker: true,
            }),
            4 => Ok(FileFeature {
                encrypted: true,
                with_marker: true,
            }),
            _ => Err(InvalidFileVersion),
        }
    }
}

impl From<FileFeature> for i32 {
    fn from(value: FileFeature) -> i32 {
        match (value.encrypted, value.with_marker) {
            (false, false) => 1,
            (true, false) => 2,
            (false, true) => 3,
            (true, true) => 4,
        }
    }
}

type CommandSize = u16;

fn print_buffer(a: &[u8]) {
    a.chunks(16).for_each(|line| {
        line.iter().for_each(|c| {
            print!("{:0>2x}", *c);
            print!(" ");
        });
        for _ in 0..((16 - line.len()) * 3) {
            print!(" ")
        }
        print!(" ");
        line.iter().for_each(|c| match c {
            0x20..=0x7e => print!("{}", *c as char),
            _ => print!("."),
        });
        for _ in 0..(16 - line.len()) {
            print!(" ")
        }
        println!();
    });
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: chrome-session <path/to/snss>");

        return Ok(());
    }

    let mut file = File::open(args[1].to_owned())?;

    let header = unsafe { transmute::from_reader::<FileHeader>(&mut file)? };

    assert!(header.signature == FILE_SIGNATURE);

    loop {
        let size = unsafe {
            match transmute::from_reader::<CommandSize>(&mut file) {
                Ok(size) => {
                    if size > 0 {
                        size
                    } else {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid size"));
                    }
                }
                Err(e) => {
                    if e.kind() == ErrorKind::UnexpectedEof {
                        return Ok(());
                    } else {
                        return Err(e);
                    }
                }
            }
        };

        let id = unsafe { transmute::from_reader::<CommandID>(&mut file) }?;
        let mut payload = vec![0; size as usize - size_of::<CommandID>()];
        file.read_exact(&mut payload)?;

        if let Some(kind) = CommandKind::from(id) {
            match kind {
                CommandKind::UpdateTabNavigation => {
                    let x: SerializedNavigationEntry = from_reader(&mut &payload[..]).unwrap();
                    println!("{} - {}", x.title, x.virtual_url.to_str().unwrap());
                }
            }
        }
    }
}
