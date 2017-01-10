use std::collections::VecDeque;

pub struct LCDC {
    pub lcd_enable: bool,
    pub window_tile_map_address: bool,
    pub window_enable: bool,
    pub bg_window_tile_data: bool,
    pub bg_tile_map_address: bool,
    pub obj_size: bool,
    pub obj_enable: bool,
    pub bg_enable: bool,
    pub val: u8,
}

impl Default for LCDC {
    fn default () -> LCDC {
        LCDC {
            lcd_enable: false,
            window_tile_map_address: false,
            window_enable: false,
            bg_window_tile_data: false,
            bg_tile_map_address: false,
            obj_size: false,
            obj_enable: false,
            bg_enable: false,
            val: 0,
        }
    }
}

impl LCDC {
    pub fn set(&mut self, val: u8) {
        self.lcd_enable =              if val & 0x80 != 0 { true } else { false };
        self.window_tile_map_address = if val & 0x40 != 0 { true } else { false };
        self.window_enable =           if val & 0x20 != 0 { true } else { false };
        self.bg_window_tile_data =     if val & 0x10 != 0 { true } else { false };
        self.bg_tile_map_address =     if val & 0x08 != 0 { true } else { false };
        self.obj_size =                if val & 0x04 != 0 { true } else { false };
        self.obj_enable =              if val & 0x02 != 0 { true } else { false };
        self.bg_enable =               if val & 0x01 != 0 { true } else { false };
        self.val = val;
    }
}

pub struct STAT {
    pub ly_interrupt: bool,
    pub mode_2_oam_interrupt: bool,
    pub mode_1_vblank_interrupt: bool,
    pub mode_0_hblank_interrupt: bool,
    pub ly_flag: bool,
    pub mode: u8,
    pub val: u8,
}

impl Default for STAT {
    fn default () -> STAT {
        STAT {
            ly_interrupt: false,
            mode_2_oam_interrupt: false,
            mode_1_vblank_interrupt: false,
            mode_0_hblank_interrupt: false,
            ly_flag: false,
            mode: 2,
            val: 0,
        }
    }
}

impl STAT {
    pub fn set(&mut self, val: u8) {
        self.ly_interrupt =             if val & 0x40 != 0 { true } else { false };
        self.mode_2_oam_interrupt =     if val & 0x20 != 0 { true } else { false };
        self.mode_1_vblank_interrupt =  if val & 0x10 != 0 { true } else { false };
        self.mode_0_hblank_interrupt =  if val & 0x08 != 0 { true } else { false };
        self.ly_flag =                  if val & 0x04 != 0 { true } else { false };
        self.mode =                     val & 0x3;
        self.val  =                     val;
    }
}

pub struct GPU {
    pub screen: [u32; 160 * 144],
    pub tiles: [[u8; 16]; 384],
    pub map: [[u8; 32]; 64],
    pub lcdc: LCDC,
    pub stat: STAT,
    pub scy: u8,
    pub scx: u8,
    pub ly: u8,
    pub lyc: u8,
    pub dma: u8,
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub wy: u8,
    pub wx: u8,
    pub mode_clock: u16,
    pub debug: bool,
    pub debug_tile_data: [u32; 16 * 8 * 24 * 8],
}

impl Default for GPU {
    fn default () -> GPU {
        GPU {
            screen: [0; 160 * 144],
            tiles: [[0; 16]; 384],
            map: [[0; 32]; 64],
            lcdc: LCDC::default(),
            stat: STAT::default(),
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            mode_clock: 0,
            debug: true,
            debug_tile_data: [0; 16 * 8 * 24 * 8],
        }
    }
}

impl GPU {
    pub fn reset(&mut self) {
        for i in 0 .. 160 * 144 {
            self.screen[i] = 0;
        }
    }

    pub fn debug_update_bg(&mut self) {
        for y in 0 .. (24 * 8) {
            for x in 0 .. (16 * 8) {
                let mut tile = self.tiles[(x / 8 + (y / 8) * 16) as usize];
                let mut top = tile[((y % 8) * 2) as usize];
                let mut bottom = tile[((y % 8) * 2 + 1) as usize];
                let mut pixel = if top & (0x80 >> (x % 8)) == 0 { 0x00 } else { 0x10 };
                pixel |= if bottom & (0x80 >> (x % 8)) == 0 { 0x00 } else { 0x01 };
                if pixel != 0 {
                    self.debug_tile_data[(y * 16 * 8 + x) as usize] = 0x000000;
                } else {
                    self.debug_tile_data[(y * 16 * 8 + x) as usize] = 0xFFFFFF;
                }
            }
        }
    }

