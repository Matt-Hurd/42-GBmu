use z80::Z80;

/*
** CP (register, (HL), $xx)
*/
pub fn cp_r(z80: &mut Z80, op: u8) {
    let cmp = match op {
        0xFE => {
            z80.r.pc += 1;
            z80.mmu.rb(z80.r.pc - 1)
        },
        0xBE => z80.mmu.rb(z80.r.get_hl()),
        0xBF => z80.r.a,
        0xB8 => z80.r.b,
        0xB9 => z80.r.c,
        0xBA => z80.r.d,
        0xBB => z80.r.e,
        0xBC => z80.r.h,
        0xBD => z80.r.l,
        _    => 0,
    };
    z80.r.clear_flags();
    z80.r.set_subtract(true);
    if z80.r.a == cmp {
        z80.r.set_zero(true);
    }
    if z80.r.a < cmp {
        z80.r.set_carry(true);
    }
    z80.set_register_clock(1);
}
