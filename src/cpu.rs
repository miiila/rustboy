use super::bitvec::prelude::*;
use super::bus;

#[derive(Debug, Default)]
pub struct Cpu {

    bus: bus::Bus,
    reg_af: BitArray<u16, bitvec::order::Msb0>,
    reg_bc: BitArray<u16, bitvec::order::Msb0>,
    reg_de: BitArray<u16, bitvec::order::Msb0>,
    reg_hl: BitArray<u16, bitvec::order::Msb0>,
    sp: u16,
    pc: u16,
    i: u64,
}

impl Cpu {

    pub fn new() -> Cpu {
        let mut cpu: Cpu = Default::default();
        // TODO: Implement BOOT ROM properly
        cpu.pc = 0x100;
        cpu.sp = 0xFFFE;
        //cpu.reg_af[8..16].store_be(0xB0);
        cpu.reg_af.store_be(0x01B0);
        cpu.reg_bc.store_be(0x0013);
        cpu.reg_de.store_be(0x00D8);
        cpu.reg_hl.store_be(0x014D);
        cpu.i = 1;
        cpu
    }

    pub fn connect_bus(&mut self, bus: bus::Bus) {
        self.bus = bus; 
    } 

    pub fn run_next_instruction(&mut self) {
        let inst = self.bus.read(self.pc);
        self.perform_instruction(inst);
    }

