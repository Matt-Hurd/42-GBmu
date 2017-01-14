use z80::Z80;

/*
** RES b, r|(hl)
** Condition Bits: ____
** Clocks:
**   (hl): 3
**   r: 2
*/
pub fn res(z80: &mut Z80, op: u8) {
    let mut val = match op & 0x7 {
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
    val |= 0b11111111 ^ (0b1 << bit_val);
    match op & 0x7 {
        0x6    => z80.mmu.wb(z80.r.get_hl(), val),
        0x7    => z80.r.a = val,
        0x0    => z80.r.b = val,
        0x1    => z80.r.c = val,
        0x2    => z80.r.d = val,
        0x3    => z80.r.e = val,
        0x4    => z80.r.h = val,
        0x5    => z80.r.l = val,
        _       => (),
    };
    if op & 0x7 == 0x6 {
        z80.set_register_clock(3);
    } else {
        z80.set_register_clock(2);
    }
}
