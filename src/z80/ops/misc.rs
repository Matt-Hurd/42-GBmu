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
    z80.r.clear_flags();
    if val == cmp {
        z80.r.set_zero(true);
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

pub fn dec(z80: &mut Z80, op: u8) {
    let mut val = match op {
        0x35    => z80.mmu.rb(z80.r.get_hl()) as u16,
        0x3d    => z80.r.a as u16,
        0x05    => z80.r.b as u16,
        0x0B    => z80.r.get_bc(),
        0x0D    => z80.r.c as u16,
        0x15    => z80.r.d as u16,
        0x1B    => z80.r.get_de(),
        0x1D    => z80.r.e as u16,
        0x25    => z80.r.h as u16,
        0x2B    => z80.r.get_hl(),
        0x2D    => z80.r.l as u16,
        0x3B    => z80.r.sp,
        _       => 0,
    };
    z80.r.clear_flags();
    if val == 1 {
        z80.r.set_zero(true);
        val = 0;
    }
    else if val == 0 {
        val = match op {
            0x05 | 0x15 | 0x25 | 0x35   => 0xFFFF,
            _                           => 0xFF,
        };
    } else {
        val -= 1;
    }
    match op {
        0x35    => z80.mmu.wb(z80.r.get_hl(), val as u8),
        0x3d    => z80.r.a = val as u8,
        0x05    => z80.r.b = val as u8,
        0x0B    => z80.r.set_bc(val),
        0x0D    => z80.r.c = val as u8,
        0x15    => z80.r.d = val as u8,
        0x1B    => z80.r.set_de(val),
        0x1D    => z80.r.e = val as u8,
        0x25    => z80.r.h = val as u8,
        0x2B    => z80.r.set_hl(val),
        0x2D    => z80.r.l = val as u8,
        0x3B    => z80.r.sp = val,
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
