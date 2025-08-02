use crate::constants::{AddressingMode, STACK_OFFSET, StackError, StatusFlag, find_opcode};

pub struct CPU {
    pub reg_a: u8,
    pub status: u8,
    pub program_counter: u16,
    pub reg_x: u8,
    pub reg_y: u8,
    pub stp: u8,
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
            stp: 0xff,
            memory: [0; 0xFFFF],
        }
    }

    // STACK COMMANDS START

    fn push(&mut self, value: u8) -> Result<(), StackError> {
        if self.stp == 0 {
            return Err(StackError {
                counter: self.program_counter,
                err_msg: "Stack Overflow".to_owned(),
            });
        }
        self.stp -= 1;
        self.mem_write(STACK_OFFSET + self.stp as u16, value);
        Ok(())
    }

    fn push_u16(&mut self, value: u16) -> Result<(), StackError> {
        if self.stp < 2 {
            return Err(StackError {
                counter: self.program_counter,
                err_msg: "Stack Overflow on u16".to_owned(),
            });
        }
        self.stp -= 2;
        self.mem_write_u16(STACK_OFFSET + self.stp as u16, value);
        Ok(())
    }

    fn pop(&mut self) -> Result<u8, StackError> {
        if self.stp == 0xff {
            return Err(StackError {
                counter: self.program_counter,
                err_msg: "Stack Underflow".to_owned(),
            });
        }
        let value = self.mem_read(STACK_OFFSET + self.stp as u16);
        self.stp += 1;
        Ok(value)
    }

    fn pop_u16(&mut self) -> Result<u16, StackError> {
        if self.stp == 0xfe {
            return Err(StackError {
                counter: self.program_counter,
                err_msg: "Stack Underflow on u16".to_owned(),
            });
        }
        let value = self.mem_read_u16(STACK_OFFSET + self.stp as u16);
        self.stp += 2;
        Ok(value)
    }

    // STACK COMMANDS START

    // INSTRUCTIONS START

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.reg_a = value;
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.reg_x = value;
        self.update_zero_and_negative_flags(self.reg_x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.reg_y = value;
        self.update_zero_and_negative_flags(self.reg_y);
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

    fn iny(&mut self) {
        if self.reg_y == 0xFF {
            self.reg_y = 0;
        } else {
            self.reg_y += 1;
        }
        self.update_zero_and_negative_flags(self.reg_y);
    }

    fn dex(&mut self) {
        if self.reg_x == 0x00 {
            self.reg_x = 0b1111_1111;
        } else {
            self.reg_x -= 1;
        }
        self.update_zero_and_negative_flags(self.reg_x);
    }

    fn dey(&mut self) {
        if self.reg_y == 0x00 {
            self.reg_y = 0b1111_1111;
        } else {
            self.reg_y -= 1;
        }
        self.update_zero_and_negative_flags(self.reg_y);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.reg_a);
    }

    fn pla(&mut self) {
        match self.pop() {
            Ok(value) => {
                self.reg_a = value;
                self.update_zero_and_negative_flags(self.reg_a);
            }
            Err(e) => eprintln!("{}", e),
        };
    }

    fn plp(&mut self) {
        match self.pop() {
            Ok(value) => {
                self.status = value;
            }
            Err(e) => eprintln!("{}", e),
        };
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.add_to_reg_a(value);
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.add_to_reg_a(((value as i8).wrapping_neg().wrapping_sub(1)) as u8);
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = value & self.reg_a;
        self.set_reg_a(result);
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = value ^ self.reg_a;
        self.set_reg_a(result);
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = value | self.reg_a;
        self.set_reg_a(result);
        self.update_zero_and_negative_flags(self.reg_a);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let mut addr: u16 = 0;
        let mut value = match mode {
            AddressingMode::Immediate => self.reg_a,
            _ => {
                addr = self.get_operand_address(mode);
                self.mem_read(addr)
            }
        };

        if value & 0b1000_0000 == 0 {
            self.clear_flag(StatusFlag::Carry);
        } else {
            self.set_flag(StatusFlag::Carry);
        }

        value = value << 1;
        self.update_zero_and_negative_flags(value);

        match mode {
            AddressingMode::Immediate => {
                self.set_reg_a(value);
            }
            _ => {
                self.mem_write(addr, value);
            }
        };
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let mut addr: u16 = 0;
        let mut value = match mode {
            AddressingMode::Immediate => self.reg_a,
            _ => {
                addr = self.get_operand_address(mode);
                self.mem_read(addr)
            }
        };

        if value & 0b1000_0000 == 0 {
            value = value << 1;
            value &= 0b1111_1110;
            value += match self.check_flag(StatusFlag::Carry) {
                true => 1,
                false => 0,
            };
            self.update_zero_and_negative_flags(value);
            self.clear_flag(StatusFlag::Carry);
        } else {
            value = value << 1;
            value &= 0b1111_1110;
            value += match self.check_flag(StatusFlag::Carry) {
                true => 1,
                false => 0,
            };
            self.update_zero_and_negative_flags(value);
            self.set_flag(StatusFlag::Carry);
        }

        match mode {
            AddressingMode::Immediate => {
                self.set_reg_a(value);
            }
            _ => {
                self.mem_write(addr, value);
            }
        };
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let mut addr: u16 = 0;
        let mut value = match mode {
            AddressingMode::Immediate => self.reg_a,
            _ => {
                addr = self.get_operand_address(mode);
                self.mem_read(addr)
            }
        };

        if value & 0b0000_0001 == 0 {
            value = value >> 1;
            value &= 0b0111_1111;
            value |= match self.check_flag(StatusFlag::Carry) {
                true => 0b1000_0000,
                false => 0,
            };
            self.update_zero_and_negative_flags(value);
            self.clear_flag(StatusFlag::Carry);
        } else {
            value = value >> 1;
            value &= 0b0111_1111;
            value |= match self.check_flag(StatusFlag::Carry) {
                true => 0b1000_0000,
                false => 0,
            };
            self.update_zero_and_negative_flags(value);
            self.set_flag(StatusFlag::Carry);
        }

        match mode {
            AddressingMode::Immediate => {
                self.set_reg_a(value);
            }
            _ => {
                self.mem_write(addr, value);
            }
        };
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let mut addr: u16 = 0;
        let mut value = match mode {
            AddressingMode::Immediate => self.reg_a,
            _ => {
                addr = self.get_operand_address(mode);
                self.mem_read(addr)
            }
        };

        if value & 0b0000_0001 == 0 {
            self.clear_flag(StatusFlag::Carry);
        } else {
            self.set_flag(StatusFlag::Carry);
        }

        value = value >> 1;
        self.update_zero_and_negative_flags(value);

        match mode {
            AddressingMode::Immediate => {
                self.set_reg_a(value);
            }
            _ => {
                self.mem_write(addr, value);
            }
        };
    }

    fn cmp(&mut self, mode: &AddressingMode, reg: u8) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        if reg >= value {
            self.set_flag(StatusFlag::Carry);
            self.clear_flag(StatusFlag::Negative);
        } else {
            self.clear_flag(StatusFlag::Carry);
            self.set_flag(StatusFlag::Negative);
        }

        if reg == value {
            self.set_flag(StatusFlag::Zero);
        } else {
            self.clear_flag(StatusFlag::Zero);
        }
    }

    fn branch(&mut self, condition: bool) {
        if !condition {
            return;
        }
        let value = self.mem_read(self.program_counter) as u16;
        self.program_counter = if value < 128 {
            self.program_counter + value
        } else {
            self.program_counter + 128 - value
        }
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        if value & 0b1000_0000 != 0 {
            self.set_flag(StatusFlag::Negative);
        } else {
            self.clear_flag(StatusFlag::Negative);
        }
        if value & 0b0100_0000 != 0 {
            self.set_flag(StatusFlag::Overflow);
        } else {
            self.clear_flag(StatusFlag::Overflow);
        }
        if value & self.reg_a == 0 {
            self.set_flag(StatusFlag::Zero);
        } else {
            self.clear_flag(StatusFlag::Zero);
        }
    }
    fn dec(&mut self, mode: &AddressingMode) {
        let mut addr: u16 = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);

        let result: u8 = match value {
            0 => 0b1111_1111,
            anything => anything - 1,
        };

        self.update_zero_and_negative_flags(result);

        self.mem_write(addr, result);
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let mut addr: u16 = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);

        let result: u8 = match value {
            0b1111_1111 => 0,
            anything => anything + 1,
        };

        self.update_zero_and_negative_flags(result);

        self.mem_write(addr, result);
    }

    fn jmp(&mut self, mode: &AddressingMode) {
        self.program_counter = self.get_operand_address(mode);
    }

    fn jsr(&mut self, mode: &AddressingMode) {
        match self.push_u16(self.program_counter + 1) {
            Err(e) => {
                eprintln!("{}", e)
            }
            _ => {}
        }
        self.program_counter = self.get_operand_address(mode);
    }

    fn brk(&mut self) {
        match self.push_u16(self.program_counter + 1) {
            Err(e) => {
                eprintln!("{}", e)
            }
            _ => {}
        }
        match self.push(self.status) {
            Err(e) => {
                eprintln!("{}", e)
            }
            _ => {}
        }
        self.set_flag(StatusFlag::Break);
        self.program_counter = self.mem_read_u16(0xfffe)
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
            AddressingMode::Indirect => {
                let ptr: u16 = self.mem_read_u16(self.program_counter);
                let addr: u16 = self.mem_read_u16(ptr);
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
            AddressingMode::Relative => 0,

            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    // OPERANDS END

    // MEM START

    pub fn mem_read(&self, addr: u16) -> u8 {
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
                "LDX" => {
                    self.ldx(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "LDY" => {
                    self.ldy(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "STA" => {
                    self.sta(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "PLA" => {
                    self.pla();
                }
                "PLP" => {
                    self.plp();
                }
                "PHA" => match self.push(self.reg_a) {
                    Err(e) => {
                        eprintln!("{}", e)
                    }
                    _ => {}
                },
                "PHP" => match self.push(self.status) {
                    Err(e) => {
                        eprintln!("{}", e)
                    }
                    _ => {}
                },
                "TAX" => {
                    self.tax();
                }
                "INX" => {
                    self.inx();
                }
                "INY" => {
                    self.iny();
                }
                "DEX" => {
                    self.dex();
                }
                "DEY" => {
                    self.dey();
                }
                "CLC" => {
                    self.clear_flag(StatusFlag::Carry);
                }
                "SEC" => {
                    self.set_flag(StatusFlag::Carry);
                }
                "CLD" => {
                    self.clear_flag(StatusFlag::DecimalMode);
                }
                "SED" => {
                    self.set_flag(StatusFlag::DecimalMode);
                }
                "CLI" => {
                    self.clear_flag(StatusFlag::InterruptDisable);
                }
                "SEI" => {
                    self.set_flag(StatusFlag::InterruptDisable);
                }
                "CLV" => {
                    self.clear_flag(StatusFlag::Overflow);
                }
                "ADC" => {
                    self.adc(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "SBC" => {
                    self.sbc(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "AND" => {
                    self.and(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "EOR" => {
                    self.eor(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "ORA" => {
                    self.ora(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "ASL" => {
                    self.asl(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "ROL" => {
                    self.rol(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "LSR" => {
                    self.lsr(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "ROR" => {
                    self.ror(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "BCC" => {
                    self.branch(!self.check_flag(StatusFlag::Carry));
                    self.program_counter += op.bytes as u16 - 1;
                }
                "BCS" => {
                    self.branch(self.check_flag(StatusFlag::Carry));
                    self.program_counter += op.bytes as u16 - 1;
                }
                "BEQ" => {
                    self.branch(self.check_flag(StatusFlag::Zero));
                    self.program_counter += op.bytes as u16 - 1;
                }
                "BNE" => {
                    self.branch(!self.check_flag(StatusFlag::Zero));
                    self.program_counter += op.bytes as u16 - 1;
                }
                "BMI" => {
                    self.branch(self.check_flag(StatusFlag::Negative));
                    self.program_counter += op.bytes as u16 - 1;
                }
                "BPL" => {
                    self.branch(!self.check_flag(StatusFlag::Negative));
                    self.program_counter += op.bytes as u16 - 1;
                }
                "BVS" => {
                    self.branch(self.check_flag(StatusFlag::Overflow));
                    self.program_counter += op.bytes as u16 - 1;
                }
                "BIT" => {
                    self.bit(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "CMP" => {
                    self.cmp(&op.add_mode, self.reg_a);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "CPX" => {
                    self.cmp(&op.add_mode, self.reg_x);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "CPY" => {
                    self.cmp(&op.add_mode, self.reg_y);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "INC" => {
                    self.inc(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "DEC" => {
                    self.dec(&op.add_mode);
                    self.program_counter += op.bytes as u16 - 1;
                }
                "JMP" => {
                    self.jmp(&op.add_mode);
                }
                "JSR" => {
                    self.jsr(&op.add_mode);
                }
                "NOP" => {
                    continue;
                }
                "BRK" => {
                    self.brk();
                    return;
                }
                _ => todo!(),
            }
        }
    }
}
