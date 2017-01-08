pub struct GPU {
    pub screen: Vec<u32>,
    pub vram: Vec<u8>,
    pub tileset: Vec<Vec<Vec<u8>>>,
    pub palette: Vec<Vec<u32>>,
    pub mode_clock: u16,
    pub mode: u8,
    pub line: u8,
    pub bgtile: bool,
    pub scx: u8,
    pub scy: u8,

}

impl Default for GPU {
    fn default () -> GPU {
        GPU {
            screen: vec![0; 160 * 144],
            vram: vec![0; 8192],
            tileset: vec![vec![vec![0; 8]; 8]; 256],
            palette: vec![vec![0; 4]; 4],
            mode: 0,
            mode_clock: 0,
            line: 0,
            bgtile: false,
            scx: 0,
            scy: 0,
        }
    }
}

impl GPU {
    pub fn reset(&mut self) {
        for i in 0 .. 160 * 144 {
            self.screen[i] = 0;
        }
    }

    //I'm not sure this is 100% accurate, might need changes down the line
    pub fn step(&mut self, register_t: u16) {
        self.mode_clock += register_t;
        match self.mode {
            0   => { //OAM Read mode
                if self.mode_clock >= 80 {
                    self.mode_clock = 0;
                    self.mode = 1;
                }
            },
            1   => { //VRAM read mode
                if self.mode_clock >= 172 {
                    self.mode_clock = 0;
                    self.mode = 2;
                    // self.render_scanline();
                }
            },
            2   => { //Hblank
                if self.mode_clock >= 204 {
                    self.mode_clock = 0;
                    self.line += 1;
                    if self.line == 143 {
                        self.mode = 3;
                        //self.draw()
                    } else {
                        self.mode = 0;
                    }
                }
            },
            3   => { //Vblank
                if self.mode_clock >= 456 {
                    self.mode_clock = 0;
                    self.line += 1;
                    if self.line == 153 {
                        self.mode = 0;
                        self.line = 0;
                    }
                }
            },
            _   => {}
        }
    }


    /* I really don't like this implementation of update_tile and render_scanline
    ** They're simply being used so that I can continue to flesh out the reset
    ** of the program, as well as have a basic working functionality.
    ** They were taken from http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-Graphics
    ** They will be replaced. Ideally update_tile will be removed.
    **
    ** Also need to find out how to avoid so many `as usize` casts.
    */

    pub fn update_tile(&mut self, addr: u16) {
        let relative: usize = (addr & 0x1FFE) as usize;
        let tile: usize = (relative >> 4) & 511;
        let y: usize = (relative >> 1) & 7;

        for x in 0 .. 8 {
            let sx = 1 << (7 - x);
            self.tileset[tile][y][x] = 0;
            if self.vram[relative] & sx != 0 {
                self.tileset[tile][y][x] |= 1;
            }
            if self.vram[relative + 1] & sx != 0 {
                self.tileset[tile][y][x] |= 2;
            }
        }
    }

    pub fn render_scanline(&mut self) {
        let offset = if self.bgtile { 0x1C00 } else { 0x1C00 };
        let map_offset = offset + (((self.line + self.scy) as usize) & 255) >> 3;
        let y = ((self.line + self.scy) & 7) as usize;
        let screen_offset = (self.line as usize) * 160 * 4;
        let mut line_offset = (self.scx >> 3) as usize;
        let mut tile = self.vram[map_offset + line_offset] as usize;
        if self.bgtile && tile < 128 {
            tile += 256;
        }
        let mut x = (self.scx & 7) as usize;
        for i in 0 .. 160 {
            let ref mut color = self.palette[self.tileset[tile][y][x] as usize];
            self.screen[screen_offset] = 0;
            self.screen[screen_offset] |= (color[0] as u32) << 24;
            self.screen[screen_offset] |= (color[1] as u32) << 16;
            self.screen[screen_offset] |= (color[2] as u32) << 8;
            self.screen[screen_offset] |= color[3] as u32;
            x += 1;
            if x == 8 {
                x = 0;
                line_offset = (line_offset + 1) & 31;
                tile = self.vram[map_offset + line_offset] as usize;
                if self.bgtile && tile < 128 {
                    tile += 256;
                }
            }
        }
    }
}