    pub fn perform_instruction(&mut self, inst: u8) {
        //println!("{}: Instruction 0x{:02X} @ {:#X}: A: {:02X}, F: {:02X}, B: {:02X}, C: {:02X}, D: {:02x}, E: {:02x}, H: {:02X}, L: {:02X}, SP: {:02X}", self.i, inst, self.pc, self.get_reg_a(), self.get_reg_f(), self.get_reg_b(), self.get_reg_c(), self.get_reg_d(), self.get_reg_e(), self.get_reg_h(), self.get_reg_l(), self.sp);
        // https://github.com/retrio/gb-test-roms/tree/master/cpu_instrs
        // CONSOLE OUTPUT
        self.i += 1;
        if self.bus.read(0xff02) == 0x81 {
            print!("{}", self.bus.read(0xff01) as char);
            self.bus.write(0xff02, 0x0);
        }
        match inst {
            // SPECIAL
            0x00 => self.pc += 1,
            0x76 => panic!("HALT INSTRUCTION"),
            0xCB => {
                self.pc += 1;
                self.perform_cb_instruction();
                self.pc += 1;
            },
            0xF3 => {
                self.bus.write(0xFFFF, 0);
                self.pc +=1;
            }
            0xFB => {
                self.bus.write(0xFFFF, 1);
                self.pc +=1;
            }
            0x37 => {
                self.set_n_flag(false);
                self.set_h_flag(false);
                self.set_c_flag(true);
            }
            0x2F => {
                self.set_reg_a(!self.get_reg_a());
                self.set_n_flag(true);
                self.set_h_flag(true);
            }
            // LD 16 bit
            0x01 => {
                self.set_reg_bc(self.read_u16());
                self.pc +=3;
            }
            0x11 => {
                self.set_reg_de(self.read_u16());
                self.pc +=3;
            }
            0x21 => {
                self.set_reg_hl(self.read_u16());
                self.pc +=3;
            }
            0x31 => {
                self.sp = self.read_u16();
                self.pc +=3;
            }
            0xF8 => {
                let res = self.add_i16(self.sp as i16, self.bus.read(self.pc+1) as i8 as i16);
                self.set_reg_hl(res as u16);
                self.pc +=2;
            }
            0xF9 => {
                self.sp = self.get_reg_hl();
                self.pc +=1;
            }
            0x08 => {
                let addr = self.read_u16();
                let [lsb, msb] = self.sp.to_le_bytes();
                self.bus.write(addr, lsb);
                self.bus.write(addr+1, msb);
                self.pc +=3;
            }
            0xEA => {
                let addr = self.read_u16();
                self.bus.write(addr, self.get_reg_a());
                self.pc +=3;
            }
            // LD 8 bit
            0x0A => {
                let addr = self.get_reg_bc();
                self.set_reg_a(self.bus.read(addr));
                self.pc +=1;
            }
            0x06 => {
                self.set_reg_b(self.bus.read(self.pc+1));
                self.pc +=2;
            }
            0x16 => {
                self.set_reg_d(self.bus.read(self.pc+1));
                self.pc +=2;
            }
            0x26 => {
                self.set_reg_h(self.bus.read(self.pc+1));
                self.pc +=2;
            }
            0x36 => {
                self.bus.write(self.get_reg_hl(), self.bus.read(self.pc+1));
                self.pc +=2;
            }
            0x0E => {
                self.set_reg_c(self.bus.read(self.pc+1));
                self.pc +=2;
            }
            0x1E => {
                self.set_reg_e(self.bus.read(self.pc+1));
                self.pc +=2;
            }
            0x2E => {
                self.set_reg_l(self.bus.read(self.pc+1));
                self.pc +=2;
            }
            0x3E => {
                self.set_reg_a(self.bus.read(self.pc+1));
                self.pc +=2;
            }
            0x1A => {
                let addr = self.get_reg_de();
                self.set_reg_a(self.bus.read(addr));
                self.pc +=1;
            }
            0x2A => {
                let addr = self.get_reg_hl();
                self.set_reg_a(self.bus.read(addr));
                self.set_reg_hl(addr+1);
                self.pc +=1;
            }
            0x3A => {
                let addr = self.get_reg_hl();
                self.set_reg_a(self.bus.read(addr));
                self.set_reg_hl(addr-1);
                self.pc +=1;
            }
            0x40 => {
                self.set_reg_b(self.get_reg_b());
                self.pc +=1;
            }
            0x41 => {
                self.set_reg_b(self.get_reg_c());
                self.pc +=1;
            }
            0x42 => {
                self.set_reg_b(self.get_reg_d());
                self.pc +=1;
            }
            0x43 => {
                self.set_reg_b(self.get_reg_e());
                self.pc +=1;
            }
            0x44 => {
                self.set_reg_b(self.get_reg_h());
                self.pc +=1;
            }
            0x45 => {
                self.set_reg_b(self.get_reg_l());
                self.pc +=1;
            }
            0x46 => {
                self.set_reg_b(self.bus.read(self.get_reg_hl()));
                self.pc +=1;
            }
            0x47 => {
                self.set_reg_b(self.get_reg_a());
                self.pc +=1;
            }
            0x48 => {
                self.set_reg_c(self.get_reg_b());
                self.pc +=1;
            }
            0x49 => {
                self.set_reg_c(self.get_reg_c());
                self.pc +=1;
            }
            0x4A => {
                self.set_reg_c(self.get_reg_d());
                self.pc +=1;
            }
            0x4B => {
                self.set_reg_c(self.get_reg_e());
                self.pc +=1;
            }
            0x4C => {
                self.set_reg_c(self.get_reg_h());
                self.pc +=1;
            }
            0x4D => {
                self.set_reg_c(self.get_reg_l());
                self.pc +=1;
            }
            0x4E => {
                self.set_reg_c(self.bus.read(self.get_reg_hl()));
                self.pc +=1;
            }
            0x4F => {
                self.set_reg_c(self.get_reg_a());
                self.pc +=1;
            }
            0x50 => {
                self.set_reg_d(self.get_reg_b());
                self.pc +=1;
            }
            0x51 => {
                self.set_reg_d(self.get_reg_c());
                self.pc +=1;
            }
            0x52 => {
                self.set_reg_d(self.get_reg_d());
                self.pc +=1;
            }
            0x53 => {
                self.set_reg_d(self.get_reg_e());
                self.pc +=1;
            }
            0x54 => {
                self.set_reg_d(self.get_reg_h());
                self.pc +=1;
            }
            0x55 => {
                self.set_reg_d(self.get_reg_l());
                self.pc +=1;
            }
            0x56 => {
                self.set_reg_d(self.bus.read(self.get_reg_hl()));
                self.pc +=1;
            }
            0x57 => {
                self.set_reg_d(self.get_reg_a());
                self.pc +=1;
            }
            0x58 => {
                self.set_reg_e(self.get_reg_b());
                self.pc +=1;
            }
            0x59 => {
                self.set_reg_e(self.get_reg_c());
                self.pc +=1;
            }
            0x5A => {
                self.set_reg_e(self.get_reg_d());
                self.pc +=1;
            }
            0x5B => {
                self.set_reg_e(self.get_reg_e());
                self.pc +=1;
            }
            0x5C => {
                self.set_reg_e(self.get_reg_h());
                self.pc +=1;
            }
            0x5D => {
                self.set_reg_e(self.get_reg_l());
                self.pc +=1;
            }
            0x5E => {
                self.set_reg_e(self.bus.read(self.get_reg_hl()));
                self.pc +=1;
            }
            0x5F => {
                self.set_reg_e(self.get_reg_a());
                self.pc +=1;
            }
            0x70 => {
                self.bus.write(self.get_reg_hl(), self.get_reg_b());
                self.pc +=1;
            }
            0x60 => {
                self.set_reg_h(self.get_reg_b());
                self.pc +=1;
            }
            0x61 => {
                self.set_reg_h(self.get_reg_c());
                self.pc +=1;
            }
            0x62 => {
                self.set_reg_h(self.get_reg_d());
                self.pc +=1;
            }
            0x63 => {
                self.set_reg_h(self.get_reg_e());
                self.pc +=1;
            }
            0x64 => {
                self.set_reg_h(self.get_reg_h());
                self.pc +=1;
            }
            0x65 => {
                self.set_reg_h(self.get_reg_l());
                self.pc +=1;
            }
            0x66 => {
                self.set_reg_h(self.bus.read(self.get_reg_hl()));
                self.pc +=1;
            }
            0x67 => {
                self.set_reg_h(self.get_reg_a());
                self.pc +=1;
            }
            0x68 => {
                self.set_reg_l(self.get_reg_b());
                self.pc +=1;
            }
            0x69 => {
                self.set_reg_l(self.get_reg_c());
                self.pc +=1;
            }
            0x6A => {
                self.set_reg_l(self.get_reg_d());
                self.pc +=1;
            }
            0x6B => {
                self.set_reg_l(self.get_reg_e());
                self.pc +=1;
            }
            0x6C => {
                self.set_reg_l(self.get_reg_h());
                self.pc +=1;
            }
            0x6D => {
                self.set_reg_l(self.get_reg_l());
                self.pc +=1;
            }
            0x6E => {
                self.set_reg_l(self.bus.read(self.get_reg_hl()));
                self.pc +=1;
            }
            0x6F => {
                self.set_reg_l(self.get_reg_a());
                self.pc +=1;
            }
            0x71 => {
                self.bus.write(self.get_reg_hl(), self.get_reg_c());
                self.pc +=1;
            }
            0x72 => {
                self.bus.write(self.get_reg_hl(), self.get_reg_d());
                self.pc +=1;
            }
            0x73 => {
                self.bus.write(self.get_reg_hl(), self.get_reg_e());
                self.pc +=1;
            }
            0x74 => {
                self.bus.write(self.get_reg_hl(), self.get_reg_h());
                self.pc +=1;
            }
            0x75 => {
                self.bus.write(self.get_reg_hl(), self.get_reg_l());
                self.pc +=1;
            }
            0x77 => {
                self.bus.write(self.get_reg_hl(), self.get_reg_a());
                self.pc +=1;
            }
            0x78 => {
                self.set_reg_a(self.get_reg_b());
                self.pc +=1;
            }
            0x79 => {
                self.set_reg_a(self.get_reg_c());
                self.pc +=1;
            }
            0x7A => {
                self.set_reg_a(self.get_reg_d());
                self.pc +=1;
            }
            0x7B => {
                self.set_reg_a(self.get_reg_e());
                self.pc +=1;
            }
            0x7C => {
                self.set_reg_a(self.get_reg_h());
                self.pc +=1;
            }
            0x7D => {
                self.set_reg_a(self.get_reg_l());
                self.pc +=1;
            }
            0x7E => {
                self.set_reg_a(self.bus.read(self.get_reg_hl()));
                self.pc +=1;
            }
            0x7F => {
                self.set_reg_a(self.get_reg_a());
                self.pc +=1;
            }
            0xFA => {
                let addr = self.read_u16();
                self.set_reg_a(self.bus.read(addr));
                self.pc +=3;
            }
            0xE2 => {
                let n: u16 = self.get_reg_c().into();
                self.bus.write(0xFF00 + n, self.get_reg_a());
                self.pc +=1;
            }
            0xF2 => {
                let n: u16 = self.get_reg_c().into();
                self.set_reg_a(self.bus.read(0xFF00 + n));
                self.pc +=1;
            }
            // ADD
            0x09 => {
                let res = self.add_u16(self.get_reg_hl(), self.get_reg_bc());
                self.set_reg_hl(res);
                self.pc +=1;
            }
            0x19 => {
                let res = self.add_u16(self.get_reg_hl(), self.get_reg_de());
                self.set_reg_hl(res);
                self.pc +=1;
            }
            0x29 => {
                let res = self.add_u16(self.get_reg_hl(), self.get_reg_hl());
                self.set_reg_hl(res);
                self.pc +=1;
            }
            0x39 => {
                let res = self.add_u16(self.get_reg_hl(), self.sp);
                self.set_reg_hl(res);
                self.pc +=1;
            }
            0xC6 => {
                let res = self.add_8(self.get_reg_a(), self.bus.read(self.pc+1));
                self.set_reg_a(res);
                self.pc +=2;
            }
            0xE8 => {
                let res = self.add_i16(self.sp as i16, self.bus.read(self.pc+1) as i8 as i16);
                self.sp = res as u16;
                self.pc +=2;
            }
            // ADD with carry
            0x88 => {
               self.add_a_c(self.get_reg_b());
               self.pc +=1;
            }
            0x89 => {
               self.add_a_c(self.get_reg_c());
               self.pc +=1;
            }
            0x8A => {
               self.add_a_c(self.get_reg_d());
               self.pc +=1;
            }
            0x8B => {
               self.add_a_c(self.get_reg_e());
               self.pc +=1;
            }
            0x8C => {
               self.add_a_c(self.get_reg_h());
               self.pc +=1;
            }
            0x8D => {
               self.add_a_c(self.get_reg_l());
               self.pc +=1;
            }
            0x8E => {
               self.add_a_c(self.bus.read(self.get_reg_hl()));
               self.pc +=1;
            }
            0x8F => {
               self.add_a_c(self.get_reg_a());
               self.pc +=1;
            }
            0xCE => {
               self.add_a_c(self.bus.read(self.pc+1));
               self.pc +=2;
            }
            // SUB
            0xD6 => {
               self.sub_a(self.bus.read(self.pc+1));
               self.pc +=2;
            }
            // SUB with carry
            0x98 => {
               self.sub_a_c(self.get_reg_b());
               self.pc +=1;
            }
            0x99 => {
               self.sub_a_c(self.get_reg_c());
               self.pc +=1;
            }
            0x9A => {
               self.sub_a_c(self.get_reg_d());
               self.pc +=1;
            }
            0x9B => {
               self.sub_a_c(self.get_reg_e());
               self.pc +=1;
            }
            0x9C => {
               self.sub_a_c(self.get_reg_h());
               self.pc +=1;
            }
            0x9D => {
               self.sub_a_c(self.get_reg_l());
               self.pc +=1;
            }
            0x9E => {
               self.sub_a_c(self.bus.read(self.get_reg_hl()));
               self.pc +=1;
            }
            0x9F => {
               self.sub_a_c(self.get_reg_a());
               self.pc +=1;
            }
            0xDE => {
               self.sub_a_c(self.bus.read(self.pc+1));
               self.pc +=2;
            }
            // DEC
            0x05 => {
                self.decrement_register("B");
                self.pc += 1;
            }
            0x0D => {
                self.decrement_register("C");
                self.pc += 1;
            }
            0x1D => {
                self.decrement_register("E");
                self.pc += 1;
            }
            0x25 => {
                self.decrement_register("H");
                self.pc += 1;
            }
            0x35 => {
                self.decrement_register("HL");
                self.pc += 1;
            }
            0x2D => {
                self.decrement_register("L");
                self.pc += 1;
            }
            0x3D => {
                self.decrement_register("A");
                self.pc += 1;
            }
            0x0B => {
                self.set_reg_bc(self.get_reg_bc().overflowing_sub(1).0);
                self.pc += 1;
            }
            0x1B => {
                self.set_reg_de(self.get_reg_de().overflowing_sub(1).0);
                self.pc += 1;
            }
            0x2B => {
                self.set_reg_hl(self.get_reg_hl().overflowing_sub(1).0);
                self.pc += 1;
            }
            0x3B => {
                self.sp = self.sp.overflowing_sub(1).0;
                self.pc += 1;
            }
            // ALU AND
            0xA0 => {
                self.and_a(self.get_reg_b());
                self.pc += 1;
            }
            0xA1 => {
                self.and_a(self.get_reg_c());
                self.pc += 1;
            }
            0xA2 => {
                self.and_a(self.get_reg_d());
                self.pc += 1;
            }
            0xA3 => {
                self.and_a(self.get_reg_e());
                self.pc += 1;
            }
            0xA4 => {
                self.and_a(self.get_reg_h());
                self.pc += 1;
            }
            0xA5 => {
                self.and_a(self.get_reg_l());
                self.pc += 1;
            }
            0xA6 => {
                self.and_a(self.bus.read(self.get_reg_hl()));
                self.pc += 1;
            }
            0xA7 => {
                self.and_a(self.get_reg_a());
                self.pc += 1;
            }
            0xE6 => {
                self.and_a(self.bus.read(self.pc+1));
                self.pc += 2;
            }
            // ALU OR
            0xB0 => {
                self.or_a(self.get_reg_b());
                self.pc += 1;
            }
            0xB1 => {
                self.or_a(self.get_reg_c());
                self.pc += 1;
            }
            0xB2 => {
                self.or_a(self.get_reg_d());
                self.pc += 1;
            }
            0xB3 => {
                self.or_a(self.get_reg_e());
                self.pc += 1;
            }
            0xB4 => {
                self.or_a(self.get_reg_h());
                self.pc += 1;
            }
            0xB5 => {
                self.or_a(self.get_reg_l());
                self.pc += 1;
            }
            0xB6 => {
                self.or_a(self.bus.read(self.get_reg_hl()));
                self.pc += 1;
            }
            0xF6 => {
                self.or_a(self.bus.read(self.pc+1));
                self.pc += 2;
            }
            0xB7 => {
                self.or_a(self.get_reg_a());
                self.pc += 1;
            }
            0x12 => {
                self.bus.write(self.get_reg_de(), self.get_reg_a());
                self.pc +=1;
            }
            // INC 8 BIT
            0x04 => {
                self.increment_register("B");
                self.pc +=1;
            }
            0x14 => {
                self.increment_register("D");
                self.pc +=1;
            }
            0x24 => {
                self.increment_register("H");
                self.pc +=1;
            }
            0x0C => {
                self.increment_register("C");
                self.pc += 1;
            }
            0x1C => {
                self.increment_register("E");
                self.pc += 1;
            }
            0x2C => {
                self.increment_register("L");
                self.pc += 1;
            }
            0x3C => {
                self.increment_register("A");
                self.pc += 1;
            }
            // INC 16 BIT
            0x03 => {
                self.set_reg_bc(self.get_reg_bc().overflowing_add(1).0);
                self.pc += 1;
            }
            0x13 => {
                self.set_reg_de(self.get_reg_de().overflowing_add(1).0);
                self.pc += 1;
            }
            0x23 => {
                self.set_reg_hl(self.get_reg_hl().overflowing_add(1).0);
                self.pc += 1;
            }
            0x33 => {
                self.sp = self.sp.overflowing_add(1).0;
                self.pc += 1;
            }
            // JR
            0x18 => {
                let addr_offset = self.bus.read(self.pc+1) as i8;
                self.pc = ((self.pc + 2) as i16 + addr_offset as i16) as u16;
            }
            0x20 => {
                let addr_offset = self.bus.read(self.pc+1) as i8;
                self.pc +=2;
                if self.get_z_flag() == false {
                    self.pc = (self.pc as i16 + addr_offset as i16) as u16;
                } 
            }
            0x28 => {
                let addr_offset = self.bus.read(self.pc+1) as i8;
                self.pc +=2;
                if self.get_z_flag() {
                    self.pc = (self.pc as i16 + addr_offset as i16) as u16;
                } 
            }
            0x30 => {
                let addr_offset = self.bus.read(self.pc+1) as i8;
                self.pc +=2;
                if self.get_c_flag() == false {
                    self.pc = (self.pc as i16 + addr_offset as i16) as u16;
                } 
            }
            0x38 => {
                let addr_offset = self.bus.read(self.pc+1) as i8;
                self.pc +=2;
                if self.get_c_flag() {
                    self.pc = (self.pc as i16 + addr_offset as i16) as u16;
                } 
            }
            0x22 => {
                let addr = self.get_reg_hl();
                self.bus.write(addr, self.get_reg_a());
                self.set_reg_hl(addr+1);
                self.pc +=1;
            }
            0x32 => {
                let addr = self.get_reg_hl();
                self.bus.write(addr, self.get_reg_a());
                self.set_reg_hl(addr-1);
                self.pc +=1;
            }
            // JUMP
            0xC2 => {
                if self.get_z_flag() == false {
                    let lsb = self.bus.read(self.pc+1);
                    let msb = self.bus.read(self.pc+2);
                    self.pc = u16::from_le_bytes([lsb, msb]);
                } else {
                    self.pc += 3;
                }
            }
            0xD2 => {
                if self.get_c_flag() == false {
                    let lsb = self.bus.read(self.pc+1);
                    let msb = self.bus.read(self.pc+2);
                    self.pc = u16::from_le_bytes([lsb, msb]);
                } else {
                    self.pc += 3;
                }
            }
            0xCA => {
                if self.get_z_flag() {
                    let lsb = self.bus.read(self.pc+1);
                    let msb = self.bus.read(self.pc+2);
                    self.pc = u16::from_le_bytes([lsb, msb]);
                } else {
                    self.pc += 3;
                }
            }
            0xDA => {
                if self.get_c_flag() {
                    let lsb = self.bus.read(self.pc+1);
                    let msb = self.bus.read(self.pc+2);
                    self.pc = u16::from_le_bytes([lsb, msb]);
                } else {
                    self.pc += 3;
                }
            }
            0xC3 => {
                let lsb = self.bus.read(self.pc+1);
                let msb = self.bus.read(self.pc+2);
                self.pc = u16::from_le_bytes([lsb, msb]);
            }
            0xE9 => {
                self.pc = self.get_reg_hl();
            }
            // RET
            0xC9 => {
                let value = self.pop();
                self.pc = value;
            }
            0xD9 => {
                let value = self.pop();
                self.bus.write(0xFFFF, 1);
                self.pc = value;
            }
            0xC0 => {
                self.pc += 1;
                if self.get_z_flag() == false {
                    let value = self.pop();
                    self.pc = value;
                }
            }
            0xC8 => {
                self.pc += 1;
                if self.get_z_flag() {
                    let value = self.pop();
                    self.pc = value;
                }
            }
            0xD0 => {
                self.pc += 1;
                if self.get_c_flag() == false {
                    let value = self.pop();
                    self.pc = value;
                }
            }
            0xD8 => {
                self.pc += 1;
                if self.get_c_flag() {
                    let value = self.pop();
                    self.pc = value;
                }
            }
            // XOR
            0xA8 => {
                self.xor_a(self.get_reg_b());
                self.pc +=1;
            }
            0xA9 => {
                self.xor_a(self.get_reg_c());
                self.pc +=1;
            }
            0xAA => {
                self.xor_a(self.get_reg_d());
                self.pc +=1;
            }
            0xAB => {
                self.xor_a(self.get_reg_e());
                self.pc +=1;
            }
            0xAC => {
                self.xor_a(self.get_reg_h());
                self.pc +=1;
            }
            0xAD => {
                self.xor_a(self.get_reg_l());
                self.pc +=1;
            }
            0xAE => {
                self.xor_a(self.bus.read(self.get_reg_hl()));
                self.pc +=1;
            }
            0xAF => {
                self.set_reg_a(0); // regA xor regA
                self.set_z_flag(true); // regA xor regA is always zero, set zero bit
                self.set_n_flag(false);
                self.set_h_flag(false);
                self.set_c_flag(false);
                self.pc +=1;
            }
            0xEE => {
                self.xor_a(self.bus.read(self.pc+1));
                self.pc +=2;
            }
            // CALL
            0xC4 => {
                let addr = self.read_u16();
                self.pc += 3;
                if self.get_z_flag() == false {
                    self.push(self.pc);
                    self.pc = addr;
                }
                
            }
            0xCC => {
                let addr = self.read_u16();
                self.pc += 3;
                if self.get_z_flag() {
                    self.push(self.pc);
                    self.pc = addr;
                }
                
            }
            0xD4 => {
                let addr = self.read_u16();
                self.pc += 3;
                if self.get_c_flag() == false {
                    self.push(self.pc);
                    self.pc = addr;
                }
                
            }
            0xDC => {
                let addr = self.read_u16();
                self.pc += 3;
                if self.get_c_flag() {
                    self.push(self.pc);
                    self.pc = addr;
                }
                
            }
            0xCD => {
                let addr = self.read_u16();
                self.push(self.pc+3);
                self.pc = addr;
            }
            // ROT A
            0x07 => {
                self.rlc("A");
                self.pc += 1;
            }
            0x17 => {
                self.rl("A");
                self.pc += 1;
            }
            0x0F => {
                self.rrc("A");
                self.pc += 1;
            }
            0x1F => {
                self.rr("A");
                self.pc += 1;
            }
            // PUSH
            0xC5 => {
                self.push(self.get_reg_bc());
                self.pc +=1;
            }
            0xD5 => {
                self.push(self.get_reg_de());
                self.pc +=1;
            }
            0xE5 => {
                self.push(self.get_reg_hl());
                self.pc +=1;
            }
            0xF5 => {
                self.push(self.get_reg_af());
                self.pc +=1;
            }
            // POP
            0xC1 => {
                let val = self.pop();
                self.set_reg_bc(val);
                self.pc +=1;
            }
            0xD1 => {
                let val = self.pop();
                self.set_reg_de(val);
                self.pc +=1;
            }
            0xE1 => {
                let val = self.pop();
                self.set_reg_hl(val);
                self.pc +=1;
            }
            0xF1 => {
                let val = self.pop();
                self.set_reg_af(val);
                self.pc +=1;
            }
            // RST
            0xC7 => {
                self.push(self.pc+1);
                self.pc = 0x0000;
            }
            0xD7 => {
                self.push(self.pc+1);
                self.pc = 0x0010;
            }
            0xE7 => {
                self.push(self.pc+1);
                self.pc = 0x0020;
            }
            0xF7 => {
                self.push(self.pc+1);
                self.pc = 0x0030;
            }
            0xCF => {
                self.push(self.pc+1);
                self.pc = 0x0008;
            }
            0xDF => {
                self.push(self.pc+1);
                self.pc = 0x0018;
            }
            0xEF => {
                self.push(self.pc+1);
                self.pc = 0x0028;
            }
            0xFF => {
                self.push(self.pc+1);
                self.pc = 0x0038;
            }
            // LDH
            0xE0 => {
                let n: u16 = self.bus.read(self.pc+1).into();
                self.bus.write(0xFF00 + n, self.get_reg_a());
                self.pc +=2;
            }
            0xF0 => {
                let n: u16 = self.bus.read(self.pc+1).into();
                self.set_reg_a(self.bus.read(0xFF00 + n));
                self.pc += 2;
            }
            // CMP
            0xB8 => {
                self.cp(self.get_reg_b());
                self.pc += 1;
            }
            0xB9 => {
                self.cp(self.get_reg_c());
                self.pc += 1;
            }
            0xBA => {
                self.cp(self.get_reg_d());
                self.pc += 1;
            }
            0xBB => {
                self.cp(self.get_reg_e());
                self.pc += 1;
            }
            0xBC => {
                self.cp(self.get_reg_h());
                self.pc += 1;
            }
            0xBD => {
                self.cp(self.get_reg_l());
                self.pc += 1;
            }
            0xBE => {
                self.cp(self.bus.read(self.get_reg_hl()));
                self.pc += 1;
            }
            0xBF => {
                self.cp(self.get_reg_a());
                self.pc += 1;
            }
            0xFE => {
                self.cp(self.bus.read(self.pc+1));
                self.pc += 2;
            }
            _ => panic!("Unknown instruction 0x{:02X}", inst)
        };
    }

