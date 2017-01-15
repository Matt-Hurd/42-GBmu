use z80::Z80;

pub fn sub(z80: &mut Z80, op: u8) {
    let a = z80.r.a;
    let val = match op {
        0xD6    => {
            z80.r.pc += 1;
            z80.mmu.rb(z80.r.pc - 1)
        },
        0x97    => z80.r.a,
        0x90    => z80.r.b,
        0x91    => z80.r.c,
        0x92    => z80.r.d,
        0x93    => z80.r.e,
        0x94    => z80.r.h,
        0x95    => z80.r.l,
        _       => 0,
    };
    z80.r.clear_flags();
    z80.r.set_subtract(true);
    if val == z80.r.a {
        z80.r.set_zero(true);
    }
    else if val > z80.r.a {
        z80.r.set_carry(true);
    }
    z80.r.a = a.wrapping_sub(val);
    if (z80.r.a ^ val ^ a) & 0x10 != 0 {
        z80.r.set_half_carry(true);
    }
    z80.set_register_clock(1);
}
