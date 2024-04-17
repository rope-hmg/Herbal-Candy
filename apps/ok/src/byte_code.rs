use crate::memory::Memory_Address;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    /// General Purpose Integer Register
    General_Purpose(u8),

    // The following registers are aliased to the general purpose registers
    Argument_0,
    Argument_1,
    Argument_2,
    Argument_3,

    Return_0,
    Return_1,
    Return_2,
    Return_3,

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
    pub fn is_general_purpose(&self) -> bool {
        use Register::*;

        matches!(
            self,
            General_Purpose(_)
                | Argument_0
                | Argument_1
                | Argument_2
                | Argument_3
                | Return_0
                | Return_1
                | Return_2
                | Return_3
        )
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

    Jump(u64),
    Jump_Not_Zero(u64, Register),

    Call(u64),
    Return,
}