    pub fn run(&mut self) {
        loop {
            self.run_next_instruction();
        }
    }

    fn cp(&mut self, value: u8) {
        let a = self.get_reg_a();
        self.set_z_flag(a == value);
        self.set_n_flag(true);
        // TODO : verify and check for bitvec/bitslice ops
        self.set_h_flag((a & 0b1111) < (value & 0b1111));
        self.set_c_flag(a < value);
    }

    fn read_u16(&self) -> u16 {
        let lsb = self.bus.read(self.pc+1);
        let msb = self.bus.read(self.pc+2);
        u16::from_le_bytes([lsb, msb])
    }

    fn push(&mut self, value: u16) {
        let [msb,lsb] = u16::to_be_bytes(value);
        self.sp -= 1;
        self.bus.write(self.sp, msb);
        self.sp -= 1;
        self.bus.write(self.sp, lsb);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read(self.sp); 
        self.sp += 1;
        let msb = self.bus.read(self.sp); 
        self.sp += 1;

        return u16::from_le_bytes([lsb,msb]);
    
    }

    fn increment_register(&mut self, reg_id: &str) {
        if reg_id == "HL" {
            let val = self.bus.read(self.get_reg_hl());
            let low_nibble = val & 0b1111;
            let (val, _) = val.overflowing_add(1);
            self.bus.write(self.get_reg_hl(), val);
            let low_nibble_new = val & 0b1111;
            self.set_z_flag(val == 0);
            self.set_n_flag(false);
            self.set_h_flag(low_nibble + low_nibble_new > 0b1111);
            return 
        }
        let reg = match reg_id {
            "A" => &mut self.reg_af[0..8],
            "B" => &mut self.reg_bc[0..8],
            "C" => &mut self.reg_bc[8..16],
            "D" => &mut self.reg_de[0..8],
            "E" => &mut self.reg_de[8..16],
            "H" => &mut self.reg_hl[0..8],
            "L" => &mut self.reg_hl[8..16],
              _ => panic!("Unknown register {} used.", reg_id)
        }; 
        let val = reg.load_be::<u8>().overflowing_add(1).0;
        let low_nibble = val & 0b1111;
        reg.store_be(val);
        let low_nibble_new = val & 0b1111;
        self.set_z_flag(val == 0);
        self.set_n_flag(false);
        self.set_h_flag(low_nibble < low_nibble_new);
    }

