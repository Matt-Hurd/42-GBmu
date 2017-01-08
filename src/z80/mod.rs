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

    pub fn do_op(&mut self, op: u8) {
        match op {
            0x00    => ops::misc::nop(self),
            0x83    => ops::add::add_r_e(self),
            0x8E    => ops::adc::adc_a_hl(self),
            0xB8    => ops::cp::cp_r_b(self),
            0xC5    => ops::misc::push_bc(self),
            0xE1    => ops::misc::pop_hl(self),
            0xFA    => ops::ld::lda_mm(self),
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
