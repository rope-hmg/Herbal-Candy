use crate::memory::Memory_Address;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    /// General Purpose Integer Register
    General_Purpose(u8),

    Instruction_Pointer,
    Stack_Pointer,
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
pub enum Micro_Op {
    Saturating_Add_I32(Register, Register, Register),
    Saturating_Sub_I32(Register, Register, Register),
    Saturating_Mul_I32(Register, Register, Register),
    Saturating_Div_I32(Register, Register, Register),
    Saturating_Add_U32(Register, Register, Register),
    Saturating_Sub_U32(Register, Register, Register),
    Saturating_Mul_U32(Register, Register, Register),
    Saturating_Div_U32(Register, Register, Register),

    Saturating_Add_I64(Register, Register, Register),
    Saturating_Sub_I64(Register, Register, Register),
    Saturating_Mul_I64(Register, Register, Register),
    Saturating_Div_I64(Register, Register, Register),
    Saturating_Add_U64(Register, Register, Register),
    Saturating_Sub_U64(Register, Register, Register),
    Saturating_Mul_U64(Register, Register, Register),
    Saturating_Div_U64(Register, Register, Register),

    Overflowing_Add_I32(Register, Register, Register),
    Overflowing_Sub_I32(Register, Register, Register),
    Overflowing_Mul_I32(Register, Register, Register),
    Overflowing_Div_I32(Register, Register, Register),
    Overflowing_Add_U32(Register, Register, Register),
    Overflowing_Sub_U32(Register, Register, Register),
    Overflowing_Mul_U32(Register, Register, Register),
    Overflowing_Div_U32(Register, Register, Register),

    Overflowing_Add_I64(Register, Register, Register),
    Overflowing_Sub_I64(Register, Register, Register),
    Overflowing_Mul_I64(Register, Register, Register),
    Overflowing_Div_I64(Register, Register, Register),
    Overflowing_Add_U64(Register, Register, Register),
    Overflowing_Sub_U64(Register, Register, Register),
    Overflowing_Mul_U64(Register, Register, Register),
    Overflowing_Div_U64(Register, Register, Register),

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

    Jump(usize),
}
