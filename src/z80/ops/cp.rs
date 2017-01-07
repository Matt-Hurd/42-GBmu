use z80::Z80;

pub fn cp_r_b(z80: &mut Z80) {
    z80.r.set_subtract();
    if z80.r.a == z80.r.b {
        z80.r.set_zero();
    }
    if z80.r.a < z80.r.b {
        z80.r.set_carry();
    }
    z80.set_register_clock(1);
}
