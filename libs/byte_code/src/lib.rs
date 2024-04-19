#![allow(non_camel_case_types)]

//         MSB                                                                                                                           LSB
//         0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
// R-Type |immediate value                                                |S|F|-|siz|register 2   |register 1   |register 0   | op code   |
// I-Type |immediate value                                                      |siz|immediate value            |register 0   | op code   |
//

pub enum Op_Code {
    // Control Flow
    // ------------
    Halt, //

    Call,          // I_Type (address in immediate)
    Jump,          // I_Type (address in immediate)
    Jump_Not_Zero, // I_Type (address in immediate, condition in register 0 )
    Jump_Zero,     // I_Type (address in immediate, condition in register 0 )
    Jump_Positive, // I_Type (address in immediate, condition in register 0 )
    Jump_Negative, // I_Type (address in immediate, condition in register 0 )

    Call_Register,          // I_Type (address in register 0)
    Jump_Register,          // I_Type (address in reigster 0)
    Jump_Not_Zero_Register, // R_Type (address in register 0, condition in register 1)
    Jump_Zero_Register,     // R_Type (address in register 0, condition in register 1)
    Jump_Positive_Register, // R_Type (address in register 0, condition in register 1)
    Jump_Negative_Register, // R_Type (address in register 0, condition in register 1)

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
    Load,           // I_Type (address in immediate,  destination is register 0)
    Load_Immediate, // I_Type (  value in immediate,  destination is register 0)
    Load_Register,  // R_Type (address in register 1, destination is register 0)
    Store,          // I_Type (address in immediate,  source is register 0)
    Store_Register, // R_Type (address in reigster 1, source is register 0)
    Push,           // I_Type (value in register 0)
    Push_Immediate, // I_Type (value in immediate)
    Pop,            // I_Type (destination is register 0)
    Move,           // R_Type (source is register 1, destination is register 0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct R_Type(u64);

impl R_Type {
    pub fn op_code(self) -> u8 {
        (self.0 & 0x3F) as u8
    }

    pub fn register_0(self) -> Register {
        Register::from_index(((self.0 >> 6) & 0x7F) as u8)
    }

    pub fn register_1(self) -> Register {
        Register::from_index(((self.0 >> 13) & 0x7F) as u8)
    }

    pub fn register_2(self) -> Register {
        Register::from_index(((self.0 >> 20) & 0x7F) as u8)
    }

    pub fn bit_width(self) -> u8 {
        let mask = ((self.0 >> 27) & 0x03) as u8;
        let size = unpack_bit_width(mask);

        size
    }

    pub fn is_float(self) -> bool {
        (self.0 & 0x40000000) != 0
    }

    pub fn is_signed(self) -> bool {
        (self.0 & 0x80000000) != 0
    }

    pub fn immediate_value(self) -> u64 {
        self.0 >> 32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I_Type(u64);

impl I_Type {
    pub fn op_code(self) -> u8 {
        (self.0 & 0x3F) as u8
    }

    pub fn register_0(self) -> Register {
        Register::from_index(((self.0 >> 6) & 0x7F) as u8)
    }

    pub fn bit_width(self) -> u8 {
        let mask = ((self.0 >> 27) & 0x03) as u8;
        let size = unpack_bit_width(mask);

        size
    }

    pub fn immediate_value(self) -> u64 {
        let size = self.bit_width();

        let small = (self.0 >> 13) & 0x3FFF;
        let value = (self.0 >> 15) | small;

        match size {
            0 => 0,
            8 => small & 0xFF,
            16 => value & 0xFFFF,
            32 => value & 0xFFFFFFFF,
            64 => value,
            _ => unreachable!("Invalid bit width"),
        }
    }
}

#[inline(always)]
fn unpack_bit_width(mask: u8) -> u8 {
    match mask {
        0b000 => 8,
        0b001 => 16,
        0b010 => 32,
        0b011 => 64,
        0b100 => 0, // Currently unused
        0b101 => 0, // Currently unused
        0b110 => 0, // Currently unused
        0b111 => 0, // Currently unused
        _ => unreachable!("There are only two bits"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    /// General Purpose Registers
    General_Purpose(u8),

    /// readonly, stores the code offset of the currently executing instruction
    Instruction_Pointer,
    /// readonly, stores the memory address of the top of the stack
    Stack_Pointer,
    /// readonly, stores the memory address of the top of the frame
    Frame_Pointer,

    /// readonly, always 0
    Zero,
    /// readonly, always 1
    One,
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
}
