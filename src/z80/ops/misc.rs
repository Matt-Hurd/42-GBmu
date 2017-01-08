use z80::Z80;

pub fn push_bc(z80: &mut Z80) {
    if z80.r.sp < 2 {
        panic!("Stack Underflow".to_string())
    }
    z80.r.sp -= 1;
    z80.mmu.wb(z80.r.sp, z80.r.b);
    z80.r.sp -= 1;
    z80.mmu.wb(z80.r.sp, z80.r.c);
    z80.set_register_clock(3);
}

pub fn pop_hl(z80: &mut Z80) {
    z80.r.l = z80.mmu.rb(z80.r.sp);
    z80.r.sp += 1;
    z80.r.h = z80.mmu.rb(z80.r.sp);
    z80.r.sp += 1;
    z80.set_register_clock(3);
}

pub fn nop(z80: &mut Z80) {
    z80.set_register_clock(1);
}

pub fn inc(z80: &mut Z80, op: u8) {
    let mut val = match op {
        0x34    => z80.mmu.rb(z80.r.get_hl()) as u16,
        0x3C    => z80.r.a as u16,
        0x04    => z80.r.b as u16,
        0x03    => z80.r.get_bc(),
        0x0C    => z80.r.c as u16,
        0x14    => z80.r.d as u16,
        0x13    => z80.r.get_de(),
        0x1C    => z80.r.e as u16,
        0x24    => z80.r.h as u16,
        0x23    => z80.r.get_hl(),
        0x2C    => z80.r.l as u16,
        0x33    => z80.r.sp,
        _       => 0,
    };
    let cmp = match op {
        0x03 | 0x13 | 0x23 | 0x33   => 0xFFFF,
        _                           => 0xFF,
    };
    if val == cmp {
        z80.r.set_zero();
        val = 0;
    } else {
        val += 1;
    }
    match op {
        0x34    => z80.mmu.wb(z80.r.get_hl(), val as u8),
        0x3C    => z80.r.a = val as u8,
        0x04    => z80.r.b = val as u8,
        0x03    => z80.r.set_bc(val),
        0x0C    => z80.r.c = val as u8,
        0x14    => z80.r.d = val as u8,
        0x13    => z80.r.set_de(val),
        0x1C    => z80.r.e = val as u8,
        0x24    => z80.r.h = val as u8,
        0x23    => z80.r.set_hl(val),
        0x2C    => z80.r.l = val as u8,
        0x33    => z80.r.sp = val,
        _       => (),
    }
    z80.set_register_clock(1);
}

pub fn unimplemented_op(z80: &mut Z80, op: u8) {
    panic!(format!("Unimplemented op 0x{:X}", op))
}

pub fn unimplemented_cb(z80: &mut Z80, op: u8) {
    panic!(format!("Unimplemented op 0xCB 0x{:X}", op))
}
