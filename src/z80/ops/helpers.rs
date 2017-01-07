use z80::Z80;

pub fn fz_one(z80: &mut Z80, i: u16) -> u8 {
    z80.r.f = 0;
    if (i & 255) == 0 {
        z80.r.f |= 128;
    }
    match z80.r.f {
        0   => 0,
        _   => 0x40,
    }
}

pub fn fz_two(z80: &mut Z80, i: u8, as: u8) -> u8 {
    z80.r.f = 0;
    if (i & 255) == 0 {
        z80.r.f |= 128;
    }
    z80.r.f |= as;
    match z80.r.f {
        0   => 0,
        _   => 0x40,
    }
}
