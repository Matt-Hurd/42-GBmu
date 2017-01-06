use z80::Z80;

pub fn lda_mm(z80: &mut Z80) {
    let addr = z80.mmu.rw(z80.r.pc);
    z80.r.pc += 2;
    z80.r.a = z80.mmu.rb(addr);
    z80.set_register_clock(4);
}
