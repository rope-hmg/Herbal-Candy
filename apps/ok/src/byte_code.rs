use crate::memory::Memory_Address;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    /// General Purpose Integer Register
    General_Purpose(u8),

    Instruction_Pointer,
    Stack_Pointer,
    Zero,
    One,
}

impl Register {
    #[inline(always)]
    pub fn general_purpose(&self) -> u8 {
        self.as_general_purpose().unwrap()
    }

    #[inline(always)]
    pub fn as_general_purpose(&self) -> Option<u8> {
        match self {
            Register::General_Purpose(address) => Some(*address),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Three_Registers {
    pub destination: Register,
    pub source_1: Register,
    pub source_2: Register,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Micro_Op {
    Halt,

    Saturating_Add_I32(Three_Registers),
    Saturating_Sub_I32(Three_Registers),
    Saturating_Mul_I32(Three_Registers),
    Saturating_Div_I32(Three_Registers),
    Saturating_Add_U32(Three_Registers),
    Saturating_Sub_U32(Three_Registers),
    Saturating_Mul_U32(Three_Registers),
    Saturating_Div_U32(Three_Registers),

    Saturating_Add_I64(Three_Registers),
    Saturating_Sub_I64(Three_Registers),
    Saturating_Mul_I64(Three_Registers),
    Saturating_Div_I64(Three_Registers),
    Saturating_Add_U64(Three_Registers),
    Saturating_Sub_U64(Three_Registers),
    Saturating_Mul_U64(Three_Registers),
    Saturating_Div_U64(Three_Registers),

    Overflowing_Add_I32(Three_Registers),
    Overflowing_Sub_I32(Three_Registers),
    Overflowing_Mul_I32(Three_Registers),
    Overflowing_Div_I32(Three_Registers),
    Overflowing_Add_U32(Three_Registers),
    Overflowing_Sub_U32(Three_Registers),
    Overflowing_Mul_U32(Three_Registers),
    Overflowing_Div_U32(Three_Registers),

    Overflowing_Add_I64(Three_Registers),
    Overflowing_Sub_I64(Three_Registers),
    Overflowing_Mul_I64(Three_Registers),
    Overflowing_Div_I64(Three_Registers),
    Overflowing_Add_U64(Three_Registers),
    Overflowing_Sub_U64(Three_Registers),
    Overflowing_Mul_U64(Three_Registers),
    Overflowing_Div_U64(Three_Registers),

    Load_8(Register, Memory_Address),
    Load_16(Register, Memory_Address),
    Load_32(Register, Memory_Address),
    Load_64(Register, Memory_Address),
    Store_8(Memory_Address, Register),
    Store_16(Memory_Address, Register),
    Store_32(Memory_Address, Register),
    Store_64(Memory_Address, Register),
    Push(Register),
    Pop(Register),

    Move(Register, Register),

    Jump(usize),
    Jump_Not_Zero(usize, Register),
}
