use z80::Z80;

/*
** RL(C) register or (hl)
*/
pub fn rl_r(z80: &mut Z80, op: u8) {
    let mut val = match op & 0xF {
        0x0 => z80.r.b,
        0x1 => z80.r.c,
        0x2 => z80.r.d,
        0x3 => z80.r.e,
        0x4 => z80.r.h,
        0x5 => z80.r.l,
        0x6 => z80.mmu.rb(z80.r.get_hl()),
        0x7 => z80.r.a,
        _   => 0,
    };
    let extra = (val & 0x80) >> 7;
    val = (val & 0x7F) << 1;
    if op & 0xF0 == 1 {
        val |= z80.r.get_carry();
    } else {
        val |= extra;
    }
    z80.r.clear_flags();
    if extra == 1 {
        z80.r.set_carry(true);
    }
    if val == 0 {
        z80.r.set_zero(true);
    }
    z80.set_register_clock(2);
    match op & 0xF {
        0x0 => z80.r.b = val,
        0x1 => z80.r.c = val,
        0x2 => z80.r.d = val,
        0x3 => z80.r.e = val,
        0x4 => z80.r.h = val,
        0x5 => z80.r.l = val,
        0x6 => {
            z80.mmu.wb(z80.r.get_hl(), val);
            z80.set_register_clock(4);
        },
        0x7 => z80.r.a = val,
        _   => (),
    }
}
