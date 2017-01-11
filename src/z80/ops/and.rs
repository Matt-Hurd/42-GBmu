use z80::Z80;

/*
** AND r|$xx|(hl)
** Condition Bits: r010
** Clocks:
**    r: 1
**    $xx: 2
**    (hl): 2
*/
pub fn and(z80: &mut Z80, op: u8) {
    let val = match op {
        0xE6    => {
            z80.r.pc += 1;
            z80.mmu.rb(z80.r.pc - 1)
        },
        0xA6    => z80.mmu.rb(z80.r.get_hl()),
        0xA7    => z80.r.a,
        0xA0    => z80.r.b,
        0xA1    => z80.r.c,
        0xA2    => z80.r.d,
        0xA3    => z80.r.e,
        0xA4    => z80.r.h,
        0xA5    => z80.r.l,
        _       => 0,
    };
    z80.r.clear_flags();
    z80.r.set_half_carry(true);
    z80.r.a &= val;
    if z80.r.a == 0 {
        z80.r.set_zero(true);
    } else {
        z80.r.set_zero(false);
    }
    if op == 0xA6 || op == 0xE6 {
        z80.set_register_clock(2);
    } else {
        z80.set_register_clock(1);
    }
}
