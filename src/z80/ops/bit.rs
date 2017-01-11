use z80::Z80;

/*
** BIT b, r|(hl)
** Condition Bits: R01_
** Clocks:
**   (hl): 3
**   r: 2
*/
pub fn bit(z80: &mut Z80, op: u8) {
    let val = match op & 0x7 {
        0x6    => z80.mmu.rb(z80.r.get_hl()),
        0x7    => z80.r.a,
        0x0    => z80.r.b,
        0x1    => z80.r.c,
        0x2    => z80.r.d,
        0x3    => z80.r.e,
        0x4    => z80.r.h,
        0x5    => z80.r.l,
        _       => 0,
    };
    let bit_val = (op >> 3) & 0x7;
    z80.r.set_half_carry(true);
    z80.r.set_subtract(false);
    if val & (1 << bit_val) == 0 {
        z80.r.set_zero(true);
    } else {
        z80.r.set_zero(false);
    }
    if op & 0x7 == 0x6 {
        z80.set_register_clock(3);
    } else {
        z80.set_register_clock(2);
    }
}
