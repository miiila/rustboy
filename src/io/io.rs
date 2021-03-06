use super::sound;
use super::lcd;
use super::super::ram::Ram;

const SB: u16 = 0xFF01;
const SC: u16 = 0xFF02;

const DIV: u16 = 0xFF04;
const TIMA: u16 = 0xFF05;
const TMA: u16 = 0xFF06;
const TAC: u16 = 0xFF07;

const IO_REGISTERS_START: usize = 0xFF08;
const IO_REGISTERS_END: usize = 0xFF80;

#[derive(Debug, Default)]
pub struct IO {
    sound_controller: sound::SoundController,
    lcd: lcd::LCD,
    sb: u8,
    sc: u8,
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
    io_registers: Ram,
}

impl IO {
    pub fn new() -> IO {
        IO {
            sound_controller: sound::SoundController::new(),
            lcd: lcd::LCD::new(),
            sb: 0,
            sc: 0,
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            io_registers: Ram::new(IO_REGISTERS_END - IO_REGISTERS_START),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if sound::START <= addr as usize && (addr as usize) < sound::END {
            self.sound_controller.write(addr.into(), value);
            return
        }
        if lcd::START <= addr as usize && (addr as usize) < lcd::END {
            self.lcd.write(addr.into(), value);
            return
        }
        if IO_REGISTERS_START <= addr as usize && (addr as usize) < IO_REGISTERS_END {
            self.io_registers.write(addr as usize - IO_REGISTERS_START, value);
            return
        }
        match addr {
            SB => self.sb = value,
            SC => self.sc = value,
            DIV => self.div = 0x00, // any write to DIV resets it
            TIMA => self.tima = value,
            TMA => self.tma = value,
            TAC => self.tac = value,
            _ => panic!("Writing to unknown IO {:#X?}", addr)
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        if sound::START <= addr as usize && (addr as usize) < sound::END {
            return self.sound_controller.read(addr.into());
        }
        if lcd::START <= addr as usize && (addr as usize) < lcd::END {
            return self.lcd.read(addr.into());
        }
        if IO_REGISTERS_START <= addr as usize && (addr as usize) < IO_REGISTERS_END {
            return self.io_registers.read(addr as usize - IO_REGISTERS_START);
        }
        match addr {
            SB => self.sb,
            SC => self.sc,
            DIV => self.div,
            TIMA => self.tima,
            TMA => self.tma,
            TAC => self.tac,
            _ => panic!("Reading from unknown IO {:#X?}", addr)
        }
    }

    pub fn increment_div(&mut self) {
        self.div += 1;
    }
}