    fn decrement_register(&mut self, reg_id: &str) {
        if reg_id == "HL" {
            let val = self.bus.read(self.get_reg_hl());
            let low_nibble = val & 0b1111;
            let (val, _) = val.overflowing_sub(1);
            self.bus.write(self.get_reg_hl(), val);
            let low_nibble_new = val & 0b1111;
            self.set_z_flag(val == 0);
            self.set_n_flag(true);
            self.set_h_flag(low_nibble < low_nibble_new);
            return 
        }
        let reg = match reg_id {
            "A" => &mut self.reg_af[0..8],
            "B" => &mut self.reg_bc[0..8],
            "C" => &mut self.reg_bc[8..16],
            "D" => &mut self.reg_de[0..8],
            "E" => &mut self.reg_de[8..16],
            "H" => &mut self.reg_hl[0..8],
            "L" => &mut self.reg_hl[8..16],
              _ => panic!("Unknown register {} used.", reg_id)
        };
        let low_nibble = reg[4..8].load_be::<u8>();
        let (val, _) = reg.load_be::<u8>().overflowing_sub(1);
        reg.store_be(val);
        let low_nibble_new = reg[4..8].load_be();
        self.set_z_flag(val == 0);
        self.set_n_flag(true);
        self.set_h_flag(low_nibble < low_nibble_new);
    }

