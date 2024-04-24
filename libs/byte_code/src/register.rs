use crate::encoding::REG_MASK;

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

    // IMPORTANT: If you add more special purpose registers, you must update SPECIAL_PURPOSE_COUNT
    /// General Purpose Registers
    General_Purpose(u8),
}

const VALID_INDEX_COUNT: u8 = REG_MASK as u8 + 1;
const SPECIAL_PURPOSE_COUNT: u8 = 6;
const GENERAL_PURPOSE_COUNT: u8 = VALID_INDEX_COUNT - SPECIAL_PURPOSE_COUNT;

impl Register {
    pub fn from_index(index: u8) -> Self {
        let general_purpose_index = index.saturating_sub(SPECIAL_PURPOSE_COUNT);

        match index {
            0 => Register::Zero,
            1 => Register::One,
            2 => Register::Instruction_Pointer,
            3 => Register::Stack_Pointer,
            4 => Register::Frame_Pointer,
            5 => Register::Link,

            _ if general_purpose_index <= GENERAL_PURPOSE_COUNT => {
                Register::General_Purpose(general_purpose_index)
            },

            // This will panic if the index is out of bounds, but that's fine because the decoder
            // should never produce an invalid register index. Well, unless someone has changed the
            // number of bits used to encode registers, but that's a different problem.
            _ => unreachable!(),
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
            Register::General_Purpose(index) => (index + SPECIAL_PURPOSE_COUNT) as usize,
        }
    }

    pub fn is_readonly(self) -> bool {
        !matches!(self, Register::General_Purpose(_))
    }
}
