use std::fs::File;
use std::io::Read;
use std::path;
use std::error::Error;

pub mod mbc0;

pub fn read_mbc(filename: path::PathBuf) -> Result<String, Box<Error>> {
    let mut data = vec![];
    File::open(&filename).and_then(|mut f| f.read_to_end(&mut data))
        .map_err(|_| "Could not read ROM")?;
    validate_rom(&data)?;
    match data[0x147] {
        0 => mbc0::read_mbc0(&data),
        _ => panic!("Unsupported MBC".to_string()),
    }
    Ok("Good".to_string())
}

pub fn validate_rom(data: &[u8]) -> Result<String, Box<Error>> {
    check_header_checksum(data)?;
    Ok("Good".to_string())
}

pub fn check_header_checksum(data: &[u8]) -> Result<(), &'static str> {
    let mut value: u8 = 0;
    for &byte in &data[0x134..0x14D] {
        value = value.wrapping_sub(byte).wrapping_sub(1);
    }
    if data[0x14D] == value {
        Ok(())
    } else {
        Err("Invalid cartridge checksum")
    }
}