    fn and_a(&mut self, value: u8) {
        self.set_reg_a(self.get_reg_a() & value);
        self.reset_flags();
        self.set_z_flag(self.get_reg_a() == 0);
        self.set_h_flag(true);
    }

    fn or_a(&mut self, value: u8) {
        self.set_reg_a(self.get_reg_a() | value);
        self.reset_flags();
        self.set_z_flag(self.get_reg_a() == 0);
    }

    fn xor_a(&mut self, value: u8) {
        self.set_reg_a(self.get_reg_a() ^ value);
        self.reset_flags();
        self.set_z_flag(self.get_reg_a() == 0);
    }

    fn add_u16(&mut self, a: u16, b: u16) -> u16 {
        let (res, carry) = a.overflowing_add(b);
        /*self.set_z_flag(res == 0);*/
        self.set_n_flag(false);
        self.set_h_flag((a & 0b1111111111) + (b & 0b1111111111) > 0b1111111111);
        self.set_c_flag(carry);

        return res
    }

    // Used in 0xE8 = ADD SP, r8
    fn add_i16(&mut self, a: i16, b: i16) -> i16 {
        let (res, _) = a.overflowing_add(b);
        self.set_z_flag(false);
        self.set_n_flag(false);
        if b >= 0 {
            self.set_h_flag((a & 0b1111) + (b & 0b1111) > 0b1111);
            self.set_c_flag((a & 0b11111111) + (b & 0b11111111) > 0b11111111);
        } else {
            self.set_h_flag((res & 0b1111) <= (a & 0b1111));
            self.set_c_flag((res & 0b11111111) <= (a & 0b11111111));
        }

        return res
    }

