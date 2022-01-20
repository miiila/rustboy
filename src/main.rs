extern crate bitvec;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod cpu;


fn main() {
    let rom_file_name = env::args().nth(1).unwrap();
    let rom = load_rom(rom_file_name);
    let mut cpu = cpu::Cpu::new();
    println!("{:#x}", rom[0x134]);
}


fn load_rom<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}
