mod mmu;
mod ops;

/*
** Z80 and MMU implementation largely ported from http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU
*/

pub struct Z80Clock {
    pub m: u16,
    pub t: u16,
}

pub struct Z80Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8,
    pub pc: u16,
    pub sp: u16,
    pub m: u16,
    pub t: u16,
}

impl Z80Registers {
    pub fn get_hl(&mut self) -> u16 {
        (self.h as u16) << 8 + (self.l as u16)
    }

    pub fn get_af(&mut self) -> u16 {
        (self.a as u16) << 8 + (self.f as u16)
    }

    pub fn get_bc(&mut self) -> u16 {
        (self.b as u16) << 8 + (self.c as u16)
    }

    pub fn get_de(&mut self) -> u16 {
        (self.d as u16) << 8 + (self.e as u16)
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 255) as u8;
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 255) as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 255) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 255) as u8;
    }
}

pub struct Z80 {
    pub clock: Z80Clock,
    pub r: Z80Registers,
    pub mmu: mmu::MMU,
}

impl Z80 {
    pub fn step(&mut self) {
        let op = self.mmu.rb(self.r.pc);
        self.r.pc += 1;
        self.do_op(op);
        self.r.pc &= 65535;
        self.clock.m += self.r.m;
        self.clock.t += self.r.t;
    }

    pub fn do_op(&mut self, op: u8) {
        match op {
            0x00    => ops::misc::nop(self),
            0x83    => ops::add::add_r_e(self),
            0xB8    => ops::cp::cp_r_b(self),
            0xC5    => ops::misc::push_bc(self),
            0xE1    => ops::misc::pop_hl(self),
            0xFA    => ops::ld::lda_mm(self),
            _       => ops::misc::unimplemented_op(self),
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
    }
}
