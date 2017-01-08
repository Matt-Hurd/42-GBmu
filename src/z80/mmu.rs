use mbc;
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
}

impl MMU {

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
            0x8000 | 0x9000                     => return 0, //gpu.vram[addr & 0x1FFF],
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
                            // Not handled *heh*
                            return 0;
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
            0x8000 | 0x9000                     => (), //gpu.vram[addr & 0x1FFF] = val,
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
                            // Not handled *heh*
                        }
                    }
                };
            },
        };
    }

    // Read 16-bit val at addr
    pub fn ww(&mut self, addr: u16, val: u16) {
        self.wb(addr, (val >> 8) as u8);
        self.wb(addr + 1, (val & 0xFF) as u8);
    }

    pub fn load(&mut self, filename: path::PathBuf) -> Result<String, Box<Error>> {
        File::open(&filename).and_then(|mut f| f.read_to_end(&mut self.rom))
            .map_err(|_| "Could not read ROM")?;;
        Ok("Good".to_string())
    }

    // Old rb implementation
    // // Read 8-bit byte at addr
    // pub fn rb(&mut self, addr: u16) -> u8 {
    //     let range = addr & 0xF000;
    //     if range <= 0x1000 { //bios
    //         if self.in_bios && addr < 0x0100 {
    //                 return self.bios[addr as usize];
    //         }
    //         return self.rom[addr as usize];
    //     }
    //     else if range <= 0x3000 { //rom0
    //         return self.rom[addr];
    //     }
    //     else if range <= 0x7000 { //rom1 (no banking)
    //         return self.rom[addr];
    //     }
    //     else if range <= 0x9000 { //GPU
    //         //return gpu.vram[addr & 0x1FFF]
    //     }
    //     else if range <= 0xB000 { //external ram
    //         return self.eram[addr & 0x1FFF];
    //     }
    //     else if range <= 0xD000 { //working ram
    //         return self.wram[addr & 0x1FFF];
    //     }
    //     else if range <= 0xE000 { //working ram shadow
    //         return self.wram[addr & 0x1FFF];
    //     } else { //working ram shadow, I/O, Zero-page RAM
    //         let wram_range = addr & 0x0F00;
    //         if wram_range <= 0xD00 { //wram shadow
    //             return self.wram[addr & 0x1FFF]
    //         }
    //         else if wram_range <= 0xE00 {
    // 		    // Graphics: object attribute memory
    // 		    // OAM is 160 bytes, remaining bytes read as 0
    //             if add < 0xFEA0 {
    //                 //return gpu.oam[addr & 0xFF];
    //             } else {
    //                 return 0;
    //             }
    //         } else {
    //             if add >= 0xFF80 {
    //                 return mmu.zram[addr & 0x7F];
    //             } else {
    //                 // I/O Control Handling
    //                 // Not handled *heh*
    //                 return 0;
    //             }
    //         }
    //     }
    //     panic!("Invalid memory read!".to_string());
    // }
}
