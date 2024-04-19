#![allow(non_camel_case_types)]

//         MSB                                                                                                                           LSB
//         0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
// R-Type |immediate value                                                |S|F|-|siz|register 2   |register 1   |register 0   | op code   |
// I-Type |immediate value                                                      |siz|immediate value            |register 0   | op code   |
//

use std::{fmt, hint::unreachable_unchecked};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction_Kind {
    V_Type,
    R_Type,
    I_Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op_Code {
    // Control Flow
    // ------------
    Halt, //

    Call,          // I_Type (address in immediate)
    Call_Register, // I_Type (address in register)
    Return,        // V_Type

    Jump,          // I_Type (address in immediate)
    Jump_Not_Zero, // I_Type (address in immediate, condition in register 0 )
    Jump_Zero,     // I_Type (address in immediate, condition in register 0 )

    Jump_Register,          // I_Type (address in reigster)
    Jump_Not_Zero_Register, // R_Type (address in register 0, condition in register 1)
    Jump_Zero_Register,     // R_Type (address in register 0, condition in register 1)

    // Comparison
    // ----------
    Compare_Equal,         // R_Type
    Compare_Not_Equal,     // R_Type
    Compare_Less,          // R_Type
    Compare_Greater,       // R_Type
    Compare_Less_Equal,    // R_Type
    Compare_Greater_Equal, // R_Type

    // Arithmetic
    // ----------
    Saturating_Absolute,    // R_Type
    Saturating_Negate,      // R_Type
    Saturating_Power,       // R_Type
    Saturating_Remainder,   // R_Type
    Saturating_Add,         // R_Type
    Saturating_Subtract,    // R_Type
    Saturating_Multiply,    // R_Type
    Saturating_Divide,      // R_Type
    Saturating_Modulus,     // R_Type
    Saturating_Shift_Left,  // R_Type
    Saturating_Shift_Right, // R_Type

    Overflowing_Absolute,    // R_Type
    Overflowing_Negate,      // R_Type
    Overflowing_Power,       // R_Type
    Overflowing_Remainder,   // R_Type
    Overflowing_Add,         // R_Type
    Overflowing_Subtract,    // R_Type
    Overflowing_Multiply,    // R_Type
    Overflowing_Divide,      // R_Type
    Overflowing_Modulus,     // R_Type
    Overflowing_Shift_Left,  // R_Type
    Overflowing_Shift_Right, // R_Type

    // Bitwise
    // -------
    And,          // R_Type
    Or,           // R_Type
    Xor,          // R_Type
    Not,          // R_Type
    Rotate_Left,  // R_Type
    Rotate_Right, // R_Type

    // Memory
    // ------
    Load,           // I_Type (address in immediate,  destination is register)
    Load_Immediate, // I_Type (  value in immediate,  destination is register)
    Load_Register,  // R_Type (address in register 1, destination is register 0)
    Store,          // I_Type (address in immediate,  source is register)
    Store_Register, // R_Type (address in reigster 1, source is register 0)
    Push,           // I_Type (value in register)
    Push_Immediate, // I_Type (value in immediate)
    Pop,            // I_Type (destination is register)
    Move,           // R_Type (source is register 1, destination is register 0)

    // Invalid
    // -------
    Invalid,
}

static INSTRUCTION_LOOKUP: [(Instruction_Kind, Op_Code); 63] = [
    (Instruction_Kind::V_Type, Op_Code::Halt),
    (Instruction_Kind::I_Type, Op_Code::Call),
    (Instruction_Kind::I_Type, Op_Code::Call_Register),
    (Instruction_Kind::V_Type, Op_Code::Return),
    (Instruction_Kind::I_Type, Op_Code::Jump),
    (Instruction_Kind::I_Type, Op_Code::Jump_Not_Zero),
    (Instruction_Kind::I_Type, Op_Code::Jump_Zero),
    (Instruction_Kind::I_Type, Op_Code::Jump_Register),
    (Instruction_Kind::R_Type, Op_Code::Jump_Not_Zero_Register),
    (Instruction_Kind::R_Type, Op_Code::Jump_Zero_Register),
    (Instruction_Kind::R_Type, Op_Code::Compare_Equal),
    (Instruction_Kind::R_Type, Op_Code::Compare_Not_Equal),
    (Instruction_Kind::R_Type, Op_Code::Compare_Less),
    (Instruction_Kind::R_Type, Op_Code::Compare_Greater),
    (Instruction_Kind::R_Type, Op_Code::Compare_Less_Equal),
    (Instruction_Kind::R_Type, Op_Code::Compare_Greater_Equal),
    (Instruction_Kind::R_Type, Op_Code::Saturating_Absolute),
    (Instruction_Kind::R_Type, Op_Code::Saturating_Negate),
    (Instruction_Kind::R_Type, Op_Code::Saturating_Power),
    (Instruction_Kind::R_Type, Op_Code::Saturating_Remainder),
    (Instruction_Kind::R_Type, Op_Code::Saturating_Add),
    (Instruction_Kind::R_Type, Op_Code::Saturating_Subtract),
    (Instruction_Kind::R_Type, Op_Code::Saturating_Multiply),
    (Instruction_Kind::R_Type, Op_Code::Saturating_Divide),
    (Instruction_Kind::R_Type, Op_Code::Saturating_Modulus),
    (Instruction_Kind::R_Type, Op_Code::Saturating_Shift_Left),
    (Instruction_Kind::R_Type, Op_Code::Saturating_Shift_Right),
    (Instruction_Kind::R_Type, Op_Code::Overflowing_Absolute),
    (Instruction_Kind::R_Type, Op_Code::Overflowing_Negate),
    (Instruction_Kind::R_Type, Op_Code::Overflowing_Power),
    (Instruction_Kind::R_Type, Op_Code::Overflowing_Remainder),
    (Instruction_Kind::R_Type, Op_Code::Overflowing_Add),
    (Instruction_Kind::R_Type, Op_Code::Overflowing_Subtract),
    (Instruction_Kind::R_Type, Op_Code::Overflowing_Multiply),
    (Instruction_Kind::R_Type, Op_Code::Overflowing_Divide),
    (Instruction_Kind::R_Type, Op_Code::Overflowing_Modulus),
    (Instruction_Kind::R_Type, Op_Code::Overflowing_Shift_Left),
    (Instruction_Kind::R_Type, Op_Code::Overflowing_Shift_Right),
    (Instruction_Kind::R_Type, Op_Code::And),
    (Instruction_Kind::R_Type, Op_Code::Or),
    (Instruction_Kind::R_Type, Op_Code::Xor),
    (Instruction_Kind::R_Type, Op_Code::Not),
    (Instruction_Kind::R_Type, Op_Code::Rotate_Left),
    (Instruction_Kind::R_Type, Op_Code::Rotate_Right),
    (Instruction_Kind::I_Type, Op_Code::Load),
    (Instruction_Kind::I_Type, Op_Code::Load_Immediate),
    (Instruction_Kind::R_Type, Op_Code::Load_Register),
    (Instruction_Kind::I_Type, Op_Code::Store),
    (Instruction_Kind::R_Type, Op_Code::Store_Register),
    (Instruction_Kind::I_Type, Op_Code::Push),
    (Instruction_Kind::I_Type, Op_Code::Push_Immediate),
    (Instruction_Kind::I_Type, Op_Code::Pop),
    (Instruction_Kind::R_Type, Op_Code::Move),
    (Instruction_Kind::V_Type, Op_Code::Invalid),
    (Instruction_Kind::V_Type, Op_Code::Invalid),
    (Instruction_Kind::V_Type, Op_Code::Invalid),
    (Instruction_Kind::V_Type, Op_Code::Invalid),
    (Instruction_Kind::V_Type, Op_Code::Invalid),
    (Instruction_Kind::V_Type, Op_Code::Invalid),
    (Instruction_Kind::V_Type, Op_Code::Invalid),
    (Instruction_Kind::V_Type, Op_Code::Invalid),
    (Instruction_Kind::V_Type, Op_Code::Invalid),
    (Instruction_Kind::V_Type, Op_Code::Invalid),
];

pub union Instruction_Variant {
    v_type: (),
    r_type: R_Type,
    i_type: I_Type,
}

impl Instruction_Variant {
    pub fn v_type() -> Self {
        Self { v_type: () }
    }

    pub fn r_type(r_type: R_Type) -> Self {
        Self { r_type }
    }

    pub fn i_type(i_type: I_Type) -> Self {
        Self { i_type }
    }
}

// TODO: Think of a better way for this to work.
pub struct Instruction {
    pub op_code: Op_Code,
    pub variant: Instruction_Variant,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.op_code)
    }
}

