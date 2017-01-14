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
    pub hram: Vec<u8>,
    pub keys: [u8; 2],
    pub column: u8,
    pub iflags: u8,
    pub ienable: u8,
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
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3c, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50
  ],
            rom: vec![],
            wram: vec![0; 8192],
            eram: vec![0; 32768],
            hram: vec![0; 128],
            keys: [0xF; 2],
            column: 0,
            iflags: 0,
            ienable: 0,
            gpu: gpu::GPU::default(),
        }
    }
}

impl MMU {
    pub fn debug_print(&mut self) {
        println!("  if: {:08b}", self.iflags);
        println!("  ie: {:08b}", self.ienable);
    }

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
            self.hram[i] = 0;
        }
    }

    pub fn rb(&mut self, addr: u16) -> u8 {
        match addr {
            //BIOS
            0x0000 ... 0x00FF  => {
                if self.in_bios {
                    return self.bios[addr as usize];
                } else {
                    return self.rom[addr as usize];
                }
            },
            //ROM0
            0x00FF ... 0x3FFF   => return self.rom[addr as usize],
            //ROM1 (unbanked)
            0x4000 ... 0x7FFF   => return self.rom[addr as usize],
            //VRAM
            0x8000 ... 0x9FFF   => return self.gpu.rb(addr & 0x1FFF),
            //External RAM
            0xA000 ... 0xBFFF   => return self.eram[(addr & 0x1FFF) as usize],
            //Working RAM
            0xC000 ... 0xDFFF   => return self.wram[(addr & 0x1FFF) as usize],
            //Working RAM Shadow
            0xE000 ... 0xFDFF   => return self.wram[(addr & 0x1FFF) as usize],
            // OAM
            0xFE00 ... 0xFE9F   => return self.gpu.roam((addr & 0xFF) as u8),
            0xFEA0 ... 0xFEFF   => return 0,
            //I/O
            0xFF00              => {
                match self.column {
                    0x10 => return self.keys[0],
                    0x20 => return self.keys[1],
                    _    => return 0xF,
                };
            },
            //More I/O?
            0xFF0F              => return self.iflags,
            //Serial Transfer (Used for Blargg's tests)
            0xFF01              => return 0,
            0xFF02 ... 0xFF7F   => return self.gpu.rb(addr),
            //HRAM
            0xFF80 ... 0xFFFE   => return self.hram[(addr & 0x7F) as usize],
            0xFFFF              => return self.ienable,
            _                   => return 0,
        };
    }

    // Read 16-bit word at addr
    pub fn rw(&mut self, addr: u16) -> u16 {
        return self.rb(addr) as u16 + ((self.rb(addr + 1) as u16) << 8);
    }

    // Write 8-bit val at addr
    // Pretty much a copy of rb, should merge implementations
    pub fn wb(&mut self, addr: u16, val: u8) {
        match addr {
            //BIOS
            0x0000 ... 0x00FF  => {
                if self.in_bios {
                    self.bios[addr as usize] = val;
                } else {
                    self.rom[addr as usize] = val;
                }
            },
            //ROM0
            0x00FF ... 0x3FFF   => self.rom[addr as usize] = val,
            //ROM1 (unbanked)
            0x4000 ... 0x7FFF   => self.rom[addr as usize] = val,
            //VRAM
            0x8000 ... 0x9FFF   => self.gpu.wb(addr & 0x1FFF, val),
            //External RAM
            0xA000 ... 0xBFFF   => self.eram[(addr & 0x1FFF) as usize] = val,
            //Working RAM
            0xC000 ... 0xDFFF   => self.wram[(addr & 0x1FFF) as usize] = val,
            //Working RAM Shadow
            0xE000 ... 0xFDFF   => self.wram[(addr & 0x1FFF) as usize] = val,
            0xFE00 ... 0xFE9F   => self.gpu.woam((addr & 0xFF) as u8, val),
            0xFEA0 ... 0xFEFF   => (),
            //I/O
            0xFF00              => self.column = val & 0b00110000,
            //Serial Transfer (Used for Blargg's tests)
            0xFF01              => print!("{}", val as char),
            0xFF0F              => self.iflags = val,
            //Sound I/O
            0xFF10 ... 0xFF3F   => (),
            0xFF40 ... 0xFF7F   => self.gpu.wb(addr, val),
            //HRAM
            0xFF80 ... 0xFFFE   => self.hram[(addr & 0x7F) as usize] = val,
            0xFFFF              => {self.ienable = val; println!("Enabling interrupts {:05b}", val)},
            _                   => (),
        };
    }

    // Read 16-bit val at addr
    pub fn ww(&mut self, addr: u16, val: u16) {
        self.wb(addr, (val & 0xFF) as u8);
        self.wb(addr + 1, ((val & 0xFF00) >> 8) as u8);
    }

    pub fn load(&mut self, filename: path::PathBuf) -> Result<String, Box<Error>> {
        File::open(&filename).and_then(|mut f| f.read_to_end(&mut self.rom))
            .map_err(|_| "Could not read ROM")?;
        Ok("Good".to_string())
    }
}