    //Doesn't handle preventing VRAM or OAM access
    pub fn step(&mut self, register_m: u16) {
        self.mode_clock += register_m;
        match self.stat.mode {
            2   => { //OAM Search
                //Decides which sprites are visible
                //Puts sprites that are visibile into array of up to 10
                //x != 0 && line >= sprite.y && line <= sprite.y + sprite.h
                if self.mode_clock >= 20 {
                    self.mode_clock = 0;
                    self.stat.mode = 3;
                }
            },
            3   => { //Pixel Transfer
                if self.mode_clock >= 43 {
                    self.mode_clock = 0;
                    self.stat.mode = 0;
                    self.render_scanline();
                }
            },
            0   => { //Hblank
                if self.mode_clock >= 51 {
                    self.mode_clock = 0;
                    self.ly += 1;
                    if self.ly == 143 {
                        self.stat.mode = 1;
                    } else {
                        self.stat.mode = 3;
                    }
                }
            },
            1   => { //Vblank
                if self.mode_clock >= 114 {
                    self.mode_clock = 0;
                    self.ly += 1;
                    if self.ly == 153 {
                        self.stat.mode = 2;
                        self.ly = 0;
                    }
                }
            },
            _   => {}
        }
    }

    //Ignoring clocks for now, not doing interweaved fetches
    pub fn render_scanline(&mut self) {
        let start_y = self.ly + self.scy;
        let mut fifo: VecDeque<u8> = VecDeque::new();
        let bg_map_offset = if self.lcdc.bg_tile_map_address { 32 } else { 0 };
        let bg_tile_offset = if self.lcdc.bg_window_tile_data { 0 } else { 128 };
        for x in 0 .. 160 {
            while fifo.len() <= 5 {
                //Only handling background atm
                for pixel_x in x + self.scx .. x + self.scx + 5 {
                    let mut tile_map = self.map[(start_y / 8 + bg_map_offset) as usize][(pixel_x / 8) as usize];
                    let mut tile = self.tiles[(bg_tile_offset + tile_map) as usize];
                    let mut top = tile[((start_y % 8) * 2) as usize];
                    let mut bottom = tile[((start_y % 8) * 2 + 1) as usize];
                    let mut pixel = if top & (0x80 >> (pixel_x % 8)) == 0 { 0x00 } else { 0x10 };
                    pixel |= if bottom & (0x80 >> (pixel_x % 8)) == 0 { 0x00 } else { 0x01 };
                    fifo.push_back(pixel);
                }
            }
            let mut pixel = fifo.pop_front().unwrap();
            if pixel != 0 {
            // if pixel != 0 {
                // println!("{:02X}", pixel);
                self.screen[((self.ly as u32) * 160 + (x as u32)) as usize] = 0x000000;
            } else {
                self.screen[((self.ly as u32) * 160 + (x as u32)) as usize] = 0xFFFFFF;
            }
        }
    }

    pub fn rb(&mut self, addr: u16) -> u8 {
        return match addr {
            0xFF40  => self.lcdc.val,
            0xFF41  => self.stat.val,
            0xFF42  => self.scy,
            0xFF43  => self.scx,
            0xFF44  => self.ly,
            0xFF45  => self.lyc,
            0xFF46  => self.dma,
            0xFF47  => self.bgp,
            0xFF48  => self.obp0,
            0xFF49  => self.obp1,
            0xFF4A  => self.wy,
            0xFF4B  => self.wx,
            _       => {
                if addr < 0x1800 {
                    self.tiles[(addr / 16) as usize][(addr % 16) as usize]
                }
                else if addr < 0x2000 {
                    let map_addr = addr % 0x1800;
                    self.map[(map_addr / 32) as usize][(map_addr % 32) as usize]
                } else { 0 }
            },
        }
    }

    pub fn wb(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF40  => self.lcdc.set(val),
            0xFF41  => self.stat.set(val),
            0xFF42  => self.scy = val,
            0xFF43  => self.scx = val,
            0xFF44  => self.ly = val,
            0xFF45  => self.lyc = val,
            0xFF46  => self.dma = val,
            0xFF47  => self.bgp = val,
            0xFF48  => self.obp0 = val,
            0xFF49  => self.obp1 = val,
            0xFF4A  => self.wy = val,
            0xFF4B  => self.wx = val,
            _       => {
                if addr < 0x1800 {
                    self.tiles[(addr / 16) as usize][(addr % 16) as usize] = val;
                }
                else if addr < 0x2000 {
                    let map_addr = addr % 0x1800;
                    self.map[(map_addr / 32) as usize][(map_addr % 32) as usize] = val;
                }
            },
        }
    }
}
