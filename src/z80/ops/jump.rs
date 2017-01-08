use z80::Z80;

pub fn jr_nz_n(z80: &mut Z80) {
    let i = z80.mmu.rb(z80.r.pc);
    z80.r.pc += 1;
    if z80.r.get_zero() == 0 {
        if i > 127 {
            z80.r.pc -= (255 - i + 1) as u16;
        } else {
            z80.r.pc += i as u16;
        }
        z80.set_register_clock(3);
    } else {
        z80.set_register_clock(2);
    }
}
