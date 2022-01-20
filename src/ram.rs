const CAPACITY: usize = 8 * 1024;

#[derive(Debug)]
pub struct Ram {
    ram: [u8; CAPACITY],
}

impl Ram {

    pub fn new() -> Ram {
        Ram {
            ram: [0; CAPACITY],
        }
    }

    pub fn write(&mut self, addr: usize, value: u8) {
       self.ram[addr] = value; 
    }

    pub fn read(&self, addr: usize) -> u8 {
       return self.ram[addr]; 
    }
}

impl Default for Ram {
    fn default() -> Self {
        Ram {
                ram: [0; CAPACITY],
        }
    }
}

