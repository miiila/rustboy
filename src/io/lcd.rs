pub const START: usize = 0xFF40;
pub const END: usize = 0xFF4C;

const CAPACITY: usize = END-START;

#[derive(Debug)]
pub struct LCD {
    regs: [u8; CAPACITY],
}

impl LCD {
    pub fn new() -> LCD {
        let mut lcd = LCD {
            regs: [0; CAPACITY],
        };
        // TODO: Test - is it necessary?
        lcd.regs[0xFF44-START] = 0x90;
        lcd
    }

    pub fn write(&mut self, addr: usize, value: u8) {
       self.regs[addr - START] = value; 
    }

    pub fn read(&self, addr: usize) -> u8 {
       return self.regs[addr - START]; 
    }
}

impl Default for LCD {
    fn default() -> Self {
        LCD {
                regs: [0; CAPACITY],
        }
    }
}

