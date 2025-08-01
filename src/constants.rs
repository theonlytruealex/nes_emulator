pub enum StatusFlag {
    Carry = 0b0000_0001,
    Zero = 0b0000_0010,
    InterruptDisable = 0b0000_0100,
    DecimalMode = 0b0000_1000,
    Break = 0b0001_0000,
    Unused = 0b0010_0000,
    Overflow = 0b0100_0000,
    Negative = 0b1000_0000,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
    Relative
}

pub struct OpCode {
    pub code: u8,
    pub name: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub add_mode: AddressingMode,
}

pub static CPU_OP_CODES: [OpCode; 80] = [
    OpCode {code: 0x00, name: "BRK", bytes: 1, cycles: 7, add_mode: AddressingMode::NoneAddressing},
    OpCode {code: 0xaa, name: "TAX", bytes: 1, cycles: 2, add_mode: AddressingMode::NoneAddressing},
    OpCode {code: 0xe8, name: "INX", bytes: 1, cycles: 2, add_mode: AddressingMode::NoneAddressing},
    OpCode {code: 0xc8, name: "INY", bytes: 1, cycles: 2, add_mode: AddressingMode::NoneAddressing},
    OpCode {code: 0xca, name: "DEX", bytes: 1, cycles: 2, add_mode: AddressingMode::NoneAddressing},
    OpCode {code: 0x88, name: "DEY", bytes: 1, cycles: 2, add_mode: AddressingMode::NoneAddressing},
    OpCode {code: 0x18, name: "CLC", bytes: 1, cycles: 2, add_mode: AddressingMode::NoneAddressing},
    OpCode {code: 0xd8, name: "CLD", bytes: 1, cycles: 2, add_mode: AddressingMode::NoneAddressing},
    OpCode {code: 0x58, name: "CLI", bytes: 1, cycles: 2, add_mode: AddressingMode::NoneAddressing},
    OpCode {code: 0xb8, name: "CLV", bytes: 1, cycles: 2, add_mode: AddressingMode::NoneAddressing},

    OpCode {code: 0x90, name: "BCC", bytes: 2, cycles: 2, /* +1 if branch succeeds +2 if to a new Page */ add_mode: AddressingMode::Relative},
    OpCode {code: 0xb0, name: "BCS", bytes: 2, cycles: 2, /* +1 if branch succeeds +2 if to a new Page */ add_mode: AddressingMode::Relative},
    OpCode {code: 0xf0, name: "BEQ", bytes: 2, cycles: 2, /* +1 if branch succeeds +2 if to a new Page */ add_mode: AddressingMode::Relative},
    OpCode {code: 0xd0, name: "BNE", bytes: 2, cycles: 2, /* +1 if branch succeeds +2 if to a new Page */ add_mode: AddressingMode::Relative},
    OpCode {code: 0x30, name: "BMI", bytes: 2, cycles: 2, /* +1 if branch succeeds +2 if to a new Page */ add_mode: AddressingMode::Relative},
    OpCode {code: 0x10, name: "BPL", bytes: 2, cycles: 2, /* +1 if branch succeeds +2 if to a new Page */ add_mode: AddressingMode::Relative},
    OpCode {code: 0x70, name: "BVS", bytes: 2, cycles: 2, /* +1 if branch succeeds +2 if to a new Page */ add_mode: AddressingMode::Relative},

    OpCode {code: 0xa9, name: "LDA", bytes: 2, cycles: 2, add_mode: AddressingMode::Immediate},
    OpCode {code: 0xa5, name: "LDA", bytes: 2, cycles: 3, add_mode: AddressingMode::ZeroPage},
    OpCode {code: 0xb5, name: "LDA", bytes: 2, cycles: 4, add_mode: AddressingMode::ZeroPage_X},
    OpCode {code: 0xad, name: "LDA", bytes: 3, cycles: 4, add_mode: AddressingMode::Absolute},
    OpCode {code: 0xbd, name: "LDA", bytes: 3, cycles: 4, /* +1 if page crossed */ add_mode: AddressingMode::Absolute_X},
    OpCode {code: 0xb9, name: "LDA", bytes: 3, cycles: 4, /* +1 if page crossed */ add_mode: AddressingMode::Absolute_Y},
    OpCode {code: 0xa1, name: "LDA", bytes: 2, cycles: 6, add_mode: AddressingMode::Indirect_X},
    OpCode {code: 0xb1, name: "LDA", bytes: 2, cycles: 5, /* +1 if page crossed */ add_mode: AddressingMode::Indirect_Y},
    
    OpCode {code: 0x85, name: "STA", bytes: 2, cycles: 3, add_mode: AddressingMode::ZeroPage},
    OpCode {code: 0x95, name: "STA", bytes: 2, cycles: 4, add_mode: AddressingMode::ZeroPage_X},

    OpCode {code: 0x69, name: "ADC", bytes: 2, cycles: 2, add_mode: AddressingMode::Immediate},
    OpCode {code: 0x65, name: "ADC", bytes: 2, cycles: 3, add_mode: AddressingMode::ZeroPage},
    OpCode {code: 0x75, name: "ADC", bytes: 2, cycles: 4, add_mode: AddressingMode::ZeroPage_X},
    OpCode {code: 0x6d, name: "ADC", bytes: 3, cycles: 4, add_mode: AddressingMode::Absolute},
    OpCode {code: 0x7d, name: "ADC", bytes: 3, cycles: 4, /* +1 if page crossed */ add_mode: AddressingMode::Absolute_X},
    OpCode {code: 0x79, name: "ADC", bytes: 3, cycles: 4, /* +1 if page crossed */ add_mode: AddressingMode::Absolute_Y},
    OpCode {code: 0x61, name: "ADC", bytes: 2, cycles: 6, add_mode: AddressingMode::Indirect_X},
    OpCode {code: 0x71, name: "ADC", bytes: 2, cycles: 5, /* +1 if page crossed */ add_mode: AddressingMode::Indirect_Y},
    
    OpCode {code: 0x29, name: "AND", bytes: 2, cycles: 2, add_mode: AddressingMode::Immediate},
    OpCode {code: 0x25, name: "AND", bytes: 2, cycles: 3, add_mode: AddressingMode::ZeroPage},
    OpCode {code: 0x35, name: "AND", bytes: 2, cycles: 4, add_mode: AddressingMode::ZeroPage_X},
    OpCode {code: 0x2d, name: "AND", bytes: 3, cycles: 4, add_mode: AddressingMode::Absolute},
    OpCode {code: 0x3d, name: "AND", bytes: 3, cycles: 4, /* +1 if page crossed */ add_mode: AddressingMode::Absolute_X},
    OpCode {code: 0x39, name: "AND", bytes: 3, cycles: 4, /* +1 if page crossed */ add_mode: AddressingMode::Absolute_Y},
    OpCode {code: 0x21, name: "AND", bytes: 2, cycles: 6, add_mode: AddressingMode::Indirect_X},
    OpCode {code: 0x31, name: "AND", bytes: 2, cycles: 5, /* +1 if page crossed */ add_mode: AddressingMode::Indirect_Y},

    OpCode {code: 0x49, name: "EOR", bytes: 2, cycles: 2, add_mode: AddressingMode::Immediate},
    OpCode {code: 0x45, name: "EOR", bytes: 2, cycles: 3, add_mode: AddressingMode::ZeroPage},
    OpCode {code: 0x55, name: "EOR", bytes: 2, cycles: 4, add_mode: AddressingMode::ZeroPage_X},
    OpCode {code: 0x4d, name: "EOR", bytes: 3, cycles: 4, add_mode: AddressingMode::Absolute},
    OpCode {code: 0x5d, name: "EOR", bytes: 3, cycles: 4, /* +1 if page crossed */ add_mode: AddressingMode::Absolute_X},
    OpCode {code: 0x59, name: "EOR", bytes: 3, cycles: 4, /* +1 if page crossed */ add_mode: AddressingMode::Absolute_Y},
    OpCode {code: 0x41, name: "EOR", bytes: 2, cycles: 6, add_mode: AddressingMode::Indirect_X},
    OpCode {code: 0x51, name: "EOR", bytes: 2, cycles: 5, /* +1 if page crossed */ add_mode: AddressingMode::Indirect_Y},
    
    OpCode {code: 0x0a, name: "ASL", bytes: 1, cycles: 2, add_mode: AddressingMode::Immediate},
    OpCode {code: 0x06, name: "ASL", bytes: 2, cycles: 5, add_mode: AddressingMode::ZeroPage},
    OpCode {code: 0x16, name: "ASL", bytes: 2, cycles: 6, add_mode: AddressingMode::ZeroPage_X},
    OpCode {code: 0x0e, name: "ASL", bytes: 3, cycles: 6, add_mode: AddressingMode::Absolute},
    OpCode {code: 0x1e, name: "ASL", bytes: 3, cycles: 7, add_mode: AddressingMode::Absolute_X},
    
    OpCode {code: 0x24, name: "BIT", bytes: 2, cycles: 3, add_mode: AddressingMode::ZeroPage},
    OpCode {code: 0x2c, name: "BIT", bytes: 3, cycles: 4, add_mode: AddressingMode::Absolute},

    OpCode {code: 0xc9, name: "CMP", bytes: 2, cycles: 2, add_mode: AddressingMode::Immediate},
    OpCode {code: 0xc5, name: "CMP", bytes: 2, cycles: 3, add_mode: AddressingMode::ZeroPage},
    OpCode {code: 0xd5, name: "CMP", bytes: 2, cycles: 4, add_mode: AddressingMode::ZeroPage_X},
    OpCode {code: 0xcd, name: "CMP", bytes: 3, cycles: 4, add_mode: AddressingMode::Absolute},
    OpCode {code: 0xdd, name: "CMP", bytes: 3, cycles: 4, /* +1 if page crossed */ add_mode: AddressingMode::Absolute_X},
    OpCode {code: 0xd9, name: "CMP", bytes: 3, cycles: 4, /* +1 if page crossed */ add_mode: AddressingMode::Absolute_Y},
    OpCode {code: 0xc1, name: "CMP", bytes: 2, cycles: 6, add_mode: AddressingMode::Indirect_X},
    OpCode {code: 0xd1, name: "CMP", bytes: 2, cycles: 5, /* +1 if page crossed */ add_mode: AddressingMode::Indirect_Y},
    
    OpCode {code: 0xe0, name: "CPX", bytes: 2, cycles: 2, add_mode: AddressingMode::Immediate},
    OpCode {code: 0xe4, name: "CPX", bytes: 2, cycles: 3, add_mode: AddressingMode::ZeroPage},
    OpCode {code: 0xec, name: "CPX", bytes: 3, cycles: 4, add_mode: AddressingMode::Absolute},
    
    OpCode {code: 0xc0, name: "CPY", bytes: 2, cycles: 2, add_mode: AddressingMode::Immediate},
    OpCode {code: 0xc4, name: "CPY", bytes: 2, cycles: 3, add_mode: AddressingMode::ZeroPage},
    OpCode {code: 0xcc, name: "CPY", bytes: 3, cycles: 4, add_mode: AddressingMode::Absolute},

    OpCode {code: 0xc6, name: "DEC", bytes: 2, cycles: 5, add_mode: AddressingMode::ZeroPage},
    OpCode {code: 0xd6, name: "DEC", bytes: 2, cycles: 6, add_mode: AddressingMode::ZeroPage_X},
    OpCode {code: 0xce, name: "DEC", bytes: 3, cycles: 6, add_mode: AddressingMode::Absolute},
    OpCode {code: 0xde, name: "DEC", bytes: 3, cycles: 7, add_mode: AddressingMode::Absolute_X},

    OpCode {code: 0xe6, name: "DEC", bytes: 2, cycles: 5, add_mode: AddressingMode::ZeroPage},
    OpCode {code: 0xf6, name: "DEC", bytes: 2, cycles: 6, add_mode: AddressingMode::ZeroPage_X},
    OpCode {code: 0xee, name: "DEC", bytes: 3, cycles: 6, add_mode: AddressingMode::Absolute},
    OpCode {code: 0xfe, name: "DEC", bytes: 3, cycles: 7, add_mode: AddressingMode::Absolute_X},
    ];
    
    pub fn find_opcode(code: u8) -> Option<&'static OpCode> {
        CPU_OP_CODES.iter().find(|op| op.code == code)
    }