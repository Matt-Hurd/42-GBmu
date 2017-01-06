use z80::Z80;

//Need to check for overflow, not sure if there's a better way to check
pub fn add_r_e(z80: &mut Z80) {
    let temp: u16 = z80.r.a as u16 + z80.r.e as u16;
    z80.r.f = 0;
    if (temp & 255 as u16) != 0 {
        z80.r.f |= 0x80;
    }
    if temp > 255 {
        z80.r.f |= 0x10;
    }
    z80.r.a = temp as u8 & 255;
    z80.set_register_clock(1);
}

pub fn cp_r_b(z80: &mut Z80) {
    let temp = z80.r.a;
    z80.r.f |= 0x40;
    if ((temp - z80.r.b) & 255) != 0 {
        z80.r.f |= 0x80;
    }
    if temp < z80.r.b {
        z80.r.f |= 0x10;
    }
    z80.set_register_clock(1);
}

pub fn nop(z80: &mut Z80) {
    z80.set_register_clock(1);
}

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

pub fn lda_mm(z80: &mut Z80) {
    let addr = z80.mmu.rw(z80.r.pc);
    z80.r.pc += 2;
    z80.r.a = z80.mmu.rb(addr);
    z80.set_register_clock(4);
}

pub fn unimplemented_op(z80: &mut Z80) {

}
