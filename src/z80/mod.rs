use std::num::Wrapping;

mod mmu;
mod ops;
mod registers;
mod gpu;
mod debug;

/*
** Z80 and MMU implementation largely ported from http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU
*/

pub struct Z80Clock {
    pub m: u32,
    pub t: u32,
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
    pub debug: bool,
    pub debug_r: bool,
}

impl Default for Z80 {
    fn default () -> Z80 {
        Z80 {
            clock: Z80Clock::default(),
            mmu: mmu::MMU::default(),
            r: registers::Z80Registers::default(),
            debug: false,
            debug_r: false,
        }
    }
}

impl Z80 {
    pub fn debug_print_cpu_time(&mut self) {
        println!("CPU mtime: {}", self.clock.m);
    }
    pub fn debug_print_stack(&mut self) {
        if self.r.sp != 0xFFFE {
            println!("  Stack Values:");
            for x in self.r.sp .. 0xFFFE {
                println!("      0x{:04X}: 0x{:02X}", x, self.mmu.rb(x));
            }
        }
    }

    pub fn step(&mut self) {
        let op = self.mmu.rb(self.r.pc);
        self.r.pc += 1;
        self.do_op(op);
        self.r.pc &= 0xFFFF;
        if self.r.pc == 0x0100 {
            self.mmu.in_bios = false;
        }
        self.clock.m = (Wrapping(self.clock.m) + Wrapping(self.r.m as u32)).0;
        self.clock.t = (Wrapping(self.clock.t) + Wrapping(self.r.t as u32)).0;
        self.mmu.gpu.step(self.r.m);
    }

    pub fn do_cb(&mut self) {
        let op = self.mmu.rb(self.r.pc);
        if self.debug {
            debug::translate_cb(op, self.r.pc);
        }
        self.r.pc += 1;
        match op {
            0x7C    => ops::bit::bit_7_h(self),
            0x16 => ops::rotate::rl_r(self, op),
            0x17 => ops::rotate::rl_r(self, op),
            0x10 => ops::rotate::rl_r(self, op),
            0x11 => ops::rotate::rl_r(self, op),
            0x12 => ops::rotate::rl_r(self, op),
            0x13 => ops::rotate::rl_r(self, op),
            0x14 => ops::rotate::rl_r(self, op),
            0x15 => ops::rotate::rl_r(self, op),
            0x06 => ops::rotate::rl_r(self, op),
            0x07 => ops::rotate::rl_r(self, op),
            0x00 => ops::rotate::rl_r(self, op),
            0x01 => ops::rotate::rl_r(self, op),
            0x02 => ops::rotate::rl_r(self, op),
            0x03 => ops::rotate::rl_r(self, op),
            0x04 => ops::rotate::rl_r(self, op),
            0x05 => ops::rotate::rl_r(self, op),
            _       => ops::misc::unimplemented_cb(self, op),
        }
        if self.debug_r {
            self.r.debug_print();
            self.debug_print_cpu_time();
            self.debug_print_stack();
            println!("");
        }
    }

