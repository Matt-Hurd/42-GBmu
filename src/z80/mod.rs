mod mmu;
mod ops;
mod registers;
mod gpu;

/*
** Z80 and MMU implementation largely ported from http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU
*/

pub struct Z80Clock {
    pub m: u16,
    pub t: u16,
}

impl Default for Z80Clock {
    fn default () -> Z80Clock {
        Z80Clock {
            m: 0,
            t: 0,
        }
    }
}
pub struct Z80 {
    pub clock: Z80Clock,
    pub r: registers::Z80Registers,
    pub mmu: mmu::MMU,
}

impl Default for Z80 {
    fn default () -> Z80 {
        Z80 {
            clock: Z80Clock::default(),
            mmu: mmu::MMU::default(),
            r: registers::Z80Registers::default(),
        }
    }
}

impl Z80 {
    pub fn step(&mut self) {
        let op = self.mmu.rb(self.r.pc);
        self.r.pc += 1;
        self.do_op(op);
        self.r.pc &= 0xFFFF;
        if self.r.pc == 0x0100 {
            self.mmu.in_bios = false;
        }
        self.clock.m += self.r.m;
        self.clock.t += self.r.t;
        self.mmu.gpu.step(self.r.t);
    }

    pub fn do_cb(&mut self) {
        let op = self.mmu.rb(self.r.pc);
        println!("  Doing cb {:X}", op);
        self.r.pc += 1;
        match op {
            0x7C    => ops::bit::bit_7_h(self),
            _       => ops::misc::unimplemented_cb(self, op),
        }
    }

