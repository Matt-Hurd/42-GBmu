use z80::Z80;

pub fn xor_a(z80: &mut Z80) {
    z80.r.a ^= z80.r.a;
    z80.r.a &= 255;
    z80.r.set_zero();
    z80.set_register_clock(1);
}
