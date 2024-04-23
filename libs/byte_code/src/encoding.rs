//! Instruction Layout:
//! ```notrust
//!         MSB                                                           LSB
//!         0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
//! V-Type |-------------------------------------------|funct  |op code    |
//! R-Type |S|F|siz|register 2 |register 1 |register 0 |funct  |op code    |
//! I-Type |16 bit immediate               |register   |funct  |op code    |
//! ```
//!
//! Instructions:
//! ```notrust
//! |- - -- ------ ------ ------|0000|000000| Halt
//! |x|-|--|xxxxxx|------|------|0001|000000| Call
//! |- - -- ------ ------ ------|0010|000000| Return
//! |x x xx xxxxxx xxxxxx|------|0011|000000| Call_Environment
//! |- - -- ------ ------ ------|0100|000000| Break
//! |x|-|--|xxxxxx|------|xxxxxx|0101|000000| Jump_And_Link
//! |x|-|--|xxxxxx|xxxxxx|xxxxxx|0110|000000| Jump_Not_Zero
//! |x|-|--|xxxxxx|xxxxxx|xxxxxx|0111|000000| Jump_Zero
//! |- - -- ------ ------ ------|1000|000000|
//! |- - -- ------ ------ ------|1001|000000|
//! |- - -- ------ ------ ------|1010|000000|
//! |- - -- ------ ------ ------|1011|000000|
//! |- - -- ------ ------ ------|1100|000000|
//! |- - -- ------ ------ ------|1101|000000|
//! |- - -- ------ ------ ------|1110|000000|
//! |- - -- ------ ------ ------|1111|000000|
//!
//! |-|x|xx|xxxxxx|xxxxxx|xxxxxx|0000|000001| Compare_Equal
//! |-|x|xx|xxxxxx|xxxxxx|xxxxxx|0001|000001| Compare_Not_Equal
//! |-|x|xx|xxxxxx|xxxxxx|xxxxxx|0010|000001| Compare_Less
//! |-|x|xx|xxxxxx|xxxxxx|xxxxxx|0011|000001| Compare_Less_Equal
//! |-|x|xx|xxxxxx|xxxxxx|xxxxxx|0100|000001| Compare_Greater
//! |-|x|xx|xxxxxx|xxxxxx|xxxxxx|0101|000001| Compare_Greater_Equal
//! |- - -- ------ ------ ------|0110|000001|
//! |- - -- ------ ------ ------|0111|000001|
//! |- - -- ------ ------ ------|1000|000001|
//! |- - -- ------ ------ ------|1001|000001|
//! |- - -- ------ ------ ------|1010|000001|
//! |- - -- ------ ------ ------|1011|000001|
//! |- - -- ------ ------ ------|1100|000001|
//! |- - -- ------ ------ ------|1101|000001|
//! |- - -- ------ ------ ------|1110|000001|
//! |- - -- ------ ------ ------|1111|000001|
//!
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0000|000010| And
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0001|000010| Or
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0010|000010| Xor
//! |x|-|xx|------|xxxxxx|xxxxxx|0011|000010| Not
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0100|000010| Shift_Left
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0101|000010| Shift_Right
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0110|000010| Rotate_Left
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0111|000010| Rotate_Right
//! |x|-|xx|------|xxxxxx|xxxxxx|1000|000010| Count_Ones
//! |x|-|xx|------|xxxxxx|xxxxxx|1001|000010| Leading_Ones
//! |x|-|xx|------|xxxxxx|xxxxxx|1010|000010| Trailing_Ones
//! |x|-|xx|------|xxxxxx|xxxxxx|1011|000010| Count_Zeros
//! |x|-|xx|------|xxxxxx|xxxxxx|1100|000010| Leading_Zeros
//! |x|-|xx|------|xxxxxx|xxxxxx|1101|000010| Trailing_Zeros
//! |x|-|xx|------|xxxxxx|xxxxxx|1110|000010| Reverse_Bytes
//! |x|-|xx|------|xxxxxx|xxxxxx|1111|000010| Reverse_Bits
//!
//! |-|-|xx|------|xxxxxx|xxxxxx|0000|000011| Load
//! |x x xx xxxxxx xxxxxx|xxxxxx|0001|000011| Load_Immediate
//! |-|-|xx|------|xxxxxx|xxxxxx|0010|000011| Store
//! |x x xx xxxxxx xxxxxx|xxxxxx|0011|000011| Store_Immediate
//! |-|-|xx|------|xxxxxx|xxxxxx|0100|000011| Move
//! |- - -- ------ ------|xxxxxx|0101|000011| Push
//! |x x xx xxxxxx xxxxxx|------|0110|000011| Push_Immediate
//! |- - -- ------ ------|xxxxxx|0111|000011| Pop
//! |- - -- ------ ------ ------|1000|000011|
//! |- - -- ------ ------ ------|1001|000011|
//! |- - -- ------ ------ ------|1010|000011|
//! |- - -- ------ ------ ------|1011|000011|
//! |- - -- ------ ------ ------|1100|000011|
//! |- - -- ------ ------ ------|1101|000011|
//! |- - -- ------ ------ ------|1110|000011|
//! |- - -- ------ ------ ------|1111|000011|
//!
//! |1|-|xx|------|xxxxxx|xxxxxx|0000|000100| Checked_Absolute
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0001|000100| Checked_Add
//! |1|-|xx|xxxxxx|xxxxxx|xxxxxx|0010|000100| Checked_Add_Unsigned
//! |0|-|xx|xxxxxx|xxxxxx|xxxxxx|0010|000100| Checked_Add_Signed
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0011|000100| Checked_Divide
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0100|000100| Checked_Divide_Euclidean
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0101|000100| Checked_Logarithm
//! |x|-|xx|------|xxxxxx|xxxxxx|0110|000100| Checked_Square_Root
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0111|000100| Checked_Multiply
//! |1|-|xx|------|xxxxxx|xxxxxx|1000|000100| Checked_Negate
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|1001|000100| Checked_Power
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|1010|000100| Checked_Remainder
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|1011|000100| Checked_Remainder_Euclidean
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|1100|000100| Checked_Shift_Left
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|1101|000100| Checked_Shift_Right
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|1110|000100| Checked_Subtract
//! |1|-|xx|xxxxxx|xxxxxx|xxxxxx|1111|000100| Checked_Subtract_Unsigned
//! |0|-|xx|xxxxxx|xxxxxx|xxxxxx|1111|000100| Checked_Subtract_Signed
//!
//! |1|-|xx|------|xxxxxx|xxxxxx|0000|000101| Overflowing_Absolute
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0001|000101| Overflowing_Add
//! |1|-|xx|xxxxxx|xxxxxx|xxxxxx|0010|000101| Overflowing_Add_Unsigned
//! |0|-|xx|xxxxxx|xxxxxx|xxxxxx|0010|000101| Overflowing_Add_Signed
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0011|000101| Overflowing_Divide
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0100|000101| Overflowing_Divide_Euclidean
//! |- - -- ------ ------ ------|0101|000101|
//! |- - -- ------ ------ ------|0110|000101|
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0111|000101| Overflowing_Multiply
//! |1|-|xx|------|xxxxxx|xxxxxx|1000|000101| Overflowing_Negate
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|1001|000101| Overflowing_Power
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|1010|000101| Overflowing_Remainder
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|1011|000101| Overflowing_Remainder_Euclidean
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|1100|000101| Overflowing_Shift_Left
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|1101|000101| Overflowing_Shift_Right
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|1110|000101| Overflowing_Subtract
//! |1|-|xx|xxxxxx|xxxxxx|xxxxxx|1111|000101| Overflowing_Subtract_Unsigned
//! |0|-|xx|xxxxxx|xxxxxx|xxxxxx|1111|000101| Overflowing_Subtract_Signed
//!
//! |1|-|xx|------|xxxxxx|xxxxxx|0000|000110| Saturating_Absolute
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0001|000110| Saturating_Add
//! |1|-|xx|xxxxxx|xxxxxx|xxxxxx|0010|000110| Saturating_Add_Unsigned
//! |0|-|xx|xxxxxx|xxxxxx|xxxxxx|0010|000110| Saturating_Add_Signed
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0011|000110| Saturating_Divide
//! |- - -- ------ ------ ------|0100|000110|
//! |- - -- ------ ------ ------|0101|000110|
//! |- - -- ------ ------ ------|0110|000110|
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|0111|000110| Saturating_Multiply
//! |1|-|xx|------|xxxxxx|xxxxxx|1000|000110| Saturating_Negate
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|1001|000110| Saturating_Power
//! |- - -- ------ ------ ------|1010|000110|
//! |- - -- ------ ------ ------|1011|000110|
//! |- - -- ------ ------ ------|1100|000110|
//! |- - -- ------ ------ ------|1101|000110|
//! |x|-|xx|xxxxxx|xxxxxx|xxxxxx|1110|000110| Saturating_Subtract
//! |1|-|xx|xxxxxx|xxxxxx|xxxxxx|1111|000110| Saturating_Subtract_Unsigned
//! |0|-|xx|xxxxxx|xxxxxx|xxxxxx|1111|000110| Saturating_Subtract_Signed
//!
//! |-|1|1x|------|xxxxxx|xxxxxx|0000|000111| Absolute_F
//! |-|1|1x|xxxxxx|xxxxxx|xxxxxx|0001|000111| Add_F
//! |-|1|1x|xxxxxx|xxxxxx|xxxxxx|0010|000111|
//! |-|1|1x|xxxxxx|xxxxxx|xxxxxx|0011|000111| Divide_F
//! |-|1|1x|xxxxxx|xxxxxx|xxxxxx|0100|000111| Divide_Euclidean_F
//! |-|1|1x|xxxxxx|xxxxxx|xxxxxx|0101|000111| Logarithm_F
//! |-|1|1x|------|xxxxxx|xxxxxx|0110|000111| Square_Root_F
//! |-|1|1x|xxxxxx|xxxxxx|xxxxxx|0111|000111| Multiply_F
//! |-|1|1x|xxxxxx|xxxxxx|xxxxxx|1000|000111| Negate_F
//! |-|1|1x|xxxxxx|xxxxxx|xxxxxx|1001|000111| Power_F
//! |-|1|1x|xxxxxx|xxxxxx|xxxxxx|1010|000111| Remainder_F
//! |-|1|1x|xxxxxx|xxxxxx|xxxxxx|1011|000111| Remainder_Euclidean_F
//! |- - -- ------ ------ ------|1100|000111|
//! |-|1|1x|------|xxxxxx|xxxxxx|1101|000111| Cube_Root_F
//! |-|1|1x|xxxxxx|xxxxxx|xxxxxx|1110|000111| Subtract_F
//! |- - -- ------ ------ ------|1111|000111|
//!
//! |x|x|xx|------|xxxxxx|xxxxxx|0000|000111| Signum
//! |x|x|xx|xxxxxx|xxxxxx|xxxxxx|0001|000111| Copy_Sign
//! |x|x|xx|xxxxxx|xxxxxx|xxxxxx|0010|000111| Min
//! |x|x|xx|xxxxxx|xxxxxx|xxxxxx|0011|000111| Max
//! |x|x|xx|------|xxxxxx|xxxxxx|0100|000111| Midpoint
//! |x|x|xx|------|xxxxxx|xxxxxx|0101|000111| Is_Positive
//! |x|x|xx|xxxxxx|xxxxxx|xxxxxx|0110|000111| Is_Negative
//! |- - -- ------ ------ ------|0111|000111| Absolute_Difference
//! |- - -- ------ ------ ------|1000|000111| Floor
//! |- - -- ------ ------ ------|1001|000111| Ceiling
//! |- - -- ------ ------ ------|1010|000111|
//! |- - -- ------ ------ ------|1011|000111|
//! |- - -- ------ ------ ------|1100|000111|
//! |- - -- ------ ------ ------|1101|000111|
//! |- - -- ------ ------ ------|1110|000111|
//! |- - -- ------ ------ ------|1111|000111|
//!
//! |-|1|xx|------|xxxxxx|xxxxxx|0000|001000| Cos
//! |-|1|xx|xxxxxx|xxxxxx|xxxxxx|0001|001000| Cos_Hyperbolic
//! |-|1|xx|xxxxxx|xxxxxx|xxxxxx|0010|001000| Arc_Cos
//! |-|1|xx|xxxxxx|xxxxxx|xxxxxx|0011|001000| Arc_Cos_Hyperbolic
//! |-|1|xx|------|xxxxxx|xxxxxx|0100|001000| Sin
//! |-|1|xx|------|xxxxxx|xxxxxx|0101|001000| Sin_Hyperbolic
//! |-|1|xx|xxxxxx|xxxxxx|xxxxxx|0110|001000| Arc_Sin
//! |-|1|xx|xxxxxx|xxxxxx|xxxxxx|0111|001000| Arc_Sin_Hyperbolic
//! |-|1|xx|xxxxxx|xxxxxx|xxxxxx|1000|001000| Tan
//! |-|1|xx|xxxxxx|xxxxxx|xxxxxx|1001|001000| Tan_Hyperbolic
//! |-|1|xx|xxxxxx|xxxxxx|xxxxxx|1010|001000| Arc_Tan
//! |-|1|xx|xxxxxx|xxxxxx|xxxxxx|1011|001000| Arc_Tan_Hyperbolic
//! |-|1|xx|xxxxxx|xxxxxx|xxxxxx|1100|001000| Arc_Tan_2
//! |-|1|xx|xxxxxx|xxxxxx|xxxxxx|1101|001000| Exponential
//! |-|1|xx|xxxxxx|xxxxxx|xxxxxx|1110|001000| Logarithm
//! |-|1|xx|xxxxxx|xxxxxx|xxxxxx|1111|001000| Square_Root
//! ```

