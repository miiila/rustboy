#[derive(Default, Debug)]
pub struct Ram {
    ram: Box<[u8]>
}

impl Ram {

    pub fn new(capacity: usize) -> Ram {
        let mut ram = Vec::<u8>::with_capacity(capacity);
        // TODO: Weird
        ram.resize(capacity, 0);
        Ram {
            ram: ram.into_boxed_slice(),
        }
    }

    pub fn write(&mut self, addr: usize, value: u8) {
       self.ram[addr] = value; 
    }

    pub fn read(&self, addr: usize) -> u8 {
       return self.ram[addr]; 
    }
}

//impl Default for Ram {
    //fn default() -> Self {
        //Ram {
            //ram: Vec::with_capacity(capacity).into_boxed_slice(),
        //}
    //}
//}

