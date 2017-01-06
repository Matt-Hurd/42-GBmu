use std::fs::File;
use std::io::Read;
use std::path;
use std::error::Error;

pub mod mbc0;

pub fn read_mbc(filename: path::PathBuf) -> Result<String, Box<Error>> {
    let mut data = vec![];
    try!(File::open(&filename)
        .and_then(|mut f| f.read_to_end(&mut data))
        .map_err(|_| "Could not read ROM"));
    try!(validate_rom(&data));
    Ok("Good".to_string())
    // mbc0::read_mbc0(&data);
}

pub fn validate_rom(data: &[u8]) -> Result<String, Box<Error>>  {
    try!(check_header_checksum(data));
    Ok("Good".to_string())
}

pub fn check_header_checksum(data: &[u8]) -> Result<(), &'static str> {
    let mut value: u8 = 0;
    for i in 0x134 .. 0x14D {
        value = value.wrapping_sub(data[i]).wrapping_sub(1);
    }
    match data[0x14D] == value
    {
        true    => Ok(()),
        false   => Err("Invalid cartridge checksum"),
    }
}
