extern crate bitvec;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod cpu;
mod ram;
mod bus;
mod io;


const WRAM_CAPACITY: usize = 8 * 1024; 
const HRAM_CAPACITY: usize = 127;
const VRAM_CAPACITY: usize = 8 * 1024;

fn main() {
    let rom_file_name = env::args().nth(1).unwrap();
    let rom = load_rom(rom_file_name);
    let ram = ram::Ram::new(WRAM_CAPACITY);
    let hram = ram::Ram::new(HRAM_CAPACITY);
    let vram = ram::Ram::new(VRAM_CAPACITY);
    let bus = bus::Bus::new(ram, rom, hram, vram);
    let mut cpu = cpu::Cpu::new();
    cpu.connect_bus(bus);
    cpu.run();
    //cpu.run_next_instruction();
    //cpu.run_next_instruction();
}


fn load_rom<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}
