use super::sound;
use super::lcd;

const SB: u16 = 0xFF01;
const SC: u16 = 0xFF02;
const TAC: u16 = 0xFF07;

#[derive(Debug, Default)]
pub struct IO {
    sound_controller: sound::SoundController,
    lcd: lcd::LCD,
    sb: u8,
    sc: u8,
    tac: u8,
}

impl IO {
    pub fn new() -> IO {
        IO {
            sound_controller: sound::SoundController::new(),
            lcd: lcd::LCD::new(),
            sb: 0,
            sc: 0,
            tac: 0,
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
        match addr {
            SB => self.sb = value,
            SC => self.sc = value,
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
        match addr {
            SB => self.sb,
            SC => self.sc,
            TAC => self.tac,
            _ => panic!("Reading from unknown IO {:#X?}", addr)
        }
    }
}

