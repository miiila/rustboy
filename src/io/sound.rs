pub const START: usize = 0xFF10;
pub const END: usize = 0xFF27;

const CAPACITY: usize = END-START;

#[derive(Debug)]
pub struct SoundController {
    regs: [u8; CAPACITY],
}

impl SoundController {
    pub fn new() -> SoundController {
        SoundController {
            regs: [0; CAPACITY],
        }
    }

    pub fn write(&mut self, addr: usize, value: u8) {
       self.regs[addr - START] = value; 
    }

    pub fn read(&self, addr: usize) -> u8 {
       return self.regs[addr - START]; 
    }
}

impl Default for SoundController {
    fn default() -> Self {
        SoundController {
                regs: [0; CAPACITY],
        }
    }
}

