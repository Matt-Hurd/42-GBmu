use z80::Z80;

//Need to check for overflow, not sure if there's a better way to check
pub fn add_r_e(z80: &mut Z80) {
    let temp: u16 = z80.r.a as u16 + z80.r.e as u16;
    z80.r.f = 0;
    if (temp & 255 as u16) != 0 {
        z80.r.f |= 0x80;
    }
    if temp > 255 {
        z80.r.f |= 0x10;
    }
    z80.r.a = temp as u8 & 255;
    z80.set_register_clock(1);
}
