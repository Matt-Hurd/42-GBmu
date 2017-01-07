use std::process;
use std::env;
use std::path;

mod mbc;
mod z80;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let result = mbc::read_mbc(path::PathBuf::from(&args[1]));
        match result {
            Ok(n) => println!("{}", n),
            Err(err) => println!("Error: {}", err),
        }
    } else {
        println!("Usage: {} [rom]", args[0]);
        process::exit(1);
    }
}
