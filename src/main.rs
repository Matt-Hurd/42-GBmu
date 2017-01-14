use std::process;
use std::env;
use std::path;
extern crate minifb;

use minifb::{Window, Key, Scale, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

mod z80;
mod debugger;

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

fn frame(z80: &mut z80::Z80, debugger: &mut debugger::simple_debug::Debugger, mut debug_window: &mut Window) {
    let fclk = z80.clock.t as u32 + 70224;
    let mut paused = true;
    while {
        debugger.step(z80, &mut debug_window);
        z80.step();
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
    match result {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
    reset(&mut core, path::PathBuf::from(&args[1]));
    let mut window = match Window::new("GBmu", WIDTH, HEIGHT,
                                       WindowOptions {
                                           resize: false,
                                           scale: Scale::X2,
                                           ..WindowOptions::default()
                                       }) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };
    // let mut tile_window = match Window::new("tile_map", 16 * 8, 24 * 8,
    let mut tile_window = match Window::new("tile_map", 0, 0,
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
    let mut debugger = debugger::simple_debug::Debugger::default();
    // debugger.enable(&mut core);
    loop {
        let old_keys1 = core.mmu.keys[0];
        let old_keys2 = core.mmu.keys[1];
        core.mmu.keys[0] = 0xF;
        core.mmu.keys[1] = 0xF;
        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::Up => core.mmu.keys[1] &= 0b1011,
                    Key::Down => core.mmu.keys[1] &= 0b0111,
                    Key::Left => core.mmu.keys[1] &= 0b1101,
                    Key::Right => core.mmu.keys[1] &= 0b1110,
                    Key::Z => core.mmu.keys[0] &= 0b1101, //B
                    Key::X => core.mmu.keys[0] &= 0b1110, //A
                    Key::Apostrophe => core.mmu.keys[0] &= 0b1011, //Select
                    Key::Enter => core.mmu.keys[0] &= 0b0111, //Start
                    Key::P => debugger.enable(&mut core),
                    _ => (),
                }
            }
        });
        if old_keys1 != core.mmu.keys[0] || old_keys2 != core.mmu.keys[1] {
            core.mmu.iflags &= 0b10000;
        }
        frame(&mut core, &mut debugger, &mut tile_window);
        core.mmu.gpu.debug_update_bg();
        window.update_with_buffer(&core.mmu.gpu.screen);
        // tile_window.update_with_buffer(&core.mmu.gpu.debug_tile_data);
    }
}
