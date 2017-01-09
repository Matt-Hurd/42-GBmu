use z80::Z80;

pub fn add_r_e(z80: &mut Z80) {
    let (sum, overflow) = z80.r.a.overflowing_add(z80.r.e);
    z80.r.f = 0;
    if sum != 0 {
        z80.r.set_zero(true);
    }
    if overflow {
        z80.r.set_carry(true);
    }
    z80.r.a = sum;
    z80.set_register_clock(1);
}
