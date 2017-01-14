use z80::Z80;

pub fn ret(z80: &mut Z80, op: u8) {
    z80.set_register_clock(1);
    match op {
        0xD8    => if z80.r.get_carry() != 1 { return ; },
        0xD0    => if z80.r.get_carry() != 0 { return ; },
        0xC0    => if z80.r.get_zero() != 0 { return ; },
        0xC8    => if z80.r.get_zero() != 1 { return ; },
        0xD9    => z80.r.ime = true,
        _       => (),
    };
    z80.r.pc = z80.mmu.rw(z80.r.sp);
    z80.r.sp += 2;
    z80.set_register_clock(3);
}
