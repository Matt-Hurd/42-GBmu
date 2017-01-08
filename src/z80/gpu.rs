pub struct GPU {
    pub screen: Vec<u32>,
    pub mode: u8,
    pub mode_clock: u16,
    pub line: u8,
}

impl Default for GPU {
    fn default () -> GPU {
        GPU {
            screen: vec![0, 160 * 144],
            mode: 0,
            mode_clock: 0,
            line: 0,
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
}
