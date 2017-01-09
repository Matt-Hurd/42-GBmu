use z80::Z80;

pub fn jr_u8(z80: &mut Z80, op: u8) {
    let i = z80.mmu.rb(z80.r.pc);
    z80.r.pc += 1;
    let case = match op {
        0x18 => true,
        0x38 => z80.r.get_carry() == 1,
        0x30 => z80.r.get_carry() == 0,
        0x20 => z80.r.get_zero() == 0,
        0x28 => z80.r.get_zero() == 1,
        _    => false,
    };
    if case {
        if i > 127 {
            z80.r.pc -= (255 - i + 1) as u16;
        } else {
            z80.r.pc += i as u16;
        }
        z80.set_register_clock(3);
    } else {
        z80.set_register_clock(2);
    }
}
