use crate::constants::{self, AddressingMode, StatusFlag, find_opcode};

pub struct CPU {
    pub reg_a: u8,
    pub status: u8,
    pub program_counter: u16,
    pub reg_x: u8,
    pub reg_y: u8,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg_a: 0,
            status: 0,
            program_counter: 0,
            reg_x: 0,
            reg_y: 0,
            memory: [0; 0xFFFF],
        }
    }

    // INSTRUCTIONS START

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.reg_a = value;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn tax(&mut self) {
        self.reg_x = self.reg_a;
        self.update_zero_and_negative_flags(self.reg_x);
    }

    fn inx(&mut self) {
        if self.reg_x == 0xFF {
            self.reg_x = 0;
        } else {
            self.reg_x += 1;
        }
        self.update_zero_and_negative_flags(self.reg_x);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.reg_a);
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.add_to_reg_a(value);
    }

    // INSTRUCTIONS END

    // FLAGS START

    pub fn set_flag(&mut self, flag: StatusFlag) {
        self.status |= flag as u8;
    }

    pub fn clear_flag(&mut self, flag: StatusFlag) {
        self.status &= !(flag as u8);
    }

    pub fn check_flag(&self, flag: StatusFlag) -> bool {
        self.status & (flag as u8) != 0
    }
    
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.set_flag(StatusFlag::Zero);
        } else {
            self.clear_flag(StatusFlag::Zero)
        }

        if result & 0b1000_0000 != 0 {
            self.set_flag(StatusFlag::Negative);
        } else {
            self.clear_flag(StatusFlag::Negative);
        }
    }

    // FLAGS END

    // REGISTERS START

    fn set_reg_a(&mut self, value: u8) {
        self.reg_a = value;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn add_to_reg_a(&mut self, data: u8) {
        let sum = self.reg_a as u16
            + data as u16
            + (if self.check_flag(StatusFlag::Carry) {
                1
            } else {
                0
            }) as u16;

        let carry = sum > 0xff;

        if carry {
            self.set_flag(StatusFlag::Carry);
        } else {
            self.clear_flag(StatusFlag::Carry);
        }

        let result = sum as u8;

        if (data ^ result) & (result ^ self.reg_a) & 0x80 != 0 {
            self.set_flag(StatusFlag::Overflow);
        } else {
            self.clear_flag(StatusFlag::Overflow);
        }

        self.set_reg_a(result);
    }

    // REGISTERS END

    // OPERANDS START

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,

            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.reg_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.reg_y) as u16;
                addr
            }

            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.reg_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.reg_y as u16);
                addr
            }

            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.reg_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.reg_y as u16);
                deref
            }

            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    // OPERANDS END

    // MEM START

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    // MEM END

    // CONTROL START

    pub fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_x = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }

    pub fn run(&mut self) {
        loop {
            let opcode_val = self.mem_read(self.program_counter);
            let op = find_opcode(opcode_val).expect("Unknown opcode");
            self.program_counter += 1;

            match op.name {
                "LDA" => {
                    self.lda(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "STA" => {
                    self.sta(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "TAX" => {
                    self.tax();
                }
                "INX" => {
                    self.inx();
                }
                "ADC" => {
                    self.adc(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "BRK" => {
                    return;
                }
                _ => todo!(),
            }
        }
    }
}
