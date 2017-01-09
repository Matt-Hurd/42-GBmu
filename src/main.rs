use std::process;
use std::env;
use std::path;
use std::io;

// mod mbc;
mod z80;

fn reset(z80: &mut z80::Z80) {
    z80.mmu.gpu.reset();
    z80.mmu.reset();
    z80.reset();

    let result = z80.mmu.load(path::PathBuf::from("roms/opus5.gb"));
    match result {
        Ok(n) => (println!("{}", n)),
        Err(err) => println!("Error: {}", err),
    }
}

fn frame(z80: &mut z80::Z80) {
    let fclk = z80.clock.t as u32 + 70224;
    let debug = true;
    let mut paused = true;
    while {
        let mut input = "break".to_string();
        if debug && paused {
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
    reset(&mut core);
    loop {
        frame(&mut core);
    }
}
