use std::process;
use std::env;
use std::path;
use std::io;
extern crate minifb;

use minifb::{Window, Key, Scale, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

mod z80;

fn reset(z80: &mut z80::Z80, rom_path: path::PathBuf) {
    z80.mmu.gpu.reset();
    z80.mmu.reset();
    z80.reset();

    let result = z80.mmu.load(rom_path);
    match result {
        Ok(n) => (println!("{}", n)),
        Err(err) => println!("Error: {}", err),
    }
}

fn frame(z80: &mut z80::Z80) {
    let fclk = z80.clock.t as u32 + 70224;
    let mut paused = true;
    while {
        let mut input = "break".to_string();
        if z80.debug && paused {
            let mut stuck = true;
            while stuck && paused
            {
                let mut input = String::new();
        		io::stdin().read_line(&mut input)
        			.expect("failed to read line");
                input = input.trim().to_string();
                if input == "continue" || input == "run" {
                    paused = false;
                }
                if input == "step" || input == "" {
                    stuck = false;
                }
            }
        }
        z80.step();
        // if z80.r.pc >= 0xA0 {
        //     panic!("Reached the loop".to_string())
        //     // z80.r.pc += 1; //Totally skipping the checksum, something is wrong
        // } else if z80.r.pc == 0x01B2 {
        //     panic!("Reached the loop".to_string())
        // } else if z80.r.pc >= 0x00F0 {
        //     ;
            // z80.debug_r = true;
            // z80.debug = true;
        //     // if z80.r.sp < 0xFFA0 {
        //     //     panic!("Stack is pretty damn low");
        //     // }
        // }
        (z80.clock.t as u32) < fclk
    } {}
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        println!("Usage: {} [rom]", args[0]);
        process::exit(1);
    }

    let mut core: z80::Z80 = z80::Z80::default();
    let result = core.mmu.load(path::PathBuf::from(&args[1]));
        // core.debug = true;
        // core.debug_r = true;
    match result {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
    reset(&mut core, path::PathBuf::from(&args[1]));
    let mut window = match Window::new("GBmu", WIDTH, HEIGHT,
                                       WindowOptions {
                                           resize: false,
                                           scale: Scale::X4,
                                           ..WindowOptions::default()
                                       }) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };
    let mut debugWindow = match Window::new("tile_map", 16 * 8, 24 * 8,
                                       WindowOptions {
                                           resize: false,
                                           scale: Scale::X4,
                                           ..WindowOptions::default()
                                       }) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };
    loop {
        frame(&mut core);
        core.mmu.gpu.debug_update_bg();
        window.update_with_buffer(&core.mmu.gpu.screen);
        debugWindow.update_with_buffer(&core.mmu.gpu.debug_tile_data);
    }
}
