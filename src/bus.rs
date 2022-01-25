use super::ram;
use super::io;

const ROM_START: u16 = 0x0000;
const ROM_END: u16 = 0x8000;

const WRAM_START: u16 = 0xC000;
const WRAM_END: u16 = 0xE000;

const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFF;

const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0xA000;

const IE: u16 = 0xFFFF;
const IF: u16 = 0xFF0F;

const IO_START: u16 = 0xFF00;
const IO_END: u16 = 0xFF80;

#[derive(Debug, Default)]
pub struct Bus {
    wram: ram::Ram,
    hram: ram::Ram,
    vram: ram::Ram,
    rom: Box<[u8]>,
    io: io::IO,
    ie: bool,
    r#if: u8,
}

impl Bus {
    pub fn new(wram: ram::Ram, rom: Box<[u8]>, hram: ram::Ram, vram: ram::Ram) -> Bus {
        Bus {
            wram,
            rom,
            hram,
            vram,
            ie: false,
            r#if: 0,
            io: io::IO::new()
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        if addr < ROM_END {
            return self.rom[addr as usize];
        }
        if WRAM_START <= addr && addr < WRAM_END {
            let ram_addr = addr - WRAM_START;
            return self.wram.read(ram_addr.into());
        }
        if HRAM_START <= addr && addr < HRAM_END {
            let ram_addr = addr - HRAM_START;
            /*dbg!(addr, self.hram.read(ram_addr.into()));*/
            return self.hram.read(ram_addr.into());
        }
        if VRAM_START <= addr && addr < VRAM_END {
            let ram_addr = addr - VRAM_START;
            return self.vram.read(ram_addr.into());
        }
        if addr == IE {
           return self.ie.into(); 
        }

        if addr == IF {
           return self.r#if; 
        }

        if IO_START <= addr && addr < IO_END {
            return self.io.read(addr);
        }

        panic!("Reading from unknown addres {:#x}", addr)
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if addr < ROM_END {
            panic!("Writing to rom to addres {:#x}", addr);
        }
        if WRAM_START <= addr && addr < WRAM_END {
            let ram_addr = addr - WRAM_START;
            self.wram.write(ram_addr.into(), value);
            return
        }
        if HRAM_START <= addr && addr < HRAM_END {
            let ram_addr = addr - HRAM_START;
            /*dbg!(addr, value);*/
            self.hram.write(ram_addr.into(), value);
            return
        }
        if VRAM_START <= addr && addr < VRAM_END {
            let ram_addr = addr - VRAM_START;
            self.vram.write(ram_addr.into(), value);
            return
        }
        if addr == IE {
            self.ie = value == 1;
            return
        }
        if addr == IF {
            self.r#if = value;
            return
        }

        if IO_START <= addr && addr < IO_END {
            self.io.write(addr, value);
            return
        }

        panic!("Writing to unknown addres {:#x}", addr)
    }

}
