use z80::Z80;
use std::num::Wrapping;

/*
** ADC A, (hl)
** Condition Bits: R0RR
** Clocks:
**   n: 2
**   (hl): 2
**   r: 1
*/
pub fn adc(z80: &mut Z80, op: u8) {
    let val = match op {
        0xCE    => {
            z80.r.pc += 1;
            z80.mmu.rb(z80.r.pc - 1)
        },
        0x8E    => z80.mmu.rb(z80.r.get_hl()),
        0x8F    => z80.r.a,
        0x88    => z80.r.b,
        0x89    => z80.r.c,
        0x8A    => z80.r.d,
        0x8B    => z80.r.e,
        0x8C    => z80.r.h,
        0x8D    => z80.r.l,
        _       => 0,
    };
    z80.r.clear_flags();
    if (val as u16) + (z80.r.a as u16) + (z80.r.get_carry() as u16) > 0xFF {
        z80.r.set_carry(true);
    }
    if (val & 0xF) + (z80.r.a & 0xF) & 0x10 != 0 {
        z80.r.set_half_carry(true);
    }
    z80.r.a = (Wrapping(val) + Wrapping(z80.r.a) + Wrapping(z80.r.get_carry())).0;
    if op == 0x8E || op == 0xCE {
        z80.set_register_clock(2);
    } else {
        z80.set_register_clock(1);
    }
}
