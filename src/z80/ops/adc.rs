use z80::Z80;
use z80::ops::helpers;

pub fn adc_a_hl(z80: &mut Z80) {
    let (mut sum, mut overflow) = z80.r.a.overflowing_add(z80.mmu.rb(z80.r.get_hl()));
    if overflow {
        z80.r.set_carry();
    }
    let (sum, overflow) = sum.overflowing_add(z80.r.get_carry());
    helpers::zero_flag_one(z80, sum as u16);
    if overflow {
        z80.r.set_carry();
    }
    z80.r.a = sum;
    z80.set_register_clock(2);
}