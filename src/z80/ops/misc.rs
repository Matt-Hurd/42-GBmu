use z80::Z80;

/*
** PUSH
** Condition Bits: ____
** Clocks: 4
*/
pub fn push(z80: &mut Z80, op: u8) {
    if z80.r.sp < 2 {
        panic!("Stack Underflow".to_string())
    }
    let val = match op {
        0xF5    => z80.r.get_af(),
        0xC5    => z80.r.get_bc(),
        0xD5    => z80.r.get_de(),
        0xE5    => z80.r.get_hl(),
        _       => 0,
    };
    z80.r.sp -= 1;
    z80.mmu.wb(z80.r.sp, ((val & 0xFF00) >> 8) as u8);
    z80.r.sp -= 1;
    z80.mmu.wb(z80.r.sp, (val & 0xFF) as u8);
    z80.set_register_clock(3);
}

/*
** POP rr
** Condition Bits: ____
** Clocks: 3
*/
pub fn pop_u16(z80: &mut Z80, op: u8) {
    let sp = z80.r.sp;
    match op {
        0xF1    => z80.r.set_af(z80.mmu.rw(sp)),
        0xC1    => z80.r.set_bc(z80.mmu.rw(sp)),
        0xD1    => z80.r.set_de(z80.mmu.rw(sp)),
        0xE1    => z80.r.set_hl(z80.mmu.rw(sp)),
        _       =>  (),
    }
    z80.r.sp += 2;
    z80.set_register_clock(3);
}

/*
** NOP
** Condition Bits: ____
** Clocks: 1
*/
pub fn nop(z80: &mut Z80) {
    z80.set_register_clock(1);
}

/*
** CPL
** Condition Bits: _11_
** Clocks: 1
*/
pub fn cpl(z80: &mut Z80) {
    z80.r.a ^= 0xFF;
    z80.r.set_half_carry(true);
    z80.r.set_subtract(true);
    z80.set_register_clock(1);
}

/*
** INC (HL)
** Condition Bits: R0R_
** Clocks: 3
*/
pub fn inc_hl(z80: &mut Z80) {
    let mut val = z80.mmu.rb(z80.r.get_hl());
    z80.r.set_subtract(false);
    z80.r.set_half_carry(val & 0xF == 0xF);
    if val == 0xFF {
        z80.r.set_zero(true);
        val = 0;
    } else {
        z80.r.set_zero(false);
        val += 1;
    }
    z80.mmu.wb(z80.r.get_hl(), val);
    z80.set_register_clock(3);
}

/*
** INC r
** Condition Bits: R0R_
** Clocks: 1
*/
pub fn inc_r(z80: &mut Z80, op: u8) {
    let mut val = match op {
        0x3C    => z80.r.a,
        0x04    => z80.r.b,
        0x0C    => z80.r.c,
        0x14    => z80.r.d,
        0x1C    => z80.r.e,
        0x24    => z80.r.h,
        0x2C    => z80.r.l,
        _       => 0,
    };
    z80.r.set_subtract(false);
    z80.r.set_half_carry(val & 0xF == 0xF);
    if val == 0xFF {
        z80.r.set_zero(true);
        val = 0;
    } else {
        z80.r.set_zero(false);
        val += 1;
    }
    match op {
        0x3C    => z80.r.a = val,
        0x04    => z80.r.b = val,
        0x0C    => z80.r.c = val,
        0x14    => z80.r.d = val,
        0x1C    => z80.r.e = val,
        0x24    => z80.r.h = val,
        0x2C    => z80.r.l = val,
        _       => (),
    }
    z80.set_register_clock(1);
}

/*
** INC rr
** Condition Bits: ____
** Clocks: 2
*/
pub fn inc_rr(z80: &mut Z80, op: u8) {
    let mut val = match op {
        0x03    => z80.r.get_bc(),
        0x13    => z80.r.get_de(),
        0x23    => z80.r.get_hl(),
        0x33    => z80.r.sp,
        _       => 0,
    };
    if val == 0xFFFF {
        val = 0;
    } else {
        val += 1;
    }
    match op {
        0x03    => z80.r.set_bc(val),
        0x13    => z80.r.set_de(val),
        0x23    => z80.r.set_hl(val),
        0x33    => z80.r.sp = val,
        _       => (),
    }
    z80.set_register_clock(2);
}

