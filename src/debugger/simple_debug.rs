use z80;
use std::io;

pub struct Debugger {
    pub enabled: bool,
    pub stopped: bool,
    pub breakpoints: Vec<u16>,
    pub skip: u16,
    pub used_ops: Vec<u16>,
    pub log: bool,
}

impl Default for Debugger {
    fn default () -> Debugger {
        Debugger {
            enabled: false,
            stopped: true,
            breakpoints: vec![],
            used_ops: vec![],
            skip: 0,
            log: false,
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

    pub fn log_ops(&mut self, z80: &mut z80::Z80) {
        let mut used = false;
        let mut opcode = z80.mmu.rb(z80.r.pc) as u16;
        if opcode == 0xCB {
            opcode = (opcode << 8) + z80.mmu.rb(z80.r.pc + 1) as u16;
        }
        for op in 0 .. self.used_ops.len() {
            if self.used_ops[op] == opcode {
                used = true;
            }
        }
        if !used {
            self.used_ops.push(opcode);
        }
    }

    pub fn print_logged_ops(&mut self, z80: &mut z80::Z80) {
        for op in 0 .. self.used_ops.len() {
            if self.used_ops[op] & 0xCB00 == 0 {
                z80::debug::translate_op(self.used_ops[op] as u8, 0, z80);
            } else {
                z80::debug::translate_cb((self.used_ops[op] & 0xFF) as u8, 0);
            }
        }
    }

    pub fn print_map_info(&mut self, z80: &mut z80::Z80) {
        for y in 0 .. 64 {
            for x in 0 .. 32 {
                print!("{:02X} ", z80.mmu.gpu.map[y][x]);
            }
            println!("");
        }
    }

    pub fn print_debug(&mut self, z80: &mut z80::Z80, manual: bool) {
        if manual || z80.debug {
            // z80.mmu.gpu.debug_update_bg();
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

    pub fn step(&mut self, z80: &mut z80::Z80) {
        // if z80.count > 35165 {
        //     self.enable(z80);
        // }
        if self.enabled {
            let mut done = false;
            self.check_breakpoints(z80);
            self.print_debug(z80, false);
            // if z80.r.pc >= 0x2AE0 || z80.r.sp < 0xFF00 {
            //     self.stopped = true;
            // }
            if self.log {
                self.log_ops(z80);
            }
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
                if line == "ops" {
                    self.log = true;
                }
                if line == "po" {
                    self.print_logged_ops(z80);
                }
                if line == "r" {
                    self.print_debug(z80, true);
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
