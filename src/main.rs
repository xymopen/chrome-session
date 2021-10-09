use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

const SESSION_PATH: &str = "./Session";

fn main() -> Result<()> {
    let mut file = File::open(SESSION_PATH)?;

    let mut buf: [u8; 4] = [0; 4];
    file.read_exact(&mut buf)?;
    assert!(buf == "SNSS".as_bytes());

    println!("{:#04x?}", buf);
    drop(buf);

    return Ok(());
}