/*
** DEC (HL)
** Condition Bits: R1R_
** Clocks: 3
*/
pub fn dec_hl(z80: &mut Z80) {
    let mut val = z80.mmu.rb(z80.r.get_hl());
    z80.r.set_subtract(true);
    z80.r.set_half_carry(val & 0b00010000 != 0);
    z80.r.set_zero(val == 0x01);
    if val == 0x0 {
        val = 0xFF;
    } else {
        val -= 1;
    }
    z80.mmu.wb(z80.r.get_hl(), val);
    z80.set_register_clock(3);
}

/*
** DEC r
** Condition Bits: R1R_
** Clocks: 1
*/
pub fn dec_r(z80: &mut Z80, op: u8) {
    let mut val = match op {
        0x3D    => z80.r.a,
        0x05    => z80.r.b,
        0x0D    => z80.r.c,
        0x15    => z80.r.d,
        0x1D    => z80.r.e,
        0x25    => z80.r.h,
        0x2D    => z80.r.l,
        _       => 0,
    };
    z80.r.set_subtract(true);
    z80.r.set_half_carry(val & 0b00010000 != 0);
    z80.r.set_zero(val == 0x01);
    if val == 0x0 {
        val = 0xFF;
    } else {
        val -= 1;
    }
    match op {
        0x3D    => z80.r.a = val,
        0x05    => z80.r.b = val,
        0x0D    => z80.r.c = val,
        0x15    => z80.r.d = val,
        0x1D    => z80.r.e = val,
        0x25    => z80.r.h = val,
        0x2D    => z80.r.l = val,
        _       => (),
    }
    z80.set_register_clock(1);
}

/*
** DEC rr
** Condition Bits: ____
** Clocks: 2
*/
pub fn dec_rr(z80: &mut Z80, op: u8) {
    let mut val = match op {
        0x0B    => z80.r.get_bc(),
        0x1B    => z80.r.get_de(),
        0x2B    => z80.r.get_hl(),
        0x3B    => z80.r.sp,
        _       => 0,
    };
    if val == 0x0 {
        val = 0xFFFF;
    } else {
        val -= 1;
    }
    match op {
        0x0B    => z80.r.set_bc(val),
        0x1B    => z80.r.set_de(val),
        0x2B    => z80.r.set_hl(val),
        0x3B    => z80.r.sp = val,
        _       => (),
    }
    z80.set_register_clock(2);
}

/*
** SWAP A
** Condition Bits: R000
** Clocks:
**   (hl): 4
**   r: 2
*/
pub fn swap(z80: &mut Z80) {
    let val = z80.r.a;
    z80.r.clear_flags();
    z80.r.set_zero(val == 0);
    z80.r.a = (val & 0xF) << 4;
    z80.r.a |= (val & 0xF0) >> 4;
    z80.set_register_clock(2);
}

/*
** SCF
** Condition Bits: _001
** Clocks: 1
*/
pub fn scf(z80: &mut Z80) {
    z80.r.set_carry(true);
    z80.r.set_half_carry(false);
    z80.r.set_subtract(false);
    z80.set_register_clock(1);
}

pub fn int_handle(z80: &mut Z80, addr: u16) {
    z80.r.ime = false;
    z80.r.sp -= 2;
    z80.mmu.ww(z80.r.sp, z80.r.pc);
    z80.r.pc = addr;
    z80.set_register_clock(3);
}

pub fn rst(z80: &mut Z80, op: u8) {
    let addr = match op {
        0xC7    => 0x0000,
        0xCF    => 0x0008,
        0xD7    => 0x0010,
        0xDF    => 0x0018,
        0xE7    => 0x0020,
        0xEF    => 0x0028,
        0xF7    => 0x0030,
        0xFF    => 0x0038,
        _       => 0x0000,
    };
    z80.r.sp -= 2;
    z80.mmu.ww(z80.r.sp, z80.r.pc);
    z80.r.pc = addr;
    z80.set_register_clock(4);
}

pub fn di(z80: &mut Z80) {
    z80.r.ime = false;
    z80.set_register_clock(1);
}

pub fn ei(z80: &mut Z80) {
    z80.r.ime = true;
    z80.set_register_clock(1);
}

pub fn unimplemented_op(z80: &mut Z80, op: u8) {
    panic!(format!("Unimplemented op 0x{:X}", op))
}

pub fn unimplemented_cb(z80: &mut Z80, op: u8) {
    panic!(format!("Unimplemented op 0xCB 0x{:X}", op))
}
