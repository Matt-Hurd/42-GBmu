mod mmu;

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
    pub m: u8,
    pub t: u8,
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
    }

    pub fn do_op(&mut self, op: u8) {
        match op {
            0x00    => self.nop(),
            0x83    => self.add_r_e(),
            0xB8    => self.cp_r_b(),
            0xC5    => self.push_bc(),
            0xE1    => self.pop_hl(),
            0xFA    => self.lda_mm(),
            _       => self.unimplemented_op(),
        }
    }

    pub fn set_register_clock(&mut self, m: u8) {
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

    //Need to check for overflow, not sure if there's a better way to check
    pub fn add_r_e(&mut self) {
        let temp: u16 = self.r.a as u16 + self.r.e as u16;
        self.r.f = 0;
        if (temp & 255 as u16) != 0 {
            self.r.f |= 0x80;
        }
        if temp > 255 {
            self.r.f |= 0x10;
        }
        self.r.a = temp as u8 & 255;
        self.set_register_clock(1);
    }

    pub fn cp_r_b(&mut self) {
        let temp = self.r.a;
        self.r.f |= 0x40;
        if ((temp - self.r.b) & 255) != 0 {
            self.r.f |= 0x80;
        }
        if temp < self.r.b {
            self.r.f |= 0x10;
        }
        self.set_register_clock(1);
    }

    pub fn nop(&mut self) {
        self.set_register_clock(1);
    }

    pub fn push_bc(&mut self) {
        if self.r.sp < 2 {
            panic!("Stack Underflow".to_string())
        }
        self.r.sp -= 1;
        self.mmu.wb(self.r.sp, self.r.b);
        self.r.sp -= 1;
        self.mmu.wb(self.r.sp, self.r.c);
        self.set_register_clock(3);
    }

    pub fn pop_hl(&mut self) {
        self.r.l = self.mmu.rb(self.r.sp);
        self.r.sp += 1;
        self.r.h = self.mmu.rb(self.r.sp);
        self.r.sp += 1;
        self.set_register_clock(3);
    }

    pub fn lda_mm(&mut self) {
        let addr = self.mmu.rw(self.r.pc);
        self.r.pc += 2;
        self.r.a = self.mmu.rb(addr);
        self.set_register_clock(4);
    }

    pub fn unimplemented_op(&mut self) {

    }
}
