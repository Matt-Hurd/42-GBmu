use z80::Z80;

pub fn lda_mm(z80: &mut Z80) {
    let addr = z80.mmu.rw(z80.r.pc);
    z80.r.pc += 2;
    z80.r.a = z80.mmu.rb(addr);
    z80.set_register_clock(4);
}

pub fn ld_sp_nn(z80: &mut Z80) {
    z80.r.sp = z80.mmu.rw(z80.r.pc);
    z80.r.pc += 2;
    z80.set_register_clock(3);
}

pub fn ld_hl_nn(z80: &mut Z80) {
    let val = z80.r.pc;
    z80.r.set_hl(z80.mmu.rw(val));
    z80.r.pc += 2;
    z80.set_register_clock(3);
}

// load from register into register (or value hl is pointing to)
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
    // I love finding bit magic
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
}

pub fn ld_hld_a(z80: &mut Z80) {
    let addr = z80.r.get_hl();
    let val = z80.r.a;
    z80.mmu.wb(addr, val);
    z80.r.set_hl(addr - 1);
    z80.set_register_clock(2);
}

pub fn ld_rn_c(z80: &mut Z80) {
    z80.r.c = z80.mmu.rb(z80.r.pc);
    z80.r.pc += 1;
    z80.set_register_clock(2);
}

pub fn ld_rn_a(z80: &mut Z80) {
    z80.r.a = z80.mmu.rb(z80.r.pc);
    z80.r.pc += 1;
    z80.set_register_clock(2);
}

pub fn ld_io_c_a(z80: &mut Z80) {
    z80.mmu.wb(0xFF00 + z80.r.c as u16, z80.r.a);
    z80.set_register_clock(2);
}
