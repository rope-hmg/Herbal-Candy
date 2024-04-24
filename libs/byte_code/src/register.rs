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
    /// readonly, stores the code offset of the instruction after the last jump
    Link,

    /// General Purpose Registers
    General_Purpose(u8),
}

impl Register {
    pub fn from_index(index: u8) -> Self {
        match index {
            0 => Register::Zero,
            1 => Register::One,
            2 => Register::Instruction_Pointer,
            3 => Register::Stack_Pointer,
            4 => Register::Frame_Pointer,
            5 => Register::Link,
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
            Register::Link => 5,
            Register::General_Purpose(index) => index as usize,
        }
    }

    pub fn is_readonly(self) -> bool {
        !matches!(self, Register::General_Purpose(_))
    }
}
