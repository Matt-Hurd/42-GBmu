use z80::Z80;

/*
** CALL cc, $aabb
** Condition Bits: ____
** Clocks:
**   cc true: 5
**   cc false: 3
*/
pub fn call(z80: &mut Z80, op: u8) {
    z80.set_register_clock(3);
    z80.r.pc += 2;
    match op {
        0xDC    => if z80.r.get_carry() != 1 { return ; },
        0xD4    => if z80.r.get_carry() != 0 { return ; },
        0xC4    => if z80.r.get_zero() != 0 { return ; },
        0xCC    => if z80.r.get_zero() != 1 { return ; },
        _       => (),
    };
    z80.r.sp -= 2;
    z80.mmu.ww(z80.r.sp, z80.r.pc);
    z80.r.pc = z80.mmu.rw(z80.r.pc - 2);
    z80.set_register_clock(5);
}