    fn add_8(&mut self, a: u8, b: u8) -> u8 {
        let (res, carry) = a.overflowing_add(b);
        self.set_z_flag(res == 0);
        self.set_n_flag(false);
        self.set_h_flag((a & 0b1111) + (b & 0b1111) > 0b1111);
        self.set_c_flag(carry);

        return res
    }

    fn add_a_c(&mut self, value: u8) {
        let carry = self.get_c_flag() as u8;
        self.add_a(value);
        let c = self.get_c_flag();
        let h = self.get_h_flag();
        self.add_a(carry);
        self.set_c_flag(c || self.get_c_flag());
        self.set_h_flag(h || self.get_h_flag());
    }

    fn add_a(&mut self, value: u8) {
        let a = self.get_reg_a();
        let (res, carry) = a.overflowing_add(value);
        self.set_reg_a(res);
        self.set_z_flag(res == 0);
        self.set_n_flag(false);
        self.set_h_flag((a & 0b1111) + (value & 0b1111) > 0b1111);
        self.set_c_flag(carry);
    }

    fn sub_a(&mut self, value: u8) {
        let a = self.get_reg_a();
        let (res, carry) = a.overflowing_sub(value);
        self.set_reg_a(res);
        self.set_z_flag(res == 0);
        self.set_n_flag(true);
        self.set_h_flag((a & 0b1111) < (value & 0b1111));
        /*self.set_c_flag((res & 0b11111111) <= (a & 0b11111111));*/
        self.set_c_flag(carry);
    }

