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
    pub ime: u8,
}

impl Default for Z80Registers {
    fn default () -> Z80Registers {
        Z80Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            pc: 0,
            sp: 0,
            m: 0,
            t: 0,
            ime: 0,
        }
    }
}

impl Z80Registers {
    pub fn debug_print(&mut self) {
        println!("  a: 0x{:02X}       pc: {:#4X}", self.a,  self.pc);
        println!("  b: 0x{:02X}       sp: {:#4X}", self.b,  self.sp);
        println!("  c: 0x{:02X}       m:  {}",      self.c,  self.m);
        println!("  d: 0x{:02X}       t: {}",      self.d,  self.t);
        println!("  e: 0x{:02X}       ime: {}",    self.e,  self.ime);
        let hl = self.get_hl();
        println!("  h: 0x{:02X}       hl: {:#X}", self.h,  hl);
        let bc = self.get_bc();
        println!("  l: 0x{:02X}       bc: {:#X}", self.l,  bc);
        let de = self.get_de();
        println!("  f: 0x{:02X}       de: {:#X}", self.f,  de);

    }

    pub fn get_hl(&mut self) -> u16 {
        ((self.h as u16) << 8) + (self.l as u16)
    }

    pub fn get_bc(&mut self) -> u16 {
        ((self.b as u16) << 8) + (self.c as u16)
    }

    pub fn get_de(&mut self) -> u16 {
        ((self.d as u16) << 8) + (self.e as u16)
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

    pub fn set_zero(&mut self, val: bool) {
        if val {
            self.f |= 0x80;
        } else {
            self.f &= 0xFF ^ 0x80;
        }
    }

    pub fn set_subtract(&mut self, val: bool) {
        if val {
            self.f |= 0x40;
        } else {
            self.f &= 0xFF ^ 0x40;
        }
    }

    pub fn set_half_carry(&mut self, val: bool) {
        if val {
            self.f |= 0x20;
        } else {
            self.f &= 0xFF ^ 0x20;
        }
    }

    pub fn set_carry(&mut self, val: bool) {
        if val {
            self.f |= 0x10;
        } else {
            self.f &= 0xFF ^ 0x10;
        }
    }

    pub fn clear_flags(&mut self) {
        self.f = 0;
    }

    pub fn get_zero(&mut self) -> u8 {
        match self.f & 0x80 {
            0   => 0,
            _   => 1
        }
    }

    pub fn get_subtract(&mut self) -> u8 {
        match self.f & 0x40 {
            0   => 0,
            _   => 1
        }
    }

    pub fn get_half_carry(&mut self) -> u8 {
        match self.f & 0x20 {
            0   => 0,
            _   => 1
        }
    }

    pub fn get_carry(&mut self) -> u8 {
        match self.f & 0x10 {
            0   => 0,
            _   => 1
        }
    }
}
