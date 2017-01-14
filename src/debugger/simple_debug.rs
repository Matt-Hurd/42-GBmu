use z80;
use std::io;
use std::i64;
use minifb::{Window, Key, Scale, WindowOptions};

pub struct Debugger {
    pub enabled: bool,
    pub stopped: bool,
    pub breakpoints: Vec<u16>,
    pub skip: u16,
}

impl Default for Debugger {
    fn default () -> Debugger {
        Debugger {
            enabled: false,
            stopped: true,
            breakpoints: vec![],
            skip: 0,
        }
    }
}

impl Debugger {
    pub fn enable(&mut self, z80: &mut z80::Z80) {
        z80.debug = true;
        z80.debug_r = true;
        self.enabled = true;
        self.stopped = true;
    }

    pub fn print_map_info(&mut self, z80: &mut z80::Z80) {
        for y in 0 .. 64 {
            for x in 0 .. 32 {
                print!("{:02X} ", z80.mmu.gpu.map[y][x]);
            }
            println!("");
        }
    }

    pub fn print_debug(&mut self, z80: &mut z80::Z80, manual: bool, debug_window: &mut Window) {
        if manual || z80.debug {
            // z80.mmu.gpu.debug_update_bg();
            // debug_window.update_with_buffer(&z80.mmu.gpu.debug_tile_data);
            let op = z80.mmu.rb(z80.r.pc);
            if manual || z80.debug_r {
                z80.r.debug_print();
                z80.mmu.debug_print();
                z80.debug_print_cpu_time();
                // z80.debug_print_stack();
            }
            print!("Next OP: ");
            match op {
                0xCB    => z80::debug::translate_cb(z80.mmu.rb(z80.r.pc + 1), z80.r.pc + 1),
                _       => z80::debug::translate_op(op, z80.r.pc, z80),
            };
            println!("");
        }
    }

    pub fn print_breakpoints(&mut self) {
        println!("Breakpoints:");
        for point in 0 .. self.breakpoints.len() {
            println!("  0x{:04X}", self.breakpoints[point]);
        }
    }

    pub fn check_breakpoints(&mut self, z80: &mut z80::Z80) {
        for point in 0 .. self.breakpoints.len() {
            if z80.r.pc == self.breakpoints[point] {
                if self.skip == 0 {
                    self.stopped = true;
                } else {
                    self.skip -= 1;
                }
                println!("Hit breakpoint set at 0x{:04X}", z80.r.pc);
                }
            }
        }

    pub fn step(&mut self, z80: &mut z80::Z80, window: &mut Window) {
        if self.enabled {
            let mut done = false;
            self.check_breakpoints(z80);
            self.print_debug(z80, false, window);
            // if z80.r.pc >= 0x2AE0 || z80.r.sp < 0xFF00 {
            //     self.stopped = true;
            // }
            while self.stopped && !done {
                let mut input = String::new();
                io::stdin().read_line(&mut input)
                    .expect("failed to read line");
                let line = input.trim();
                if line == "" || line == "step" {
                    done = true;
                }
                if line == "continue" || line == "run"{
                    self.stopped = false;
                }
                if line == "dro" || line == "disable_r" {
                    z80.debug_r = false;
                }
                if line == "ero" || line == "enable_r" {
                    z80.debug_r = true;
                }
                if line == "ddo" || line == "disable_d" {
                    z80.debug = false;
                }
                if line == "edo" || line == "enable_d" {
                    z80.debug = true;
                }
                if line == "r" {
                    self.print_debug(z80, true, window);
                }
                if line == "b" {
                    self.print_breakpoints();
                }
                if line == "print map" {
                    self.print_map_info(z80);
                }
                let split = line.split(" ").collect::<Vec<&str>>();
                if split[0] == "break" {
                    if split.len() > 1 {
                        for point in 1 .. split.len() {
                            let val = u16::from_str_radix(split[point], 16);
                            match val {
                                Ok(n)       => {
                                    println!("Adding breakpoint at 0x{:04X}", n);
                                    self.breakpoints.push(n);
                                },
                                Err(err)    => println!("Invalid breakpoint {}", split[point]),
                            }
                        }
                    }
                }
                if split[0] == "rmb" {
                    if split.len() > 1 {
                        for point in 1 .. split.len() {
                            let val = u16::from_str_radix(split[point], 16);
                            match val {
                                Ok(n)       => {
                                    println!("Removing breakpoint at 0x{:04X}", n);
                                    self.breakpoints.retain(|&x| x != n);
                                },
                                Err(err)    => println!("Invalid breakpoint {}", split[point]),
                            }
                        }
                    }
                }
                if split[0] == "skip" {
                    if split.len() == 2 {
                        for point in 1 .. split.len() {
                            let val = u16::from_str_radix(split[point], 10);
                            match val {
                                Ok(n)       => {
                                    println!("Skipping {} breakpoints", n);
                                    self.skip = n;
                                },
                                Err(err)    => println!("Invalid Number: {}", split[point]),
                            }
                        }
                    }
                }
            }
        }
    }
}
