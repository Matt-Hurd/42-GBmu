use z80::Z80;
use z80::helpers;

pub fn adc_a_hl(z80: &mut Z80) {
    let mut temp: u16 = z80.r.a as u16 + self.mmu.rb(self.r.get_hl()) as u16;
    temp += match (z80.r.f & 10) {
        0   => 0,
        _   => 1,
    };
    fz_one(z80, temp as u16);
    if temp > 255 {
        self.r.f |= 0x10;
    }
    temp &= 255;
    self.r.a = temp as u8;
    z80.set_register_clock(2);
}
