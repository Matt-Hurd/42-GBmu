use z80::Z80;

/*
** JR cc, $aa
** Condition Bits: ____
** Clocks:
**   cc true: 3
**   cc false: 2
*/
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

/*
** JP cc, $aabb|(HL)
** Condition Bits: ____
** Clocks:
**   (hl): 1
**   $aabb: 2
**   cc true: 2
**   cc false: 1
*/
pub fn jp_u16(z80: &mut Z80, op: u8) {

    let i = match op {
        0xE9 => {
            z80.mmu.rw(z80.r.get_hl())
        },
        _   => {
        z80.r.pc += 2;
        z80.mmu.rw(z80.r.pc - 2)
        }
    };
    let case = match op {
        0xC3 => true,
        0xDA => z80.r.get_carry() == 1,
        0xD2 => z80.r.get_carry() == 0,
        0xC2 => z80.r.get_zero() == 0,
        0xCA => z80.r.get_zero() == 1,
        _    => true,
    };
    if case {
        z80.r.pc = i;
        if op == 0xE9 {
            z80.set_register_clock(3);
        } else {
            z80.set_register_clock(4);
        }
    }
    else if op == 0xE9 {
        z80.set_register_clock(1);
    } else {
        z80.set_register_clock(2);
    }
}
