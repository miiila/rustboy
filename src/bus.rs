use super::ram;

const ROM_START: u16 = 0x0000;
const ROM_END: u16 = 0x8000;

const RAM_START: u16 = 0xC000;
const RAM_END: u16 = 0xE000;

#[derive(Debug, Default)]
pub struct Bus {
    ram: ram::Ram,
    rom: Box<[u8]> 
}

impl Bus {
    pub fn new(ram: ram::Ram, rom: Box<[u8]>) -> Bus {
        Bus {
            ram,
            rom
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        if addr < ROM_END {
            return self.rom[addr as usize];
        }
        if RAM_START < addr && addr < RAM_END {
            let ram_addr = addr - RAM_START;
            return self.ram.read(ram_addr.into());
        }
        panic!("Reading from unknown addres {:#x}", addr)
    }
}
