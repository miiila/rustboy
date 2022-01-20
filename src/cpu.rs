use super::bitvec::prelude::*;

#[derive(Debug, Default)]
pub struct Cpu {

    reg_af: BitArray<u16, bitvec::order::Msb0>,
    reg_bc: BitArray<u16, bitvec::order::Msb0>,
    reg_de: BitArray<u16, bitvec::order::Msb0>,
    reg_hl: BitArray<u16, bitvec::order::Msb0>,
    sp: u16,
    pc: u16
}

impl Cpu {

    pub fn new() -> Cpu {
        Default::default()
    }

    pub fn set_reg_af(&mut self, value: u16) {
        self.reg_af.store_be(value);
    }

    pub fn set_reg_bc(&mut self, value: u16) {
        self.reg_bc.store_be(value);
    }

    pub fn set_reg_de(&mut self, value: u16) {
        self.reg_de.store_be(value);
    }

    pub fn set_reg_hl(&mut self, value: u16) {
        self.reg_hl.store_be(value);
    }

    pub fn set_reg_a(&mut self, value: u8) {
        self.reg_af[0..8].store_be(value);
    }

    pub fn set_reg_b(&mut self, value: u8) {
        self.reg_bc[0..8].store_be(value);

    }
    pub fn set_reg_c(&mut self, value: u8) {
        self.reg_bc[8..16].store_be(value);
    }

    pub fn set_reg_d(&mut self, value: u8) {
        self.reg_de[0..8].store_be(value);
    }

    pub fn set_reg_e(&mut self, value: u8) {
        self.reg_de[8..16].store_be(value);
    }

    pub fn set_reg_h(&mut self, value: u8) {
        self.reg_hl[0..8].store_be(value);
    }

    pub fn set_reg_l(&mut self, value: u8) {
        self.reg_hl[8..16].store_be(value);
    }

    pub fn get_reg_af(&self) -> u16 {
        self.reg_af.load_be::<u16>()
    }

    pub fn get_reg_bc(&self) -> u16 {
        self.reg_bc.load_be::<u16>()
    }

    pub fn get_reg_de(&self) -> u16 {
        self.reg_de.load_be::<u16>()
    }

    pub fn get_reg_hl(&self) -> u16 {
        self.reg_hl.load_be::<u16>()
    }

    pub fn get_reg_a(&self) -> u8 {
        self.reg_af[0..8].load_be::<u8>()
    }

    pub fn get_reg_b(&self) -> u8 {
        self.reg_bc[0..8].load_be::<u8>()
    }
    pub fn get_reg_c(&self) -> u8 {
        self.reg_bc[8..16].load_be::<u8>()
    }

    pub fn get_reg_d(&self) -> u8 {
        self.reg_de[0..8].load_be::<u8>()
    }
    pub fn get_reg_e(&self) -> u8 {
        self.reg_de[8..16].load_be::<u8>()
    }

    pub fn get_reg_h(&self) -> u8 {
        self.reg_hl[0..8].load_be::<u8>()
    }
    pub fn get_reg_l(&self) -> u8 {
        self.reg_hl[8..16].load_be::<u8>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reg_af() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.get_reg_af(), 0);
        cpu.set_reg_af(0b1010101010101010);
        assert_eq!(cpu.get_reg_af(), 0b1010101010101010);
        cpu.set_reg_a(0b11100000);
        assert_eq!(cpu.get_reg_af(), 0b1110000010101010);
        assert_eq!(cpu.get_reg_a(), 0b11100000);
    }

    #[test]
    fn reg_bc() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.get_reg_bc(), 0);
        cpu.set_reg_bc(0b1010101010101010);
        assert_eq!(cpu.get_reg_bc(), 0b1010101010101010);
        cpu.set_reg_b(0b11100000);
        assert_eq!(cpu.get_reg_bc(), 0b1110000010101010);
        assert_eq!(cpu.get_reg_b(), 0b11100000);
        cpu.set_reg_c(0b00000101);
        assert_eq!(cpu.get_reg_bc(), 0b1110000000000101);
        assert_eq!(cpu.get_reg_c(), 0b00000101);
    }

    #[test]
    fn reg_de() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.get_reg_de(), 0);
        cpu.set_reg_de(0b1010101010101010);
        assert_eq!(cpu.get_reg_de(), 0b1010101010101010);
        cpu.set_reg_d(0b11100000);
        assert_eq!(cpu.get_reg_de(), 0b1110000010101010);
        assert_eq!(cpu.get_reg_d(), 0b11100000);
        cpu.set_reg_e(0b00000101);
        assert_eq!(cpu.get_reg_de(), 0b1110000000000101);
        assert_eq!(cpu.get_reg_e(), 0b00000101);
    }

    #[test]
    fn reg_hl() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.get_reg_hl(), 0);
        cpu.set_reg_hl(0b1010101010101010);
        assert_eq!(cpu.get_reg_hl(), 0b1010101010101010);
        cpu.set_reg_h(0b11100000);
        assert_eq!(cpu.get_reg_hl(), 0b1110000010101010);
        assert_eq!(cpu.get_reg_h(), 0b11100000);
        cpu.set_reg_l(0b00000101);
        assert_eq!(cpu.get_reg_hl(), 0b1110000000000101);
        assert_eq!(cpu.get_reg_l(), 0b00000101);
    }
}
