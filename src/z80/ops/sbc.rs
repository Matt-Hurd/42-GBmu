use z80::Z80;
/*
** SBC A, $xx|(HL)|register
*/
pub fn sbc(z80: &mut Z80, op: u8) {
    let sub = match op {
        0xDE => {
            z80.r.pc += 1;
            z80.mmu.rb(z80.r.pc - 1)
        },
        0x9E => z80.mmu.rb(z80.r.get_hl()),
        0x9F => z80.r.a,
        0x98 => z80.r.b,
        0x99 => z80.r.c,
        0x9A => z80.r.d,
        0x9B => z80.r.e,
        0x9C => z80.r.h,
        0x9D => z80.r.l,
        _    => 0,
    }.wrapping_add(z80.r.get_carry());
    let val = z80.r.a;
    z80.r.clear_flags();
    z80.r.set_subtract(true);
    if sub > val {
        z80.r.set_carry(true);
    }
    z80.r.a = val.wrapping_sub(sub);
    if z80.r.a == 0 {
        z80.r.set_zero(true);
    }
    if (z80.r.a ^ (val + 1) ^ sub) & 0x10 != 0 {
        z80.r.set_half_carry(true);
    }
    z80.set_register_clock(1);
}
