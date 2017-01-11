use z80::Z80;
use std::num::Wrapping;

/*
** LD r8|(HL), r8|(HL)
** Condition Bits: ____
** Clocks:
**   (hl): 2
**   r, r: 1
*/
pub fn ld_u8_r_r(z80: &mut Z80, op: u8) {
    let val = match op & 0xF {
        0x7 | 0xF   => z80.r.a,
        0x0 | 0x8   => z80.r.b,
        0x1 | 0x9   => z80.r.c,
        0x2 | 0xA   => z80.r.d,
        0x3 | 0xB   => z80.r.e,
        0x4 | 0xC   => z80.r.h,
        0x5 | 0xD   => z80.r.l,
        0x6 | 0xE   => z80.mmu.rb(z80.r.get_hl()),
        _           => 0,
    };
    match (op & 0x38) >> 3 {
        0x0 => z80.r.b = val,
        0x1 => z80.r.c = val,
        0x2 => z80.r.d = val,
        0x3 => z80.r.e = val,
        0x4 => z80.r.h = val,
        0x5 => z80.r.l = val,
        0x6 => z80.mmu.wb(z80.r.get_hl(), val),
        0x7 => z80.r.a = val,
        _   => (),
    }
    if op & 0xF == 0x6 || op & 0xF == 0xE || (op & 0x38) >> 3 == 0x6 {
        z80.set_register_clock(2);
    } else {
        z80.set_register_clock(1);
    }
}

/*
** LD (C), A
** Condition Bits: ____
** Clocks: 2
*/
pub fn ld_c_a(z80: &mut Z80) {
    z80.mmu.wb(0xFF00 + z80.r.c as u16, z80.r.a);
    z80.set_register_clock(2);
}


/*
** LD (HL), $xx
** Condition Bits: ____
** Clocks: 3
*/
pub fn ld_hl_n(z80: &mut Z80) {
    let val = z80.mmu.rb(z80.r.pc);
    z80.r.pc += 1;
    z80.mmu.wb(z80.r.get_hl(), val);
    z80.set_register_clock(3);
}

/*
** LD ($aabb), A
** Condition Bits: ____
** Clocks: 4
*/
pub fn ld_nn_a(z80: &mut Z80) {
    let addr = z80.mmu.rw(z80.r.pc) as u16;
    z80.r.pc += 2;
    z80.mmu.wb(addr, z80.r.a);
    z80.set_register_clock(4);
}

/*
** LD ($xx), A
** Condition Bits: ____
** Clocks: 2
*/
pub fn ld_n_a(z80: &mut Z80) {
    let addr = z80.mmu.rb(z80.r.pc) as u16;
    z80.r.pc += 1;
    z80.mmu.wb(0xFF00 + addr, z80.r.a);
    z80.set_register_clock(2);
}

/*
** LD ($aabb), SP
** Condition Bits: ____
** Clocks: 5
*/
pub fn ld_nn_sp(z80: &mut Z80) {
    let addr = z80.mmu.rw(z80.r.pc) as u16;
    z80.r.pc += 2;
    z80.mmu.ww(addr, z80.r.sp);
    z80.set_register_clock(5);
}

/*
** LD (rr), A
** Condition Bits: ____
** Clocks: 2
*/
pub fn ld_rr_a(z80: &mut Z80, op: u8) {
    let addr = match op {
        0x02    => z80.r.get_bc(),
        0x12    => z80.r.get_de(),
        0x77    => z80.r.get_hl(),
        _       => 0,
    };
    z80.mmu.wb(addr, z80.r.a);
    z80.set_register_clock(2);
}

/*
** LD A, (C)
** Condition Bits: ____
** Clocks: 2
*/
pub fn ld_a_c(z80: &mut Z80) {
    z80.r.a = z80.mmu.rb(0xFF00 + z80.r.c as u16);
    z80.set_register_clock(2);
}

/*
** LD A, ($xx)
** Condition Bits: ____
** Clocks: 2
*/
pub fn ld_a_n(z80: &mut Z80) {
    let addr = z80.mmu.rb(z80.r.pc);
    z80.r.pc += 1;
    z80.r.a = z80.mmu.rb(0xFF00 + addr as u16);
    z80.set_register_clock(3);
}

/*
** LD A, ($aabb)
** Condition Bits: ____
** Clocks: 4
*/
pub fn ld_a_nn(z80: &mut Z80) {
    let addr = z80.mmu.rw(z80.r.pc) as u16;
    z80.r.pc += 2;
    z80.r.a = z80.mmu.rb(addr);
    z80.set_register_clock(4);
}

/*
** LD A, (rr)
** Condition Bits: ____
** Clocks: 2
*/
pub fn ld_a_rr(z80: &mut Z80, op: u8) {
    let addr = match op {
        0x0A    => z80.r.get_bc(),
        0x1A    => z80.r.get_de(),
        0x7E    => z80.r.get_hl(),
        _       => 0,
    };
    z80.r.a = z80.mmu.rb(addr);
    z80.set_register_clock(2);
}
/*
** LD r8, $xx
** Condition Bits: ____
** Clocks: 2
*/
pub fn ld_u8_r_n(z80: &mut Z80, op: u8) {
    let val = z80.mmu.rb(z80.r.pc);
    z80.r.pc += 1;
    match op {
        0x06 => z80.r.b = val,
        0x0E => z80.r.c = val,
        0x16 => z80.r.d = val,
        0x1E => z80.r.e = val,
        0x26 => z80.r.h = val,
        0x2E => z80.r.l = val,
        0x3E => z80.r.a = val,
        _   => (),
    }
    z80.set_register_clock(2);
}

/*
** LD rr, $aabb
** Condition Bits: ____
** Clocks: 3
*/
pub fn ld_rr_nn(z80: &mut Z80, op: u8) {
    let val = z80.mmu.rw(z80.r.pc);
    z80.r.pc += 2;
    match op {
        0x01    => z80.r.set_bc(val),
        0x11    => z80.r.set_de(val),
        0x21    => z80.r.set_hl(val),
        0x31    => z80.r.sp = val,
        _       => (),
    };
    z80.set_register_clock(3);
}

/*
** LDD A|(hl), A|(hl)
** Condition Bits: ____
** Clocks: 2
*/
pub fn ldd_hl(z80: &mut Z80, op: u8) {
    let hl = z80.r.get_hl();
    z80.r.set_hl((Wrapping(hl) - Wrapping(1)).0);
    if op == 0x3A {
        z80.r.a = z80.mmu.rb(z80.r.get_hl());
    } else {
        z80.mmu.wb(z80.r.get_hl(), z80.r.a);
    }
    z80.set_register_clock(2);
}

/*
** LDI A|(hl), A|(hl)
** Condition Bits: ____
** Clocks: 2
*/
pub fn ldi_hl(z80: &mut Z80, op: u8) {
    if op == 0x2A {
        z80.r.a = z80.mmu.rb(z80.r.get_hl());
    } else {
        z80.mmu.wb(z80.r.get_hl(), z80.r.a);
    }
    let hl = z80.r.get_hl();
    z80.r.set_hl((Wrapping(hl) + Wrapping(1)).0);
    z80.set_register_clock(2);
}
