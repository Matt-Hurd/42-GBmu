use z80::Z80;

pub fn zero_flag_one(z80: &mut Z80, left: u16) -> u8 {
    z80.r.f = 0;
    if left == 0 {
        z80.r.set_zero();
    }
    match z80.r.f {
        0   => 0,
        _   => 0x40,
    }
}

pub fn zero_flag_two(z80: &mut Z80, left: u8, right: u8) -> u8 {
    z80.r.f = 0;
    if left == 0 {
        z80.r.set_zero();
    }
    z80.r.f |= right;
    match z80.r.f {
        0   => 0,
        _   => 0x40,
    }
}
