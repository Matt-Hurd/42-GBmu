use z80::Z80;

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
    z80.set_register_clock(1);
}

pub fn ld_i_on_a(z80: &mut Z80) {
    let addr = z80.mmu.rb(z80.r.pc) as u16;
    z80.mmu.wb(0xFF00 + addr, z80.r.a);
    z80.r.pc += 1;
    z80.set_register_clock(2);
}

/*
** LD (16bits), (16bits)
*/
pub fn ld_u16(z80: &mut Z80, op: u8) {
    let val = match op & 0xF {
        0x1     => {
            z80.r.pc += 2;
            z80.mmu.rw(z80.r.pc - 2)
        },
        0x8     => z80.r.sp,
        0x9     => z80.r.get_hl(),
        _       => 0,
    };
    match op {
        0x01        => z80.r.set_bc(val),
        0x08        => {
            let addr = z80.mmu.rw(z80.r.pc);
            z80.r.pc += 2;
            z80.mmu.ww(addr, val);
        },
        0x11        => z80.r.set_de(val),
        0x21 | 0xF8 => z80.r.set_hl(val),
        0x31 | 0x49 => z80.r.sp = val,
        _           => (),
    };
    z80.set_register_clock(3);
}

/*
** LD A, pointer
*/
pub fn ld_a_p(z80: &mut Z80, op: u8) {
    let val = match op {
        0x0A        => z80.mmu.rb(z80.r.get_bc()),
        0x1A        => z80.mmu.rb(z80.r.get_de()),
        0x2A | 0x3A => z80.mmu.rb(z80.r.get_hl()),
        0xFA        => {
            let addr = z80.mmu.rw(z80.r.pc);
            z80.r.pc += 2;
            z80.mmu.rb(addr)
        },
        0xF0        => {
            let addr = 0xFF00 + z80.mmu.rb(z80.r.pc) as u16;
            z80.r.pc += 1;
            z80.mmu.rb(addr)
        },
        0xF2        => z80.mmu.rb(0xFF00 + z80.r.c as u16),
        _           => 0,
    };
    let hl = z80.r.get_hl();
    if op == 0x20 {
        z80.r.set_hl(hl + 1);
    }
    else if op == 0x30 {
        z80.r.set_hl(hl - 1);
    }
    z80.r.a = val;
    z80.set_register_clock(2);
}

/*
** LD pointer, A
*/
pub fn ld_p_a(z80: &mut Z80, op: u8) {
    match op {
        0x02        => z80.mmu.wb(z80.r.get_bc(), z80.r.a),
        0x12        => z80.mmu.wb(z80.r.get_de(), z80.r.a),
        0x22 | 0x32 => z80.mmu.wb(z80.r.get_hl(), z80.r.a),
        0xE2        => z80.mmu.wb(0xFF00 + z80.r.c as u16, z80.r.a),
        _           => (),
    };
    if op == 0x22 {
        if z80.r.l == 0xFF {
            z80.r.h += 1;
            z80.r.l = 0;
        } else {
            z80.r.l += 1;
        }
    }
    else if op == 0x32 {
        if z80.r.l == 0x00 {
            z80.r.h -= 1;
            z80.r.l = 0xFF;
        } else {
            z80.r.l -= 1;
        }
    }
    z80.set_register_clock(2);
}

/*
** LD r, $xx
*/
pub fn ld_r_xx(z80: &mut Z80, op: u8) {
    let val = z80.mmu.rb(z80.r.pc);
    z80.r.pc += 1;
    match op {
        0x06       => z80.r.b = val,
        0x0E       => z80.r.c = val,
        0x16       => z80.r.d = val,
        0x1E       => z80.r.e = val,
        0x26       => z80.r.h = val,
        0x2E       => z80.r.l = val,
        0x3E       => z80.r.a = val,
        0x36       => {
            let addr = z80.r.get_hl();
            z80.mmu.wb(addr, val);
        },
        _          => (),
    }
    z80.set_register_clock(2);
}

/*
** LD ($aabb), A
*/
pub fn ld_aabb_a(z80: &mut Z80, op: u8) {
    let dest = z80.mmu.rw(z80.r.pc);
    z80.r.pc += 2;
    z80.mmu.wb(dest, z80.r.a);
    z80.set_register_clock(4);
}

/*
** LD ($aabb), SP
*/
pub fn ld_aabb_sp(z80: &mut Z80, op: u8) {
    let dest = z80.mmu.rw(z80.r.pc);
    z80.r.pc += 2;
    z80.mmu.ww(dest, z80.r.get_hl());
    z80.set_register_clock(5);
}