impl Instruction {
    pub fn decode(raw_instruction: u64) -> Self {
        let raw_op_code = (raw_instruction & 0x3F) as usize;

        let (instruction_type, op_code) = INSTRUCTION_LOOKUP[raw_op_code];

        match instruction_type {
            Instruction_Kind::V_Type => Self {
                op_code,
                variant: Instruction_Variant { v_type: () },
            },

            Instruction_Kind::R_Type => Self {
                op_code,
                variant: Instruction_Variant {
                    r_type: R_Type::decode(raw_instruction),
                },
            },

            Instruction_Kind::I_Type => Self {
                op_code,
                variant: Instruction_Variant {
                    i_type: I_Type::decode(raw_instruction),
                },
            },
        }
    }

    pub fn op_code(&self) -> Op_Code {
        self.op_code
    }

    // FIXME: This is wrong. Well, it'll work, but it's not good API design.
    pub fn r_type(&self) -> &R_Type {
        unsafe { &self.variant.r_type }
    }

    pub fn i_type(&self) -> &I_Type {
        unsafe { &self.variant.i_type }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit_Width {
    Eight,
    Sixteen,
    Thirty_Two,
    Sixty_Four,
}

impl Bit_Width {
    pub fn from_mask(mask: u8) -> Self {
        match mask & 0x3 {
            0b00 => Self::Eight,
            0b01 => Self::Sixteen,
            0b10 => Self::Thirty_Two,
            0b11 => Self::Sixty_Four,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn byte_count(&self) -> u8 {
        match self {
            Self::Eight => 1,
            Self::Sixteen => 2,
            Self::Thirty_Two => 4,
            Self::Sixty_Four => 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct R_Type {
    pub register_0: Register,
    pub register_1: Register,
    pub register_2: Register,
    pub bit_width: Bit_Width,
    pub is_float: bool,
    pub is_signed: bool,
    pub immediate_value: u64,
}

impl R_Type {
    pub fn decode(raw_instruction: u64) -> Self {
        Self {
            register_0: Register::from_index(((raw_instruction >> 6) & 0x7F) as u8),
            register_1: Register::from_index(((raw_instruction >> 13) & 0x7F) as u8),
            register_2: Register::from_index(((raw_instruction >> 20) & 0x7F) as u8),
            bit_width: {
                let mask = ((raw_instruction >> 27) & 0x03) as u8;
                let size = Bit_Width::from_mask(mask);

                size
            },
            is_float: (raw_instruction & 0x40000000) != 0,
            is_signed: (raw_instruction & 0x80000000) != 0,
            immediate_value: raw_instruction >> 32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I_Type {
    pub register: Register,
    pub bit_width: Bit_Width,
    pub immediate_value: u64,
}

impl I_Type {
    pub fn decode(raw_instruction: u64) -> Self {
        let mask = ((raw_instruction >> 27) & 0x03) as u8;
        let size = Bit_Width::from_mask(mask);

        Self {
            register: Register::from_index(((raw_instruction >> 6) & 0x7F) as u8),
            bit_width: size,
            immediate_value: {
                let small = (raw_instruction >> 13) & 0x3FFF;
                let value = (raw_instruction >> 15) | small;

                match size {
                    Bit_Width::Eight => small & 0xFF,
                    Bit_Width::Sixteen => value & 0xFFFF,
                    Bit_Width::Thirty_Two => value & 0xFFFFFFFF,
                    Bit_Width::Sixty_Four => value,
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    /// readonly, always 0
    Zero,
    /// readonly, always 1
    One,

    /// readonly, stores the code offset of the currently executing instruction
    Instruction_Pointer,
    /// readonly, stores the memory address of the top of the stack
    Stack_Pointer,
    /// readonly, stores the memory address of the top of the frame
    Frame_Pointer,

    /// General Purpose Registers
    General_Purpose(u8),
}

impl Register {
    fn from_index(index: u8) -> Self {
        match index {
            0 => Register::Zero,
            1 => Register::One,
            2 => Register::Instruction_Pointer,
            3 => Register::Stack_Pointer,
            4 => Register::Frame_Pointer,
            _ => Register::General_Purpose(index),
        }
    }

    pub fn index(self) -> usize {
        match self {
            Register::Zero => 0,
            Register::One => 1,
            Register::Instruction_Pointer => 2,
            Register::Stack_Pointer => 3,
            Register::Frame_Pointer => 4,
            Register::General_Purpose(index) => index as usize,
        }
    }

    pub fn is_readonly(self) -> bool {
        matches!(
            self,
            Register::Zero
                | Register::One
                | Register::Instruction_Pointer
                | Register::Stack_Pointer
                | Register::Frame_Pointer
        )
    }
}