    fn sub_a_c(&mut self, value: u8) {
        let carry = self.get_c_flag() as u8;
        self.sub_a(value);
        let c = self.get_c_flag();
        let h = self.get_h_flag();
        self.sub_a(carry);
        self.set_c_flag(c || self.get_c_flag());
        self.set_h_flag(h || self.get_h_flag());
    }

    fn reset_flags(&mut self) {
        self.set_z_flag(false);
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn set_reg_af(&mut self, value: u16) {
        self.reg_af.store_be(value);
        self.reg_af[12..16].store_be(0);
    }

    fn set_reg_bc(&mut self, value: u16) {
        self.reg_bc.store_be(value);
    }

    fn set_reg_de(&mut self, value: u16) {
        self.reg_de.store_be(value);
    }

    fn set_reg_hl(&mut self, value: u16) {
        self.reg_hl.store_be(value);
    }

    fn set_reg_a(&mut self, value: u8) {
        self.reg_af[0..8].store_be(value);
    }

    fn set_reg_b(&mut self, value: u8) {
        self.reg_bc[0..8].store_be(value);

    }
    fn set_reg_c(&mut self, value: u8) {
        self.reg_bc[8..16].store_be(value);
    }

    fn set_reg_d(&mut self, value: u8) {
        self.reg_de[0..8].store_be(value);
    }

    fn set_reg_e(&mut self, value: u8) {
        self.reg_de[8..16].store_be(value);
    }

    fn set_reg_h(&mut self, value: u8) {
        self.reg_hl[0..8].store_be(value);
    }

    fn set_reg_l(&mut self, value: u8) {
        self.reg_hl[8..16].store_be(value);
    }

    fn get_reg_af(&self) -> u16 {
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

    pub fn get_reg_f(&self) -> u8 {
        self.reg_af[8..16].load_be::<u8>()
    }

    pub fn get_reg_b(&self) -> u8 {
        self.reg_bc[0..8].load_be::<u8>()
    }

    pub fn view_reg_b(&self) -> &BitSlice<u16, Msb0> {
        &self.reg_bc[0..8]
    }

    pub fn view_reg_c(&self) -> &BitSlice<u16, Msb0> {
        &self.reg_bc[8..16]
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

    pub fn get_z_flag(&self) -> bool {
        self.reg_af[8]
    }

    pub fn get_n_flag(&self) -> bool {
        self.reg_af[9]
    }

    pub fn get_h_flag(&self) -> bool {
        self.reg_af[10]
    }

    pub fn get_c_flag(&self) -> bool {
        self.reg_af[11]
    }

    fn set_z_flag(&mut self, value: bool) {
        self.reg_af.set(8, value);
    }

    fn set_n_flag(&mut self, value: bool) {
        self.reg_af.set(9, value);
    }

    fn set_h_flag(&mut self, value: bool) {
        self.reg_af.set(10, value);
    }

    fn set_c_flag(&mut self, value: bool) {
        self.reg_af.set(11, value);
    }

    pub fn perform_cb_instruction(&mut self) {
        let inst = self.bus.read(self.pc);
        match inst {
            0x00 => self.rlc("B"),
            0x01 => self.rlc("C"),
            0x02 => self.rlc("D"),
            0x03 => self.rlc("E"),
            0x04 => self.rlc("H"),
            0x05 => self.rlc("L"),
            0x07 => self.rlc("A"),
            0x08 => self.rrc("B"),
            0x09 => self.rrc("C"),
            0x0A => self.rrc("D"),
            0x0B => self.rrc("E"),
            0x0C => self.rrc("H"),
            0x0D => self.rrc("L"),
            0x0F => self.rrc("A"),
            0x10 => self.rl("B"),
            0x11 => self.rl("C"),
            0x12 => self.rl("D"),
            0x13 => self.rl("E"),
            0x14 => self.rl("H"),
            0x15 => self.rl("L"),
            0x17 => self.rl("A"),
            0x18 => self.rr("B"),
            0x19 => self.rr("C"),
            0x1A => self.rr("D"),
            0x1B => self.rr("E"),
            0x1C => self.rr("H"),
            0x1D => self.rr("L"),
            0x1F => self.rr("A"),
            0x30 => self.swap("B"),
            0x31 => self.swap("C"),
            0x32 => self.swap("D"),
            0x33 => self.swap("E"),
            0x34 => self.swap("F"),
            0x35 => self.swap("H"),
            0x36 => self.swap("L"),
            0x37 => self.swap("A"),
            0x38 => self.srl("B"),
            0x39 => self.srl("C"),
            0x3A => self.srl("D"),
            0x3B => self.srl("E"),
            0x3C => self.srl("H"),
            0x3D => self.srl("L"),
            0x3F => self.srl("A"),
            _ => panic!("Unknown CB instruction 0x{:02X}", inst)
        }
    }

    fn rlc(&mut self, reg_id: &str) {
        let reg = match reg_id {
            "A" => &mut self.reg_af[0..8],
            "B" => &mut self.reg_bc[0..8],
            "C" => &mut self.reg_bc[8..16],
            "D" => &mut self.reg_de[0..8],
            "E" => &mut self.reg_de[8..16],
            "H" => &mut self.reg_hl[0..8],
            "L" => &mut self.reg_hl[8..16],
              _ => panic!("Unknown register {} used.", reg_id)
        };
        let carry = reg[0];
        reg.rotate_left(1);
        let zero = reg_id != "A" && reg.load_be::<u8>() == 0;
        self.reset_flags();
        self.set_c_flag(carry);
        self.set_z_flag(zero);

    }

    fn rrc(&mut self, reg_id: &str) {
        let reg = match reg_id {
            "A" => &mut self.reg_af[0..8],
            "B" => &mut self.reg_bc[0..8],
            "C" => &mut self.reg_bc[8..16],
            "D" => &mut self.reg_de[0..8],
            "E" => &mut self.reg_de[8..16],
            "H" => &mut self.reg_hl[0..8],
            "L" => &mut self.reg_hl[8..16],
              _ => panic!("Unknown register {} used.", reg_id)
        };
        let carry = reg[7];
        reg.shift_right(1);
        let zero = reg_id != "A" && reg.load_be::<u8>() == 0;
        self.reset_flags();
        self.set_c_flag(carry);
        self.set_z_flag(zero);

    }

    fn rl(&mut self, reg_id: &str) {
        let carry = self.get_c_flag();
        let reg = match reg_id {
            "A" => &mut self.reg_af[0..8],
            "B" => &mut self.reg_bc[0..8],
            "C" => &mut self.reg_bc[8..16],
            "D" => &mut self.reg_de[0..8],
            "E" => &mut self.reg_de[8..16],
            "H" => &mut self.reg_hl[0..8],
            "L" => &mut self.reg_hl[8..16],
              _ => panic!("Unknown register {} used.", reg_id)
        };
        let new_carry = reg[0];
        reg.shift_left(1);
        reg.set(7,carry);
        let zero = reg_id != "A" && reg.load_be::<u8>() == 0;
        self.reset_flags();
        self.set_c_flag(new_carry);
        self.set_z_flag(zero);

    }

    fn swap(&mut self, reg_id: &str) {
        let reg = match reg_id {
            "A" => &mut self.reg_af[0..8],
            "B" => &mut self.reg_bc[0..8],
            "C" => &mut self.reg_bc[8..16],
            "D" => &mut self.reg_de[0..8],
            "E" => &mut self.reg_de[8..16],
            "H" => &mut self.reg_hl[0..8],
            "L" => &mut self.reg_hl[8..16],
              _ => panic!("Unknown register {} used.", reg_id)
        };
        reg.swap(0,4);
        reg.swap(1,5);
        reg.swap(2,6);
        reg.swap(3,7);
        let zero = reg.load_be::<u8>() == 0;
        self.reset_flags();
        self.set_z_flag(zero);

    }

    fn rr(&mut self, reg_id: &str) {
        let carry = self.get_c_flag();
        let reg = match reg_id {
            "A" => &mut self.reg_af[0..8],
            "B" => &mut self.reg_bc[0..8],
            "C" => &mut self.reg_bc[8..16],
            "D" => &mut self.reg_de[0..8],
            "E" => &mut self.reg_de[8..16],
            "H" => &mut self.reg_hl[0..8],
            "L" => &mut self.reg_hl[8..16],
              _ => panic!("Unknown register {} used.", reg_id)
        };
        let new_carry = reg[7];
        reg.shift_right(1);
        reg.set(0,carry);
        let zero = reg_id != "A" && reg.load_be::<u8>() == 0;
        self.reset_flags();
        self.set_c_flag(new_carry);
        self.set_z_flag(zero);

    }

    fn srl(&mut self, reg_id: &str) {
        let reg = match reg_id {
            "A" => &mut self.reg_af[0..8],
            "B" => &mut self.reg_bc[0..8],
            "C" => &mut self.reg_bc[8..16],
            "D" => &mut self.reg_de[0..8],
            "E" => &mut self.reg_de[8..16],
            "H" => &mut self.reg_hl[0..8],
            "L" => &mut self.reg_hl[8..16],
              _ => panic!("Unknown register {} used.", reg_id)
        };
        let carry = reg[7];
        reg.shift_right(1);
        let zero = reg.load_be::<u8>() == 0;
        self.reset_flags();
        self.set_c_flag(carry);
        self.set_z_flag(zero);

    }

    /*fn decrement_register(&mut self, reg_id: &str) {*/
        /*let reg = match reg_id {*/
            /*"A" => &mut self.reg_af[0..8],*/
            /*"B" => &mut self.reg_bc[0..8],*/
            /*"C" => &mut self.reg_bc[8..16],*/
            /*"L" => &mut self.reg_hl[8..16],*/
              /*_ => panic!("Unknown register {} used.", reg_id)*/
        /*};*/
        /*let low_nibble = reg[4..8].load_be::<u8>();*/
        /*let (val, _) = reg.load_be::<u8>().overflowing_sub(1);*/
        /*reg.store_be(val);*/
        /*let low_nibble_new = reg[4..8].load_be();*/
        /*self.set_z_flag(val == 0);*/
        /*self.set_n_flag(true);*/
        /*self.set_h_flag(low_nibble < low_nibble_new);*/
    /*}*/
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

    #[test]
    fn inst_0x05() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.get_h_flag(), false);
        cpu.set_reg_b(0b10000);
        cpu.perform_instruction(0x05);
        assert_eq!(cpu.get_reg_b(), 0b1111);
        assert_eq!(cpu.get_n_flag(), true);
        assert_eq!(cpu.get_h_flag(), true);
        cpu.reset_flags();
        cpu.set_reg_b(0b100);
        cpu.perform_instruction(0x05);
        assert_eq!(cpu.get_reg_b(), 0b11);
        assert_eq!(cpu.get_n_flag(), true);
        assert_eq!(cpu.get_h_flag(), false);
        cpu.reset_flags();
        cpu.set_reg_b(0b0);
        cpu.perform_instruction(0x05);
        assert_eq!(cpu.get_reg_b(), 0b11111111);
        assert_eq!(cpu.get_n_flag(), true);
        assert_eq!(cpu.get_h_flag(), true);
    }

    #[test]
    fn inst_0x0D() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.get_h_flag(), false);
        cpu.set_reg_c(0b10000);
        cpu.perform_instruction(0x0D);
        assert_eq!(cpu.get_reg_c(), 0b1111);
        assert_eq!(cpu.get_n_flag(), true);
        assert_eq!(cpu.get_h_flag(), true);
        cpu.reset_flags();
        cpu.set_reg_c(0b100);
        cpu.perform_instruction(0x0D);
        assert_eq!(cpu.get_reg_c(), 0b11);
        assert_eq!(cpu.get_n_flag(), true);
        assert_eq!(cpu.get_h_flag(), false);
        cpu.reset_flags();
        cpu.set_reg_c(0b0);
        cpu.perform_instruction(0x0D);
        assert_eq!(cpu.get_reg_c(), 0b11111111);
        assert_eq!(cpu.get_n_flag(), true);
        assert_eq!(cpu.get_h_flag(), true);
    }
}
