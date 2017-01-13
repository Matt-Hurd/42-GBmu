use z80::Z80;

/*
** XOR r|$xx|(hl)
** Condition Bits: R000
** Clocks:
**    r: 1
**    $xx: 2
**    (hl): 2
*/
pub fn xor(z80: &mut Z80, op: u8) {
    let val = match op {
        0xEE    => {
            z80.r.pc += 1;
            z80.mmu.rb(z80.r.pc - 1)
        },
        0xAE    => z80.mmu.rb(z80.r.get_hl()),
        0xAF    => z80.r.a,
        0xA8    => z80.r.b,
        0xA9    => z80.r.c,
        0xAA    => z80.r.d,
        0xAB    => z80.r.e,
        0xAC    => z80.r.h,
        0xAD    => z80.r.l,
        _       => 0
    };
    z80.r.clear_flags();
    z80.r.a ^= val;
    if z80.r.a == 0 {
        z80.r.set_zero(true);
    } else {
        z80.r.set_zero(false);
    }
    if op == 0xEE || op == 0xAE {
        z80.set_register_clock(2);
    } else {
        z80.set_register_clock(1);
    }
}