    pub fn do_op(&mut self, op: u8) {
        if op != 0xCB && self.debug {
            debug::translate_op(op, self.r.pc, self);
        }
        match op {
            0x00    => ops::misc::nop(self),
            0x03    => ops::misc::inc(self, op),
            0x04    => ops::misc::inc(self, op),
            0x0C    => ops::misc::inc(self, op),
            0x13    => ops::misc::inc(self, op),
            0x14    => ops::misc::inc(self, op),
            0x1C    => ops::misc::inc(self, op),
            0x23    => ops::misc::inc(self, op),
            0x24    => ops::misc::inc(self, op),
            0x2C    => ops::misc::inc(self, op),
            0x33    => ops::misc::inc(self, op),
            0x34    => ops::misc::inc(self, op),
            0x3C    => ops::misc::inc(self, op),
            0x8E    => ops::adc::adc_a_hl(self),
            0xAF    => ops::xor::xor_a(self),
            0xCB    => self.do_cb(),
            0xC5    => ops::misc::push_bc(self),
            0xF1    => ops::misc::pop_u16(self, op),
            0xC1    => ops::misc::pop_u16(self, op),
            0xD1    => ops::misc::pop_u16(self, op),
            0xE1    => ops::misc::pop_u16(self, op),
            0xE0    => ops::ld::ld_i_on_a(self),
            0x02    => ops::ld::ld_p_a(self, op),
            0x12    => ops::ld::ld_p_a(self, op),
            0x22    => ops::ld::ld_p_a(self, op),
            0x32    => ops::ld::ld_p_a(self, op),
            0xE2    => ops::ld::ld_p_a(self, op),
            0xCD    => ops::call::call(self, op),
            0xDC    => ops::call::call(self, op),
            0xD4    => ops::call::call(self, op),
            0xC4    => ops::call::call(self, op),
            0xCC    => ops::call::call(self, op),
            0x01    => ops::ld::ld_u16(self, op),
            0x11    => ops::ld::ld_u16(self, op),
            0x21    => ops::ld::ld_u16(self, op),
            0x31    => ops::ld::ld_u16(self, op),
            0xF8    => ops::ld::ld_u16(self, op),
            0xF9    => ops::ld::ld_u16(self, op),
            0xFA    => ops::ld::ld_a_p(self, op),
            0xF0    => ops::ld::ld_a_p(self, op),
            0xF2    => ops::ld::ld_a_p(self, op),
            0x0A    => ops::ld::ld_a_p(self, op),
            0x1A    => ops::ld::ld_a_p(self, op),
            0x2A    => ops::ld::ld_a_p(self, op),
            0x3A    => ops::ld::ld_a_p(self, op),
            0x06    => ops::ld::ld_r_xx(self, op),
            0x0E    => ops::ld::ld_r_xx(self, op),
            0x16    => ops::ld::ld_r_xx(self, op),
            0x1E    => ops::ld::ld_r_xx(self, op),
            0x26    => ops::ld::ld_r_xx(self, op),
            0x2E    => ops::ld::ld_r_xx(self, op),
            0x36    => ops::ld::ld_r_xx(self, op),
            0x3E    => ops::ld::ld_r_xx(self, op),
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
            0x17    => ops::rotate::rl_r(self, op),
            0x35    => ops::misc::dec(self, op),
            0x3d    => ops::misc::dec(self, op),
            0x05    => ops::misc::dec(self, op),
            0x0B    => ops::misc::dec(self, op),
            0x0D    => ops::misc::dec(self, op),
            0x15    => ops::misc::dec(self, op),
            0x1B    => ops::misc::dec(self, op),
            0x1D    => ops::misc::dec(self, op),
            0x25    => ops::misc::dec(self, op),
            0x2B    => ops::misc::dec(self, op),
            0x2D    => ops::misc::dec(self, op),
            0x3B    => ops::misc::dec(self, op),
            0xD8    => ops::ret::ret(self, op),
            0xD0    => ops::ret::ret(self, op),
            0xC0    => ops::ret::ret(self, op),
            0xC8    => ops::ret::ret(self, op),
            0xD9    => ops::ret::ret(self, op),
            0xC9    => ops::ret::ret(self, op),
            0xFE    => ops::cp::cp_r(self, op),
            0xBE    => ops::cp::cp_r(self, op),
            0xBF    => ops::cp::cp_r(self, op),
            0xB8    => ops::cp::cp_r(self, op),
            0xB9    => ops::cp::cp_r(self, op),
            0xBA    => ops::cp::cp_r(self, op),
            0xBB    => ops::cp::cp_r(self, op),
            0xBC    => ops::cp::cp_r(self, op),
            0xBD    => ops::cp::cp_r(self, op),
            0xEA    => ops::ld::ld_aabb_a(self, op),
            0x08    => ops::ld::ld_aabb_sp(self, op),
            0x18    => ops::jump::jr_u8(self, op),
            0x38    => ops::jump::jr_u8(self, op),
            0x30    => ops::jump::jr_u8(self, op),
            0x20    => ops::jump::jr_u8(self, op),
            0x28    => ops::jump::jr_u8(self, op),
            0xDE    => ops::sbc::sbc(self, op),
            0x9E    => ops::sbc::sbc(self, op),
            0x9F    => ops::sbc::sbc(self, op),
            0x98    => ops::sbc::sbc(self, op),
            0x99    => ops::sbc::sbc(self, op),
            0x9A    => ops::sbc::sbc(self, op),
            0x9B    => ops::sbc::sbc(self, op),
            0x9C    => ops::sbc::sbc(self, op),
            0x9D    => ops::sbc::sbc(self, op),
            0xF3    => ops::misc::di(self),
            0xFB    => ops::misc::ei(self),
            0xD6    => ops::sub::sub(self, op),
            0x97    => ops::sub::sub(self, op),
            0x90    => ops::sub::sub(self, op),
            0x91    => ops::sub::sub(self, op),
            0x92    => ops::sub::sub(self, op),
            0x93    => ops::sub::sub(self, op),
            0x94    => ops::sub::sub(self, op),
            0x95    => ops::sub::sub(self, op),
            0xC6    => ops::add::add_a(self, op),
            0x86    => ops::add::add_a(self, op),
            0x87    => ops::add::add_a(self, op),
            0x80    => ops::add::add_a(self, op),
            0x81    => ops::add::add_a(self, op),
            0x82    => ops::add::add_a(self, op),
            0x83    => ops::add::add_a(self, op),
            0x84    => ops::add::add_a(self, op),
            0x85    => ops::add::add_a(self, op),
            _       => ops::misc::unimplemented_op(self, op),
        }
        if op != 0xCB && self.debug_r {
            self.r.debug_print();
            self.debug_print_cpu_time();
            self.debug_print_stack();
            println!("");
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
        self.r.sp = 0xFFFF;
        self.r.pc = 0;
        self.r.m = 0;
        self.r.t = 0;
        self.clock.m = 0;
        self.clock.t = 0;
        self.mmu.in_bios = true;
        self.mmu.gpu.reset();
    }
}
