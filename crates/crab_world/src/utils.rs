use std::io;
use std::io::{Read, Write};

pub(crate) const MEGABYTE: usize = 1024 * 1024;

pub fn read_u32<R: Read>(reader: &mut R) -> Result<u32, io::Error> {
    let mut bytes = [0u8; 4];
    reader.read_exact(&mut bytes)?;
    Ok(u32::from_be_bytes(bytes))
}

pub fn write_u32<W: Write>(writer: &mut W, value: u32) -> Result<(), io::Error> {
    let bytes = value.to_be_bytes();
    writer.write_all(&bytes)
}
