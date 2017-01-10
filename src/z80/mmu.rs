use z80::gpu;
// use mbc;
use std::path;
use std::fs::File;
use std::io::Read;
use std::error::Error;


/*
** Z80 and MMU implementation largely ported from http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU
** I'm not sure how I feel about the struct layout, probably will refactor later
*/

pub struct MMU {
    pub in_bios: bool,
    pub bios: Vec<u8>,
    pub rom: Vec<u8>,
    pub wram: Vec<u8>,
    pub eram: Vec<u8>,
    pub zram: Vec<u8>,
    pub gpu: gpu::GPU,
}

impl Default for MMU {
    fn default () -> MMU {
        MMU {
            in_bios: true,
            bios: vec![
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
    0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
    0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
    0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
    0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
    0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
    0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xF2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
    0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
    0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3c, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x4C,
    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50
  ],
            rom: vec![],
            wram: vec![0; 8192],
            eram: vec![0; 32768],
            zram: vec![0; 127],
            gpu: gpu::GPU::default(),
        }
    }
}

impl MMU {

    pub fn reset(&mut self) {
        self.in_bios = true;
        self.rom = vec![];
        for i in 0 .. 8192 {
            self.wram[i] = 0;
        }
        for i in 0 .. 32768 {
            self.eram[i] = 0;
        }
        for i in 0 .. 127 {
            self.zram[i] = 0;
        }
    }
    // Read 8-bit byte at addr
    pub fn rb(&mut self, addr: u16) -> u8 {
        match addr & 0xF000 {
            //BIOS
            0x0000                              => {
                if self.in_bios && addr < 0x0100 {
                    return self.bios[addr as usize];
                } else {
                    return self.rom[addr as usize];
                }
            },
            //ROM0
            0x1000 | 0x2000 | 0x3000            => return self.rom[addr as usize],
            //ROM1 (unbanked)
            0x4000 | 0x5000 | 0x6000 | 0x7000   => return self.rom[addr as usize],
            //VRAM
            0x8000 | 0x9000                     => return self.gpu.rb(addr & 0x1FFF),
            //External RAM
            0xA000 | 0xB000                     => return self.eram[(addr & 0x1FFF) as usize],
            //Working RAM
            0xC000 | 0xD000                     => return self.wram[(addr & 0x1FFF) as usize],
            //Working RAM Shadow
            0xE000                              => return self.wram[(addr & 0x1FFF) as usize],
            // Working RAM shadow, I/O, Zero-page RAM
            _                                   => {
                match addr & 0x0F00 {
                    0xE00   => {
                        if addr < 0xFEA0 {
                            return 0; //gpu.oam[addr & 0xFF]
                        } else {
                            return 0;
                        }
                    },
                    _   => {
                        if addr >= 0xFF80 {
                            return self.zram[(addr & 0x7F) as usize];
                        } else {
                            // I/O Control Handling
                            match addr & 0x00F0 {
                                0x40 | 0x50 | 0x60 | 0x70   => {
                                    return self.gpu.rb(addr);
                                },
                                // Not handled *heh*
                                _                           => return 0,
                            }
                        }
                    }
                };
            },
        };
    }
    // Read 16-bit word at addr
    pub fn rw(&mut self, addr: u16) -> u16 {
        return self.rb(addr) as u16 + ((self.rb(addr + 1) as u16) << 8);
    }

    // Write 8-bit val at addr
    // Pretty much a copy of rb, should merge implementations
    pub fn wb(&mut self, addr: u16, val: u8) {
        match addr & 0xF000 {
            //BIOS
            0x0000                              => {
                if self.in_bios && addr < 0x0100 {
                    self.bios[addr as usize] = val;
                } else {
                    self.rom[addr as usize] = val;
                }
            },
            //ROM0
            0x1000 | 0x2000 | 0x3000            => self.rom[addr as usize] = val,
            //ROM1 (unbanked)
            0x4000 | 0x5000 | 0x6000 | 0x7000   => self.rom[addr as usize] = val,
            //VRAM
            0x8000 | 0x9000                     => {
                self.gpu.wb(addr & 0x1FFF, val);
            },
            //External RAM
            0xA000 | 0xB000                     => self.eram[(addr & 0x1FFF) as usize] = val,
            //Working RAM
            0xC000 | 0xD000                     => self.wram[(addr & 0x1FFF) as usize] = val,
            //Working RAM Shadow
            0xE000                              => self.wram[(addr & 0x1FFF) as usize] = val,
            // Working RAM shadow, I/O, Zero-page RAM
            _                                   => {
                match addr & 0x0F00 {
                    0xE00   => {
                        if addr < 0xFEA0 {
                            //gpu.oam[addr & 0xFF]
                        } else {
                        }
                    },
                    _   => {
                        if addr >= 0xFF80 {
                            self.zram[(addr & 0x7F) as usize] = val;
                        } else {
                            // I/O Control Handling
                            match addr & 0x00F0 {
                                0x40 | 0x50 | 0x60 | 0x70   => {
                                    self.gpu.wb(addr, val);
                                },
                                // Not handled *heh*
                                _                           => (),
                            }
                        }
                    }
                };
            },
        };
    }

    // Read 16-bit val at addr
    pub fn ww(&mut self, addr: u16, val: u16) {
        self.wb(addr, (val & 0xFF) as u8);
        self.wb(addr + 1, ((val & 0xFF00) >> 8) as u8);
    }

    pub fn load(&mut self, filename: path::PathBuf) -> Result<String, Box<Error>> {
        File::open(&filename).and_then(|mut f| f.read_to_end(&mut self.rom))
            .map_err(|_| "Could not read ROM")?;;
        Ok("Good".to_string())
    }
}