use core::fmt;

use crate::{bit_width::Bit_Width, register::Register};

// 0b00000000000000000000000000111111
const OP_CODE_SHIFT: u32 = 0;
const OP_CODE_MASK: u32 = 0x3F;

// 0b00000000000000000000001111000000
const FUNCTION_SHIFT: u32 = 6;
const FUNCTION_MASK: u32 = 0xF;

// 0b00000000000000001111110000000000
// 0b00000000001111110000000000000000
// 0b00001111110000000000000000000000
const R0_SHIFT: u32 = 10;
const R1_SHIFT: u32 = 16;
const R2_SHIFT: u32 = 22;
const REIGSTER_MASK: u32 = 0x3F;

// 0b00110000000000000000000000000000
const SIZE_SHIFT: u32 = 28;
const SIZE_MASK: u32 = 0x3;

// 0b01000000000000000000000000000000
const F_FLAG_SHIFT: u32 = 30;
const F_FLAG_MASK: u32 = 0x1;

// 0b10000000000000000000000000000000
const S_FLAG_SHIFT: u32 = 31;
const S_FLAG_MASK: u32 = 0x1;

// 0b11111111111111110000000000000000
const IMMEDIATE_SHIFT: u32 = 16;
const IMMEDIATE_MASK: u32 = 0xFFFF;

