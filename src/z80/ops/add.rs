use z80::Z80;
use std::num::Wrapping;

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
    z80.set_register_clock(1);
}

pub fn add_hl(z80: &mut Z80, op: u8) {
    let hl = z80.r.get_hl();
    let mut val = match op {
        0x09    => z80.r.get_bc(),
        0x19    => z80.r.get_de(),
        0x29    => z80.r.get_hl(),
        0x39    => z80.r.sp,
        _       => 0,
    };
    z80.r.clear_flags();
    z80.r.set_hl((Wrapping(hl) + Wrapping(val)).0);
    if z80.r.get_hl() == 0 {
        z80.r.set_zero(true);
    }
    else if z80.r.get_hl() < hl {
        z80.r.set_carry(true);
    }
    if (z80.r.get_hl() ^ val ^ hl) & 0x10 != 0 {
        z80.r.set_half_carry(true);
    }
    z80.set_register_clock(1);
}
