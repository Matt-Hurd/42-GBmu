use z80::Z80;

pub fn cp_r_b(z80: &mut Z80) {
    let temp = z80.r.a;
    z80.r.f |= 0x40;
    if ((temp - z80.r.b) & 255) != 0 {
        z80.r.f |= 0x80;
    }
    if temp < z80.r.b {
        z80.r.f |= 0x10;
    }
    z80.set_register_clock(1);
}