pub fn op_code(i: u32) -> u32 {
    (i >> OP_CODE_SHIFT) & OP_CODE_MASK
}

pub fn function(i: u32) -> u32 {
    (i >> FUNCTION_SHIFT) & FUNCTION_MASK
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct R_Type(pub u32);

impl R_Type {
    #[inline(always)]
    pub fn register_0(self) -> Register {
        Register::from_index(((self.0 >> R0_SHIFT) & REIGSTER_MASK) as u8)
    }

    #[inline(always)]
    pub fn register_1(self) -> Register {
        Register::from_index(((self.0 >> R1_SHIFT) & REIGSTER_MASK) as u8)
    }

    #[inline(always)]
    pub fn register_2(self) -> Register {
        Register::from_index(((self.0 >> R2_SHIFT) & REIGSTER_MASK) as u8)
    }

    #[inline(always)]
    pub fn bit_width(self) -> Bit_Width {
        Bit_Width::from_mask(((self.0 >> SIZE_SHIFT) & SIZE_MASK) as u8)
    }

    #[inline(always)]
    pub fn f_flag(self) -> bool {
        ((self.0 >> F_FLAG_SHIFT) & F_FLAG_MASK) == 1
    }

    #[inline(always)]
    pub fn s_flag(self) -> bool {
        ((self.0 >> S_FLAG_SHIFT) & S_FLAG_MASK) == 1
    }
}

impl fmt::Debug for R_Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("R_Type")
            .field("op_code", &op_code(self.0))
            .field("function", &function(self.0))
            .field("register_0", &self.register_0())
            .field("register_1", &self.register_1())
            .field("register_2", &self.register_2())
            .field("bit_width", &self.bit_width())
            .field("f_flag", &self.f_flag())
            .field("s_flag", &self.s_flag())
            .finish()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct I_Type(pub u32);

impl I_Type {
    #[inline(always)]
    pub fn register(self) -> Register {
        Register::from_index(((self.0 >> R0_SHIFT) & REIGSTER_MASK) as u8)
    }

    #[inline(always)]
    pub fn immediate(self) -> i16 {
        ((self.0 >> IMMEDIATE_SHIFT) & IMMEDIATE_MASK) as i16
    }
}

impl fmt::Debug for I_Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("I_Type")
            .field("op_code", &op_code(self.0))
            .field("function", &function(self.0))
            .field("register", &self.register())
            .field("immediate", &self.immediate())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_code() {
        assert_eq!(op_code(0b00000000000000000000000000_101010), 0x2A);
        assert_eq!(op_code(0b00000000000000000000000000_111001), 0x39);
        assert_eq!(op_code(0b00000000000000000000000000_100111), 0x27);
        assert_eq!(op_code(0b00000000000000000000000000_111111), 0x3F);
    }

    #[test]
    fn test_function() {
        assert_eq!(function(0b0000000000000000000000_1111_000000), 0xF);
        assert_eq!(function(0b0000000000000000000000_1011_000000), 0xB);
        assert_eq!(function(0b0000000000000000000000_1101_000000), 0xD);
        assert_eq!(function(0b0000000000000000000000_1000_000000), 0x8);
    }

    #[test]
    fn test_r_type_register_0() {
        assert_eq!(
            R_Type(0b0000000000000000_000000_0000000000).register_0(),
            Register::Zero,
        );
        assert_eq!(
            R_Type(0b0000000000000000_000101_0000000000).register_0(),
            Register::General_Purpose(0x05),
        );
        assert_eq!(
            R_Type(0b0000000000000000_100111_0000000000).register_0(),
            Register::General_Purpose(0x27),
        );
        assert_eq!(
            R_Type(0b0000000000000000_111111_0000000000).register_0(),
            Register::General_Purpose(0x3F),
        );
    }

    #[test]
    fn test_r_type_register_1() {
        assert_eq!(
            R_Type(0b0000000000_000001_0000000000000000).register_1(),
            Register::One,
        );
        assert_eq!(
            R_Type(0b0000000000_000111_0000000000000000).register_1(),
            Register::General_Purpose(0x07),
        );
        assert_eq!(
            R_Type(0b0000000000_101011_0000000000000000).register_1(),
            Register::General_Purpose(0x2B),
        );
        assert_eq!(
            R_Type(0b0000000000_100001_0000000000000000).register_1(),
            Register::General_Purpose(0x21),
        );
    }

    #[test]
    fn test_r_type_register_2() {
        assert_eq!(
            R_Type(0b0000_000010_0000000000000000000000).register_2(),
            Register::Instruction_Pointer,
        );
        assert_eq!(
            R_Type(0b0000_000011_0000000000000000000000).register_2(),
            Register::Stack_Pointer,
        );
        assert_eq!(
            R_Type(0b0000_001011_0000000000000000000000).register_2(),
            Register::General_Purpose(0x0B),
        );
        assert_eq!(
            R_Type(0b0000_100011_0000000000000000000000).register_2(),
            Register::General_Purpose(0x23),
        );
    }

    #[test]
    fn test_r_type_bit_width() {
        assert_eq!(
            R_Type(0b00_00_0000000000000000000000000000).bit_width(),
            Bit_Width::Eight
        );
        assert_eq!(
            R_Type(0b00_01_0000000000000000000000000000).bit_width(),
            Bit_Width::Sixteen
        );
        assert_eq!(
            R_Type(0b00_10_0000000000000000000000000000).bit_width(),
            Bit_Width::Thirty_Two
        );
        assert_eq!(
            R_Type(0b00_11_0000000000000000000000000000).bit_width(),
            Bit_Width::Sixty_Four
        );
    }

    #[test]
    fn test_r_type_f_flag() {
        assert_eq!(R_Type(0b1_0_111111111111111111111111111111).f_flag(), false);
        assert_eq!(R_Type(0b0_1_000000000000000000000000000000).f_flag(), true);
    }

    #[test]
    fn test_r_type_s_flag() {
        assert_eq!(R_Type(0b_0_0000000000000000000000000000000).s_flag(), false);
        assert_eq!(R_Type(0b_1_0000000000000000000000000000000).s_flag(), true);
    }

    #[test]
    fn test_i_type_register() {
        assert_eq!(
            I_Type(0b0000000000000000_000100_0000000000).register(),
            Register::Frame_Pointer,
        );
        assert_eq!(
            I_Type(0b0000000000000000_001111_0000000000).register(),
            Register::General_Purpose(0x0F),
        );
        assert_eq!(
            I_Type(0b0000000000000000_001101_0000000000).register(),
            Register::General_Purpose(0x0D),
        );
        assert_eq!(
            I_Type(0b0000000000000000_110011_0000000000).register(),
            Register::General_Purpose(0x33),
        );
    }

    #[test]
    fn test_i_type_immediate() {
        assert_eq!(I_Type(0b_1111111111111110_0000000000000000).immediate(), -2);
    }

    #[test]
    fn test() {
        let i = 0b10100000010001010001011110000110;
        let r = R_Type(i);

        println!("{:032b}", i);
        println!("{:032b}", i >> F_FLAG_SHIFT);
        println!("{:032b}", (i >> F_FLAG_SHIFT) & F_FLAG_MASK);

        assert_eq!(op_code(i), 0x06);
        assert_eq!(function(i), 0xE);

        assert_eq!(
            r.register_0(),
            Register::General_Purpose(0x05),
            "Register 0 should be GP(5)"
        );
        assert_eq!(
            r.register_1(),
            Register::General_Purpose(0x05),
            "Register 1 should be GP(5)"
        );
        assert_eq!(r.register_2(), Register::One, "Register 2 should be one");
        assert_eq!(
            r.bit_width(),
            Bit_Width::Thirty_Two,
            "Bit width should be 32"
        );
        assert_eq!(r.f_flag(), false, "F flag should be false");
        assert_eq!(r.s_flag(), true, "S flag should be true");
    }
}
