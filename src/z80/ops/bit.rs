use z80::Z80;

pub fn bit_7_h(z80: &mut Z80) {
    if z80.r.h & 0x80 == 0 {
        z80.r.set_zero();
    }
    z80.set_register_clock(2);
}
