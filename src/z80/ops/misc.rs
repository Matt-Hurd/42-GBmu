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

pub fn unimplemented_op(z80: &mut Z80, op: u8) {
    panic!(format!("Unimplemented op 0x{:x}", op))
}
