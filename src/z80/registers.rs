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

    pub fn set_zero(&mut self) {
        self.f |= 0x80;
    }

    pub fn set_subtract(&mut self) {
        self.f |= 0x40;
    }

    pub fn set_half_carry(&mut self) {
        self.f |= 0x20;
    }

    pub fn set_carry(&mut self) {
        self.f |= 0x10;
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
