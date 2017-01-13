use z80::Z80;
use std::num::Wrapping;

/*
** ADD A, r|(hl)|$xx
** Condition Bits: R0RR
** Clocks:
**    r: 1
**    $xx: 2
**    hl: 2
*/
pub fn add_a(z80: &mut Z80, op: u8) {
    let a = z80.r.a;
    let mut val = match op {
        0xC6    => {
            z80.r.pc += 1;
            z80.mmu.rb(z80.r.pc - 1)
        },
        0x86    => z80.mmu.rb(z80.r.get_hl()),
        0x87    => z80.r.a,
        0x80    => z80.r.b,
        0x81    => z80.r.c,
        0x82    => z80.r.d,
        0x83    => z80.r.e,
        0x84    => z80.r.h,
        0x85    => z80.r.l,
        _       => 0,
    };
    z80.r.clear_flags();
    z80.r.a = (Wrapping(a) + Wrapping(val)).0;
    if z80.r.a == 0 {
        z80.r.set_zero(true);
    }
    else if z80.r.a < a {
        z80.r.set_carry(true);
    }
    if (z80.r.a ^ val ^ a) & 0x10 != 0 {
        z80.r.set_half_carry(true);
    }
    if op == 0x86 || op == 0xC6 {
        z80.set_register_clock(2);
    } else {
        z80.set_register_clock(1);
    }
}

/*
** ADD SP, $xx
** Condition Bits: 00RR
** Clocks: 2
*/
pub fn add_sp_n(z80: &mut Z80) {
    let val = z80.mmu.rb(z80.r.pc) as u16;
    let sp = z80.r.sp;
    z80.r.pc += 1;
    z80.r.clear_flags();
    if val > 127 {
        z80.r.sp = (Wrapping(z80.r.sp) - Wrapping(256 - val)).0;
    } else {
        z80.r.sp = (Wrapping(z80.r.sp) + Wrapping(val)).0;
    }
    if z80.r.sp == 0 {
        z80.r.set_zero(true);
    }
    else if z80.r.sp < sp {
        z80.r.set_carry(true);
    }
    if (z80.r.sp ^ val ^ sp) & 0x10 != 0 {
        z80.r.set_half_carry(true);
    }
    z80.set_register_clock(2);
}

/*
** ADD hl, rr|sp
** Condition Bits: _0RR
** Clocks:
**    All: 3
*/
pub fn add_hl(z80: &mut Z80, op: u8) {
    let hl = z80.r.get_hl();
    let mut val = match op {
        0x09    => z80.r.get_bc(),
        0x19    => z80.r.get_de(),
        0x29    => z80.r.get_hl(),
        0x39    => z80.r.sp,
        _       => 0,
    };
    z80.r.set_hl((Wrapping(hl) + Wrapping(val)).0);
    z80.r.set_subtract(false);
    if z80.r.get_hl() < hl {
        z80.r.set_carry(true);
    } else {
        z80.r.set_carry(false);
    }
    if (z80.r.get_hl() ^ val ^ hl) & 0x10 != 0 {
        z80.r.set_half_carry(true);
    } else {
        z80.r.set_half_carry(false);
    }
    z80.set_register_clock(3);
}
