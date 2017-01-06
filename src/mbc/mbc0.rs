
pub fn read_mbc0(data: &[u8]) {
    let len = match data[0x143] & 80 {
        0x80 => 11,
        _ => 16,
    };
    let title = data[0x134..][..len]
        .iter()
        .cloned()
        .map(|b| b as char)
        .collect::<String>();

    println!("ROM Info:");
    println!("Title: {}", title);
    println!("CGB Flag: {}", data[0x143]);
    println!("New Licensee Code: {} {}", data[0x144], data[0x145]);
    println!("SGB Flag: {}", data[0x146]);
    println!("Cartridge Type {}", data[0x147]);
    println!("ROM Size {}", data[0x148]);
    println!("RAM Size {}", data[0x149]);
    println!("Destination Code {}", data[0x14A]);
    println!("Old Licensee Code {}", data[0x14B]);
    println!("Mask ROM Version number {}", data[0x14C]);
}