    pub fn do_op(&mut self, op: u8) {
        println!("Doing op {:X}", op);
        match op {
            0x00    => ops::misc::nop(self),
            0x03    => ops::misc::inc(self, op),
            0x04    => ops::misc::inc(self, op),
            0x0C    => ops::misc::inc(self, op),
            0x0E    => ops::ld::ld_rn_c(self),
            0x13    => ops::misc::inc(self, op),
            0x14    => ops::misc::inc(self, op),
            0x1C    => ops::misc::inc(self, op),
            0x20    => ops::jump::jr_nz_n(self),
            0x21    => ops::ld::ld_hl_nn(self),
            0x23    => ops::misc::inc(self, op),
            0x24    => ops::misc::inc(self, op),
            0x2C    => ops::misc::inc(self, op),
            0x31    => ops::ld::ld_sp_nn(self),
            0x32    => ops::ld::ld_hld_a(self),
            0x33    => ops::misc::inc(self, op),
            0x34    => ops::misc::inc(self, op),
            0x3C    => ops::misc::inc(self, op),
            0x3E    => ops::ld::ld_rn_a(self),
            0x83    => ops::add::add_r_e(self),
            0x8E    => ops::adc::adc_a_hl(self),
            0xAF    => ops::xor::xor_a(self),
            0xB8    => ops::cp::cp_r_b(self),
            0xCB    => self.do_cb(),
            0xC5    => ops::misc::push_bc(self),
            0xE1    => ops::misc::pop_hl(self),
            0xE2    => ops::ld::ld_io_c_a(self),
            0xFA    => ops::ld::lda_mm(self),
            0x7F    => ops::ld::ld_u8_r_r(self, op),
            0x78    => ops::ld::ld_u8_r_r(self, op),
            0x79    => ops::ld::ld_u8_r_r(self, op),
            0x7A    => ops::ld::ld_u8_r_r(self, op),
            0x7B    => ops::ld::ld_u8_r_r(self, op),
            0x7C    => ops::ld::ld_u8_r_r(self, op),
            0x7D    => ops::ld::ld_u8_r_r(self, op),
            0x46    => ops::ld::ld_u8_r_r(self, op),
            0x47    => ops::ld::ld_u8_r_r(self, op),
            0x40    => ops::ld::ld_u8_r_r(self, op),
            0x41    => ops::ld::ld_u8_r_r(self, op),
            0x42    => ops::ld::ld_u8_r_r(self, op),
            0x43    => ops::ld::ld_u8_r_r(self, op),
            0x44    => ops::ld::ld_u8_r_r(self, op),
            0x45    => ops::ld::ld_u8_r_r(self, op),
            0x4E    => ops::ld::ld_u8_r_r(self, op),
            0x4F    => ops::ld::ld_u8_r_r(self, op),
            0x48    => ops::ld::ld_u8_r_r(self, op),
            0x49    => ops::ld::ld_u8_r_r(self, op),
            0x4A    => ops::ld::ld_u8_r_r(self, op),
            0x4B    => ops::ld::ld_u8_r_r(self, op),
            0x4C    => ops::ld::ld_u8_r_r(self, op),
            0x4D    => ops::ld::ld_u8_r_r(self, op),
            0x56    => ops::ld::ld_u8_r_r(self, op),
            0x57    => ops::ld::ld_u8_r_r(self, op),
            0x50    => ops::ld::ld_u8_r_r(self, op),
            0x51    => ops::ld::ld_u8_r_r(self, op),
            0x52    => ops::ld::ld_u8_r_r(self, op),
            0x53    => ops::ld::ld_u8_r_r(self, op),
            0x54    => ops::ld::ld_u8_r_r(self, op),
            0x55    => ops::ld::ld_u8_r_r(self, op),
            0x5E    => ops::ld::ld_u8_r_r(self, op),
            0x5F    => ops::ld::ld_u8_r_r(self, op),
            0x58    => ops::ld::ld_u8_r_r(self, op),
            0x59    => ops::ld::ld_u8_r_r(self, op),
            0x5A    => ops::ld::ld_u8_r_r(self, op),
            0x5B    => ops::ld::ld_u8_r_r(self, op),
            0x5C    => ops::ld::ld_u8_r_r(self, op),
            0x5D    => ops::ld::ld_u8_r_r(self, op),
            0x66    => ops::ld::ld_u8_r_r(self, op),
            0x67    => ops::ld::ld_u8_r_r(self, op),
            0x6F    => ops::ld::ld_u8_r_r(self, op),
            0x60    => ops::ld::ld_u8_r_r(self, op),
            0x61    => ops::ld::ld_u8_r_r(self, op),
            0x62    => ops::ld::ld_u8_r_r(self, op),
            0x63    => ops::ld::ld_u8_r_r(self, op),
            0x64    => ops::ld::ld_u8_r_r(self, op),
            0x65    => ops::ld::ld_u8_r_r(self, op),
            0x6E    => ops::ld::ld_u8_r_r(self, op),
            0x68    => ops::ld::ld_u8_r_r(self, op),
            0x69    => ops::ld::ld_u8_r_r(self, op),
            0x6A    => ops::ld::ld_u8_r_r(self, op),
            0x6B    => ops::ld::ld_u8_r_r(self, op),
            0x6C    => ops::ld::ld_u8_r_r(self, op),
            0x6D    => ops::ld::ld_u8_r_r(self, op),
            0x77    => ops::ld::ld_u8_r_r(self, op),
            0x70    => ops::ld::ld_u8_r_r(self, op),
            0x71    => ops::ld::ld_u8_r_r(self, op),
            0x72    => ops::ld::ld_u8_r_r(self, op),
            0x73    => ops::ld::ld_u8_r_r(self, op),
            0x74    => ops::ld::ld_u8_r_r(self, op),

            _       => ops::misc::unimplemented_op(self, op),
        }
    }

    pub fn set_register_clock(&mut self, m: u16) {
        self.r.m = m;
        self.r.t = m * 4;
    }

    pub fn reset(&mut self) {
        self.r.a = 0;
        self.r.b = 0;
        self.r.c = 0;
        self.r.d = 0;
        self.r.e = 0;
        self.r.h = 0;
        self.r.l = 0;
        self.r.f = 0;
        self.r.sp = 0;
        self.r.pc = 0;
        self.r.m = 0;
        self.r.t = 0;
        self.clock.m = 0;
        self.clock.t = 0;
        self.mmu.in_bios = true;
        self.mmu.gpu.reset();
    }
}
