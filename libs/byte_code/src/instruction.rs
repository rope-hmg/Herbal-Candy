use crate::register::Register;
use crate::encoding::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Halt,
    Trap,
    Call { rs2: Register },
    Call_R { rs2: Register },
    Ret,
    Ecall { imm: i16 },
    Break,
    Jal { rd: Register, rs2: Register },
    Jal_R { rd: Register, rs2: Register },
    Jnz { rd: Register, rs1: Register, rs2: Register },
    Jnz_R { rd: Register, rs1: Register, rs2: Register },
    Jiz { rd: Register, rs1: Register, rs2: Register },
    Jiz_R { rd: Register, rs1: Register, rs2: Register },
    Load { rd: Register, rs1: Register, size: u8 },
    Loadi { rd: Register, imm: i16 },
    Store { rd: Register, rs1: Register, size: u8 },
    Storei { rd: Register, imm: i16 },
    Move { rd: Register, rs1: Register },
    Push { rd: Register },
    Pushi { imm: i16 },
    Pop { rd: Register },
    Ie { rd: Register, rs1: Register, rs2: Register },
    Ie_f32 { rd: Register, rs1: Register, rs2: Register },
    Ie_f64 { rd: Register, rs1: Register, rs2: Register },
    Ne { rd: Register, rs1: Register, rs2: Register },
    Ne_f32 { rd: Register, rs1: Register, rs2: Register },
    Ne_f64 { rd: Register, rs1: Register, rs2: Register },
    Lt { rd: Register, rs1: Register, rs2: Register },
    Lt_f32 { rd: Register, rs1: Register, rs2: Register },
    Lt_f64 { rd: Register, rs1: Register, rs2: Register },
    Le { rd: Register, rs1: Register, rs2: Register },
    Le_f32 { rd: Register, rs1: Register, rs2: Register },
    Le_f64 { rd: Register, rs1: Register, rs2: Register },
    Gt { rd: Register, rs1: Register, rs2: Register },
    Gt_f32 { rd: Register, rs1: Register, rs2: Register },
    Gt_f64 { rd: Register, rs1: Register, rs2: Register },
    Ge { rd: Register, rs1: Register, rs2: Register },
    Ge_f32 { rd: Register, rs1: Register, rs2: Register },
    Ge_f64 { rd: Register, rs1: Register, rs2: Register },
    And_i8 { rd: Register, rs1: Register, rs2: Register },
    And_i16 { rd: Register, rs1: Register, rs2: Register },
    And_i32 { rd: Register, rs1: Register, rs2: Register },
    And_i64 { rd: Register, rs1: Register, rs2: Register },
    And_u8 { rd: Register, rs1: Register, rs2: Register },
    And_u16 { rd: Register, rs1: Register, rs2: Register },
    And_u32 { rd: Register, rs1: Register, rs2: Register },
    And_u64 { rd: Register, rs1: Register, rs2: Register },
    Or_i8 { rd: Register, rs1: Register, rs2: Register },
    Or_i16 { rd: Register, rs1: Register, rs2: Register },
    Or_i32 { rd: Register, rs1: Register, rs2: Register },
    Or_i64 { rd: Register, rs1: Register, rs2: Register },
    Or_u8 { rd: Register, rs1: Register, rs2: Register },
    Or_u16 { rd: Register, rs1: Register, rs2: Register },
    Or_u32 { rd: Register, rs1: Register, rs2: Register },
    Or_u64 { rd: Register, rs1: Register, rs2: Register },
    Xor_i8 { rd: Register, rs1: Register, rs2: Register },
    Xor_i16 { rd: Register, rs1: Register, rs2: Register },
    Xor_i32 { rd: Register, rs1: Register, rs2: Register },
    Xor_i64 { rd: Register, rs1: Register, rs2: Register },
    Xor_u8 { rd: Register, rs1: Register, rs2: Register },
    Xor_u16 { rd: Register, rs1: Register, rs2: Register },
    Xor_u32 { rd: Register, rs1: Register, rs2: Register },
    Xor_u64 { rd: Register, rs1: Register, rs2: Register },
    Not_i8 { rd: Register, rs1: Register },
    Not_i16 { rd: Register, rs1: Register },
    Not_i32 { rd: Register, rs1: Register },
    Not_i64 { rd: Register, rs1: Register },
    Not_u8 { rd: Register, rs1: Register },
    Not_u16 { rd: Register, rs1: Register },
    Not_u32 { rd: Register, rs1: Register },
    Not_u64 { rd: Register, rs1: Register },
    Shl_i8 { rd: Register, rs1: Register, rs2: Register },
    Shl_i16 { rd: Register, rs1: Register, rs2: Register },
    Shl_i32 { rd: Register, rs1: Register, rs2: Register },
    Shl_i64 { rd: Register, rs1: Register, rs2: Register },
    Shl_u8 { rd: Register, rs1: Register, rs2: Register },
    Shl_u16 { rd: Register, rs1: Register, rs2: Register },
    Shl_u32 { rd: Register, rs1: Register, rs2: Register },
    Shl_u64 { rd: Register, rs1: Register, rs2: Register },
    Shr_i8 { rd: Register, rs1: Register, rs2: Register },
    Shr_i16 { rd: Register, rs1: Register, rs2: Register },
    Shr_i32 { rd: Register, rs1: Register, rs2: Register },
    Shr_i64 { rd: Register, rs1: Register, rs2: Register },
    Shr_u8 { rd: Register, rs1: Register, rs2: Register },
    Shr_u16 { rd: Register, rs1: Register, rs2: Register },
    Shr_u32 { rd: Register, rs1: Register, rs2: Register },
    Shr_u64 { rd: Register, rs1: Register, rs2: Register },
    Rotl_i8 { rd: Register, rs1: Register, rs2: Register },
    Rotl_i16 { rd: Register, rs1: Register, rs2: Register },
    Rotl_i32 { rd: Register, rs1: Register, rs2: Register },
    Rotl_i64 { rd: Register, rs1: Register, rs2: Register },
    Rotl_u8 { rd: Register, rs1: Register, rs2: Register },
    Rotl_u16 { rd: Register, rs1: Register, rs2: Register },
    Rotl_u32 { rd: Register, rs1: Register, rs2: Register },
    Rotl_u64 { rd: Register, rs1: Register, rs2: Register },
    Rotr_i8 { rd: Register, rs1: Register, rs2: Register },
    Rotr_i16 { rd: Register, rs1: Register, rs2: Register },
    Rotr_i32 { rd: Register, rs1: Register, rs2: Register },
    Rotr_i64 { rd: Register, rs1: Register, rs2: Register },
    Rotr_u8 { rd: Register, rs1: Register, rs2: Register },
    Rotr_u16 { rd: Register, rs1: Register, rs2: Register },
    Rotr_u32 { rd: Register, rs1: Register, rs2: Register },
    Rotr_u64 { rd: Register, rs1: Register, rs2: Register },
    Count_Ones_i8 { rd: Register, rs1: Register },
    Count_Ones_i16 { rd: Register, rs1: Register },
    Count_Ones_i32 { rd: Register, rs1: Register },
    Count_Ones_i64 { rd: Register, rs1: Register },
    Count_Ones_u8 { rd: Register, rs1: Register },
    Count_Ones_u16 { rd: Register, rs1: Register },
    Count_Ones_u32 { rd: Register, rs1: Register },
    Count_Ones_u64 { rd: Register, rs1: Register },
    Leading_Ones_i8 { rd: Register, rs1: Register },
    Leading_Ones_i16 { rd: Register, rs1: Register },
    Leading_Ones_i32 { rd: Register, rs1: Register },
    Leading_Ones_i64 { rd: Register, rs1: Register },
    Leading_Ones_u8 { rd: Register, rs1: Register },
    Leading_Ones_u16 { rd: Register, rs1: Register },
    Leading_Ones_u32 { rd: Register, rs1: Register },
    Leading_Ones_u64 { rd: Register, rs1: Register },
    Trailing_Ones_i8 { rd: Register, rs1: Register },
    Trailing_Ones_i16 { rd: Register, rs1: Register },
    Trailing_Ones_i32 { rd: Register, rs1: Register },
    Trailing_Ones_i64 { rd: Register, rs1: Register },
    Trailing_Ones_u8 { rd: Register, rs1: Register },
    Trailing_Ones_u16 { rd: Register, rs1: Register },
    Trailing_Ones_u32 { rd: Register, rs1: Register },
    Trailing_Ones_u64 { rd: Register, rs1: Register },
    Count_Zeros_i8 { rd: Register, rs1: Register },
    Count_Zeros_i16 { rd: Register, rs1: Register },
    Count_Zeros_i32 { rd: Register, rs1: Register },
    Count_Zeros_i64 { rd: Register, rs1: Register },
    Count_Zeros_u8 { rd: Register, rs1: Register },
    Count_Zeros_u16 { rd: Register, rs1: Register },
    Count_Zeros_u32 { rd: Register, rs1: Register },
    Count_Zeros_u64 { rd: Register, rs1: Register },
    Leading_Zeros_i8 { rd: Register, rs1: Register },
    Leading_Zeros_i16 { rd: Register, rs1: Register },
    Leading_Zeros_i32 { rd: Register, rs1: Register },
    Leading_Zeros_i64 { rd: Register, rs1: Register },
    Leading_Zeros_u8 { rd: Register, rs1: Register },
    Leading_Zeros_u16 { rd: Register, rs1: Register },
    Leading_Zeros_u32 { rd: Register, rs1: Register },
    Leading_Zeros_u64 { rd: Register, rs1: Register },
    Trailing_Zeros_i8 { rd: Register, rs1: Register },
    Trailing_Zeros_i16 { rd: Register, rs1: Register },
    Trailing_Zeros_i32 { rd: Register, rs1: Register },
    Trailing_Zeros_i64 { rd: Register, rs1: Register },
    Trailing_Zeros_u8 { rd: Register, rs1: Register },
    Trailing_Zeros_u16 { rd: Register, rs1: Register },
    Trailing_Zeros_u32 { rd: Register, rs1: Register },
    Trailing_Zeros_u64 { rd: Register, rs1: Register },
    Reverse_Bytes_i8 { rd: Register, rs1: Register },
    Reverse_Bytes_i16 { rd: Register, rs1: Register },
    Reverse_Bytes_i32 { rd: Register, rs1: Register },
    Reverse_Bytes_i64 { rd: Register, rs1: Register },
    Reverse_Bytes_u8 { rd: Register, rs1: Register },
    Reverse_Bytes_u16 { rd: Register, rs1: Register },
    Reverse_Bytes_u32 { rd: Register, rs1: Register },
    Reverse_Bytes_u64 { rd: Register, rs1: Register },
    Reverse_Bits_i8 { rd: Register, rs1: Register },
    Reverse_Bits_i16 { rd: Register, rs1: Register },
    Reverse_Bits_i32 { rd: Register, rs1: Register },
    Reverse_Bits_i64 { rd: Register, rs1: Register },
    Reverse_Bits_u8 { rd: Register, rs1: Register },
    Reverse_Bits_u16 { rd: Register, rs1: Register },
    Reverse_Bits_u32 { rd: Register, rs1: Register },
    Reverse_Bits_u64 { rd: Register, rs1: Register },
    C_Abs_i8 { rd: Register, rs1: Register },
    C_Abs_i16 { rd: Register, rs1: Register },
    C_Abs_i32 { rd: Register, rs1: Register },
    C_Abs_i64 { rd: Register, rs1: Register },
    C_Add_i8 { rd: Register, rs1: Register, rs2: Register },
    C_Add_i16 { rd: Register, rs1: Register, rs2: Register },
    C_Add_i32 { rd: Register, rs1: Register, rs2: Register },
    C_Add_i64 { rd: Register, rs1: Register, rs2: Register },
    C_Add_u8 { rd: Register, rs1: Register, rs2: Register },
    C_Add_u16 { rd: Register, rs1: Register, rs2: Register },
    C_Add_u32 { rd: Register, rs1: Register, rs2: Register },
    C_Add_u64 { rd: Register, rs1: Register, rs2: Register },
    C_Add_U_i8 { rd: Register, rs1: Register, rs2: Register },
    C_Add_U_i16 { rd: Register, rs1: Register, rs2: Register },
    C_Add_U_i32 { rd: Register, rs1: Register, rs2: Register },
    C_Add_U_i64 { rd: Register, rs1: Register, rs2: Register },
    C_Add_S_u8 { rd: Register, rs1: Register, rs2: Register },
    C_Add_S_u16 { rd: Register, rs1: Register, rs2: Register },
    C_Add_S_u32 { rd: Register, rs1: Register, rs2: Register },
    C_Add_S_u64 { rd: Register, rs1: Register, rs2: Register },
    C_Div_i8 { rd: Register, rs1: Register, rs2: Register },
    C_Div_i16 { rd: Register, rs1: Register, rs2: Register },
    C_Div_i32 { rd: Register, rs1: Register, rs2: Register },
    C_Div_i64 { rd: Register, rs1: Register, rs2: Register },
    C_Div_u8 { rd: Register, rs1: Register, rs2: Register },
    C_Div_u16 { rd: Register, rs1: Register, rs2: Register },
    C_Div_u32 { rd: Register, rs1: Register, rs2: Register },
    C_Div_u64 { rd: Register, rs1: Register, rs2: Register },
    C_Div_E_i8 { rd: Register, rs1: Register, rs2: Register },
    C_Div_E_i16 { rd: Register, rs1: Register, rs2: Register },
    C_Div_E_i32 { rd: Register, rs1: Register, rs2: Register },
    C_Div_E_i64 { rd: Register, rs1: Register, rs2: Register },
    C_Div_E_u8 { rd: Register, rs1: Register, rs2: Register },
    C_Div_E_u16 { rd: Register, rs1: Register, rs2: Register },
    C_Div_E_u32 { rd: Register, rs1: Register, rs2: Register },
    C_Div_E_u64 { rd: Register, rs1: Register, rs2: Register },
    C_Log_i8 { rd: Register, rs1: Register, rs2: Register },
    C_Log_i16 { rd: Register, rs1: Register, rs2: Register },
    C_Log_i32 { rd: Register, rs1: Register, rs2: Register },
    C_Log_i64 { rd: Register, rs1: Register, rs2: Register },
    C_Log_u8 { rd: Register, rs1: Register, rs2: Register },
    C_Log_u16 { rd: Register, rs1: Register, rs2: Register },
    C_Log_u32 { rd: Register, rs1: Register, rs2: Register },
    C_Log_u64 { rd: Register, rs1: Register, rs2: Register },
    C_Sqrt_i8 { rd: Register, rs1: Register },
    C_Sqrt_i16 { rd: Register, rs1: Register },
    C_Sqrt_i32 { rd: Register, rs1: Register },
    C_Sqrt_i64 { rd: Register, rs1: Register },
    C_Sqrt_u8 { rd: Register, rs1: Register },
    C_Sqrt_u16 { rd: Register, rs1: Register },
    C_Sqrt_u32 { rd: Register, rs1: Register },
    C_Sqrt_u64 { rd: Register, rs1: Register },
    C_Mul_i8 { rd: Register, rs1: Register, rs2: Register },
    C_Mul_i16 { rd: Register, rs1: Register, rs2: Register },
    C_Mul_i32 { rd: Register, rs1: Register, rs2: Register },
    C_Mul_i64 { rd: Register, rs1: Register, rs2: Register },
    C_Mul_u8 { rd: Register, rs1: Register, rs2: Register },
    C_Mul_u16 { rd: Register, rs1: Register, rs2: Register },
    C_Mul_u32 { rd: Register, rs1: Register, rs2: Register },
    C_Mul_u64 { rd: Register, rs1: Register, rs2: Register },
    C_Neg_i8 { rd: Register, rs1: Register },
    C_Neg_i16 { rd: Register, rs1: Register },
    C_Neg_i32 { rd: Register, rs1: Register },
    C_Neg_i64 { rd: Register, rs1: Register },
    C_Pow_i8 { rd: Register, rs1: Register, rs2: Register },
    C_Pow_i16 { rd: Register, rs1: Register, rs2: Register },
    C_Pow_i32 { rd: Register, rs1: Register, rs2: Register },
    C_Pow_i64 { rd: Register, rs1: Register, rs2: Register },
    C_Pow_u8 { rd: Register, rs1: Register, rs2: Register },
    C_Pow_u16 { rd: Register, rs1: Register, rs2: Register },
    C_Pow_u32 { rd: Register, rs1: Register, rs2: Register },
    C_Pow_u64 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_i8 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_i16 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_i32 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_i64 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_u8 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_u16 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_u32 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_u64 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_E_i8 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_E_i16 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_E_i32 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_E_i64 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_E_u8 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_E_u16 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_E_u32 { rd: Register, rs1: Register, rs2: Register },
    C_Rem_E_u64 { rd: Register, rs1: Register, rs2: Register },
    C_Shl_i8 { rd: Register, rs1: Register, rs2: Register },
    C_Shl_i16 { rd: Register, rs1: Register, rs2: Register },
    C_Shl_i32 { rd: Register, rs1: Register, rs2: Register },
    C_Shl_i64 { rd: Register, rs1: Register, rs2: Register },
    C_Shl_u8 { rd: Register, rs1: Register, rs2: Register },
    C_Shl_u16 { rd: Register, rs1: Register, rs2: Register },
    C_Shl_u32 { rd: Register, rs1: Register, rs2: Register },
    C_Shl_u64 { rd: Register, rs1: Register, rs2: Register },
    C_Shr_i8 { rd: Register, rs1: Register, rs2: Register },
    C_Shr_i16 { rd: Register, rs1: Register, rs2: Register },
    C_Shr_i32 { rd: Register, rs1: Register, rs2: Register },
    C_Shr_i64 { rd: Register, rs1: Register, rs2: Register },
    C_Shr_u8 { rd: Register, rs1: Register, rs2: Register },
    C_Shr_u16 { rd: Register, rs1: Register, rs2: Register },
    C_Shr_u32 { rd: Register, rs1: Register, rs2: Register },
    C_Shr_u64 { rd: Register, rs1: Register, rs2: Register },
    C_Sub_i8 { rd: Register, rs1: Register, rs2: Register },
    C_Sub_i16 { rd: Register, rs1: Register, rs2: Register },
    C_Sub_i32 { rd: Register, rs1: Register, rs2: Register },
    C_Sub_i64 { rd: Register, rs1: Register, rs2: Register },
    C_Sub_u8 { rd: Register, rs1: Register, rs2: Register },
    C_Sub_u16 { rd: Register, rs1: Register, rs2: Register },
    C_Sub_u32 { rd: Register, rs1: Register, rs2: Register },
    C_Sub_u64 { rd: Register, rs1: Register, rs2: Register },
    C_Sub_U_i8 { rd: Register, rs1: Register, rs2: Register },
    C_Sub_U_i16 { rd: Register, rs1: Register, rs2: Register },
    C_Sub_U_i32 { rd: Register, rs1: Register, rs2: Register },
    C_Sub_U_i64 { rd: Register, rs1: Register, rs2: Register },
    O_Abs_i8 { rd: Register, rs1: Register },
    O_Abs_i16 { rd: Register, rs1: Register },
    O_Abs_i32 { rd: Register, rs1: Register },
    O_Abs_i64 { rd: Register, rs1: Register },
    O_Add_i8 { rd: Register, rs1: Register, rs2: Register },
    O_Add_i16 { rd: Register, rs1: Register, rs2: Register },
    O_Add_i32 { rd: Register, rs1: Register, rs2: Register },
    O_Add_i64 { rd: Register, rs1: Register, rs2: Register },
    O_Add_u8 { rd: Register, rs1: Register, rs2: Register },
    O_Add_u16 { rd: Register, rs1: Register, rs2: Register },
    O_Add_u32 { rd: Register, rs1: Register, rs2: Register },
    O_Add_u64 { rd: Register, rs1: Register, rs2: Register },
    O_Add_U_i8 { rd: Register, rs1: Register, rs2: Register },
    O_Add_U_i16 { rd: Register, rs1: Register, rs2: Register },
    O_Add_U_i32 { rd: Register, rs1: Register, rs2: Register },
    O_Add_U_i64 { rd: Register, rs1: Register, rs2: Register },
    O_Add_S_u8 { rd: Register, rs1: Register, rs2: Register },
    O_Add_S_u16 { rd: Register, rs1: Register, rs2: Register },
    O_Add_S_u32 { rd: Register, rs1: Register, rs2: Register },
    O_Add_S_u64 { rd: Register, rs1: Register, rs2: Register },
    O_Div_i8 { rd: Register, rs1: Register, rs2: Register },
    O_Div_i16 { rd: Register, rs1: Register, rs2: Register },
    O_Div_i32 { rd: Register, rs1: Register, rs2: Register },
    O_Div_i64 { rd: Register, rs1: Register, rs2: Register },
    O_Div_u8 { rd: Register, rs1: Register, rs2: Register },
    O_Div_u16 { rd: Register, rs1: Register, rs2: Register },
    O_Div_u32 { rd: Register, rs1: Register, rs2: Register },
    O_Div_u64 { rd: Register, rs1: Register, rs2: Register },
    O_Div_E_i8 { rd: Register, rs1: Register, rs2: Register },
    O_Div_E_i16 { rd: Register, rs1: Register, rs2: Register },
    O_Div_E_i32 { rd: Register, rs1: Register, rs2: Register },
    O_Div_E_i64 { rd: Register, rs1: Register, rs2: Register },
    O_Div_E_u8 { rd: Register, rs1: Register, rs2: Register },
    O_Div_E_u16 { rd: Register, rs1: Register, rs2: Register },
    O_Div_E_u32 { rd: Register, rs1: Register, rs2: Register },
    O_Div_E_u64 { rd: Register, rs1: Register, rs2: Register },
    O_Mul_i8 { rd: Register, rs1: Register, rs2: Register },
    O_Mul_i16 { rd: Register, rs1: Register, rs2: Register },
    O_Mul_i32 { rd: Register, rs1: Register, rs2: Register },
    O_Mul_i64 { rd: Register, rs1: Register, rs2: Register },
    O_Mul_u8 { rd: Register, rs1: Register, rs2: Register },
    O_Mul_u16 { rd: Register, rs1: Register, rs2: Register },
    O_Mul_u32 { rd: Register, rs1: Register, rs2: Register },
    O_Mul_u64 { rd: Register, rs1: Register, rs2: Register },
    O_Neg_i8 { rd: Register, rs1: Register },
    O_Neg_i16 { rd: Register, rs1: Register },
    O_Neg_i32 { rd: Register, rs1: Register },
    O_Neg_i64 { rd: Register, rs1: Register },
    O_Pow_i8 { rd: Register, rs1: Register, rs2: Register },
    O_Pow_i16 { rd: Register, rs1: Register, rs2: Register },
    O_Pow_i32 { rd: Register, rs1: Register, rs2: Register },
    O_Pow_i64 { rd: Register, rs1: Register, rs2: Register },
    O_Pow_u8 { rd: Register, rs1: Register, rs2: Register },
    O_Pow_u16 { rd: Register, rs1: Register, rs2: Register },
    O_Pow_u32 { rd: Register, rs1: Register, rs2: Register },
    O_Pow_u64 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_i8 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_i16 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_i32 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_i64 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_u8 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_u16 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_u32 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_u64 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_E_i8 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_E_i16 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_E_i32 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_E_i64 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_E_u8 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_E_u16 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_E_u32 { rd: Register, rs1: Register, rs2: Register },
    O_Rem_E_u64 { rd: Register, rs1: Register, rs2: Register },
    O_Shl_i8 { rd: Register, rs1: Register, rs2: Register },
    O_Shl_i16 { rd: Register, rs1: Register, rs2: Register },
    O_Shl_i32 { rd: Register, rs1: Register, rs2: Register },
    O_Shl_i64 { rd: Register, rs1: Register, rs2: Register },
    O_Shl_u8 { rd: Register, rs1: Register, rs2: Register },
    O_Shl_u16 { rd: Register, rs1: Register, rs2: Register },
    O_Shl_u32 { rd: Register, rs1: Register, rs2: Register },
    O_Shl_u64 { rd: Register, rs1: Register, rs2: Register },
    O_Shr_i8 { rd: Register, rs1: Register, rs2: Register },
    O_Shr_i16 { rd: Register, rs1: Register, rs2: Register },
    O_Shr_i32 { rd: Register, rs1: Register, rs2: Register },
    O_Shr_i64 { rd: Register, rs1: Register, rs2: Register },
    O_Shr_u8 { rd: Register, rs1: Register, rs2: Register },
    O_Shr_u16 { rd: Register, rs1: Register, rs2: Register },
    O_Shr_u32 { rd: Register, rs1: Register, rs2: Register },
    O_Shr_u64 { rd: Register, rs1: Register, rs2: Register },
    O_Sub_i8 { rd: Register, rs1: Register, rs2: Register },
    O_Sub_i16 { rd: Register, rs1: Register, rs2: Register },
    O_Sub_i32 { rd: Register, rs1: Register, rs2: Register },
    O_Sub_i64 { rd: Register, rs1: Register, rs2: Register },
    O_Sub_u8 { rd: Register, rs1: Register, rs2: Register },
    O_Sub_u16 { rd: Register, rs1: Register, rs2: Register },
    O_Sub_u32 { rd: Register, rs1: Register, rs2: Register },
    O_Sub_u64 { rd: Register, rs1: Register, rs2: Register },
    O_Sub_U_i8 { rd: Register, rs1: Register, rs2: Register },
    O_Sub_U_i16 { rd: Register, rs1: Register, rs2: Register },
    O_Sub_U_i32 { rd: Register, rs1: Register, rs2: Register },
    O_Sub_U_i64 { rd: Register, rs1: Register, rs2: Register },
    S_Abs_i8 { rd: Register, rs1: Register },
    S_Abs_i16 { rd: Register, rs1: Register },
    S_Abs_i32 { rd: Register, rs1: Register },
    S_Abs_i64 { rd: Register, rs1: Register },
    S_Add_i8 { rd: Register, rs1: Register, rs2: Register },
    S_Add_i16 { rd: Register, rs1: Register, rs2: Register },
    S_Add_i32 { rd: Register, rs1: Register, rs2: Register },
    S_Add_i64 { rd: Register, rs1: Register, rs2: Register },
    S_Add_u8 { rd: Register, rs1: Register, rs2: Register },
    S_Add_u16 { rd: Register, rs1: Register, rs2: Register },
    S_Add_u32 { rd: Register, rs1: Register, rs2: Register },
    S_Add_u64 { rd: Register, rs1: Register, rs2: Register },
    S_Add_U_i8 { rd: Register, rs1: Register, rs2: Register },
    S_Add_U_i16 { rd: Register, rs1: Register, rs2: Register },
    S_Add_U_i32 { rd: Register, rs1: Register, rs2: Register },
    S_Add_U_i64 { rd: Register, rs1: Register, rs2: Register },
    S_Add_S_u8 { rd: Register, rs1: Register, rs2: Register },
    S_Add_S_u16 { rd: Register, rs1: Register, rs2: Register },
    S_Add_S_u32 { rd: Register, rs1: Register, rs2: Register },
    S_Add_S_u64 { rd: Register, rs1: Register, rs2: Register },
    S_Div_i8 { rd: Register, rs1: Register, rs2: Register },
    S_Div_i16 { rd: Register, rs1: Register, rs2: Register },
    S_Div_i32 { rd: Register, rs1: Register, rs2: Register },
    S_Div_i64 { rd: Register, rs1: Register, rs2: Register },
    S_Div_u8 { rd: Register, rs1: Register, rs2: Register },
    S_Div_u16 { rd: Register, rs1: Register, rs2: Register },
    S_Div_u32 { rd: Register, rs1: Register, rs2: Register },
    S_Div_u64 { rd: Register, rs1: Register, rs2: Register },
    S_Mul_i8 { rd: Register, rs1: Register, rs2: Register },
    S_Mul_i16 { rd: Register, rs1: Register, rs2: Register },
    S_Mul_i32 { rd: Register, rs1: Register, rs2: Register },
    S_Mul_i64 { rd: Register, rs1: Register, rs2: Register },
    S_Mul_u8 { rd: Register, rs1: Register, rs2: Register },
    S_Mul_u16 { rd: Register, rs1: Register, rs2: Register },
    S_Mul_u32 { rd: Register, rs1: Register, rs2: Register },
    S_Mul_u64 { rd: Register, rs1: Register, rs2: Register },
    S_Neg_i8 { rd: Register, rs1: Register },
    S_Neg_i16 { rd: Register, rs1: Register },
    S_Neg_i32 { rd: Register, rs1: Register },
    S_Neg_i64 { rd: Register, rs1: Register },
    S_Pow_i8 { rd: Register, rs1: Register, rs2: Register },
    S_Pow_i16 { rd: Register, rs1: Register, rs2: Register },
    S_Pow_i32 { rd: Register, rs1: Register, rs2: Register },
    S_Pow_i64 { rd: Register, rs1: Register, rs2: Register },
    S_Pow_u8 { rd: Register, rs1: Register, rs2: Register },
    S_Pow_u16 { rd: Register, rs1: Register, rs2: Register },
    S_Pow_u32 { rd: Register, rs1: Register, rs2: Register },
    S_Pow_u64 { rd: Register, rs1: Register, rs2: Register },
    S_Sub_i8 { rd: Register, rs1: Register, rs2: Register },
    S_Sub_i16 { rd: Register, rs1: Register, rs2: Register },
    S_Sub_i32 { rd: Register, rs1: Register, rs2: Register },
    S_Sub_i64 { rd: Register, rs1: Register, rs2: Register },
    S_Sub_u8 { rd: Register, rs1: Register, rs2: Register },
    S_Sub_u16 { rd: Register, rs1: Register, rs2: Register },
    S_Sub_u32 { rd: Register, rs1: Register, rs2: Register },
    S_Sub_u64 { rd: Register, rs1: Register, rs2: Register },
    S_Sub_U_i8 { rd: Register, rs1: Register, rs2: Register },
    S_Sub_U_i16 { rd: Register, rs1: Register, rs2: Register },
    S_Sub_U_i32 { rd: Register, rs1: Register, rs2: Register },
    S_Sub_U_i64 { rd: Register, rs1: Register, rs2: Register },
    Abs_f32 { rd: Register, rs1: Register },
    Abs_f64 { rd: Register, rs1: Register },
    Add_f32 { rd: Register, rs1: Register, rs2: Register },
    Add_f64 { rd: Register, rs1: Register, rs2: Register },
    Div_f32 { rd: Register, rs1: Register, rs2: Register },
    Div_f64 { rd: Register, rs1: Register, rs2: Register },
    Div_E_f32 { rd: Register, rs1: Register, rs2: Register },
    Div_E_f64 { rd: Register, rs1: Register, rs2: Register },
    Log_f32 { rd: Register, rs1: Register, rs2: Register },
    Log_f64 { rd: Register, rs1: Register, rs2: Register },
    Sqrt_f32 { rd: Register, rs1: Register },
    Sqrt_f64 { rd: Register, rs1: Register },
    Mul_f32 { rd: Register, rs1: Register, rs2: Register },
    Mul_f64 { rd: Register, rs1: Register, rs2: Register },
    Neg_f32 { rd: Register, rs1: Register },
    Neg_f64 { rd: Register, rs1: Register },
    Pow_f32 { rd: Register, rs1: Register, rs2: Register },
    Pow_f64 { rd: Register, rs1: Register, rs2: Register },
    Rem_f32 { rd: Register, rs1: Register, rs2: Register },
    Rem_f64 { rd: Register, rs1: Register, rs2: Register },
    Rem_E_f32 { rd: Register, rs1: Register, rs2: Register },
    Rem_E_f64 { rd: Register, rs1: Register, rs2: Register },
    Cbrt_f32 { rd: Register, rs1: Register },
    Cbrt_f64 { rd: Register, rs1: Register },
    Sub_f32 { rd: Register, rs1: Register, rs2: Register },
    Sub_f64 { rd: Register, rs1: Register, rs2: Register },
    Invalid(u32),
}
impl Instruction {
    pub fn decode(instr: u32) -> Instruction {
        let op_code = decode_op_code(instr);
        let funct = decode_funct(instr);
        let rd = decode_rd(instr);
        let rs1 = decode_rs1(instr);
        let rs2 = decode_rs2(instr);
        let size = decode_size(instr);
        let f = decode_f(instr);
        let s = decode_s(instr);
        let imm = decode_imm(instr);
        match op_code {
            0u8 if funct == 0u8 => Instruction::Halt,
            0u8 if funct == 1u8 => Instruction::Trap,
            0u8 if funct == 2u8 && s == 0u8 => Instruction::Call { rs2 },
            0u8 if funct == 2u8 && s == 1u8 => Instruction::Call_R { rs2 },
            0u8 if funct == 3u8 => Instruction::Ret,
            0u8 if funct == 4u8 => Instruction::Ecall { imm },
            0u8 if funct == 5u8 => Instruction::Break,
            0u8 if funct == 6u8 && s == 0u8 && !rd.is_readonly() => {
                Instruction::Jal { rd, rs2 }
            }
            0u8 if funct == 6u8 && s == 1u8 && !rd.is_readonly() => {
                Instruction::Jal_R { rd, rs2 }
            }
            0u8 if funct == 7u8 && s == 0u8 && !rd.is_readonly() => {
                Instruction::Jnz { rd, rs1, rs2 }
            }
            0u8 if funct == 7u8 && s == 1u8 && !rd.is_readonly() => {
                Instruction::Jnz_R { rd, rs1, rs2 }
            }
            0u8 if funct == 8u8 && s == 0u8 && !rd.is_readonly() => {
                Instruction::Jiz { rd, rs1, rs2 }
            }
            0u8 if funct == 8u8 && s == 1u8 && !rd.is_readonly() => {
                Instruction::Jiz_R { rd, rs1, rs2 }
            }
            1u8 if funct == 0u8 && !rd.is_readonly() => {
                Instruction::Load { rd, rs1, size }
            }
            1u8 if funct == 1u8 && !rd.is_readonly() => Instruction::Loadi { rd, imm },
            1u8 if funct == 2u8 && !rd.is_readonly() => {
                Instruction::Store {
                    rd,
                    rs1,
                    size,
                }
            }
            1u8 if funct == 3u8 && !rd.is_readonly() => Instruction::Storei { rd, imm },
            1u8 if funct == 4u8 && !rd.is_readonly() => Instruction::Move { rd, rs1 },
            1u8 if funct == 5u8 && !rd.is_readonly() => Instruction::Push { rd },
            1u8 if funct == 6u8 => Instruction::Pushi { imm },
            1u8 if funct == 7u8 && !rd.is_readonly() => Instruction::Pop { rd },
            2u8 if funct == 0u8 && f == 0u8 && !rd.is_readonly() => {
                Instruction::Ie { rd, rs1, rs2 }
            }
            2u8 if funct == 0u8 && size == 32u8 && f == 1u8 && !rd.is_readonly() => {
                Instruction::Ie_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            2u8 if funct == 0u8 && size == 64u8 && f == 1u8 && !rd.is_readonly() => {
                Instruction::Ie_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            2u8 if funct == 1u8 && f == 0u8 && !rd.is_readonly() => {
                Instruction::Ne { rd, rs1, rs2 }
            }
            2u8 if funct == 1u8 && size == 32u8 && f == 1u8 && !rd.is_readonly() => {
                Instruction::Ne_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            2u8 if funct == 1u8 && size == 64u8 && f == 1u8 && !rd.is_readonly() => {
                Instruction::Ne_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            2u8 if funct == 2u8 && f == 0u8 && !rd.is_readonly() => {
                Instruction::Lt { rd, rs1, rs2 }
            }
            2u8 if funct == 2u8 && size == 32u8 && f == 1u8 && !rd.is_readonly() => {
                Instruction::Lt_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            2u8 if funct == 2u8 && size == 64u8 && f == 1u8 && !rd.is_readonly() => {
                Instruction::Lt_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            2u8 if funct == 3u8 && f == 0u8 && !rd.is_readonly() => {
                Instruction::Le { rd, rs1, rs2 }
            }
            2u8 if funct == 3u8 && size == 32u8 && f == 1u8 && !rd.is_readonly() => {
                Instruction::Le_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            2u8 if funct == 3u8 && size == 64u8 && f == 1u8 && !rd.is_readonly() => {
                Instruction::Le_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            2u8 if funct == 4u8 && f == 0u8 && !rd.is_readonly() => {
                Instruction::Gt { rd, rs1, rs2 }
            }
            2u8 if funct == 4u8 && size == 32u8 && f == 1u8 && !rd.is_readonly() => {
                Instruction::Gt_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            2u8 if funct == 4u8 && size == 64u8 && f == 1u8 && !rd.is_readonly() => {
                Instruction::Gt_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            2u8 if funct == 5u8 && f == 0u8 && !rd.is_readonly() => {
                Instruction::Ge { rd, rs1, rs2 }
            }
            2u8 if funct == 5u8 && size == 32u8 && f == 1u8 && !rd.is_readonly() => {
                Instruction::Ge_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            2u8 if funct == 5u8 && size == 64u8 && f == 1u8 && !rd.is_readonly() => {
                Instruction::Ge_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 0u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::And_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 0u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::And_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 0u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::And_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 0u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::And_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 0u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::And_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 0u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::And_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 0u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::And_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 0u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::And_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::Or_i8 { rd, rs1, rs2 },
            3u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Or_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Or_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Or_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Instruction::Or_u8 { rd, rs1, rs2 },
            3u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Or_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Or_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Or_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Xor_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Xor_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Xor_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Xor_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Xor_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Xor_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Xor_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Xor_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::Not_i8 { rd, rs1 },
            3u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::Not_i16 { rd, rs1 },
            3u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::Not_i32 { rd, rs1 },
            3u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::Not_i64 { rd, rs1 },
            3u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Instruction::Not_u8 { rd, rs1 },
            3u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Instruction::Not_u16 { rd, rs1 },
            3u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Instruction::Not_u32 { rd, rs1 },
            3u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Instruction::Not_u64 { rd, rs1 },
            3u8 if funct == 4u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Shl_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 4u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Shl_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 4u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Shl_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 4u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Shl_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 4u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Shl_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 4u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Shl_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 4u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Shl_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 4u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Shl_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 5u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Shr_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 5u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Shr_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 5u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Shr_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 5u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Shr_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 5u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Shr_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 5u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Shr_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 5u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Shr_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 5u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Shr_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 6u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Rotl_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 6u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Rotl_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 6u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Rotl_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 6u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Rotl_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 6u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Rotl_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 6u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Rotl_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 6u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Rotl_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 6u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Rotl_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Rotr_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Rotr_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Rotr_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Rotr_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Rotr_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Rotr_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Rotr_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Rotr_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            3u8 if funct == 8u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Count_Ones_i8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 8u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Count_Ones_i16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 8u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Count_Ones_i32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 8u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Count_Ones_i64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 8u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Count_Ones_u8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 8u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Count_Ones_u16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 8u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Count_Ones_u32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 8u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Count_Ones_u64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Leading_Ones_i8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Leading_Ones_i16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Leading_Ones_i32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Leading_Ones_i64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Leading_Ones_u8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Leading_Ones_u16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Leading_Ones_u32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Leading_Ones_u64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 10u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Ones_i8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 10u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Ones_i16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 10u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Ones_i32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 10u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Ones_i64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 10u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Ones_u8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 10u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Ones_u16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 10u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Ones_u32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 10u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Ones_u64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 11u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Count_Zeros_i8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 11u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Count_Zeros_i16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 11u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Count_Zeros_i32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 11u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Count_Zeros_i64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 11u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Count_Zeros_u8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 11u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Count_Zeros_u16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 11u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Count_Zeros_u32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 11u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Count_Zeros_u64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 12u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Leading_Zeros_i8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 12u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Leading_Zeros_i16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 12u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Leading_Zeros_i32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 12u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Leading_Zeros_i64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 12u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Leading_Zeros_u8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 12u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Leading_Zeros_u16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 12u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Leading_Zeros_u32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 12u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Leading_Zeros_u64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 13u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Zeros_i8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 13u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Zeros_i16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 13u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Zeros_i32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 13u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Zeros_i64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 13u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Zeros_u8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 13u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Zeros_u16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 13u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Zeros_u32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 13u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Trailing_Zeros_u64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bytes_i8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bytes_i16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bytes_i32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bytes_i64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bytes_u8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bytes_u16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bytes_u32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bytes_u64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 15u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bits_i8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 15u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bits_i16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 15u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bits_i32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 15u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bits_i64 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 15u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bits_u8 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 15u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bits_u16 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 15u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bits_u32 {
                    rd,
                    rs1,
                }
            }
            3u8 if funct == 15u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::Reverse_Bits_u64 {
                    rd,
                    rs1,
                }
            }
            4u8 if funct == 0u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::C_Abs_i8 { rd, rs1 },
            4u8 if funct == 0u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::C_Abs_i16 { rd, rs1 },
            4u8 if funct == 0u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::C_Abs_i32 { rd, rs1 },
            4u8 if funct == 0u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::C_Abs_i64 { rd, rs1 },
            4u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Add_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Add_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Add_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Add_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Add_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Add_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Add_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Add_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Add_U_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Add_U_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Add_U_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Add_U_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Add_S_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Add_S_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Add_S_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Add_S_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Div_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Div_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Div_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Div_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Div_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Div_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Div_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Div_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 4u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Div_E_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 4u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Div_E_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 4u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Div_E_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 4u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Div_E_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 4u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Div_E_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 4u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Div_E_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 4u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Div_E_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 4u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Div_E_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 5u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Log_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 5u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Log_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 5u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Log_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 5u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Log_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 5u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Log_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 5u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Log_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 5u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Log_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 5u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Log_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 6u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::C_Sqrt_i8 { rd, rs1 },
            4u8 if funct == 6u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::C_Sqrt_i16 { rd, rs1 },
            4u8 if funct == 6u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::C_Sqrt_i32 { rd, rs1 },
            4u8 if funct == 6u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::C_Sqrt_i64 { rd, rs1 },
            4u8 if funct == 6u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Instruction::C_Sqrt_u8 { rd, rs1 },
            4u8 if funct == 6u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Instruction::C_Sqrt_u16 { rd, rs1 },
            4u8 if funct == 6u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Instruction::C_Sqrt_u32 { rd, rs1 },
            4u8 if funct == 6u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Instruction::C_Sqrt_u64 { rd, rs1 },
            4u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Mul_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Mul_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Mul_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Mul_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Mul_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Mul_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Mul_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Mul_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 8u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::C_Neg_i8 { rd, rs1 },
            4u8 if funct == 8u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::C_Neg_i16 { rd, rs1 },
            4u8 if funct == 8u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::C_Neg_i32 { rd, rs1 },
            4u8 if funct == 8u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::C_Neg_i64 { rd, rs1 },
            4u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Pow_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Pow_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Pow_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Pow_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Pow_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Pow_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Pow_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Pow_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 10u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 10u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 10u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 10u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 10u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 10u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 10u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 10u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 11u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_E_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 11u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_E_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 11u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_E_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 11u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_E_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 11u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_E_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 11u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_E_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 11u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_E_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 11u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Rem_E_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 12u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Shl_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 12u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Shl_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 12u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Shl_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 12u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Shl_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 12u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Shl_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 12u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Shl_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 12u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Shl_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 12u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Shl_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 13u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Shr_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 13u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Shr_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 13u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Shr_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 13u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Shr_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 13u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Shr_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 13u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Shr_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 13u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Shr_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 13u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Shr_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Sub_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Sub_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Sub_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Sub_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Sub_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Sub_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Sub_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::C_Sub_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 15u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Sub_U_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 15u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Sub_U_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 15u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Sub_U_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            4u8 if funct == 15u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::C_Sub_U_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 0u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::O_Abs_i8 { rd, rs1 },
            5u8 if funct == 0u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::O_Abs_i16 { rd, rs1 },
            5u8 if funct == 0u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::O_Abs_i32 { rd, rs1 },
            5u8 if funct == 0u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::O_Abs_i64 { rd, rs1 },
            5u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Add_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Add_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Add_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Add_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Add_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Add_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Add_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Add_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Add_U_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Add_U_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Add_U_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Add_U_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Add_S_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Add_S_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Add_S_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Add_S_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Div_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Div_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Div_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Div_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Div_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Div_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Div_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Div_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 4u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Div_E_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 4u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Div_E_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 4u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Div_E_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 4u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Div_E_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 4u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Div_E_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 4u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Div_E_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 4u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Div_E_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 4u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Div_E_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Mul_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Mul_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Mul_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Mul_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Mul_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Mul_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Mul_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Mul_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 8u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::O_Neg_i8 { rd, rs1 },
            5u8 if funct == 8u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::O_Neg_i16 { rd, rs1 },
            5u8 if funct == 8u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::O_Neg_i32 { rd, rs1 },
            5u8 if funct == 8u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::O_Neg_i64 { rd, rs1 },
            5u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Pow_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Pow_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Pow_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Pow_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Pow_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Pow_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Pow_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Pow_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 10u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 10u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 10u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 10u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 10u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 10u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 10u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 10u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 11u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_E_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 11u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_E_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 11u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_E_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 11u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_E_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 11u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_E_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 11u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_E_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 11u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_E_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 11u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Rem_E_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 12u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Shl_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 12u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Shl_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 12u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Shl_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 12u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Shl_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 12u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Shl_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 12u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Shl_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 12u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Shl_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 12u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Shl_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 13u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Shr_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 13u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Shr_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 13u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Shr_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 13u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Shr_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 13u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Shr_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 13u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Shr_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 13u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Shr_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 13u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Shr_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Sub_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Sub_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Sub_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Sub_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Sub_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Sub_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Sub_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::O_Sub_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 15u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Sub_U_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 15u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Sub_U_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 15u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Sub_U_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            5u8 if funct == 15u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::O_Sub_U_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 0u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::S_Abs_i8 { rd, rs1 },
            6u8 if funct == 0u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::S_Abs_i16 { rd, rs1 },
            6u8 if funct == 0u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::S_Abs_i32 { rd, rs1 },
            6u8 if funct == 0u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::S_Abs_i64 { rd, rs1 },
            6u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Add_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Add_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Add_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Add_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Add_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Add_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Add_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Add_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Add_U_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Add_U_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Add_U_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Add_U_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Add_S_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Add_S_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Add_S_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Add_S_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Div_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Div_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Div_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Div_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Div_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Div_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Div_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Div_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Mul_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Mul_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Mul_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Mul_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Mul_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Mul_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Mul_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Mul_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 8u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::S_Neg_i8 { rd, rs1 },
            6u8 if funct == 8u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::S_Neg_i16 { rd, rs1 },
            6u8 if funct == 8u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::S_Neg_i32 { rd, rs1 },
            6u8 if funct == 8u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Instruction::S_Neg_i64 { rd, rs1 },
            6u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Pow_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Pow_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Pow_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Pow_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Pow_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Pow_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Pow_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Pow_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Sub_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Sub_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Sub_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Sub_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Sub_u8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Sub_u16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Sub_u32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => {
                Instruction::S_Sub_u64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 15u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Sub_U_i8 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 15u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Sub_U_i16 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 15u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Sub_U_i32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            6u8 if funct == 15u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::S_Sub_U_i64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 0u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Instruction::Abs_f32 { rd, rs1 },
            7u8 if funct == 0u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Instruction::Abs_f64 { rd, rs1 },
            7u8 if funct == 1u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Add_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 1u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Add_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 3u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Div_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 3u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Div_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 4u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Div_E_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 4u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Div_E_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 5u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Log_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 5u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Log_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 6u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Instruction::Sqrt_f32 { rd, rs1 },
            7u8 if funct == 6u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Instruction::Sqrt_f64 { rd, rs1 },
            7u8 if funct == 7u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Mul_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 7u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Mul_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 8u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Instruction::Neg_f32 { rd, rs1 },
            7u8 if funct == 8u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Instruction::Neg_f64 { rd, rs1 },
            7u8 if funct == 9u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Pow_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 9u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Pow_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 10u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Rem_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 10u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Rem_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 11u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Rem_E_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 11u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Rem_E_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 13u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Instruction::Cbrt_f32 { rd, rs1 },
            7u8 if funct == 13u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Instruction::Cbrt_f64 { rd, rs1 },
            7u8 if funct == 14u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Sub_f32 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            7u8 if funct == 14u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => {
                Instruction::Sub_f64 {
                    rd,
                    rs1,
                    rs2,
                }
            }
            _ => Instruction::Invalid(instr),
        }
    }
    pub fn encode(&self) -> u32 {
        match self {
            Instruction::Halt => 0u32,
            Instruction::Trap => 64u32,
            Instruction::Call { rs2 } => {
                encode_s(0u8) | 0 | 0 | encode_rs2(*rs2) | 0 | 0 | encode_funct(2u8)
                    | encode_op_code(0u8)
            }
            Instruction::Call_R { rs2 } => {
                encode_s(1u8) | 0 | 0 | encode_rs2(*rs2) | 0 | 0 | encode_funct(2u8)
                    | encode_op_code(0u8)
            }
            Instruction::Ret => 192u32,
            Instruction::Ecall { imm } => {
                encode_imm(*imm) | 0 | encode_funct(4u8) | encode_op_code(0u8)
            }
            Instruction::Break => 320u32,
            Instruction::Jal { rd, rs2 } => {
                encode_s(0u8) | 0 | 0 | encode_rs2(*rs2) | 0 | encode_rd(*rd)
                    | encode_funct(6u8) | encode_op_code(0u8)
            }
            Instruction::Jal_R { rd, rs2 } => {
                encode_s(1u8) | 0 | 0 | encode_rs2(*rs2) | 0 | encode_rd(*rd)
                    | encode_funct(6u8) | encode_op_code(0u8)
            }
            Instruction::Jnz { rd, rs1, rs2 } => {
                encode_s(0u8) | 0 | 0 | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(7u8) | encode_op_code(0u8)
            }
            Instruction::Jnz_R { rd, rs1, rs2 } => {
                encode_s(1u8) | 0 | 0 | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(7u8) | encode_op_code(0u8)
            }
            Instruction::Jiz { rd, rs1, rs2 } => {
                encode_s(0u8) | 0 | 0 | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(0u8)
            }
            Instruction::Jiz_R { rd, rs1, rs2 } => {
                encode_s(1u8) | 0 | 0 | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(0u8)
            }
            Instruction::Load { rd, rs1, size } => {
                0 | 0 | encode_size(*size) | 0 | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(0u8) | encode_op_code(1u8)
            }
            Instruction::Loadi { rd, imm } => {
                encode_imm(*imm) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(1u8)
            }
            Instruction::Store { rd, rs1, size } => {
                0 | 0 | encode_size(*size) | 0 | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(2u8) | encode_op_code(1u8)
            }
            Instruction::Storei { rd, imm } => {
                encode_imm(*imm) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(1u8)
            }
            Instruction::Move { rd, rs1 } => {
                0 | 0 | 0 | 0 | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(1u8)
            }
            Instruction::Push { rd } => {
                0 | encode_rd(*rd) | encode_funct(5u8) | encode_op_code(1u8)
            }
            Instruction::Pushi { imm } => {
                encode_imm(*imm) | 0 | encode_funct(6u8) | encode_op_code(1u8)
            }
            Instruction::Pop { rd } => {
                0 | encode_rd(*rd) | encode_funct(7u8) | encode_op_code(1u8)
            }
            Instruction::Ie { rd, rs1, rs2 } => {
                0 | encode_f(0u8) | 0 | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(2u8)
            }
            Instruction::Ie_f32 { rd, rs1, rs2 } => {
                0 | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(2u8)
            }
            Instruction::Ie_f64 { rd, rs1, rs2 } => {
                0 | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(2u8)
            }
            Instruction::Ne { rd, rs1, rs2 } => {
                0 | encode_f(0u8) | 0 | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(1u8) | encode_op_code(2u8)
            }
            Instruction::Ne_f32 { rd, rs1, rs2 } => {
                0 | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(2u8)
            }
            Instruction::Ne_f64 { rd, rs1, rs2 } => {
                0 | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(2u8)
            }
            Instruction::Lt { rd, rs1, rs2 } => {
                0 | encode_f(0u8) | 0 | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(2u8) | encode_op_code(2u8)
            }
            Instruction::Lt_f32 { rd, rs1, rs2 } => {
                0 | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(2u8)
            }
            Instruction::Lt_f64 { rd, rs1, rs2 } => {
                0 | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(2u8)
            }
            Instruction::Le { rd, rs1, rs2 } => {
                0 | encode_f(0u8) | 0 | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(2u8)
            }
            Instruction::Le_f32 { rd, rs1, rs2 } => {
                0 | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(2u8)
            }
            Instruction::Le_f64 { rd, rs1, rs2 } => {
                0 | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(2u8)
            }
            Instruction::Gt { rd, rs1, rs2 } => {
                0 | encode_f(0u8) | 0 | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(4u8) | encode_op_code(2u8)
            }
            Instruction::Gt_f32 { rd, rs1, rs2 } => {
                0 | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(2u8)
            }
            Instruction::Gt_f64 { rd, rs1, rs2 } => {
                0 | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(2u8)
            }
            Instruction::Ge { rd, rs1, rs2 } => {
                0 | encode_f(0u8) | 0 | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(5u8) | encode_op_code(2u8)
            }
            Instruction::Ge_f32 { rd, rs1, rs2 } => {
                0 | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(2u8)
            }
            Instruction::Ge_f64 { rd, rs1, rs2 } => {
                0 | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(2u8)
            }
            Instruction::And_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Instruction::And_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Instruction::And_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Instruction::And_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Instruction::And_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Instruction::And_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Instruction::And_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Instruction::And_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Instruction::Or_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Instruction::Or_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Instruction::Or_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Instruction::Or_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Instruction::Or_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Instruction::Or_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Instruction::Or_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Instruction::Or_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Instruction::Xor_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Instruction::Xor_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Instruction::Xor_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Instruction::Xor_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Instruction::Xor_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Instruction::Xor_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Instruction::Xor_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Instruction::Xor_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Instruction::Not_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Instruction::Not_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Instruction::Not_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Instruction::Not_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Instruction::Not_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Instruction::Not_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Instruction::Not_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Instruction::Not_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Instruction::Shl_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shl_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shl_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shl_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shl_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shl_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shl_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shl_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shr_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shr_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shr_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shr_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shr_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shr_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shr_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Instruction::Shr_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotl_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotl_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotl_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotl_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotl_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotl_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotl_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotl_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotr_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotr_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotr_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotr_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotr_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotr_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotr_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Instruction::Rotr_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Instruction::Count_Ones_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Instruction::Count_Ones_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Instruction::Count_Ones_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Instruction::Count_Ones_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Instruction::Count_Ones_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Instruction::Count_Ones_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Instruction::Count_Ones_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Instruction::Count_Ones_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Ones_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Ones_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Ones_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Ones_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Ones_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Ones_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Ones_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Ones_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Ones_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Ones_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Ones_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Ones_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Ones_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Ones_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Ones_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Ones_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Instruction::Count_Zeros_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Instruction::Count_Zeros_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Instruction::Count_Zeros_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Instruction::Count_Zeros_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Instruction::Count_Zeros_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Instruction::Count_Zeros_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Instruction::Count_Zeros_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Instruction::Count_Zeros_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Zeros_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Zeros_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Zeros_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Zeros_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Zeros_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Zeros_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Zeros_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Instruction::Leading_Zeros_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Zeros_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Zeros_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Zeros_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Zeros_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Zeros_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Zeros_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Zeros_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Instruction::Trailing_Zeros_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bytes_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bytes_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bytes_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bytes_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bytes_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bytes_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bytes_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bytes_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bits_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bits_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bits_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bits_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bits_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bits_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bits_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Instruction::Reverse_Bits_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Instruction::C_Abs_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(4u8)
            }
            Instruction::C_Abs_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(4u8)
            }
            Instruction::C_Abs_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(4u8)
            }
            Instruction::C_Abs_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(4u8)
            }
            Instruction::C_Add_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_U_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_U_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_U_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_U_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_S_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_S_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_S_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Add_S_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_E_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_E_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_E_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_E_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_E_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_E_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_E_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Div_E_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Log_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Log_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Log_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Log_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Log_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Log_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Log_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Log_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Sqrt_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Instruction::C_Sqrt_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Instruction::C_Sqrt_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Instruction::C_Sqrt_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Instruction::C_Sqrt_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Instruction::C_Sqrt_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Instruction::C_Sqrt_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Instruction::C_Sqrt_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Instruction::C_Mul_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Mul_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Mul_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Mul_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Mul_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Mul_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Mul_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Mul_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Neg_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(4u8)
            }
            Instruction::C_Neg_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(4u8)
            }
            Instruction::C_Neg_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(4u8)
            }
            Instruction::C_Neg_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(4u8)
            }
            Instruction::C_Pow_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Pow_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Pow_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Pow_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Pow_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Pow_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Pow_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Pow_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_E_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_E_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_E_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_E_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_E_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_E_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_E_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Rem_E_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shl_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shl_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shl_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shl_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shl_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shl_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shl_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shl_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shr_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shr_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shr_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shr_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shr_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shr_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shr_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Shr_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Sub_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Sub_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Sub_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Sub_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Sub_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Sub_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Sub_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Sub_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Sub_U_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Sub_U_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Sub_U_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(4u8)
            }
            Instruction::C_Sub_U_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(4u8)
            }
            Instruction::O_Abs_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(5u8)
            }
            Instruction::O_Abs_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(5u8)
            }
            Instruction::O_Abs_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(5u8)
            }
            Instruction::O_Abs_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(5u8)
            }
            Instruction::O_Add_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_U_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_U_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_U_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_U_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_S_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_S_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_S_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Add_S_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_E_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_E_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_E_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_E_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_E_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_E_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_E_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Div_E_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Mul_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Mul_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Mul_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Mul_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Mul_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Mul_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Mul_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Mul_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Neg_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(5u8)
            }
            Instruction::O_Neg_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(5u8)
            }
            Instruction::O_Neg_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(5u8)
            }
            Instruction::O_Neg_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(5u8)
            }
            Instruction::O_Pow_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Pow_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Pow_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Pow_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Pow_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Pow_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Pow_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Pow_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_E_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_E_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_E_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_E_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_E_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_E_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_E_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Rem_E_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shl_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shl_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shl_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shl_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shl_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shl_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shl_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shl_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shr_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shr_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shr_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shr_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shr_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shr_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shr_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Shr_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Sub_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Sub_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Sub_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Sub_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Sub_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Sub_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Sub_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Sub_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Sub_U_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Sub_U_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Sub_U_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(5u8)
            }
            Instruction::O_Sub_U_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(5u8)
            }
            Instruction::S_Abs_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(6u8)
            }
            Instruction::S_Abs_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(6u8)
            }
            Instruction::S_Abs_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(6u8)
            }
            Instruction::S_Abs_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(6u8)
            }
            Instruction::S_Add_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_U_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_U_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_U_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_U_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_S_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_S_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_S_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Add_S_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Div_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Div_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Div_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Div_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Div_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Div_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Div_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Div_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Mul_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Mul_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Mul_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Mul_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Mul_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Mul_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Mul_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Mul_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Neg_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(6u8)
            }
            Instruction::S_Neg_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(6u8)
            }
            Instruction::S_Neg_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(6u8)
            }
            Instruction::S_Neg_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(6u8)
            }
            Instruction::S_Pow_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Pow_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Pow_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Pow_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Pow_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Pow_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Pow_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Pow_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Sub_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Sub_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Sub_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Sub_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Sub_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Sub_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Sub_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Sub_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Sub_U_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Sub_U_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Sub_U_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(6u8)
            }
            Instruction::S_Sub_U_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(6u8)
            }
            Instruction::Abs_f32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(7u8)
            }
            Instruction::Abs_f64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(7u8)
            }
            Instruction::Add_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(7u8)
            }
            Instruction::Add_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(7u8)
            }
            Instruction::Div_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(7u8)
            }
            Instruction::Div_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(7u8)
            }
            Instruction::Div_E_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(7u8)
            }
            Instruction::Div_E_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(7u8)
            }
            Instruction::Log_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(7u8)
            }
            Instruction::Log_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(7u8)
            }
            Instruction::Sqrt_f32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(7u8)
            }
            Instruction::Sqrt_f64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(7u8)
            }
            Instruction::Mul_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(7u8)
            }
            Instruction::Mul_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(7u8)
            }
            Instruction::Neg_f32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(7u8)
            }
            Instruction::Neg_f64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(7u8)
            }
            Instruction::Pow_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(7u8)
            }
            Instruction::Pow_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(7u8)
            }
            Instruction::Rem_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(7u8)
            }
            Instruction::Rem_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(7u8)
            }
            Instruction::Rem_E_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(7u8)
            }
            Instruction::Rem_E_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(7u8)
            }
            Instruction::Cbrt_f32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(7u8)
            }
            Instruction::Cbrt_f64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | 0 | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(7u8)
            }
            Instruction::Sub_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(7u8)
            }
            Instruction::Sub_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(7u8)
            }
            Instruction::Invalid(instr) => *instr,
        }
    }
}
#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    #[test]
    fn decode_Halt() {
        assert_eq!(Instruction::decode(0u32), Instruction::Halt);
    }
    #[test]
    fn encode_Halt() {
        println!("{:032b}", Instruction::Halt.encode());
        println!("{:032b}", 0u32);
        assert_eq!(Instruction::Halt.encode(), 0u32);
    }
    #[test]
    fn decode_Trap() {
        assert_eq!(Instruction::decode(64u32), Instruction::Trap);
    }
    #[test]
    fn encode_Trap() {
        println!("{:032b}", Instruction::Trap.encode());
        println!("{:032b}", 64u32);
        assert_eq!(Instruction::Trap.encode(), 64u32);
    }
    #[test]
    fn decode_Call() {
        assert_eq!(
            Instruction::decode(25165952u32), Instruction::Call { rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Call() {
        println!(
            "{:032b}", Instruction::Call { rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 25165952u32);
        assert_eq!(
            Instruction::Call { rs2 : Register::General_Purpose(6), } .encode(),
            25165952u32
        );
    }
    #[test]
    fn decode_Call_R() {
        assert_eq!(
            Instruction::decode(2172649600u32), Instruction::Call_R { rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Call_R() {
        println!(
            "{:032b}", Instruction::Call_R { rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2172649600u32);
        assert_eq!(
            Instruction::Call_R { rs2 : Register::General_Purpose(6), } .encode(),
            2172649600u32
        );
    }
    #[test]
    fn decode_Ret() {
        assert_eq!(Instruction::decode(192u32), Instruction::Ret);
    }
    #[test]
    fn encode_Ret() {
        println!("{:032b}", Instruction::Ret.encode());
        println!("{:032b}", 192u32);
        assert_eq!(Instruction::Ret.encode(), 192u32);
    }
    #[test]
    fn decode_Ecall() {
        assert_eq!(
            Instruction::decode(4293918976u32), Instruction::Ecall { imm : - 16, }
        );
    }
    #[test]
    fn encode_Ecall() {
        println!("{:032b}", Instruction::Ecall { imm : - 16, } .encode());
        println!("{:032b}", 4293918976u32);
        assert_eq!(Instruction::Ecall { imm : - 16, } .encode(), 4293918976u32);
    }
    #[test]
    fn decode_Break() {
        assert_eq!(Instruction::decode(320u32), Instruction::Break);
    }
    #[test]
    fn encode_Break() {
        println!("{:032b}", Instruction::Break.encode());
        println!("{:032b}", 320u32);
        assert_eq!(Instruction::Break.encode(), 320u32);
    }
    #[test]
    fn decode_Jal() {
        assert_eq!(
            Instruction::decode(25171328u32), Instruction::Jal { rd :
            Register::General_Purpose(5), rs2 : Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Jal() {
        println!(
            "{:032b}", Instruction::Jal { rd : Register::General_Purpose(5), rs2 :
            Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25171328u32);
        assert_eq!(
            Instruction::Jal { rd : Register::General_Purpose(5), rs2 :
            Register::General_Purpose(6), } .encode(), 25171328u32
        );
    }
    #[test]
    fn decode_Jal_R() {
        assert_eq!(
            Instruction::decode(2172654976u32), Instruction::Jal_R { rd :
            Register::General_Purpose(5), rs2 : Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Jal_R() {
        println!(
            "{:032b}", Instruction::Jal_R { rd : Register::General_Purpose(5), rs2 :
            Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2172654976u32);
        assert_eq!(
            Instruction::Jal_R { rd : Register::General_Purpose(5), rs2 :
            Register::General_Purpose(6), } .encode(), 2172654976u32
        );
    }
    #[test]
    fn decode_Jnz() {
        assert_eq!(
            Instruction::decode(25695680u32), Instruction::Jnz { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Jnz() {
        println!(
            "{:032b}", Instruction::Jnz { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695680u32);
        assert_eq!(
            Instruction::Jnz { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695680u32
        );
    }
    #[test]
    fn decode_Jnz_R() {
        assert_eq!(
            Instruction::decode(2173179328u32), Instruction::Jnz_R { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Jnz_R() {
        println!(
            "{:032b}", Instruction::Jnz_R { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179328u32);
        assert_eq!(
            Instruction::Jnz_R { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179328u32
        );
    }
    #[test]
    fn decode_Jiz() {
        assert_eq!(
            Instruction::decode(25695744u32), Instruction::Jiz { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Jiz() {
        println!(
            "{:032b}", Instruction::Jiz { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695744u32);
        assert_eq!(
            Instruction::Jiz { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695744u32
        );
    }
    #[test]
    fn decode_Jiz_R() {
        assert_eq!(
            Instruction::decode(2173179392u32), Instruction::Jiz_R { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Jiz_R() {
        println!(
            "{:032b}", Instruction::Jiz_R { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179392u32);
        assert_eq!(
            Instruction::Jiz_R { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179392u32
        );
    }
    #[test]
    fn decode_Load() {
        assert_eq!(
            Instruction::decode(805835777u32), Instruction::Load { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), size :
            64u8, }
        );
    }
    #[test]
    fn encode_Load() {
        println!(
            "{:032b}", Instruction::Load { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), size : 64u8, } .encode()
        );
        println!("{:032b}", 805835777u32);
        assert_eq!(
            Instruction::Load { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), size : 64u8, } .encode(), 805835777u32
        );
    }
    #[test]
    fn decode_Loadi() {
        assert_eq!(
            Instruction::decode(4293923905u32), Instruction::Loadi { rd :
            Register::General_Purpose(5), imm : - 16, }
        );
    }
    #[test]
    fn encode_Loadi() {
        println!(
            "{:032b}", Instruction::Loadi { rd : Register::General_Purpose(5), imm : -
            16, } .encode()
        );
        println!("{:032b}", 4293923905u32);
        assert_eq!(
            Instruction::Loadi { rd : Register::General_Purpose(5), imm : - 16, }
            .encode(), 4293923905u32
        );
    }
    #[test]
    fn decode_Store() {
        assert_eq!(
            Instruction::decode(805835905u32), Instruction::Store { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), size :
            64u8, }
        );
    }
    #[test]
    fn encode_Store() {
        println!(
            "{:032b}", Instruction::Store { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), size : 64u8, } .encode()
        );
        println!("{:032b}", 805835905u32);
        assert_eq!(
            Instruction::Store { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), size : 64u8, } .encode(), 805835905u32
        );
    }
    #[test]
    fn decode_Storei() {
        assert_eq!(
            Instruction::decode(4293924033u32), Instruction::Storei { rd :
            Register::General_Purpose(5), imm : - 16, }
        );
    }
    #[test]
    fn encode_Storei() {
        println!(
            "{:032b}", Instruction::Storei { rd : Register::General_Purpose(5), imm : -
            16, } .encode()
        );
        println!("{:032b}", 4293924033u32);
        assert_eq!(
            Instruction::Storei { rd : Register::General_Purpose(5), imm : - 16, }
            .encode(), 4293924033u32
        );
    }
    #[test]
    fn decode_Move() {
        assert_eq!(
            Instruction::decode(529665u32), Instruction::Move { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Move() {
        println!(
            "{:032b}", Instruction::Move { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 529665u32);
        assert_eq!(
            Instruction::Move { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 529665u32
        );
    }
    #[test]
    fn decode_Push() {
        assert_eq!(
            Instruction::decode(5441u32), Instruction::Push { rd :
            Register::General_Purpose(5), }
        );
    }
    #[test]
    fn encode_Push() {
        println!(
            "{:032b}", Instruction::Push { rd : Register::General_Purpose(5), } .encode()
        );
        println!("{:032b}", 5441u32);
        assert_eq!(
            Instruction::Push { rd : Register::General_Purpose(5), } .encode(), 5441u32
        );
    }
    #[test]
    fn decode_Pushi() {
        assert_eq!(
            Instruction::decode(4293919105u32), Instruction::Pushi { imm : - 16, }
        );
    }
    #[test]
    fn encode_Pushi() {
        println!("{:032b}", Instruction::Pushi { imm : - 16, } .encode());
        println!("{:032b}", 4293919105u32);
        assert_eq!(Instruction::Pushi { imm : - 16, } .encode(), 4293919105u32);
    }
    #[test]
    fn decode_Pop() {
        assert_eq!(
            Instruction::decode(5569u32), Instruction::Pop { rd :
            Register::General_Purpose(5), }
        );
    }
    #[test]
    fn encode_Pop() {
        println!(
            "{:032b}", Instruction::Pop { rd : Register::General_Purpose(5), } .encode()
        );
        println!("{:032b}", 5569u32);
        assert_eq!(
            Instruction::Pop { rd : Register::General_Purpose(5), } .encode(), 5569u32
        );
    }
    #[test]
    fn decode_Ie() {
        assert_eq!(
            Instruction::decode(25695234u32), Instruction::Ie { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ie() {
        println!(
            "{:032b}", Instruction::Ie { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695234u32);
        assert_eq!(
            Instruction::Ie { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695234u32
        );
    }
    #[test]
    fn decode_Ie_f32() {
        assert_eq!(
            Instruction::decode(1636307970u32), Instruction::Ie_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ie_f32() {
        println!(
            "{:032b}", Instruction::Ie_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 1636307970u32);
        assert_eq!(
            Instruction::Ie_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1636307970u32
        );
    }
    #[test]
    fn decode_Ie_f64() {
        assert_eq!(
            Instruction::decode(1904743426u32), Instruction::Ie_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ie_f64() {
        println!(
            "{:032b}", Instruction::Ie_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 1904743426u32);
        assert_eq!(
            Instruction::Ie_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1904743426u32
        );
    }
    #[test]
    fn decode_Ne() {
        assert_eq!(
            Instruction::decode(25695298u32), Instruction::Ne { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ne() {
        println!(
            "{:032b}", Instruction::Ne { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695298u32);
        assert_eq!(
            Instruction::Ne { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695298u32
        );
    }
    #[test]
    fn decode_Ne_f32() {
        assert_eq!(
            Instruction::decode(1636308034u32), Instruction::Ne_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ne_f32() {
        println!(
            "{:032b}", Instruction::Ne_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 1636308034u32);
        assert_eq!(
            Instruction::Ne_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1636308034u32
        );
    }
    #[test]
    fn decode_Ne_f64() {
        assert_eq!(
            Instruction::decode(1904743490u32), Instruction::Ne_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ne_f64() {
        println!(
            "{:032b}", Instruction::Ne_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 1904743490u32);
        assert_eq!(
            Instruction::Ne_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1904743490u32
        );
    }
    #[test]
    fn decode_Lt() {
        assert_eq!(
            Instruction::decode(25695362u32), Instruction::Lt { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Lt() {
        println!(
            "{:032b}", Instruction::Lt { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695362u32);
        assert_eq!(
            Instruction::Lt { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695362u32
        );
    }
    #[test]
    fn decode_Lt_f32() {
        assert_eq!(
            Instruction::decode(1636308098u32), Instruction::Lt_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Lt_f32() {
        println!(
            "{:032b}", Instruction::Lt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 1636308098u32);
        assert_eq!(
            Instruction::Lt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1636308098u32
        );
    }
    #[test]
    fn decode_Lt_f64() {
        assert_eq!(
            Instruction::decode(1904743554u32), Instruction::Lt_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Lt_f64() {
        println!(
            "{:032b}", Instruction::Lt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 1904743554u32);
        assert_eq!(
            Instruction::Lt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1904743554u32
        );
    }
    #[test]
    fn decode_Le() {
        assert_eq!(
            Instruction::decode(25695426u32), Instruction::Le { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Le() {
        println!(
            "{:032b}", Instruction::Le { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695426u32);
        assert_eq!(
            Instruction::Le { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695426u32
        );
    }
    #[test]
    fn decode_Le_f32() {
        assert_eq!(
            Instruction::decode(1636308162u32), Instruction::Le_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Le_f32() {
        println!(
            "{:032b}", Instruction::Le_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 1636308162u32);
        assert_eq!(
            Instruction::Le_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1636308162u32
        );
    }
    #[test]
    fn decode_Le_f64() {
        assert_eq!(
            Instruction::decode(1904743618u32), Instruction::Le_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Le_f64() {
        println!(
            "{:032b}", Instruction::Le_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 1904743618u32);
        assert_eq!(
            Instruction::Le_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1904743618u32
        );
    }
    #[test]
    fn decode_Gt() {
        assert_eq!(
            Instruction::decode(25695490u32), Instruction::Gt { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Gt() {
        println!(
            "{:032b}", Instruction::Gt { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695490u32);
        assert_eq!(
            Instruction::Gt { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695490u32
        );
    }
    #[test]
    fn decode_Gt_f32() {
        assert_eq!(
            Instruction::decode(1636308226u32), Instruction::Gt_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Gt_f32() {
        println!(
            "{:032b}", Instruction::Gt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 1636308226u32);
        assert_eq!(
            Instruction::Gt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1636308226u32
        );
    }
    #[test]
    fn decode_Gt_f64() {
        assert_eq!(
            Instruction::decode(1904743682u32), Instruction::Gt_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Gt_f64() {
        println!(
            "{:032b}", Instruction::Gt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 1904743682u32);
        assert_eq!(
            Instruction::Gt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1904743682u32
        );
    }
    #[test]
    fn decode_Ge() {
        assert_eq!(
            Instruction::decode(25695554u32), Instruction::Ge { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ge() {
        println!(
            "{:032b}", Instruction::Ge { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695554u32);
        assert_eq!(
            Instruction::Ge { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695554u32
        );
    }
    #[test]
    fn decode_Ge_f32() {
        assert_eq!(
            Instruction::decode(1636308290u32), Instruction::Ge_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ge_f32() {
        println!(
            "{:032b}", Instruction::Ge_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 1636308290u32);
        assert_eq!(
            Instruction::Ge_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1636308290u32
        );
    }
    #[test]
    fn decode_Ge_f64() {
        assert_eq!(
            Instruction::decode(1904743746u32), Instruction::Ge_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ge_f64() {
        println!(
            "{:032b}", Instruction::Ge_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 1904743746u32);
        assert_eq!(
            Instruction::Ge_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1904743746u32
        );
    }
    #[test]
    fn decode_And_i8() {
        assert_eq!(
            Instruction::decode(2173178883u32), Instruction::And_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_i8() {
        println!(
            "{:032b}", Instruction::And_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173178883u32);
        assert_eq!(
            Instruction::And_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173178883u32
        );
    }
    #[test]
    fn decode_And_i16() {
        assert_eq!(
            Instruction::decode(2441614339u32), Instruction::And_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_i16() {
        println!(
            "{:032b}", Instruction::And_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614339u32);
        assert_eq!(
            Instruction::And_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614339u32
        );
    }
    #[test]
    fn decode_And_i32() {
        assert_eq!(
            Instruction::decode(2710049795u32), Instruction::And_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_i32() {
        println!(
            "{:032b}", Instruction::And_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710049795u32);
        assert_eq!(
            Instruction::And_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710049795u32
        );
    }
    #[test]
    fn decode_And_i64() {
        assert_eq!(
            Instruction::decode(2978485251u32), Instruction::And_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_i64() {
        println!(
            "{:032b}", Instruction::And_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485251u32);
        assert_eq!(
            Instruction::And_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485251u32
        );
    }
    #[test]
    fn decode_And_u8() {
        assert_eq!(
            Instruction::decode(25695235u32), Instruction::And_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_u8() {
        println!(
            "{:032b}", Instruction::And_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695235u32);
        assert_eq!(
            Instruction::And_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695235u32
        );
    }
    #[test]
    fn decode_And_u16() {
        assert_eq!(
            Instruction::decode(294130691u32), Instruction::And_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_u16() {
        println!(
            "{:032b}", Instruction::And_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294130691u32);
        assert_eq!(
            Instruction::And_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130691u32
        );
    }
    #[test]
    fn decode_And_u32() {
        assert_eq!(
            Instruction::decode(562566147u32), Instruction::And_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_u32() {
        println!(
            "{:032b}", Instruction::And_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566147u32);
        assert_eq!(
            Instruction::And_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566147u32
        );
    }
    #[test]
    fn decode_And_u64() {
        assert_eq!(
            Instruction::decode(831001603u32), Instruction::And_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_u64() {
        println!(
            "{:032b}", Instruction::And_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831001603u32);
        assert_eq!(
            Instruction::And_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001603u32
        );
    }
    #[test]
    fn decode_Or_i8() {
        assert_eq!(
            Instruction::decode(2173178947u32), Instruction::Or_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_i8() {
        println!(
            "{:032b}", Instruction::Or_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173178947u32);
        assert_eq!(
            Instruction::Or_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173178947u32
        );
    }
    #[test]
    fn decode_Or_i16() {
        assert_eq!(
            Instruction::decode(2441614403u32), Instruction::Or_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_i16() {
        println!(
            "{:032b}", Instruction::Or_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614403u32);
        assert_eq!(
            Instruction::Or_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614403u32
        );
    }
    #[test]
    fn decode_Or_i32() {
        assert_eq!(
            Instruction::decode(2710049859u32), Instruction::Or_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_i32() {
        println!(
            "{:032b}", Instruction::Or_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710049859u32);
        assert_eq!(
            Instruction::Or_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710049859u32
        );
    }
    #[test]
    fn decode_Or_i64() {
        assert_eq!(
            Instruction::decode(2978485315u32), Instruction::Or_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_i64() {
        println!(
            "{:032b}", Instruction::Or_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485315u32);
        assert_eq!(
            Instruction::Or_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485315u32
        );
    }
    #[test]
    fn decode_Or_u8() {
        assert_eq!(
            Instruction::decode(25695299u32), Instruction::Or_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_u8() {
        println!(
            "{:032b}", Instruction::Or_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695299u32);
        assert_eq!(
            Instruction::Or_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695299u32
        );
    }
    #[test]
    fn decode_Or_u16() {
        assert_eq!(
            Instruction::decode(294130755u32), Instruction::Or_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_u16() {
        println!(
            "{:032b}", Instruction::Or_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294130755u32);
        assert_eq!(
            Instruction::Or_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130755u32
        );
    }
    #[test]
    fn decode_Or_u32() {
        assert_eq!(
            Instruction::decode(562566211u32), Instruction::Or_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_u32() {
        println!(
            "{:032b}", Instruction::Or_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566211u32);
        assert_eq!(
            Instruction::Or_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566211u32
        );
    }
    #[test]
    fn decode_Or_u64() {
        assert_eq!(
            Instruction::decode(831001667u32), Instruction::Or_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_u64() {
        println!(
            "{:032b}", Instruction::Or_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831001667u32);
        assert_eq!(
            Instruction::Or_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001667u32
        );
    }
    #[test]
    fn decode_Xor_i8() {
        assert_eq!(
            Instruction::decode(2173179011u32), Instruction::Xor_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_i8() {
        println!(
            "{:032b}", Instruction::Xor_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179011u32);
        assert_eq!(
            Instruction::Xor_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179011u32
        );
    }
    #[test]
    fn decode_Xor_i16() {
        assert_eq!(
            Instruction::decode(2441614467u32), Instruction::Xor_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_i16() {
        println!(
            "{:032b}", Instruction::Xor_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614467u32);
        assert_eq!(
            Instruction::Xor_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614467u32
        );
    }
    #[test]
    fn decode_Xor_i32() {
        assert_eq!(
            Instruction::decode(2710049923u32), Instruction::Xor_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_i32() {
        println!(
            "{:032b}", Instruction::Xor_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710049923u32);
        assert_eq!(
            Instruction::Xor_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710049923u32
        );
    }
    #[test]
    fn decode_Xor_i64() {
        assert_eq!(
            Instruction::decode(2978485379u32), Instruction::Xor_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_i64() {
        println!(
            "{:032b}", Instruction::Xor_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485379u32);
        assert_eq!(
            Instruction::Xor_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485379u32
        );
    }
    #[test]
    fn decode_Xor_u8() {
        assert_eq!(
            Instruction::decode(25695363u32), Instruction::Xor_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_u8() {
        println!(
            "{:032b}", Instruction::Xor_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695363u32);
        assert_eq!(
            Instruction::Xor_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695363u32
        );
    }
    #[test]
    fn decode_Xor_u16() {
        assert_eq!(
            Instruction::decode(294130819u32), Instruction::Xor_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_u16() {
        println!(
            "{:032b}", Instruction::Xor_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294130819u32);
        assert_eq!(
            Instruction::Xor_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130819u32
        );
    }
    #[test]
    fn decode_Xor_u32() {
        assert_eq!(
            Instruction::decode(562566275u32), Instruction::Xor_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_u32() {
        println!(
            "{:032b}", Instruction::Xor_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566275u32);
        assert_eq!(
            Instruction::Xor_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566275u32
        );
    }
    #[test]
    fn decode_Xor_u64() {
        assert_eq!(
            Instruction::decode(831001731u32), Instruction::Xor_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_u64() {
        println!(
            "{:032b}", Instruction::Xor_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831001731u32);
        assert_eq!(
            Instruction::Xor_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001731u32
        );
    }
    #[test]
    fn decode_Not_i8() {
        assert_eq!(
            Instruction::decode(2148013251u32), Instruction::Not_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_i8() {
        println!(
            "{:032b}", Instruction::Not_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013251u32);
        assert_eq!(
            Instruction::Not_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013251u32
        );
    }
    #[test]
    fn decode_Not_i16() {
        assert_eq!(
            Instruction::decode(2416448707u32), Instruction::Not_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_i16() {
        println!(
            "{:032b}", Instruction::Not_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416448707u32);
        assert_eq!(
            Instruction::Not_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416448707u32
        );
    }
    #[test]
    fn decode_Not_i32() {
        assert_eq!(
            Instruction::decode(2684884163u32), Instruction::Not_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_i32() {
        println!(
            "{:032b}", Instruction::Not_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684884163u32);
        assert_eq!(
            Instruction::Not_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684884163u32
        );
    }
    #[test]
    fn decode_Not_i64() {
        assert_eq!(
            Instruction::decode(2953319619u32), Instruction::Not_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_i64() {
        println!(
            "{:032b}", Instruction::Not_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953319619u32);
        assert_eq!(
            Instruction::Not_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953319619u32
        );
    }
    #[test]
    fn decode_Not_u8() {
        assert_eq!(
            Instruction::decode(529603u32), Instruction::Not_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_u8() {
        println!(
            "{:032b}", Instruction::Not_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 529603u32);
        assert_eq!(
            Instruction::Not_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 529603u32
        );
    }
    #[test]
    fn decode_Not_u16() {
        assert_eq!(
            Instruction::decode(268965059u32), Instruction::Not_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_u16() {
        println!(
            "{:032b}", Instruction::Not_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 268965059u32);
        assert_eq!(
            Instruction::Not_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 268965059u32
        );
    }
    #[test]
    fn decode_Not_u32() {
        assert_eq!(
            Instruction::decode(537400515u32), Instruction::Not_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_u32() {
        println!(
            "{:032b}", Instruction::Not_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 537400515u32);
        assert_eq!(
            Instruction::Not_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537400515u32
        );
    }
    #[test]
    fn decode_Not_u64() {
        assert_eq!(
            Instruction::decode(805835971u32), Instruction::Not_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_u64() {
        println!(
            "{:032b}", Instruction::Not_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 805835971u32);
        assert_eq!(
            Instruction::Not_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 805835971u32
        );
    }
    #[test]
    fn decode_Shl_i8() {
        assert_eq!(
            Instruction::decode(2173179139u32), Instruction::Shl_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_i8() {
        println!(
            "{:032b}", Instruction::Shl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179139u32);
        assert_eq!(
            Instruction::Shl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179139u32
        );
    }
    #[test]
    fn decode_Shl_i16() {
        assert_eq!(
            Instruction::decode(2441614595u32), Instruction::Shl_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_i16() {
        println!(
            "{:032b}", Instruction::Shl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614595u32);
        assert_eq!(
            Instruction::Shl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614595u32
        );
    }
    #[test]
    fn decode_Shl_i32() {
        assert_eq!(
            Instruction::decode(2710050051u32), Instruction::Shl_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_i32() {
        println!(
            "{:032b}", Instruction::Shl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050051u32);
        assert_eq!(
            Instruction::Shl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050051u32
        );
    }
    #[test]
    fn decode_Shl_i64() {
        assert_eq!(
            Instruction::decode(2978485507u32), Instruction::Shl_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_i64() {
        println!(
            "{:032b}", Instruction::Shl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485507u32);
        assert_eq!(
            Instruction::Shl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485507u32
        );
    }
    #[test]
    fn decode_Shl_u8() {
        assert_eq!(
            Instruction::decode(25695491u32), Instruction::Shl_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_u8() {
        println!(
            "{:032b}", Instruction::Shl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695491u32);
        assert_eq!(
            Instruction::Shl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695491u32
        );
    }
    #[test]
    fn decode_Shl_u16() {
        assert_eq!(
            Instruction::decode(294130947u32), Instruction::Shl_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_u16() {
        println!(
            "{:032b}", Instruction::Shl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294130947u32);
        assert_eq!(
            Instruction::Shl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130947u32
        );
    }
    #[test]
    fn decode_Shl_u32() {
        assert_eq!(
            Instruction::decode(562566403u32), Instruction::Shl_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_u32() {
        println!(
            "{:032b}", Instruction::Shl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566403u32);
        assert_eq!(
            Instruction::Shl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566403u32
        );
    }
    #[test]
    fn decode_Shl_u64() {
        assert_eq!(
            Instruction::decode(831001859u32), Instruction::Shl_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_u64() {
        println!(
            "{:032b}", Instruction::Shl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831001859u32);
        assert_eq!(
            Instruction::Shl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001859u32
        );
    }
    #[test]
    fn decode_Shr_i8() {
        assert_eq!(
            Instruction::decode(2173179203u32), Instruction::Shr_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_i8() {
        println!(
            "{:032b}", Instruction::Shr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179203u32);
        assert_eq!(
            Instruction::Shr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179203u32
        );
    }
    #[test]
    fn decode_Shr_i16() {
        assert_eq!(
            Instruction::decode(2441614659u32), Instruction::Shr_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_i16() {
        println!(
            "{:032b}", Instruction::Shr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614659u32);
        assert_eq!(
            Instruction::Shr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614659u32
        );
    }
    #[test]
    fn decode_Shr_i32() {
        assert_eq!(
            Instruction::decode(2710050115u32), Instruction::Shr_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_i32() {
        println!(
            "{:032b}", Instruction::Shr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050115u32);
        assert_eq!(
            Instruction::Shr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050115u32
        );
    }
    #[test]
    fn decode_Shr_i64() {
        assert_eq!(
            Instruction::decode(2978485571u32), Instruction::Shr_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_i64() {
        println!(
            "{:032b}", Instruction::Shr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485571u32);
        assert_eq!(
            Instruction::Shr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485571u32
        );
    }
    #[test]
    fn decode_Shr_u8() {
        assert_eq!(
            Instruction::decode(25695555u32), Instruction::Shr_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_u8() {
        println!(
            "{:032b}", Instruction::Shr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695555u32);
        assert_eq!(
            Instruction::Shr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695555u32
        );
    }
    #[test]
    fn decode_Shr_u16() {
        assert_eq!(
            Instruction::decode(294131011u32), Instruction::Shr_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_u16() {
        println!(
            "{:032b}", Instruction::Shr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131011u32);
        assert_eq!(
            Instruction::Shr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131011u32
        );
    }
    #[test]
    fn decode_Shr_u32() {
        assert_eq!(
            Instruction::decode(562566467u32), Instruction::Shr_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_u32() {
        println!(
            "{:032b}", Instruction::Shr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566467u32);
        assert_eq!(
            Instruction::Shr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566467u32
        );
    }
    #[test]
    fn decode_Shr_u64() {
        assert_eq!(
            Instruction::decode(831001923u32), Instruction::Shr_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_u64() {
        println!(
            "{:032b}", Instruction::Shr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831001923u32);
        assert_eq!(
            Instruction::Shr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001923u32
        );
    }
    #[test]
    fn decode_Rotl_i8() {
        assert_eq!(
            Instruction::decode(2173179267u32), Instruction::Rotl_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_i8() {
        println!(
            "{:032b}", Instruction::Rotl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179267u32);
        assert_eq!(
            Instruction::Rotl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179267u32
        );
    }
    #[test]
    fn decode_Rotl_i16() {
        assert_eq!(
            Instruction::decode(2441614723u32), Instruction::Rotl_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_i16() {
        println!(
            "{:032b}", Instruction::Rotl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614723u32);
        assert_eq!(
            Instruction::Rotl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614723u32
        );
    }
    #[test]
    fn decode_Rotl_i32() {
        assert_eq!(
            Instruction::decode(2710050179u32), Instruction::Rotl_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_i32() {
        println!(
            "{:032b}", Instruction::Rotl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050179u32);
        assert_eq!(
            Instruction::Rotl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050179u32
        );
    }
    #[test]
    fn decode_Rotl_i64() {
        assert_eq!(
            Instruction::decode(2978485635u32), Instruction::Rotl_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_i64() {
        println!(
            "{:032b}", Instruction::Rotl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485635u32);
        assert_eq!(
            Instruction::Rotl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485635u32
        );
    }
    #[test]
    fn decode_Rotl_u8() {
        assert_eq!(
            Instruction::decode(25695619u32), Instruction::Rotl_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_u8() {
        println!(
            "{:032b}", Instruction::Rotl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695619u32);
        assert_eq!(
            Instruction::Rotl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695619u32
        );
    }
    #[test]
    fn decode_Rotl_u16() {
        assert_eq!(
            Instruction::decode(294131075u32), Instruction::Rotl_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_u16() {
        println!(
            "{:032b}", Instruction::Rotl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131075u32);
        assert_eq!(
            Instruction::Rotl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131075u32
        );
    }
    #[test]
    fn decode_Rotl_u32() {
        assert_eq!(
            Instruction::decode(562566531u32), Instruction::Rotl_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_u32() {
        println!(
            "{:032b}", Instruction::Rotl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566531u32);
        assert_eq!(
            Instruction::Rotl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566531u32
        );
    }
    #[test]
    fn decode_Rotl_u64() {
        assert_eq!(
            Instruction::decode(831001987u32), Instruction::Rotl_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_u64() {
        println!(
            "{:032b}", Instruction::Rotl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831001987u32);
        assert_eq!(
            Instruction::Rotl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001987u32
        );
    }
    #[test]
    fn decode_Rotr_i8() {
        assert_eq!(
            Instruction::decode(2173179331u32), Instruction::Rotr_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_i8() {
        println!(
            "{:032b}", Instruction::Rotr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179331u32);
        assert_eq!(
            Instruction::Rotr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179331u32
        );
    }
    #[test]
    fn decode_Rotr_i16() {
        assert_eq!(
            Instruction::decode(2441614787u32), Instruction::Rotr_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_i16() {
        println!(
            "{:032b}", Instruction::Rotr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614787u32);
        assert_eq!(
            Instruction::Rotr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614787u32
        );
    }
    #[test]
    fn decode_Rotr_i32() {
        assert_eq!(
            Instruction::decode(2710050243u32), Instruction::Rotr_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_i32() {
        println!(
            "{:032b}", Instruction::Rotr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050243u32);
        assert_eq!(
            Instruction::Rotr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050243u32
        );
    }
    #[test]
    fn decode_Rotr_i64() {
        assert_eq!(
            Instruction::decode(2978485699u32), Instruction::Rotr_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_i64() {
        println!(
            "{:032b}", Instruction::Rotr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485699u32);
        assert_eq!(
            Instruction::Rotr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485699u32
        );
    }
    #[test]
    fn decode_Rotr_u8() {
        assert_eq!(
            Instruction::decode(25695683u32), Instruction::Rotr_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_u8() {
        println!(
            "{:032b}", Instruction::Rotr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695683u32);
        assert_eq!(
            Instruction::Rotr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695683u32
        );
    }
    #[test]
    fn decode_Rotr_u16() {
        assert_eq!(
            Instruction::decode(294131139u32), Instruction::Rotr_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_u16() {
        println!(
            "{:032b}", Instruction::Rotr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131139u32);
        assert_eq!(
            Instruction::Rotr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131139u32
        );
    }
    #[test]
    fn decode_Rotr_u32() {
        assert_eq!(
            Instruction::decode(562566595u32), Instruction::Rotr_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_u32() {
        println!(
            "{:032b}", Instruction::Rotr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566595u32);
        assert_eq!(
            Instruction::Rotr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566595u32
        );
    }
    #[test]
    fn decode_Rotr_u64() {
        assert_eq!(
            Instruction::decode(831002051u32), Instruction::Rotr_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_u64() {
        println!(
            "{:032b}", Instruction::Rotr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002051u32);
        assert_eq!(
            Instruction::Rotr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002051u32
        );
    }
    #[test]
    fn decode_Count_Ones_i8() {
        assert_eq!(
            Instruction::decode(2148013571u32), Instruction::Count_Ones_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_i8() {
        println!(
            "{:032b}", Instruction::Count_Ones_i8 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013571u32);
        assert_eq!(
            Instruction::Count_Ones_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013571u32
        );
    }
    #[test]
    fn decode_Count_Ones_i16() {
        assert_eq!(
            Instruction::decode(2416449027u32), Instruction::Count_Ones_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_i16() {
        println!(
            "{:032b}", Instruction::Count_Ones_i16 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416449027u32);
        assert_eq!(
            Instruction::Count_Ones_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416449027u32
        );
    }
    #[test]
    fn decode_Count_Ones_i32() {
        assert_eq!(
            Instruction::decode(2684884483u32), Instruction::Count_Ones_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_i32() {
        println!(
            "{:032b}", Instruction::Count_Ones_i32 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684884483u32);
        assert_eq!(
            Instruction::Count_Ones_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684884483u32
        );
    }
    #[test]
    fn decode_Count_Ones_i64() {
        assert_eq!(
            Instruction::decode(2953319939u32), Instruction::Count_Ones_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_i64() {
        println!(
            "{:032b}", Instruction::Count_Ones_i64 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953319939u32);
        assert_eq!(
            Instruction::Count_Ones_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953319939u32
        );
    }
    #[test]
    fn decode_Count_Ones_u8() {
        assert_eq!(
            Instruction::decode(529923u32), Instruction::Count_Ones_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_u8() {
        println!(
            "{:032b}", Instruction::Count_Ones_u8 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 529923u32);
        assert_eq!(
            Instruction::Count_Ones_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 529923u32
        );
    }
    #[test]
    fn decode_Count_Ones_u16() {
        assert_eq!(
            Instruction::decode(268965379u32), Instruction::Count_Ones_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_u16() {
        println!(
            "{:032b}", Instruction::Count_Ones_u16 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 268965379u32);
        assert_eq!(
            Instruction::Count_Ones_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 268965379u32
        );
    }
    #[test]
    fn decode_Count_Ones_u32() {
        assert_eq!(
            Instruction::decode(537400835u32), Instruction::Count_Ones_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_u32() {
        println!(
            "{:032b}", Instruction::Count_Ones_u32 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 537400835u32);
        assert_eq!(
            Instruction::Count_Ones_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537400835u32
        );
    }
    #[test]
    fn decode_Count_Ones_u64() {
        assert_eq!(
            Instruction::decode(805836291u32), Instruction::Count_Ones_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_u64() {
        println!(
            "{:032b}", Instruction::Count_Ones_u64 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 805836291u32);
        assert_eq!(
            Instruction::Count_Ones_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 805836291u32
        );
    }
    #[test]
    fn decode_Leading_Ones_i8() {
        assert_eq!(
            Instruction::decode(2148013635u32), Instruction::Leading_Ones_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_i8() {
        println!(
            "{:032b}", Instruction::Leading_Ones_i8 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013635u32);
        assert_eq!(
            Instruction::Leading_Ones_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013635u32
        );
    }
    #[test]
    fn decode_Leading_Ones_i16() {
        assert_eq!(
            Instruction::decode(2416449091u32), Instruction::Leading_Ones_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_i16() {
        println!(
            "{:032b}", Instruction::Leading_Ones_i16 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416449091u32);
        assert_eq!(
            Instruction::Leading_Ones_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416449091u32
        );
    }
    #[test]
    fn decode_Leading_Ones_i32() {
        assert_eq!(
            Instruction::decode(2684884547u32), Instruction::Leading_Ones_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_i32() {
        println!(
            "{:032b}", Instruction::Leading_Ones_i32 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684884547u32);
        assert_eq!(
            Instruction::Leading_Ones_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684884547u32
        );
    }
    #[test]
    fn decode_Leading_Ones_i64() {
        assert_eq!(
            Instruction::decode(2953320003u32), Instruction::Leading_Ones_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_i64() {
        println!(
            "{:032b}", Instruction::Leading_Ones_i64 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953320003u32);
        assert_eq!(
            Instruction::Leading_Ones_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953320003u32
        );
    }
    #[test]
    fn decode_Leading_Ones_u8() {
        assert_eq!(
            Instruction::decode(529987u32), Instruction::Leading_Ones_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_u8() {
        println!(
            "{:032b}", Instruction::Leading_Ones_u8 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 529987u32);
        assert_eq!(
            Instruction::Leading_Ones_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 529987u32
        );
    }
    #[test]
    fn decode_Leading_Ones_u16() {
        assert_eq!(
            Instruction::decode(268965443u32), Instruction::Leading_Ones_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_u16() {
        println!(
            "{:032b}", Instruction::Leading_Ones_u16 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 268965443u32);
        assert_eq!(
            Instruction::Leading_Ones_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 268965443u32
        );
    }
    #[test]
    fn decode_Leading_Ones_u32() {
        assert_eq!(
            Instruction::decode(537400899u32), Instruction::Leading_Ones_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_u32() {
        println!(
            "{:032b}", Instruction::Leading_Ones_u32 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 537400899u32);
        assert_eq!(
            Instruction::Leading_Ones_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537400899u32
        );
    }
    #[test]
    fn decode_Leading_Ones_u64() {
        assert_eq!(
            Instruction::decode(805836355u32), Instruction::Leading_Ones_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_u64() {
        println!(
            "{:032b}", Instruction::Leading_Ones_u64 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 805836355u32);
        assert_eq!(
            Instruction::Leading_Ones_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 805836355u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_i8() {
        assert_eq!(
            Instruction::decode(2148013699u32), Instruction::Trailing_Ones_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_i8() {
        println!(
            "{:032b}", Instruction::Trailing_Ones_i8 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013699u32);
        assert_eq!(
            Instruction::Trailing_Ones_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013699u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_i16() {
        assert_eq!(
            Instruction::decode(2416449155u32), Instruction::Trailing_Ones_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_i16() {
        println!(
            "{:032b}", Instruction::Trailing_Ones_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416449155u32);
        assert_eq!(
            Instruction::Trailing_Ones_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416449155u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_i32() {
        assert_eq!(
            Instruction::decode(2684884611u32), Instruction::Trailing_Ones_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_i32() {
        println!(
            "{:032b}", Instruction::Trailing_Ones_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684884611u32);
        assert_eq!(
            Instruction::Trailing_Ones_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684884611u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_i64() {
        assert_eq!(
            Instruction::decode(2953320067u32), Instruction::Trailing_Ones_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_i64() {
        println!(
            "{:032b}", Instruction::Trailing_Ones_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953320067u32);
        assert_eq!(
            Instruction::Trailing_Ones_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953320067u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_u8() {
        assert_eq!(
            Instruction::decode(530051u32), Instruction::Trailing_Ones_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_u8() {
        println!(
            "{:032b}", Instruction::Trailing_Ones_u8 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 530051u32);
        assert_eq!(
            Instruction::Trailing_Ones_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 530051u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_u16() {
        assert_eq!(
            Instruction::decode(268965507u32), Instruction::Trailing_Ones_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_u16() {
        println!(
            "{:032b}", Instruction::Trailing_Ones_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 268965507u32);
        assert_eq!(
            Instruction::Trailing_Ones_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 268965507u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_u32() {
        assert_eq!(
            Instruction::decode(537400963u32), Instruction::Trailing_Ones_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_u32() {
        println!(
            "{:032b}", Instruction::Trailing_Ones_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 537400963u32);
        assert_eq!(
            Instruction::Trailing_Ones_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537400963u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_u64() {
        assert_eq!(
            Instruction::decode(805836419u32), Instruction::Trailing_Ones_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_u64() {
        println!(
            "{:032b}", Instruction::Trailing_Ones_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 805836419u32);
        assert_eq!(
            Instruction::Trailing_Ones_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 805836419u32
        );
    }
    #[test]
    fn decode_Count_Zeros_i8() {
        assert_eq!(
            Instruction::decode(2148013763u32), Instruction::Count_Zeros_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_i8() {
        println!(
            "{:032b}", Instruction::Count_Zeros_i8 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013763u32);
        assert_eq!(
            Instruction::Count_Zeros_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013763u32
        );
    }
    #[test]
    fn decode_Count_Zeros_i16() {
        assert_eq!(
            Instruction::decode(2416449219u32), Instruction::Count_Zeros_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_i16() {
        println!(
            "{:032b}", Instruction::Count_Zeros_i16 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416449219u32);
        assert_eq!(
            Instruction::Count_Zeros_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416449219u32
        );
    }
    #[test]
    fn decode_Count_Zeros_i32() {
        assert_eq!(
            Instruction::decode(2684884675u32), Instruction::Count_Zeros_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_i32() {
        println!(
            "{:032b}", Instruction::Count_Zeros_i32 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684884675u32);
        assert_eq!(
            Instruction::Count_Zeros_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684884675u32
        );
    }
    #[test]
    fn decode_Count_Zeros_i64() {
        assert_eq!(
            Instruction::decode(2953320131u32), Instruction::Count_Zeros_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_i64() {
        println!(
            "{:032b}", Instruction::Count_Zeros_i64 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953320131u32);
        assert_eq!(
            Instruction::Count_Zeros_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953320131u32
        );
    }
    #[test]
    fn decode_Count_Zeros_u8() {
        assert_eq!(
            Instruction::decode(530115u32), Instruction::Count_Zeros_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_u8() {
        println!(
            "{:032b}", Instruction::Count_Zeros_u8 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 530115u32);
        assert_eq!(
            Instruction::Count_Zeros_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 530115u32
        );
    }
    #[test]
    fn decode_Count_Zeros_u16() {
        assert_eq!(
            Instruction::decode(268965571u32), Instruction::Count_Zeros_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_u16() {
        println!(
            "{:032b}", Instruction::Count_Zeros_u16 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 268965571u32);
        assert_eq!(
            Instruction::Count_Zeros_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 268965571u32
        );
    }
    #[test]
    fn decode_Count_Zeros_u32() {
        assert_eq!(
            Instruction::decode(537401027u32), Instruction::Count_Zeros_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_u32() {
        println!(
            "{:032b}", Instruction::Count_Zeros_u32 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 537401027u32);
        assert_eq!(
            Instruction::Count_Zeros_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537401027u32
        );
    }
    #[test]
    fn decode_Count_Zeros_u64() {
        assert_eq!(
            Instruction::decode(805836483u32), Instruction::Count_Zeros_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_u64() {
        println!(
            "{:032b}", Instruction::Count_Zeros_u64 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 805836483u32);
        assert_eq!(
            Instruction::Count_Zeros_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 805836483u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_i8() {
        assert_eq!(
            Instruction::decode(2148013827u32), Instruction::Leading_Zeros_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_i8() {
        println!(
            "{:032b}", Instruction::Leading_Zeros_i8 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013827u32);
        assert_eq!(
            Instruction::Leading_Zeros_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013827u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_i16() {
        assert_eq!(
            Instruction::decode(2416449283u32), Instruction::Leading_Zeros_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_i16() {
        println!(
            "{:032b}", Instruction::Leading_Zeros_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416449283u32);
        assert_eq!(
            Instruction::Leading_Zeros_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416449283u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_i32() {
        assert_eq!(
            Instruction::decode(2684884739u32), Instruction::Leading_Zeros_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_i32() {
        println!(
            "{:032b}", Instruction::Leading_Zeros_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684884739u32);
        assert_eq!(
            Instruction::Leading_Zeros_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684884739u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_i64() {
        assert_eq!(
            Instruction::decode(2953320195u32), Instruction::Leading_Zeros_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_i64() {
        println!(
            "{:032b}", Instruction::Leading_Zeros_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953320195u32);
        assert_eq!(
            Instruction::Leading_Zeros_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953320195u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_u8() {
        assert_eq!(
            Instruction::decode(530179u32), Instruction::Leading_Zeros_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_u8() {
        println!(
            "{:032b}", Instruction::Leading_Zeros_u8 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 530179u32);
        assert_eq!(
            Instruction::Leading_Zeros_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 530179u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_u16() {
        assert_eq!(
            Instruction::decode(268965635u32), Instruction::Leading_Zeros_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_u16() {
        println!(
            "{:032b}", Instruction::Leading_Zeros_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 268965635u32);
        assert_eq!(
            Instruction::Leading_Zeros_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 268965635u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_u32() {
        assert_eq!(
            Instruction::decode(537401091u32), Instruction::Leading_Zeros_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_u32() {
        println!(
            "{:032b}", Instruction::Leading_Zeros_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 537401091u32);
        assert_eq!(
            Instruction::Leading_Zeros_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537401091u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_u64() {
        assert_eq!(
            Instruction::decode(805836547u32), Instruction::Leading_Zeros_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_u64() {
        println!(
            "{:032b}", Instruction::Leading_Zeros_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 805836547u32);
        assert_eq!(
            Instruction::Leading_Zeros_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 805836547u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_i8() {
        assert_eq!(
            Instruction::decode(2148013891u32), Instruction::Trailing_Zeros_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_i8() {
        println!(
            "{:032b}", Instruction::Trailing_Zeros_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013891u32);
        assert_eq!(
            Instruction::Trailing_Zeros_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013891u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_i16() {
        assert_eq!(
            Instruction::decode(2416449347u32), Instruction::Trailing_Zeros_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_i16() {
        println!(
            "{:032b}", Instruction::Trailing_Zeros_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416449347u32);
        assert_eq!(
            Instruction::Trailing_Zeros_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416449347u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_i32() {
        assert_eq!(
            Instruction::decode(2684884803u32), Instruction::Trailing_Zeros_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_i32() {
        println!(
            "{:032b}", Instruction::Trailing_Zeros_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684884803u32);
        assert_eq!(
            Instruction::Trailing_Zeros_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684884803u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_i64() {
        assert_eq!(
            Instruction::decode(2953320259u32), Instruction::Trailing_Zeros_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_i64() {
        println!(
            "{:032b}", Instruction::Trailing_Zeros_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953320259u32);
        assert_eq!(
            Instruction::Trailing_Zeros_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953320259u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_u8() {
        assert_eq!(
            Instruction::decode(530243u32), Instruction::Trailing_Zeros_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_u8() {
        println!(
            "{:032b}", Instruction::Trailing_Zeros_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 530243u32);
        assert_eq!(
            Instruction::Trailing_Zeros_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 530243u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_u16() {
        assert_eq!(
            Instruction::decode(268965699u32), Instruction::Trailing_Zeros_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_u16() {
        println!(
            "{:032b}", Instruction::Trailing_Zeros_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 268965699u32);
        assert_eq!(
            Instruction::Trailing_Zeros_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 268965699u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_u32() {
        assert_eq!(
            Instruction::decode(537401155u32), Instruction::Trailing_Zeros_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_u32() {
        println!(
            "{:032b}", Instruction::Trailing_Zeros_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 537401155u32);
        assert_eq!(
            Instruction::Trailing_Zeros_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537401155u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_u64() {
        assert_eq!(
            Instruction::decode(805836611u32), Instruction::Trailing_Zeros_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_u64() {
        println!(
            "{:032b}", Instruction::Trailing_Zeros_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 805836611u32);
        assert_eq!(
            Instruction::Trailing_Zeros_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 805836611u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_i8() {
        assert_eq!(
            Instruction::decode(2148013955u32), Instruction::Reverse_Bytes_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_i8() {
        println!(
            "{:032b}", Instruction::Reverse_Bytes_i8 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013955u32);
        assert_eq!(
            Instruction::Reverse_Bytes_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013955u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_i16() {
        assert_eq!(
            Instruction::decode(2416449411u32), Instruction::Reverse_Bytes_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_i16() {
        println!(
            "{:032b}", Instruction::Reverse_Bytes_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416449411u32);
        assert_eq!(
            Instruction::Reverse_Bytes_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416449411u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_i32() {
        assert_eq!(
            Instruction::decode(2684884867u32), Instruction::Reverse_Bytes_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_i32() {
        println!(
            "{:032b}", Instruction::Reverse_Bytes_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684884867u32);
        assert_eq!(
            Instruction::Reverse_Bytes_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684884867u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_i64() {
        assert_eq!(
            Instruction::decode(2953320323u32), Instruction::Reverse_Bytes_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_i64() {
        println!(
            "{:032b}", Instruction::Reverse_Bytes_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953320323u32);
        assert_eq!(
            Instruction::Reverse_Bytes_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953320323u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_u8() {
        assert_eq!(
            Instruction::decode(530307u32), Instruction::Reverse_Bytes_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_u8() {
        println!(
            "{:032b}", Instruction::Reverse_Bytes_u8 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 530307u32);
        assert_eq!(
            Instruction::Reverse_Bytes_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 530307u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_u16() {
        assert_eq!(
            Instruction::decode(268965763u32), Instruction::Reverse_Bytes_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_u16() {
        println!(
            "{:032b}", Instruction::Reverse_Bytes_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 268965763u32);
        assert_eq!(
            Instruction::Reverse_Bytes_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 268965763u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_u32() {
        assert_eq!(
            Instruction::decode(537401219u32), Instruction::Reverse_Bytes_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_u32() {
        println!(
            "{:032b}", Instruction::Reverse_Bytes_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 537401219u32);
        assert_eq!(
            Instruction::Reverse_Bytes_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537401219u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_u64() {
        assert_eq!(
            Instruction::decode(805836675u32), Instruction::Reverse_Bytes_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_u64() {
        println!(
            "{:032b}", Instruction::Reverse_Bytes_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 805836675u32);
        assert_eq!(
            Instruction::Reverse_Bytes_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 805836675u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_i8() {
        assert_eq!(
            Instruction::decode(2148014019u32), Instruction::Reverse_Bits_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_i8() {
        println!(
            "{:032b}", Instruction::Reverse_Bits_i8 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148014019u32);
        assert_eq!(
            Instruction::Reverse_Bits_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148014019u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_i16() {
        assert_eq!(
            Instruction::decode(2416449475u32), Instruction::Reverse_Bits_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_i16() {
        println!(
            "{:032b}", Instruction::Reverse_Bits_i16 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416449475u32);
        assert_eq!(
            Instruction::Reverse_Bits_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416449475u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_i32() {
        assert_eq!(
            Instruction::decode(2684884931u32), Instruction::Reverse_Bits_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_i32() {
        println!(
            "{:032b}", Instruction::Reverse_Bits_i32 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684884931u32);
        assert_eq!(
            Instruction::Reverse_Bits_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684884931u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_i64() {
        assert_eq!(
            Instruction::decode(2953320387u32), Instruction::Reverse_Bits_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_i64() {
        println!(
            "{:032b}", Instruction::Reverse_Bits_i64 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953320387u32);
        assert_eq!(
            Instruction::Reverse_Bits_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953320387u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_u8() {
        assert_eq!(
            Instruction::decode(530371u32), Instruction::Reverse_Bits_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_u8() {
        println!(
            "{:032b}", Instruction::Reverse_Bits_u8 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 530371u32);
        assert_eq!(
            Instruction::Reverse_Bits_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 530371u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_u16() {
        assert_eq!(
            Instruction::decode(268965827u32), Instruction::Reverse_Bits_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_u16() {
        println!(
            "{:032b}", Instruction::Reverse_Bits_u16 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 268965827u32);
        assert_eq!(
            Instruction::Reverse_Bits_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 268965827u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_u32() {
        assert_eq!(
            Instruction::decode(537401283u32), Instruction::Reverse_Bits_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_u32() {
        println!(
            "{:032b}", Instruction::Reverse_Bits_u32 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 537401283u32);
        assert_eq!(
            Instruction::Reverse_Bits_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537401283u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_u64() {
        assert_eq!(
            Instruction::decode(805836739u32), Instruction::Reverse_Bits_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_u64() {
        println!(
            "{:032b}", Instruction::Reverse_Bits_u64 { rd : Register::General_Purpose(5),
            rs1 : Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 805836739u32);
        assert_eq!(
            Instruction::Reverse_Bits_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 805836739u32
        );
    }
    #[test]
    fn decode_C_Abs_i8() {
        assert_eq!(
            Instruction::decode(2148013060u32), Instruction::C_Abs_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Abs_i8() {
        println!(
            "{:032b}", Instruction::C_Abs_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013060u32);
        assert_eq!(
            Instruction::C_Abs_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013060u32
        );
    }
    #[test]
    fn decode_C_Abs_i16() {
        assert_eq!(
            Instruction::decode(2416448516u32), Instruction::C_Abs_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Abs_i16() {
        println!(
            "{:032b}", Instruction::C_Abs_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416448516u32);
        assert_eq!(
            Instruction::C_Abs_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416448516u32
        );
    }
    #[test]
    fn decode_C_Abs_i32() {
        assert_eq!(
            Instruction::decode(2684883972u32), Instruction::C_Abs_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Abs_i32() {
        println!(
            "{:032b}", Instruction::C_Abs_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684883972u32);
        assert_eq!(
            Instruction::C_Abs_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684883972u32
        );
    }
    #[test]
    fn decode_C_Abs_i64() {
        assert_eq!(
            Instruction::decode(2953319428u32), Instruction::C_Abs_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Abs_i64() {
        println!(
            "{:032b}", Instruction::C_Abs_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953319428u32);
        assert_eq!(
            Instruction::C_Abs_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953319428u32
        );
    }
    #[test]
    fn decode_C_Add_i8() {
        assert_eq!(
            Instruction::decode(2173178948u32), Instruction::C_Add_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_i8() {
        println!(
            "{:032b}", Instruction::C_Add_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173178948u32);
        assert_eq!(
            Instruction::C_Add_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173178948u32
        );
    }
    #[test]
    fn decode_C_Add_i16() {
        assert_eq!(
            Instruction::decode(2441614404u32), Instruction::C_Add_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_i16() {
        println!(
            "{:032b}", Instruction::C_Add_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614404u32);
        assert_eq!(
            Instruction::C_Add_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614404u32
        );
    }
    #[test]
    fn decode_C_Add_i32() {
        assert_eq!(
            Instruction::decode(2710049860u32), Instruction::C_Add_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_i32() {
        println!(
            "{:032b}", Instruction::C_Add_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710049860u32);
        assert_eq!(
            Instruction::C_Add_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710049860u32
        );
    }
    #[test]
    fn decode_C_Add_i64() {
        assert_eq!(
            Instruction::decode(2978485316u32), Instruction::C_Add_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_i64() {
        println!(
            "{:032b}", Instruction::C_Add_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485316u32);
        assert_eq!(
            Instruction::C_Add_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485316u32
        );
    }
    #[test]
    fn decode_C_Add_u8() {
        assert_eq!(
            Instruction::decode(25695300u32), Instruction::C_Add_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_u8() {
        println!(
            "{:032b}", Instruction::C_Add_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695300u32);
        assert_eq!(
            Instruction::C_Add_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695300u32
        );
    }
    #[test]
    fn decode_C_Add_u16() {
        assert_eq!(
            Instruction::decode(294130756u32), Instruction::C_Add_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_u16() {
        println!(
            "{:032b}", Instruction::C_Add_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294130756u32);
        assert_eq!(
            Instruction::C_Add_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130756u32
        );
    }
    #[test]
    fn decode_C_Add_u32() {
        assert_eq!(
            Instruction::decode(562566212u32), Instruction::C_Add_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_u32() {
        println!(
            "{:032b}", Instruction::C_Add_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566212u32);
        assert_eq!(
            Instruction::C_Add_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566212u32
        );
    }
    #[test]
    fn decode_C_Add_u64() {
        assert_eq!(
            Instruction::decode(831001668u32), Instruction::C_Add_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_u64() {
        println!(
            "{:032b}", Instruction::C_Add_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831001668u32);
        assert_eq!(
            Instruction::C_Add_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001668u32
        );
    }
    #[test]
    fn decode_C_Add_U_i8() {
        assert_eq!(
            Instruction::decode(2173179012u32), Instruction::C_Add_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_U_i8() {
        println!(
            "{:032b}", Instruction::C_Add_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179012u32);
        assert_eq!(
            Instruction::C_Add_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179012u32
        );
    }
    #[test]
    fn decode_C_Add_U_i16() {
        assert_eq!(
            Instruction::decode(2441614468u32), Instruction::C_Add_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_U_i16() {
        println!(
            "{:032b}", Instruction::C_Add_U_i16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2441614468u32);
        assert_eq!(
            Instruction::C_Add_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614468u32
        );
    }
    #[test]
    fn decode_C_Add_U_i32() {
        assert_eq!(
            Instruction::decode(2710049924u32), Instruction::C_Add_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_U_i32() {
        println!(
            "{:032b}", Instruction::C_Add_U_i32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2710049924u32);
        assert_eq!(
            Instruction::C_Add_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710049924u32
        );
    }
    #[test]
    fn decode_C_Add_U_i64() {
        assert_eq!(
            Instruction::decode(2978485380u32), Instruction::C_Add_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_U_i64() {
        println!(
            "{:032b}", Instruction::C_Add_U_i64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2978485380u32);
        assert_eq!(
            Instruction::C_Add_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485380u32
        );
    }
    #[test]
    fn decode_C_Add_S_u8() {
        assert_eq!(
            Instruction::decode(25695364u32), Instruction::C_Add_S_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_S_u8() {
        println!(
            "{:032b}", Instruction::C_Add_S_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695364u32);
        assert_eq!(
            Instruction::C_Add_S_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695364u32
        );
    }
    #[test]
    fn decode_C_Add_S_u16() {
        assert_eq!(
            Instruction::decode(294130820u32), Instruction::C_Add_S_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_S_u16() {
        println!(
            "{:032b}", Instruction::C_Add_S_u16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 294130820u32);
        assert_eq!(
            Instruction::C_Add_S_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130820u32
        );
    }
    #[test]
    fn decode_C_Add_S_u32() {
        assert_eq!(
            Instruction::decode(562566276u32), Instruction::C_Add_S_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_S_u32() {
        println!(
            "{:032b}", Instruction::C_Add_S_u32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 562566276u32);
        assert_eq!(
            Instruction::C_Add_S_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566276u32
        );
    }
    #[test]
    fn decode_C_Add_S_u64() {
        assert_eq!(
            Instruction::decode(831001732u32), Instruction::C_Add_S_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_S_u64() {
        println!(
            "{:032b}", Instruction::C_Add_S_u64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 831001732u32);
        assert_eq!(
            Instruction::C_Add_S_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001732u32
        );
    }
    #[test]
    fn decode_C_Div_i8() {
        assert_eq!(
            Instruction::decode(2173179076u32), Instruction::C_Div_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_i8() {
        println!(
            "{:032b}", Instruction::C_Div_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179076u32);
        assert_eq!(
            Instruction::C_Div_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179076u32
        );
    }
    #[test]
    fn decode_C_Div_i16() {
        assert_eq!(
            Instruction::decode(2441614532u32), Instruction::C_Div_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_i16() {
        println!(
            "{:032b}", Instruction::C_Div_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614532u32);
        assert_eq!(
            Instruction::C_Div_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614532u32
        );
    }
    #[test]
    fn decode_C_Div_i32() {
        assert_eq!(
            Instruction::decode(2710049988u32), Instruction::C_Div_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_i32() {
        println!(
            "{:032b}", Instruction::C_Div_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710049988u32);
        assert_eq!(
            Instruction::C_Div_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710049988u32
        );
    }
    #[test]
    fn decode_C_Div_i64() {
        assert_eq!(
            Instruction::decode(2978485444u32), Instruction::C_Div_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_i64() {
        println!(
            "{:032b}", Instruction::C_Div_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485444u32);
        assert_eq!(
            Instruction::C_Div_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485444u32
        );
    }
    #[test]
    fn decode_C_Div_u8() {
        assert_eq!(
            Instruction::decode(25695428u32), Instruction::C_Div_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_u8() {
        println!(
            "{:032b}", Instruction::C_Div_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695428u32);
        assert_eq!(
            Instruction::C_Div_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695428u32
        );
    }
    #[test]
    fn decode_C_Div_u16() {
        assert_eq!(
            Instruction::decode(294130884u32), Instruction::C_Div_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_u16() {
        println!(
            "{:032b}", Instruction::C_Div_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294130884u32);
        assert_eq!(
            Instruction::C_Div_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130884u32
        );
    }
    #[test]
    fn decode_C_Div_u32() {
        assert_eq!(
            Instruction::decode(562566340u32), Instruction::C_Div_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_u32() {
        println!(
            "{:032b}", Instruction::C_Div_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566340u32);
        assert_eq!(
            Instruction::C_Div_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566340u32
        );
    }
    #[test]
    fn decode_C_Div_u64() {
        assert_eq!(
            Instruction::decode(831001796u32), Instruction::C_Div_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_u64() {
        println!(
            "{:032b}", Instruction::C_Div_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831001796u32);
        assert_eq!(
            Instruction::C_Div_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001796u32
        );
    }
    #[test]
    fn decode_C_Div_E_i8() {
        assert_eq!(
            Instruction::decode(2173179140u32), Instruction::C_Div_E_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_i8() {
        println!(
            "{:032b}", Instruction::C_Div_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179140u32);
        assert_eq!(
            Instruction::C_Div_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179140u32
        );
    }
    #[test]
    fn decode_C_Div_E_i16() {
        assert_eq!(
            Instruction::decode(2441614596u32), Instruction::C_Div_E_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_i16() {
        println!(
            "{:032b}", Instruction::C_Div_E_i16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2441614596u32);
        assert_eq!(
            Instruction::C_Div_E_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614596u32
        );
    }
    #[test]
    fn decode_C_Div_E_i32() {
        assert_eq!(
            Instruction::decode(2710050052u32), Instruction::C_Div_E_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_i32() {
        println!(
            "{:032b}", Instruction::C_Div_E_i32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2710050052u32);
        assert_eq!(
            Instruction::C_Div_E_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050052u32
        );
    }
    #[test]
    fn decode_C_Div_E_i64() {
        assert_eq!(
            Instruction::decode(2978485508u32), Instruction::C_Div_E_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_i64() {
        println!(
            "{:032b}", Instruction::C_Div_E_i64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2978485508u32);
        assert_eq!(
            Instruction::C_Div_E_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485508u32
        );
    }
    #[test]
    fn decode_C_Div_E_u8() {
        assert_eq!(
            Instruction::decode(25695492u32), Instruction::C_Div_E_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_u8() {
        println!(
            "{:032b}", Instruction::C_Div_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695492u32);
        assert_eq!(
            Instruction::C_Div_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695492u32
        );
    }
    #[test]
    fn decode_C_Div_E_u16() {
        assert_eq!(
            Instruction::decode(294130948u32), Instruction::C_Div_E_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_u16() {
        println!(
            "{:032b}", Instruction::C_Div_E_u16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 294130948u32);
        assert_eq!(
            Instruction::C_Div_E_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130948u32
        );
    }
    #[test]
    fn decode_C_Div_E_u32() {
        assert_eq!(
            Instruction::decode(562566404u32), Instruction::C_Div_E_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_u32() {
        println!(
            "{:032b}", Instruction::C_Div_E_u32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 562566404u32);
        assert_eq!(
            Instruction::C_Div_E_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566404u32
        );
    }
    #[test]
    fn decode_C_Div_E_u64() {
        assert_eq!(
            Instruction::decode(831001860u32), Instruction::C_Div_E_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_u64() {
        println!(
            "{:032b}", Instruction::C_Div_E_u64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 831001860u32);
        assert_eq!(
            Instruction::C_Div_E_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001860u32
        );
    }
    #[test]
    fn decode_C_Log_i8() {
        assert_eq!(
            Instruction::decode(2173179204u32), Instruction::C_Log_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_i8() {
        println!(
            "{:032b}", Instruction::C_Log_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179204u32);
        assert_eq!(
            Instruction::C_Log_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179204u32
        );
    }
    #[test]
    fn decode_C_Log_i16() {
        assert_eq!(
            Instruction::decode(2441614660u32), Instruction::C_Log_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_i16() {
        println!(
            "{:032b}", Instruction::C_Log_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614660u32);
        assert_eq!(
            Instruction::C_Log_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614660u32
        );
    }
    #[test]
    fn decode_C_Log_i32() {
        assert_eq!(
            Instruction::decode(2710050116u32), Instruction::C_Log_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_i32() {
        println!(
            "{:032b}", Instruction::C_Log_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050116u32);
        assert_eq!(
            Instruction::C_Log_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050116u32
        );
    }
    #[test]
    fn decode_C_Log_i64() {
        assert_eq!(
            Instruction::decode(2978485572u32), Instruction::C_Log_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_i64() {
        println!(
            "{:032b}", Instruction::C_Log_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485572u32);
        assert_eq!(
            Instruction::C_Log_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485572u32
        );
    }
    #[test]
    fn decode_C_Log_u8() {
        assert_eq!(
            Instruction::decode(25695556u32), Instruction::C_Log_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_u8() {
        println!(
            "{:032b}", Instruction::C_Log_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695556u32);
        assert_eq!(
            Instruction::C_Log_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695556u32
        );
    }
    #[test]
    fn decode_C_Log_u16() {
        assert_eq!(
            Instruction::decode(294131012u32), Instruction::C_Log_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_u16() {
        println!(
            "{:032b}", Instruction::C_Log_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131012u32);
        assert_eq!(
            Instruction::C_Log_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131012u32
        );
    }
    #[test]
    fn decode_C_Log_u32() {
        assert_eq!(
            Instruction::decode(562566468u32), Instruction::C_Log_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_u32() {
        println!(
            "{:032b}", Instruction::C_Log_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566468u32);
        assert_eq!(
            Instruction::C_Log_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566468u32
        );
    }
    #[test]
    fn decode_C_Log_u64() {
        assert_eq!(
            Instruction::decode(831001924u32), Instruction::C_Log_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_u64() {
        println!(
            "{:032b}", Instruction::C_Log_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831001924u32);
        assert_eq!(
            Instruction::C_Log_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001924u32
        );
    }
    #[test]
    fn decode_C_Sqrt_i8() {
        assert_eq!(
            Instruction::decode(2148013444u32), Instruction::C_Sqrt_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_i8() {
        println!(
            "{:032b}", Instruction::C_Sqrt_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013444u32);
        assert_eq!(
            Instruction::C_Sqrt_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013444u32
        );
    }
    #[test]
    fn decode_C_Sqrt_i16() {
        assert_eq!(
            Instruction::decode(2416448900u32), Instruction::C_Sqrt_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_i16() {
        println!(
            "{:032b}", Instruction::C_Sqrt_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416448900u32);
        assert_eq!(
            Instruction::C_Sqrt_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416448900u32
        );
    }
    #[test]
    fn decode_C_Sqrt_i32() {
        assert_eq!(
            Instruction::decode(2684884356u32), Instruction::C_Sqrt_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_i32() {
        println!(
            "{:032b}", Instruction::C_Sqrt_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684884356u32);
        assert_eq!(
            Instruction::C_Sqrt_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684884356u32
        );
    }
    #[test]
    fn decode_C_Sqrt_i64() {
        assert_eq!(
            Instruction::decode(2953319812u32), Instruction::C_Sqrt_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_i64() {
        println!(
            "{:032b}", Instruction::C_Sqrt_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953319812u32);
        assert_eq!(
            Instruction::C_Sqrt_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953319812u32
        );
    }
    #[test]
    fn decode_C_Sqrt_u8() {
        assert_eq!(
            Instruction::decode(529796u32), Instruction::C_Sqrt_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_u8() {
        println!(
            "{:032b}", Instruction::C_Sqrt_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 529796u32);
        assert_eq!(
            Instruction::C_Sqrt_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 529796u32
        );
    }
    #[test]
    fn decode_C_Sqrt_u16() {
        assert_eq!(
            Instruction::decode(268965252u32), Instruction::C_Sqrt_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_u16() {
        println!(
            "{:032b}", Instruction::C_Sqrt_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 268965252u32);
        assert_eq!(
            Instruction::C_Sqrt_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 268965252u32
        );
    }
    #[test]
    fn decode_C_Sqrt_u32() {
        assert_eq!(
            Instruction::decode(537400708u32), Instruction::C_Sqrt_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_u32() {
        println!(
            "{:032b}", Instruction::C_Sqrt_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 537400708u32);
        assert_eq!(
            Instruction::C_Sqrt_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537400708u32
        );
    }
    #[test]
    fn decode_C_Sqrt_u64() {
        assert_eq!(
            Instruction::decode(805836164u32), Instruction::C_Sqrt_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_u64() {
        println!(
            "{:032b}", Instruction::C_Sqrt_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 805836164u32);
        assert_eq!(
            Instruction::C_Sqrt_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 805836164u32
        );
    }
    #[test]
    fn decode_C_Mul_i8() {
        assert_eq!(
            Instruction::decode(2173179332u32), Instruction::C_Mul_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_i8() {
        println!(
            "{:032b}", Instruction::C_Mul_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179332u32);
        assert_eq!(
            Instruction::C_Mul_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179332u32
        );
    }
    #[test]
    fn decode_C_Mul_i16() {
        assert_eq!(
            Instruction::decode(2441614788u32), Instruction::C_Mul_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_i16() {
        println!(
            "{:032b}", Instruction::C_Mul_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614788u32);
        assert_eq!(
            Instruction::C_Mul_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614788u32
        );
    }
    #[test]
    fn decode_C_Mul_i32() {
        assert_eq!(
            Instruction::decode(2710050244u32), Instruction::C_Mul_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_i32() {
        println!(
            "{:032b}", Instruction::C_Mul_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050244u32);
        assert_eq!(
            Instruction::C_Mul_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050244u32
        );
    }
    #[test]
    fn decode_C_Mul_i64() {
        assert_eq!(
            Instruction::decode(2978485700u32), Instruction::C_Mul_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_i64() {
        println!(
            "{:032b}", Instruction::C_Mul_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485700u32);
        assert_eq!(
            Instruction::C_Mul_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485700u32
        );
    }
    #[test]
    fn decode_C_Mul_u8() {
        assert_eq!(
            Instruction::decode(25695684u32), Instruction::C_Mul_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_u8() {
        println!(
            "{:032b}", Instruction::C_Mul_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695684u32);
        assert_eq!(
            Instruction::C_Mul_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695684u32
        );
    }
    #[test]
    fn decode_C_Mul_u16() {
        assert_eq!(
            Instruction::decode(294131140u32), Instruction::C_Mul_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_u16() {
        println!(
            "{:032b}", Instruction::C_Mul_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131140u32);
        assert_eq!(
            Instruction::C_Mul_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131140u32
        );
    }
    #[test]
    fn decode_C_Mul_u32() {
        assert_eq!(
            Instruction::decode(562566596u32), Instruction::C_Mul_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_u32() {
        println!(
            "{:032b}", Instruction::C_Mul_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566596u32);
        assert_eq!(
            Instruction::C_Mul_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566596u32
        );
    }
    #[test]
    fn decode_C_Mul_u64() {
        assert_eq!(
            Instruction::decode(831002052u32), Instruction::C_Mul_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_u64() {
        println!(
            "{:032b}", Instruction::C_Mul_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002052u32);
        assert_eq!(
            Instruction::C_Mul_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002052u32
        );
    }
    #[test]
    fn decode_C_Neg_i8() {
        assert_eq!(
            Instruction::decode(2148013572u32), Instruction::C_Neg_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Neg_i8() {
        println!(
            "{:032b}", Instruction::C_Neg_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013572u32);
        assert_eq!(
            Instruction::C_Neg_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013572u32
        );
    }
    #[test]
    fn decode_C_Neg_i16() {
        assert_eq!(
            Instruction::decode(2416449028u32), Instruction::C_Neg_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Neg_i16() {
        println!(
            "{:032b}", Instruction::C_Neg_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416449028u32);
        assert_eq!(
            Instruction::C_Neg_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416449028u32
        );
    }
    #[test]
    fn decode_C_Neg_i32() {
        assert_eq!(
            Instruction::decode(2684884484u32), Instruction::C_Neg_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Neg_i32() {
        println!(
            "{:032b}", Instruction::C_Neg_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684884484u32);
        assert_eq!(
            Instruction::C_Neg_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684884484u32
        );
    }
    #[test]
    fn decode_C_Neg_i64() {
        assert_eq!(
            Instruction::decode(2953319940u32), Instruction::C_Neg_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Neg_i64() {
        println!(
            "{:032b}", Instruction::C_Neg_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953319940u32);
        assert_eq!(
            Instruction::C_Neg_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953319940u32
        );
    }
    #[test]
    fn decode_C_Pow_i8() {
        assert_eq!(
            Instruction::decode(2173179460u32), Instruction::C_Pow_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_i8() {
        println!(
            "{:032b}", Instruction::C_Pow_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179460u32);
        assert_eq!(
            Instruction::C_Pow_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179460u32
        );
    }
    #[test]
    fn decode_C_Pow_i16() {
        assert_eq!(
            Instruction::decode(2441614916u32), Instruction::C_Pow_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_i16() {
        println!(
            "{:032b}", Instruction::C_Pow_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614916u32);
        assert_eq!(
            Instruction::C_Pow_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614916u32
        );
    }
    #[test]
    fn decode_C_Pow_i32() {
        assert_eq!(
            Instruction::decode(2710050372u32), Instruction::C_Pow_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_i32() {
        println!(
            "{:032b}", Instruction::C_Pow_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050372u32);
        assert_eq!(
            Instruction::C_Pow_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050372u32
        );
    }
    #[test]
    fn decode_C_Pow_i64() {
        assert_eq!(
            Instruction::decode(2978485828u32), Instruction::C_Pow_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_i64() {
        println!(
            "{:032b}", Instruction::C_Pow_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485828u32);
        assert_eq!(
            Instruction::C_Pow_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485828u32
        );
    }
    #[test]
    fn decode_C_Pow_u8() {
        assert_eq!(
            Instruction::decode(25695812u32), Instruction::C_Pow_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_u8() {
        println!(
            "{:032b}", Instruction::C_Pow_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695812u32);
        assert_eq!(
            Instruction::C_Pow_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695812u32
        );
    }
    #[test]
    fn decode_C_Pow_u16() {
        assert_eq!(
            Instruction::decode(294131268u32), Instruction::C_Pow_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_u16() {
        println!(
            "{:032b}", Instruction::C_Pow_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131268u32);
        assert_eq!(
            Instruction::C_Pow_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131268u32
        );
    }
    #[test]
    fn decode_C_Pow_u32() {
        assert_eq!(
            Instruction::decode(562566724u32), Instruction::C_Pow_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_u32() {
        println!(
            "{:032b}", Instruction::C_Pow_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566724u32);
        assert_eq!(
            Instruction::C_Pow_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566724u32
        );
    }
    #[test]
    fn decode_C_Pow_u64() {
        assert_eq!(
            Instruction::decode(831002180u32), Instruction::C_Pow_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_u64() {
        println!(
            "{:032b}", Instruction::C_Pow_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002180u32);
        assert_eq!(
            Instruction::C_Pow_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002180u32
        );
    }
    #[test]
    fn decode_C_Rem_i8() {
        assert_eq!(
            Instruction::decode(2173179524u32), Instruction::C_Rem_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_i8() {
        println!(
            "{:032b}", Instruction::C_Rem_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179524u32);
        assert_eq!(
            Instruction::C_Rem_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179524u32
        );
    }
    #[test]
    fn decode_C_Rem_i16() {
        assert_eq!(
            Instruction::decode(2441614980u32), Instruction::C_Rem_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_i16() {
        println!(
            "{:032b}", Instruction::C_Rem_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614980u32);
        assert_eq!(
            Instruction::C_Rem_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614980u32
        );
    }
    #[test]
    fn decode_C_Rem_i32() {
        assert_eq!(
            Instruction::decode(2710050436u32), Instruction::C_Rem_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_i32() {
        println!(
            "{:032b}", Instruction::C_Rem_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050436u32);
        assert_eq!(
            Instruction::C_Rem_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050436u32
        );
    }
    #[test]
    fn decode_C_Rem_i64() {
        assert_eq!(
            Instruction::decode(2978485892u32), Instruction::C_Rem_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_i64() {
        println!(
            "{:032b}", Instruction::C_Rem_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485892u32);
        assert_eq!(
            Instruction::C_Rem_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485892u32
        );
    }
    #[test]
    fn decode_C_Rem_u8() {
        assert_eq!(
            Instruction::decode(25695876u32), Instruction::C_Rem_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_u8() {
        println!(
            "{:032b}", Instruction::C_Rem_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695876u32);
        assert_eq!(
            Instruction::C_Rem_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695876u32
        );
    }
    #[test]
    fn decode_C_Rem_u16() {
        assert_eq!(
            Instruction::decode(294131332u32), Instruction::C_Rem_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_u16() {
        println!(
            "{:032b}", Instruction::C_Rem_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131332u32);
        assert_eq!(
            Instruction::C_Rem_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131332u32
        );
    }
    #[test]
    fn decode_C_Rem_u32() {
        assert_eq!(
            Instruction::decode(562566788u32), Instruction::C_Rem_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_u32() {
        println!(
            "{:032b}", Instruction::C_Rem_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566788u32);
        assert_eq!(
            Instruction::C_Rem_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566788u32
        );
    }
    #[test]
    fn decode_C_Rem_u64() {
        assert_eq!(
            Instruction::decode(831002244u32), Instruction::C_Rem_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_u64() {
        println!(
            "{:032b}", Instruction::C_Rem_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002244u32);
        assert_eq!(
            Instruction::C_Rem_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002244u32
        );
    }
    #[test]
    fn decode_C_Rem_E_i8() {
        assert_eq!(
            Instruction::decode(2173179588u32), Instruction::C_Rem_E_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_i8() {
        println!(
            "{:032b}", Instruction::C_Rem_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179588u32);
        assert_eq!(
            Instruction::C_Rem_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179588u32
        );
    }
    #[test]
    fn decode_C_Rem_E_i16() {
        assert_eq!(
            Instruction::decode(2441615044u32), Instruction::C_Rem_E_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_i16() {
        println!(
            "{:032b}", Instruction::C_Rem_E_i16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2441615044u32);
        assert_eq!(
            Instruction::C_Rem_E_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441615044u32
        );
    }
    #[test]
    fn decode_C_Rem_E_i32() {
        assert_eq!(
            Instruction::decode(2710050500u32), Instruction::C_Rem_E_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_i32() {
        println!(
            "{:032b}", Instruction::C_Rem_E_i32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2710050500u32);
        assert_eq!(
            Instruction::C_Rem_E_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050500u32
        );
    }
    #[test]
    fn decode_C_Rem_E_i64() {
        assert_eq!(
            Instruction::decode(2978485956u32), Instruction::C_Rem_E_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_i64() {
        println!(
            "{:032b}", Instruction::C_Rem_E_i64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2978485956u32);
        assert_eq!(
            Instruction::C_Rem_E_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485956u32
        );
    }
    #[test]
    fn decode_C_Rem_E_u8() {
        assert_eq!(
            Instruction::decode(25695940u32), Instruction::C_Rem_E_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_u8() {
        println!(
            "{:032b}", Instruction::C_Rem_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695940u32);
        assert_eq!(
            Instruction::C_Rem_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695940u32
        );
    }
    #[test]
    fn decode_C_Rem_E_u16() {
        assert_eq!(
            Instruction::decode(294131396u32), Instruction::C_Rem_E_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_u16() {
        println!(
            "{:032b}", Instruction::C_Rem_E_u16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 294131396u32);
        assert_eq!(
            Instruction::C_Rem_E_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131396u32
        );
    }
    #[test]
    fn decode_C_Rem_E_u32() {
        assert_eq!(
            Instruction::decode(562566852u32), Instruction::C_Rem_E_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_u32() {
        println!(
            "{:032b}", Instruction::C_Rem_E_u32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 562566852u32);
        assert_eq!(
            Instruction::C_Rem_E_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566852u32
        );
    }
    #[test]
    fn decode_C_Rem_E_u64() {
        assert_eq!(
            Instruction::decode(831002308u32), Instruction::C_Rem_E_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_u64() {
        println!(
            "{:032b}", Instruction::C_Rem_E_u64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 831002308u32);
        assert_eq!(
            Instruction::C_Rem_E_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002308u32
        );
    }
    #[test]
    fn decode_C_Shl_i8() {
        assert_eq!(
            Instruction::decode(2173179652u32), Instruction::C_Shl_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_i8() {
        println!(
            "{:032b}", Instruction::C_Shl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179652u32);
        assert_eq!(
            Instruction::C_Shl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179652u32
        );
    }
    #[test]
    fn decode_C_Shl_i16() {
        assert_eq!(
            Instruction::decode(2441615108u32), Instruction::C_Shl_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_i16() {
        println!(
            "{:032b}", Instruction::C_Shl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441615108u32);
        assert_eq!(
            Instruction::C_Shl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441615108u32
        );
    }
    #[test]
    fn decode_C_Shl_i32() {
        assert_eq!(
            Instruction::decode(2710050564u32), Instruction::C_Shl_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_i32() {
        println!(
            "{:032b}", Instruction::C_Shl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050564u32);
        assert_eq!(
            Instruction::C_Shl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050564u32
        );
    }
    #[test]
    fn decode_C_Shl_i64() {
        assert_eq!(
            Instruction::decode(2978486020u32), Instruction::C_Shl_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_i64() {
        println!(
            "{:032b}", Instruction::C_Shl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978486020u32);
        assert_eq!(
            Instruction::C_Shl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978486020u32
        );
    }
    #[test]
    fn decode_C_Shl_u8() {
        assert_eq!(
            Instruction::decode(25696004u32), Instruction::C_Shl_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_u8() {
        println!(
            "{:032b}", Instruction::C_Shl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25696004u32);
        assert_eq!(
            Instruction::C_Shl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25696004u32
        );
    }
    #[test]
    fn decode_C_Shl_u16() {
        assert_eq!(
            Instruction::decode(294131460u32), Instruction::C_Shl_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_u16() {
        println!(
            "{:032b}", Instruction::C_Shl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131460u32);
        assert_eq!(
            Instruction::C_Shl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131460u32
        );
    }
    #[test]
    fn decode_C_Shl_u32() {
        assert_eq!(
            Instruction::decode(562566916u32), Instruction::C_Shl_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_u32() {
        println!(
            "{:032b}", Instruction::C_Shl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566916u32);
        assert_eq!(
            Instruction::C_Shl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566916u32
        );
    }
    #[test]
    fn decode_C_Shl_u64() {
        assert_eq!(
            Instruction::decode(831002372u32), Instruction::C_Shl_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_u64() {
        println!(
            "{:032b}", Instruction::C_Shl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002372u32);
        assert_eq!(
            Instruction::C_Shl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002372u32
        );
    }
    #[test]
    fn decode_C_Shr_i8() {
        assert_eq!(
            Instruction::decode(2173179716u32), Instruction::C_Shr_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_i8() {
        println!(
            "{:032b}", Instruction::C_Shr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179716u32);
        assert_eq!(
            Instruction::C_Shr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179716u32
        );
    }
    #[test]
    fn decode_C_Shr_i16() {
        assert_eq!(
            Instruction::decode(2441615172u32), Instruction::C_Shr_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_i16() {
        println!(
            "{:032b}", Instruction::C_Shr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441615172u32);
        assert_eq!(
            Instruction::C_Shr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441615172u32
        );
    }
    #[test]
    fn decode_C_Shr_i32() {
        assert_eq!(
            Instruction::decode(2710050628u32), Instruction::C_Shr_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_i32() {
        println!(
            "{:032b}", Instruction::C_Shr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050628u32);
        assert_eq!(
            Instruction::C_Shr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050628u32
        );
    }
    #[test]
    fn decode_C_Shr_i64() {
        assert_eq!(
            Instruction::decode(2978486084u32), Instruction::C_Shr_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_i64() {
        println!(
            "{:032b}", Instruction::C_Shr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978486084u32);
        assert_eq!(
            Instruction::C_Shr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978486084u32
        );
    }
    #[test]
    fn decode_C_Shr_u8() {
        assert_eq!(
            Instruction::decode(25696068u32), Instruction::C_Shr_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_u8() {
        println!(
            "{:032b}", Instruction::C_Shr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25696068u32);
        assert_eq!(
            Instruction::C_Shr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25696068u32
        );
    }
    #[test]
    fn decode_C_Shr_u16() {
        assert_eq!(
            Instruction::decode(294131524u32), Instruction::C_Shr_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_u16() {
        println!(
            "{:032b}", Instruction::C_Shr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131524u32);
        assert_eq!(
            Instruction::C_Shr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131524u32
        );
    }
    #[test]
    fn decode_C_Shr_u32() {
        assert_eq!(
            Instruction::decode(562566980u32), Instruction::C_Shr_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_u32() {
        println!(
            "{:032b}", Instruction::C_Shr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566980u32);
        assert_eq!(
            Instruction::C_Shr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566980u32
        );
    }
    #[test]
    fn decode_C_Shr_u64() {
        assert_eq!(
            Instruction::decode(831002436u32), Instruction::C_Shr_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_u64() {
        println!(
            "{:032b}", Instruction::C_Shr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002436u32);
        assert_eq!(
            Instruction::C_Shr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002436u32
        );
    }
    #[test]
    fn decode_C_Sub_i8() {
        assert_eq!(
            Instruction::decode(2173179780u32), Instruction::C_Sub_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_i8() {
        println!(
            "{:032b}", Instruction::C_Sub_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179780u32);
        assert_eq!(
            Instruction::C_Sub_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179780u32
        );
    }
    #[test]
    fn decode_C_Sub_i16() {
        assert_eq!(
            Instruction::decode(2441615236u32), Instruction::C_Sub_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_i16() {
        println!(
            "{:032b}", Instruction::C_Sub_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441615236u32);
        assert_eq!(
            Instruction::C_Sub_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441615236u32
        );
    }
    #[test]
    fn decode_C_Sub_i32() {
        assert_eq!(
            Instruction::decode(2710050692u32), Instruction::C_Sub_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_i32() {
        println!(
            "{:032b}", Instruction::C_Sub_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050692u32);
        assert_eq!(
            Instruction::C_Sub_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050692u32
        );
    }
    #[test]
    fn decode_C_Sub_i64() {
        assert_eq!(
            Instruction::decode(2978486148u32), Instruction::C_Sub_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_i64() {
        println!(
            "{:032b}", Instruction::C_Sub_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978486148u32);
        assert_eq!(
            Instruction::C_Sub_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978486148u32
        );
    }
    #[test]
    fn decode_C_Sub_u8() {
        assert_eq!(
            Instruction::decode(25696132u32), Instruction::C_Sub_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_u8() {
        println!(
            "{:032b}", Instruction::C_Sub_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25696132u32);
        assert_eq!(
            Instruction::C_Sub_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25696132u32
        );
    }
    #[test]
    fn decode_C_Sub_u16() {
        assert_eq!(
            Instruction::decode(294131588u32), Instruction::C_Sub_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_u16() {
        println!(
            "{:032b}", Instruction::C_Sub_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131588u32);
        assert_eq!(
            Instruction::C_Sub_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131588u32
        );
    }
    #[test]
    fn decode_C_Sub_u32() {
        assert_eq!(
            Instruction::decode(562567044u32), Instruction::C_Sub_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_u32() {
        println!(
            "{:032b}", Instruction::C_Sub_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562567044u32);
        assert_eq!(
            Instruction::C_Sub_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562567044u32
        );
    }
    #[test]
    fn decode_C_Sub_u64() {
        assert_eq!(
            Instruction::decode(831002500u32), Instruction::C_Sub_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_u64() {
        println!(
            "{:032b}", Instruction::C_Sub_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002500u32);
        assert_eq!(
            Instruction::C_Sub_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002500u32
        );
    }
    #[test]
    fn decode_C_Sub_U_i8() {
        assert_eq!(
            Instruction::decode(2173179844u32), Instruction::C_Sub_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_U_i8() {
        println!(
            "{:032b}", Instruction::C_Sub_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179844u32);
        assert_eq!(
            Instruction::C_Sub_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179844u32
        );
    }
    #[test]
    fn decode_C_Sub_U_i16() {
        assert_eq!(
            Instruction::decode(2441615300u32), Instruction::C_Sub_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_U_i16() {
        println!(
            "{:032b}", Instruction::C_Sub_U_i16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2441615300u32);
        assert_eq!(
            Instruction::C_Sub_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441615300u32
        );
    }
    #[test]
    fn decode_C_Sub_U_i32() {
        assert_eq!(
            Instruction::decode(2710050756u32), Instruction::C_Sub_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_U_i32() {
        println!(
            "{:032b}", Instruction::C_Sub_U_i32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2710050756u32);
        assert_eq!(
            Instruction::C_Sub_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050756u32
        );
    }
    #[test]
    fn decode_C_Sub_U_i64() {
        assert_eq!(
            Instruction::decode(2978486212u32), Instruction::C_Sub_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_U_i64() {
        println!(
            "{:032b}", Instruction::C_Sub_U_i64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2978486212u32);
        assert_eq!(
            Instruction::C_Sub_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978486212u32
        );
    }
    #[test]
    fn decode_O_Abs_i8() {
        assert_eq!(
            Instruction::decode(2148013061u32), Instruction::O_Abs_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Abs_i8() {
        println!(
            "{:032b}", Instruction::O_Abs_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013061u32);
        assert_eq!(
            Instruction::O_Abs_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013061u32
        );
    }
    #[test]
    fn decode_O_Abs_i16() {
        assert_eq!(
            Instruction::decode(2416448517u32), Instruction::O_Abs_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Abs_i16() {
        println!(
            "{:032b}", Instruction::O_Abs_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416448517u32);
        assert_eq!(
            Instruction::O_Abs_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416448517u32
        );
    }
    #[test]
    fn decode_O_Abs_i32() {
        assert_eq!(
            Instruction::decode(2684883973u32), Instruction::O_Abs_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Abs_i32() {
        println!(
            "{:032b}", Instruction::O_Abs_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684883973u32);
        assert_eq!(
            Instruction::O_Abs_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684883973u32
        );
    }
    #[test]
    fn decode_O_Abs_i64() {
        assert_eq!(
            Instruction::decode(2953319429u32), Instruction::O_Abs_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Abs_i64() {
        println!(
            "{:032b}", Instruction::O_Abs_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953319429u32);
        assert_eq!(
            Instruction::O_Abs_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953319429u32
        );
    }
    #[test]
    fn decode_O_Add_i8() {
        assert_eq!(
            Instruction::decode(2173178949u32), Instruction::O_Add_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_i8() {
        println!(
            "{:032b}", Instruction::O_Add_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173178949u32);
        assert_eq!(
            Instruction::O_Add_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173178949u32
        );
    }
    #[test]
    fn decode_O_Add_i16() {
        assert_eq!(
            Instruction::decode(2441614405u32), Instruction::O_Add_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_i16() {
        println!(
            "{:032b}", Instruction::O_Add_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614405u32);
        assert_eq!(
            Instruction::O_Add_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614405u32
        );
    }
    #[test]
    fn decode_O_Add_i32() {
        assert_eq!(
            Instruction::decode(2710049861u32), Instruction::O_Add_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_i32() {
        println!(
            "{:032b}", Instruction::O_Add_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710049861u32);
        assert_eq!(
            Instruction::O_Add_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710049861u32
        );
    }
    #[test]
    fn decode_O_Add_i64() {
        assert_eq!(
            Instruction::decode(2978485317u32), Instruction::O_Add_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_i64() {
        println!(
            "{:032b}", Instruction::O_Add_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485317u32);
        assert_eq!(
            Instruction::O_Add_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485317u32
        );
    }
    #[test]
    fn decode_O_Add_u8() {
        assert_eq!(
            Instruction::decode(25695301u32), Instruction::O_Add_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_u8() {
        println!(
            "{:032b}", Instruction::O_Add_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695301u32);
        assert_eq!(
            Instruction::O_Add_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695301u32
        );
    }
    #[test]
    fn decode_O_Add_u16() {
        assert_eq!(
            Instruction::decode(294130757u32), Instruction::O_Add_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_u16() {
        println!(
            "{:032b}", Instruction::O_Add_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294130757u32);
        assert_eq!(
            Instruction::O_Add_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130757u32
        );
    }
    #[test]
    fn decode_O_Add_u32() {
        assert_eq!(
            Instruction::decode(562566213u32), Instruction::O_Add_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_u32() {
        println!(
            "{:032b}", Instruction::O_Add_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566213u32);
        assert_eq!(
            Instruction::O_Add_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566213u32
        );
    }
    #[test]
    fn decode_O_Add_u64() {
        assert_eq!(
            Instruction::decode(831001669u32), Instruction::O_Add_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_u64() {
        println!(
            "{:032b}", Instruction::O_Add_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831001669u32);
        assert_eq!(
            Instruction::O_Add_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001669u32
        );
    }
    #[test]
    fn decode_O_Add_U_i8() {
        assert_eq!(
            Instruction::decode(2173179013u32), Instruction::O_Add_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_U_i8() {
        println!(
            "{:032b}", Instruction::O_Add_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179013u32);
        assert_eq!(
            Instruction::O_Add_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179013u32
        );
    }
    #[test]
    fn decode_O_Add_U_i16() {
        assert_eq!(
            Instruction::decode(2441614469u32), Instruction::O_Add_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_U_i16() {
        println!(
            "{:032b}", Instruction::O_Add_U_i16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2441614469u32);
        assert_eq!(
            Instruction::O_Add_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614469u32
        );
    }
    #[test]
    fn decode_O_Add_U_i32() {
        assert_eq!(
            Instruction::decode(2710049925u32), Instruction::O_Add_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_U_i32() {
        println!(
            "{:032b}", Instruction::O_Add_U_i32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2710049925u32);
        assert_eq!(
            Instruction::O_Add_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710049925u32
        );
    }
    #[test]
    fn decode_O_Add_U_i64() {
        assert_eq!(
            Instruction::decode(2978485381u32), Instruction::O_Add_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_U_i64() {
        println!(
            "{:032b}", Instruction::O_Add_U_i64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2978485381u32);
        assert_eq!(
            Instruction::O_Add_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485381u32
        );
    }
    #[test]
    fn decode_O_Add_S_u8() {
        assert_eq!(
            Instruction::decode(25695365u32), Instruction::O_Add_S_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_S_u8() {
        println!(
            "{:032b}", Instruction::O_Add_S_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695365u32);
        assert_eq!(
            Instruction::O_Add_S_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695365u32
        );
    }
    #[test]
    fn decode_O_Add_S_u16() {
        assert_eq!(
            Instruction::decode(294130821u32), Instruction::O_Add_S_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_S_u16() {
        println!(
            "{:032b}", Instruction::O_Add_S_u16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 294130821u32);
        assert_eq!(
            Instruction::O_Add_S_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130821u32
        );
    }
    #[test]
    fn decode_O_Add_S_u32() {
        assert_eq!(
            Instruction::decode(562566277u32), Instruction::O_Add_S_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_S_u32() {
        println!(
            "{:032b}", Instruction::O_Add_S_u32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 562566277u32);
        assert_eq!(
            Instruction::O_Add_S_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566277u32
        );
    }
    #[test]
    fn decode_O_Add_S_u64() {
        assert_eq!(
            Instruction::decode(831001733u32), Instruction::O_Add_S_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_S_u64() {
        println!(
            "{:032b}", Instruction::O_Add_S_u64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 831001733u32);
        assert_eq!(
            Instruction::O_Add_S_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001733u32
        );
    }
    #[test]
    fn decode_O_Div_i8() {
        assert_eq!(
            Instruction::decode(2173179077u32), Instruction::O_Div_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_i8() {
        println!(
            "{:032b}", Instruction::O_Div_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179077u32);
        assert_eq!(
            Instruction::O_Div_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179077u32
        );
    }
    #[test]
    fn decode_O_Div_i16() {
        assert_eq!(
            Instruction::decode(2441614533u32), Instruction::O_Div_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_i16() {
        println!(
            "{:032b}", Instruction::O_Div_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614533u32);
        assert_eq!(
            Instruction::O_Div_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614533u32
        );
    }
    #[test]
    fn decode_O_Div_i32() {
        assert_eq!(
            Instruction::decode(2710049989u32), Instruction::O_Div_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_i32() {
        println!(
            "{:032b}", Instruction::O_Div_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710049989u32);
        assert_eq!(
            Instruction::O_Div_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710049989u32
        );
    }
    #[test]
    fn decode_O_Div_i64() {
        assert_eq!(
            Instruction::decode(2978485445u32), Instruction::O_Div_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_i64() {
        println!(
            "{:032b}", Instruction::O_Div_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485445u32);
        assert_eq!(
            Instruction::O_Div_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485445u32
        );
    }
    #[test]
    fn decode_O_Div_u8() {
        assert_eq!(
            Instruction::decode(25695429u32), Instruction::O_Div_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_u8() {
        println!(
            "{:032b}", Instruction::O_Div_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695429u32);
        assert_eq!(
            Instruction::O_Div_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695429u32
        );
    }
    #[test]
    fn decode_O_Div_u16() {
        assert_eq!(
            Instruction::decode(294130885u32), Instruction::O_Div_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_u16() {
        println!(
            "{:032b}", Instruction::O_Div_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294130885u32);
        assert_eq!(
            Instruction::O_Div_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130885u32
        );
    }
    #[test]
    fn decode_O_Div_u32() {
        assert_eq!(
            Instruction::decode(562566341u32), Instruction::O_Div_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_u32() {
        println!(
            "{:032b}", Instruction::O_Div_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566341u32);
        assert_eq!(
            Instruction::O_Div_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566341u32
        );
    }
    #[test]
    fn decode_O_Div_u64() {
        assert_eq!(
            Instruction::decode(831001797u32), Instruction::O_Div_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_u64() {
        println!(
            "{:032b}", Instruction::O_Div_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831001797u32);
        assert_eq!(
            Instruction::O_Div_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001797u32
        );
    }
    #[test]
    fn decode_O_Div_E_i8() {
        assert_eq!(
            Instruction::decode(2173179141u32), Instruction::O_Div_E_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_i8() {
        println!(
            "{:032b}", Instruction::O_Div_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179141u32);
        assert_eq!(
            Instruction::O_Div_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179141u32
        );
    }
    #[test]
    fn decode_O_Div_E_i16() {
        assert_eq!(
            Instruction::decode(2441614597u32), Instruction::O_Div_E_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_i16() {
        println!(
            "{:032b}", Instruction::O_Div_E_i16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2441614597u32);
        assert_eq!(
            Instruction::O_Div_E_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614597u32
        );
    }
    #[test]
    fn decode_O_Div_E_i32() {
        assert_eq!(
            Instruction::decode(2710050053u32), Instruction::O_Div_E_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_i32() {
        println!(
            "{:032b}", Instruction::O_Div_E_i32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2710050053u32);
        assert_eq!(
            Instruction::O_Div_E_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050053u32
        );
    }
    #[test]
    fn decode_O_Div_E_i64() {
        assert_eq!(
            Instruction::decode(2978485509u32), Instruction::O_Div_E_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_i64() {
        println!(
            "{:032b}", Instruction::O_Div_E_i64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2978485509u32);
        assert_eq!(
            Instruction::O_Div_E_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485509u32
        );
    }
    #[test]
    fn decode_O_Div_E_u8() {
        assert_eq!(
            Instruction::decode(25695493u32), Instruction::O_Div_E_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_u8() {
        println!(
            "{:032b}", Instruction::O_Div_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695493u32);
        assert_eq!(
            Instruction::O_Div_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695493u32
        );
    }
    #[test]
    fn decode_O_Div_E_u16() {
        assert_eq!(
            Instruction::decode(294130949u32), Instruction::O_Div_E_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_u16() {
        println!(
            "{:032b}", Instruction::O_Div_E_u16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 294130949u32);
        assert_eq!(
            Instruction::O_Div_E_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130949u32
        );
    }
    #[test]
    fn decode_O_Div_E_u32() {
        assert_eq!(
            Instruction::decode(562566405u32), Instruction::O_Div_E_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_u32() {
        println!(
            "{:032b}", Instruction::O_Div_E_u32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 562566405u32);
        assert_eq!(
            Instruction::O_Div_E_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566405u32
        );
    }
    #[test]
    fn decode_O_Div_E_u64() {
        assert_eq!(
            Instruction::decode(831001861u32), Instruction::O_Div_E_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_u64() {
        println!(
            "{:032b}", Instruction::O_Div_E_u64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 831001861u32);
        assert_eq!(
            Instruction::O_Div_E_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001861u32
        );
    }
    #[test]
    fn decode_O_Mul_i8() {
        assert_eq!(
            Instruction::decode(2173179333u32), Instruction::O_Mul_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_i8() {
        println!(
            "{:032b}", Instruction::O_Mul_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179333u32);
        assert_eq!(
            Instruction::O_Mul_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179333u32
        );
    }
    #[test]
    fn decode_O_Mul_i16() {
        assert_eq!(
            Instruction::decode(2441614789u32), Instruction::O_Mul_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_i16() {
        println!(
            "{:032b}", Instruction::O_Mul_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614789u32);
        assert_eq!(
            Instruction::O_Mul_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614789u32
        );
    }
    #[test]
    fn decode_O_Mul_i32() {
        assert_eq!(
            Instruction::decode(2710050245u32), Instruction::O_Mul_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_i32() {
        println!(
            "{:032b}", Instruction::O_Mul_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050245u32);
        assert_eq!(
            Instruction::O_Mul_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050245u32
        );
    }
    #[test]
    fn decode_O_Mul_i64() {
        assert_eq!(
            Instruction::decode(2978485701u32), Instruction::O_Mul_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_i64() {
        println!(
            "{:032b}", Instruction::O_Mul_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485701u32);
        assert_eq!(
            Instruction::O_Mul_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485701u32
        );
    }
    #[test]
    fn decode_O_Mul_u8() {
        assert_eq!(
            Instruction::decode(25695685u32), Instruction::O_Mul_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_u8() {
        println!(
            "{:032b}", Instruction::O_Mul_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695685u32);
        assert_eq!(
            Instruction::O_Mul_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695685u32
        );
    }
    #[test]
    fn decode_O_Mul_u16() {
        assert_eq!(
            Instruction::decode(294131141u32), Instruction::O_Mul_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_u16() {
        println!(
            "{:032b}", Instruction::O_Mul_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131141u32);
        assert_eq!(
            Instruction::O_Mul_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131141u32
        );
    }
    #[test]
    fn decode_O_Mul_u32() {
        assert_eq!(
            Instruction::decode(562566597u32), Instruction::O_Mul_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_u32() {
        println!(
            "{:032b}", Instruction::O_Mul_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566597u32);
        assert_eq!(
            Instruction::O_Mul_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566597u32
        );
    }
    #[test]
    fn decode_O_Mul_u64() {
        assert_eq!(
            Instruction::decode(831002053u32), Instruction::O_Mul_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_u64() {
        println!(
            "{:032b}", Instruction::O_Mul_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002053u32);
        assert_eq!(
            Instruction::O_Mul_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002053u32
        );
    }
    #[test]
    fn decode_O_Neg_i8() {
        assert_eq!(
            Instruction::decode(2148013573u32), Instruction::O_Neg_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Neg_i8() {
        println!(
            "{:032b}", Instruction::O_Neg_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013573u32);
        assert_eq!(
            Instruction::O_Neg_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013573u32
        );
    }
    #[test]
    fn decode_O_Neg_i16() {
        assert_eq!(
            Instruction::decode(2416449029u32), Instruction::O_Neg_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Neg_i16() {
        println!(
            "{:032b}", Instruction::O_Neg_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416449029u32);
        assert_eq!(
            Instruction::O_Neg_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416449029u32
        );
    }
    #[test]
    fn decode_O_Neg_i32() {
        assert_eq!(
            Instruction::decode(2684884485u32), Instruction::O_Neg_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Neg_i32() {
        println!(
            "{:032b}", Instruction::O_Neg_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684884485u32);
        assert_eq!(
            Instruction::O_Neg_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684884485u32
        );
    }
    #[test]
    fn decode_O_Neg_i64() {
        assert_eq!(
            Instruction::decode(2953319941u32), Instruction::O_Neg_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Neg_i64() {
        println!(
            "{:032b}", Instruction::O_Neg_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953319941u32);
        assert_eq!(
            Instruction::O_Neg_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953319941u32
        );
    }
    #[test]
    fn decode_O_Pow_i8() {
        assert_eq!(
            Instruction::decode(2173179461u32), Instruction::O_Pow_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_i8() {
        println!(
            "{:032b}", Instruction::O_Pow_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179461u32);
        assert_eq!(
            Instruction::O_Pow_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179461u32
        );
    }
    #[test]
    fn decode_O_Pow_i16() {
        assert_eq!(
            Instruction::decode(2441614917u32), Instruction::O_Pow_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_i16() {
        println!(
            "{:032b}", Instruction::O_Pow_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614917u32);
        assert_eq!(
            Instruction::O_Pow_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614917u32
        );
    }
    #[test]
    fn decode_O_Pow_i32() {
        assert_eq!(
            Instruction::decode(2710050373u32), Instruction::O_Pow_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_i32() {
        println!(
            "{:032b}", Instruction::O_Pow_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050373u32);
        assert_eq!(
            Instruction::O_Pow_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050373u32
        );
    }
    #[test]
    fn decode_O_Pow_i64() {
        assert_eq!(
            Instruction::decode(2978485829u32), Instruction::O_Pow_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_i64() {
        println!(
            "{:032b}", Instruction::O_Pow_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485829u32);
        assert_eq!(
            Instruction::O_Pow_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485829u32
        );
    }
    #[test]
    fn decode_O_Pow_u8() {
        assert_eq!(
            Instruction::decode(25695813u32), Instruction::O_Pow_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_u8() {
        println!(
            "{:032b}", Instruction::O_Pow_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695813u32);
        assert_eq!(
            Instruction::O_Pow_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695813u32
        );
    }
    #[test]
    fn decode_O_Pow_u16() {
        assert_eq!(
            Instruction::decode(294131269u32), Instruction::O_Pow_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_u16() {
        println!(
            "{:032b}", Instruction::O_Pow_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131269u32);
        assert_eq!(
            Instruction::O_Pow_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131269u32
        );
    }
    #[test]
    fn decode_O_Pow_u32() {
        assert_eq!(
            Instruction::decode(562566725u32), Instruction::O_Pow_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_u32() {
        println!(
            "{:032b}", Instruction::O_Pow_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566725u32);
        assert_eq!(
            Instruction::O_Pow_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566725u32
        );
    }
    #[test]
    fn decode_O_Pow_u64() {
        assert_eq!(
            Instruction::decode(831002181u32), Instruction::O_Pow_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_u64() {
        println!(
            "{:032b}", Instruction::O_Pow_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002181u32);
        assert_eq!(
            Instruction::O_Pow_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002181u32
        );
    }
    #[test]
    fn decode_O_Rem_i8() {
        assert_eq!(
            Instruction::decode(2173179525u32), Instruction::O_Rem_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_i8() {
        println!(
            "{:032b}", Instruction::O_Rem_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179525u32);
        assert_eq!(
            Instruction::O_Rem_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179525u32
        );
    }
    #[test]
    fn decode_O_Rem_i16() {
        assert_eq!(
            Instruction::decode(2441614981u32), Instruction::O_Rem_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_i16() {
        println!(
            "{:032b}", Instruction::O_Rem_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614981u32);
        assert_eq!(
            Instruction::O_Rem_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614981u32
        );
    }
    #[test]
    fn decode_O_Rem_i32() {
        assert_eq!(
            Instruction::decode(2710050437u32), Instruction::O_Rem_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_i32() {
        println!(
            "{:032b}", Instruction::O_Rem_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050437u32);
        assert_eq!(
            Instruction::O_Rem_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050437u32
        );
    }
    #[test]
    fn decode_O_Rem_i64() {
        assert_eq!(
            Instruction::decode(2978485893u32), Instruction::O_Rem_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_i64() {
        println!(
            "{:032b}", Instruction::O_Rem_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485893u32);
        assert_eq!(
            Instruction::O_Rem_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485893u32
        );
    }
    #[test]
    fn decode_O_Rem_u8() {
        assert_eq!(
            Instruction::decode(25695877u32), Instruction::O_Rem_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_u8() {
        println!(
            "{:032b}", Instruction::O_Rem_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695877u32);
        assert_eq!(
            Instruction::O_Rem_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695877u32
        );
    }
    #[test]
    fn decode_O_Rem_u16() {
        assert_eq!(
            Instruction::decode(294131333u32), Instruction::O_Rem_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_u16() {
        println!(
            "{:032b}", Instruction::O_Rem_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131333u32);
        assert_eq!(
            Instruction::O_Rem_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131333u32
        );
    }
    #[test]
    fn decode_O_Rem_u32() {
        assert_eq!(
            Instruction::decode(562566789u32), Instruction::O_Rem_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_u32() {
        println!(
            "{:032b}", Instruction::O_Rem_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566789u32);
        assert_eq!(
            Instruction::O_Rem_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566789u32
        );
    }
    #[test]
    fn decode_O_Rem_u64() {
        assert_eq!(
            Instruction::decode(831002245u32), Instruction::O_Rem_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_u64() {
        println!(
            "{:032b}", Instruction::O_Rem_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002245u32);
        assert_eq!(
            Instruction::O_Rem_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002245u32
        );
    }
    #[test]
    fn decode_O_Rem_E_i8() {
        assert_eq!(
            Instruction::decode(2173179589u32), Instruction::O_Rem_E_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_i8() {
        println!(
            "{:032b}", Instruction::O_Rem_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179589u32);
        assert_eq!(
            Instruction::O_Rem_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179589u32
        );
    }
    #[test]
    fn decode_O_Rem_E_i16() {
        assert_eq!(
            Instruction::decode(2441615045u32), Instruction::O_Rem_E_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_i16() {
        println!(
            "{:032b}", Instruction::O_Rem_E_i16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2441615045u32);
        assert_eq!(
            Instruction::O_Rem_E_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441615045u32
        );
    }
    #[test]
    fn decode_O_Rem_E_i32() {
        assert_eq!(
            Instruction::decode(2710050501u32), Instruction::O_Rem_E_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_i32() {
        println!(
            "{:032b}", Instruction::O_Rem_E_i32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2710050501u32);
        assert_eq!(
            Instruction::O_Rem_E_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050501u32
        );
    }
    #[test]
    fn decode_O_Rem_E_i64() {
        assert_eq!(
            Instruction::decode(2978485957u32), Instruction::O_Rem_E_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_i64() {
        println!(
            "{:032b}", Instruction::O_Rem_E_i64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2978485957u32);
        assert_eq!(
            Instruction::O_Rem_E_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485957u32
        );
    }
    #[test]
    fn decode_O_Rem_E_u8() {
        assert_eq!(
            Instruction::decode(25695941u32), Instruction::O_Rem_E_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_u8() {
        println!(
            "{:032b}", Instruction::O_Rem_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695941u32);
        assert_eq!(
            Instruction::O_Rem_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695941u32
        );
    }
    #[test]
    fn decode_O_Rem_E_u16() {
        assert_eq!(
            Instruction::decode(294131397u32), Instruction::O_Rem_E_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_u16() {
        println!(
            "{:032b}", Instruction::O_Rem_E_u16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 294131397u32);
        assert_eq!(
            Instruction::O_Rem_E_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131397u32
        );
    }
    #[test]
    fn decode_O_Rem_E_u32() {
        assert_eq!(
            Instruction::decode(562566853u32), Instruction::O_Rem_E_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_u32() {
        println!(
            "{:032b}", Instruction::O_Rem_E_u32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 562566853u32);
        assert_eq!(
            Instruction::O_Rem_E_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566853u32
        );
    }
    #[test]
    fn decode_O_Rem_E_u64() {
        assert_eq!(
            Instruction::decode(831002309u32), Instruction::O_Rem_E_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_u64() {
        println!(
            "{:032b}", Instruction::O_Rem_E_u64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 831002309u32);
        assert_eq!(
            Instruction::O_Rem_E_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002309u32
        );
    }
    #[test]
    fn decode_O_Shl_i8() {
        assert_eq!(
            Instruction::decode(2173179653u32), Instruction::O_Shl_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_i8() {
        println!(
            "{:032b}", Instruction::O_Shl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179653u32);
        assert_eq!(
            Instruction::O_Shl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179653u32
        );
    }
    #[test]
    fn decode_O_Shl_i16() {
        assert_eq!(
            Instruction::decode(2441615109u32), Instruction::O_Shl_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_i16() {
        println!(
            "{:032b}", Instruction::O_Shl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441615109u32);
        assert_eq!(
            Instruction::O_Shl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441615109u32
        );
    }
    #[test]
    fn decode_O_Shl_i32() {
        assert_eq!(
            Instruction::decode(2710050565u32), Instruction::O_Shl_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_i32() {
        println!(
            "{:032b}", Instruction::O_Shl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050565u32);
        assert_eq!(
            Instruction::O_Shl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050565u32
        );
    }
    #[test]
    fn decode_O_Shl_i64() {
        assert_eq!(
            Instruction::decode(2978486021u32), Instruction::O_Shl_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_i64() {
        println!(
            "{:032b}", Instruction::O_Shl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978486021u32);
        assert_eq!(
            Instruction::O_Shl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978486021u32
        );
    }
    #[test]
    fn decode_O_Shl_u8() {
        assert_eq!(
            Instruction::decode(25696005u32), Instruction::O_Shl_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_u8() {
        println!(
            "{:032b}", Instruction::O_Shl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25696005u32);
        assert_eq!(
            Instruction::O_Shl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25696005u32
        );
    }
    #[test]
    fn decode_O_Shl_u16() {
        assert_eq!(
            Instruction::decode(294131461u32), Instruction::O_Shl_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_u16() {
        println!(
            "{:032b}", Instruction::O_Shl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131461u32);
        assert_eq!(
            Instruction::O_Shl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131461u32
        );
    }
    #[test]
    fn decode_O_Shl_u32() {
        assert_eq!(
            Instruction::decode(562566917u32), Instruction::O_Shl_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_u32() {
        println!(
            "{:032b}", Instruction::O_Shl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566917u32);
        assert_eq!(
            Instruction::O_Shl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566917u32
        );
    }
    #[test]
    fn decode_O_Shl_u64() {
        assert_eq!(
            Instruction::decode(831002373u32), Instruction::O_Shl_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_u64() {
        println!(
            "{:032b}", Instruction::O_Shl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002373u32);
        assert_eq!(
            Instruction::O_Shl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002373u32
        );
    }
    #[test]
    fn decode_O_Shr_i8() {
        assert_eq!(
            Instruction::decode(2173179717u32), Instruction::O_Shr_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_i8() {
        println!(
            "{:032b}", Instruction::O_Shr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179717u32);
        assert_eq!(
            Instruction::O_Shr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179717u32
        );
    }
    #[test]
    fn decode_O_Shr_i16() {
        assert_eq!(
            Instruction::decode(2441615173u32), Instruction::O_Shr_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_i16() {
        println!(
            "{:032b}", Instruction::O_Shr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441615173u32);
        assert_eq!(
            Instruction::O_Shr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441615173u32
        );
    }
    #[test]
    fn decode_O_Shr_i32() {
        assert_eq!(
            Instruction::decode(2710050629u32), Instruction::O_Shr_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_i32() {
        println!(
            "{:032b}", Instruction::O_Shr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050629u32);
        assert_eq!(
            Instruction::O_Shr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050629u32
        );
    }
    #[test]
    fn decode_O_Shr_i64() {
        assert_eq!(
            Instruction::decode(2978486085u32), Instruction::O_Shr_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_i64() {
        println!(
            "{:032b}", Instruction::O_Shr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978486085u32);
        assert_eq!(
            Instruction::O_Shr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978486085u32
        );
    }
    #[test]
    fn decode_O_Shr_u8() {
        assert_eq!(
            Instruction::decode(25696069u32), Instruction::O_Shr_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_u8() {
        println!(
            "{:032b}", Instruction::O_Shr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25696069u32);
        assert_eq!(
            Instruction::O_Shr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25696069u32
        );
    }
    #[test]
    fn decode_O_Shr_u16() {
        assert_eq!(
            Instruction::decode(294131525u32), Instruction::O_Shr_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_u16() {
        println!(
            "{:032b}", Instruction::O_Shr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131525u32);
        assert_eq!(
            Instruction::O_Shr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131525u32
        );
    }
    #[test]
    fn decode_O_Shr_u32() {
        assert_eq!(
            Instruction::decode(562566981u32), Instruction::O_Shr_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_u32() {
        println!(
            "{:032b}", Instruction::O_Shr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566981u32);
        assert_eq!(
            Instruction::O_Shr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566981u32
        );
    }
    #[test]
    fn decode_O_Shr_u64() {
        assert_eq!(
            Instruction::decode(831002437u32), Instruction::O_Shr_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_u64() {
        println!(
            "{:032b}", Instruction::O_Shr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002437u32);
        assert_eq!(
            Instruction::O_Shr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002437u32
        );
    }
    #[test]
    fn decode_O_Sub_i8() {
        assert_eq!(
            Instruction::decode(2173179781u32), Instruction::O_Sub_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_i8() {
        println!(
            "{:032b}", Instruction::O_Sub_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179781u32);
        assert_eq!(
            Instruction::O_Sub_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179781u32
        );
    }
    #[test]
    fn decode_O_Sub_i16() {
        assert_eq!(
            Instruction::decode(2441615237u32), Instruction::O_Sub_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_i16() {
        println!(
            "{:032b}", Instruction::O_Sub_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441615237u32);
        assert_eq!(
            Instruction::O_Sub_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441615237u32
        );
    }
    #[test]
    fn decode_O_Sub_i32() {
        assert_eq!(
            Instruction::decode(2710050693u32), Instruction::O_Sub_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_i32() {
        println!(
            "{:032b}", Instruction::O_Sub_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050693u32);
        assert_eq!(
            Instruction::O_Sub_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050693u32
        );
    }
    #[test]
    fn decode_O_Sub_i64() {
        assert_eq!(
            Instruction::decode(2978486149u32), Instruction::O_Sub_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_i64() {
        println!(
            "{:032b}", Instruction::O_Sub_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978486149u32);
        assert_eq!(
            Instruction::O_Sub_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978486149u32
        );
    }
    #[test]
    fn decode_O_Sub_u8() {
        assert_eq!(
            Instruction::decode(25696133u32), Instruction::O_Sub_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_u8() {
        println!(
            "{:032b}", Instruction::O_Sub_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25696133u32);
        assert_eq!(
            Instruction::O_Sub_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25696133u32
        );
    }
    #[test]
    fn decode_O_Sub_u16() {
        assert_eq!(
            Instruction::decode(294131589u32), Instruction::O_Sub_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_u16() {
        println!(
            "{:032b}", Instruction::O_Sub_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131589u32);
        assert_eq!(
            Instruction::O_Sub_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131589u32
        );
    }
    #[test]
    fn decode_O_Sub_u32() {
        assert_eq!(
            Instruction::decode(562567045u32), Instruction::O_Sub_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_u32() {
        println!(
            "{:032b}", Instruction::O_Sub_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562567045u32);
        assert_eq!(
            Instruction::O_Sub_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562567045u32
        );
    }
    #[test]
    fn decode_O_Sub_u64() {
        assert_eq!(
            Instruction::decode(831002501u32), Instruction::O_Sub_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_u64() {
        println!(
            "{:032b}", Instruction::O_Sub_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002501u32);
        assert_eq!(
            Instruction::O_Sub_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002501u32
        );
    }
    #[test]
    fn decode_O_Sub_U_i8() {
        assert_eq!(
            Instruction::decode(2173179845u32), Instruction::O_Sub_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_U_i8() {
        println!(
            "{:032b}", Instruction::O_Sub_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179845u32);
        assert_eq!(
            Instruction::O_Sub_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179845u32
        );
    }
    #[test]
    fn decode_O_Sub_U_i16() {
        assert_eq!(
            Instruction::decode(2441615301u32), Instruction::O_Sub_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_U_i16() {
        println!(
            "{:032b}", Instruction::O_Sub_U_i16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2441615301u32);
        assert_eq!(
            Instruction::O_Sub_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441615301u32
        );
    }
    #[test]
    fn decode_O_Sub_U_i32() {
        assert_eq!(
            Instruction::decode(2710050757u32), Instruction::O_Sub_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_U_i32() {
        println!(
            "{:032b}", Instruction::O_Sub_U_i32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2710050757u32);
        assert_eq!(
            Instruction::O_Sub_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050757u32
        );
    }
    #[test]
    fn decode_O_Sub_U_i64() {
        assert_eq!(
            Instruction::decode(2978486213u32), Instruction::O_Sub_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_U_i64() {
        println!(
            "{:032b}", Instruction::O_Sub_U_i64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2978486213u32);
        assert_eq!(
            Instruction::O_Sub_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978486213u32
        );
    }
    #[test]
    fn decode_S_Abs_i8() {
        assert_eq!(
            Instruction::decode(2148013062u32), Instruction::S_Abs_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Abs_i8() {
        println!(
            "{:032b}", Instruction::S_Abs_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013062u32);
        assert_eq!(
            Instruction::S_Abs_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013062u32
        );
    }
    #[test]
    fn decode_S_Abs_i16() {
        assert_eq!(
            Instruction::decode(2416448518u32), Instruction::S_Abs_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Abs_i16() {
        println!(
            "{:032b}", Instruction::S_Abs_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416448518u32);
        assert_eq!(
            Instruction::S_Abs_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416448518u32
        );
    }
    #[test]
    fn decode_S_Abs_i32() {
        assert_eq!(
            Instruction::decode(2684883974u32), Instruction::S_Abs_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Abs_i32() {
        println!(
            "{:032b}", Instruction::S_Abs_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684883974u32);
        assert_eq!(
            Instruction::S_Abs_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684883974u32
        );
    }
    #[test]
    fn decode_S_Abs_i64() {
        assert_eq!(
            Instruction::decode(2953319430u32), Instruction::S_Abs_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Abs_i64() {
        println!(
            "{:032b}", Instruction::S_Abs_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953319430u32);
        assert_eq!(
            Instruction::S_Abs_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953319430u32
        );
    }
    #[test]
    fn decode_S_Add_i8() {
        assert_eq!(
            Instruction::decode(2173178950u32), Instruction::S_Add_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_i8() {
        println!(
            "{:032b}", Instruction::S_Add_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173178950u32);
        assert_eq!(
            Instruction::S_Add_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173178950u32
        );
    }
    #[test]
    fn decode_S_Add_i16() {
        assert_eq!(
            Instruction::decode(2441614406u32), Instruction::S_Add_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_i16() {
        println!(
            "{:032b}", Instruction::S_Add_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614406u32);
        assert_eq!(
            Instruction::S_Add_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614406u32
        );
    }
    #[test]
    fn decode_S_Add_i32() {
        assert_eq!(
            Instruction::decode(2710049862u32), Instruction::S_Add_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_i32() {
        println!(
            "{:032b}", Instruction::S_Add_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710049862u32);
        assert_eq!(
            Instruction::S_Add_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710049862u32
        );
    }
    #[test]
    fn decode_S_Add_i64() {
        assert_eq!(
            Instruction::decode(2978485318u32), Instruction::S_Add_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_i64() {
        println!(
            "{:032b}", Instruction::S_Add_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485318u32);
        assert_eq!(
            Instruction::S_Add_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485318u32
        );
    }
    #[test]
    fn decode_S_Add_u8() {
        assert_eq!(
            Instruction::decode(25695302u32), Instruction::S_Add_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_u8() {
        println!(
            "{:032b}", Instruction::S_Add_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695302u32);
        assert_eq!(
            Instruction::S_Add_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695302u32
        );
    }
    #[test]
    fn decode_S_Add_u16() {
        assert_eq!(
            Instruction::decode(294130758u32), Instruction::S_Add_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_u16() {
        println!(
            "{:032b}", Instruction::S_Add_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294130758u32);
        assert_eq!(
            Instruction::S_Add_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130758u32
        );
    }
    #[test]
    fn decode_S_Add_u32() {
        assert_eq!(
            Instruction::decode(562566214u32), Instruction::S_Add_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_u32() {
        println!(
            "{:032b}", Instruction::S_Add_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566214u32);
        assert_eq!(
            Instruction::S_Add_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566214u32
        );
    }
    #[test]
    fn decode_S_Add_u64() {
        assert_eq!(
            Instruction::decode(831001670u32), Instruction::S_Add_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_u64() {
        println!(
            "{:032b}", Instruction::S_Add_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831001670u32);
        assert_eq!(
            Instruction::S_Add_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001670u32
        );
    }
    #[test]
    fn decode_S_Add_U_i8() {
        assert_eq!(
            Instruction::decode(2173179014u32), Instruction::S_Add_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_U_i8() {
        println!(
            "{:032b}", Instruction::S_Add_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179014u32);
        assert_eq!(
            Instruction::S_Add_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179014u32
        );
    }
    #[test]
    fn decode_S_Add_U_i16() {
        assert_eq!(
            Instruction::decode(2441614470u32), Instruction::S_Add_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_U_i16() {
        println!(
            "{:032b}", Instruction::S_Add_U_i16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2441614470u32);
        assert_eq!(
            Instruction::S_Add_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614470u32
        );
    }
    #[test]
    fn decode_S_Add_U_i32() {
        assert_eq!(
            Instruction::decode(2710049926u32), Instruction::S_Add_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_U_i32() {
        println!(
            "{:032b}", Instruction::S_Add_U_i32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2710049926u32);
        assert_eq!(
            Instruction::S_Add_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710049926u32
        );
    }
    #[test]
    fn decode_S_Add_U_i64() {
        assert_eq!(
            Instruction::decode(2978485382u32), Instruction::S_Add_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_U_i64() {
        println!(
            "{:032b}", Instruction::S_Add_U_i64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2978485382u32);
        assert_eq!(
            Instruction::S_Add_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485382u32
        );
    }
    #[test]
    fn decode_S_Add_S_u8() {
        assert_eq!(
            Instruction::decode(25695366u32), Instruction::S_Add_S_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_S_u8() {
        println!(
            "{:032b}", Instruction::S_Add_S_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695366u32);
        assert_eq!(
            Instruction::S_Add_S_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695366u32
        );
    }
    #[test]
    fn decode_S_Add_S_u16() {
        assert_eq!(
            Instruction::decode(294130822u32), Instruction::S_Add_S_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_S_u16() {
        println!(
            "{:032b}", Instruction::S_Add_S_u16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 294130822u32);
        assert_eq!(
            Instruction::S_Add_S_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130822u32
        );
    }
    #[test]
    fn decode_S_Add_S_u32() {
        assert_eq!(
            Instruction::decode(562566278u32), Instruction::S_Add_S_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_S_u32() {
        println!(
            "{:032b}", Instruction::S_Add_S_u32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 562566278u32);
        assert_eq!(
            Instruction::S_Add_S_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566278u32
        );
    }
    #[test]
    fn decode_S_Add_S_u64() {
        assert_eq!(
            Instruction::decode(831001734u32), Instruction::S_Add_S_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_S_u64() {
        println!(
            "{:032b}", Instruction::S_Add_S_u64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 831001734u32);
        assert_eq!(
            Instruction::S_Add_S_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001734u32
        );
    }
    #[test]
    fn decode_S_Div_i8() {
        assert_eq!(
            Instruction::decode(2173179078u32), Instruction::S_Div_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_i8() {
        println!(
            "{:032b}", Instruction::S_Div_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179078u32);
        assert_eq!(
            Instruction::S_Div_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179078u32
        );
    }
    #[test]
    fn decode_S_Div_i16() {
        assert_eq!(
            Instruction::decode(2441614534u32), Instruction::S_Div_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_i16() {
        println!(
            "{:032b}", Instruction::S_Div_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614534u32);
        assert_eq!(
            Instruction::S_Div_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614534u32
        );
    }
    #[test]
    fn decode_S_Div_i32() {
        assert_eq!(
            Instruction::decode(2710049990u32), Instruction::S_Div_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_i32() {
        println!(
            "{:032b}", Instruction::S_Div_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710049990u32);
        assert_eq!(
            Instruction::S_Div_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710049990u32
        );
    }
    #[test]
    fn decode_S_Div_i64() {
        assert_eq!(
            Instruction::decode(2978485446u32), Instruction::S_Div_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_i64() {
        println!(
            "{:032b}", Instruction::S_Div_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485446u32);
        assert_eq!(
            Instruction::S_Div_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485446u32
        );
    }
    #[test]
    fn decode_S_Div_u8() {
        assert_eq!(
            Instruction::decode(25695430u32), Instruction::S_Div_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_u8() {
        println!(
            "{:032b}", Instruction::S_Div_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695430u32);
        assert_eq!(
            Instruction::S_Div_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695430u32
        );
    }
    #[test]
    fn decode_S_Div_u16() {
        assert_eq!(
            Instruction::decode(294130886u32), Instruction::S_Div_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_u16() {
        println!(
            "{:032b}", Instruction::S_Div_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294130886u32);
        assert_eq!(
            Instruction::S_Div_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294130886u32
        );
    }
    #[test]
    fn decode_S_Div_u32() {
        assert_eq!(
            Instruction::decode(562566342u32), Instruction::S_Div_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_u32() {
        println!(
            "{:032b}", Instruction::S_Div_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566342u32);
        assert_eq!(
            Instruction::S_Div_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566342u32
        );
    }
    #[test]
    fn decode_S_Div_u64() {
        assert_eq!(
            Instruction::decode(831001798u32), Instruction::S_Div_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_u64() {
        println!(
            "{:032b}", Instruction::S_Div_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831001798u32);
        assert_eq!(
            Instruction::S_Div_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831001798u32
        );
    }
    #[test]
    fn decode_S_Mul_i8() {
        assert_eq!(
            Instruction::decode(2173179334u32), Instruction::S_Mul_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_i8() {
        println!(
            "{:032b}", Instruction::S_Mul_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179334u32);
        assert_eq!(
            Instruction::S_Mul_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179334u32
        );
    }
    #[test]
    fn decode_S_Mul_i16() {
        assert_eq!(
            Instruction::decode(2441614790u32), Instruction::S_Mul_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_i16() {
        println!(
            "{:032b}", Instruction::S_Mul_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614790u32);
        assert_eq!(
            Instruction::S_Mul_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614790u32
        );
    }
    #[test]
    fn decode_S_Mul_i32() {
        assert_eq!(
            Instruction::decode(2710050246u32), Instruction::S_Mul_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_i32() {
        println!(
            "{:032b}", Instruction::S_Mul_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050246u32);
        assert_eq!(
            Instruction::S_Mul_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050246u32
        );
    }
    #[test]
    fn decode_S_Mul_i64() {
        assert_eq!(
            Instruction::decode(2978485702u32), Instruction::S_Mul_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_i64() {
        println!(
            "{:032b}", Instruction::S_Mul_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485702u32);
        assert_eq!(
            Instruction::S_Mul_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485702u32
        );
    }
    #[test]
    fn decode_S_Mul_u8() {
        assert_eq!(
            Instruction::decode(25695686u32), Instruction::S_Mul_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_u8() {
        println!(
            "{:032b}", Instruction::S_Mul_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695686u32);
        assert_eq!(
            Instruction::S_Mul_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695686u32
        );
    }
    #[test]
    fn decode_S_Mul_u16() {
        assert_eq!(
            Instruction::decode(294131142u32), Instruction::S_Mul_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_u16() {
        println!(
            "{:032b}", Instruction::S_Mul_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131142u32);
        assert_eq!(
            Instruction::S_Mul_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131142u32
        );
    }
    #[test]
    fn decode_S_Mul_u32() {
        assert_eq!(
            Instruction::decode(562566598u32), Instruction::S_Mul_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_u32() {
        println!(
            "{:032b}", Instruction::S_Mul_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566598u32);
        assert_eq!(
            Instruction::S_Mul_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566598u32
        );
    }
    #[test]
    fn decode_S_Mul_u64() {
        assert_eq!(
            Instruction::decode(831002054u32), Instruction::S_Mul_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_u64() {
        println!(
            "{:032b}", Instruction::S_Mul_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002054u32);
        assert_eq!(
            Instruction::S_Mul_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002054u32
        );
    }
    #[test]
    fn decode_S_Neg_i8() {
        assert_eq!(
            Instruction::decode(2148013574u32), Instruction::S_Neg_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Neg_i8() {
        println!(
            "{:032b}", Instruction::S_Neg_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2148013574u32);
        assert_eq!(
            Instruction::S_Neg_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148013574u32
        );
    }
    #[test]
    fn decode_S_Neg_i16() {
        assert_eq!(
            Instruction::decode(2416449030u32), Instruction::S_Neg_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Neg_i16() {
        println!(
            "{:032b}", Instruction::S_Neg_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2416449030u32);
        assert_eq!(
            Instruction::S_Neg_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416449030u32
        );
    }
    #[test]
    fn decode_S_Neg_i32() {
        assert_eq!(
            Instruction::decode(2684884486u32), Instruction::S_Neg_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Neg_i32() {
        println!(
            "{:032b}", Instruction::S_Neg_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2684884486u32);
        assert_eq!(
            Instruction::S_Neg_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2684884486u32
        );
    }
    #[test]
    fn decode_S_Neg_i64() {
        assert_eq!(
            Instruction::decode(2953319942u32), Instruction::S_Neg_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Neg_i64() {
        println!(
            "{:032b}", Instruction::S_Neg_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 2953319942u32);
        assert_eq!(
            Instruction::S_Neg_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953319942u32
        );
    }
    #[test]
    fn decode_S_Pow_i8() {
        assert_eq!(
            Instruction::decode(2173179462u32), Instruction::S_Pow_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_i8() {
        println!(
            "{:032b}", Instruction::S_Pow_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179462u32);
        assert_eq!(
            Instruction::S_Pow_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179462u32
        );
    }
    #[test]
    fn decode_S_Pow_i16() {
        assert_eq!(
            Instruction::decode(2441614918u32), Instruction::S_Pow_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_i16() {
        println!(
            "{:032b}", Instruction::S_Pow_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441614918u32);
        assert_eq!(
            Instruction::S_Pow_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441614918u32
        );
    }
    #[test]
    fn decode_S_Pow_i32() {
        assert_eq!(
            Instruction::decode(2710050374u32), Instruction::S_Pow_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_i32() {
        println!(
            "{:032b}", Instruction::S_Pow_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050374u32);
        assert_eq!(
            Instruction::S_Pow_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050374u32
        );
    }
    #[test]
    fn decode_S_Pow_i64() {
        assert_eq!(
            Instruction::decode(2978485830u32), Instruction::S_Pow_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_i64() {
        println!(
            "{:032b}", Instruction::S_Pow_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978485830u32);
        assert_eq!(
            Instruction::S_Pow_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978485830u32
        );
    }
    #[test]
    fn decode_S_Pow_u8() {
        assert_eq!(
            Instruction::decode(25695814u32), Instruction::S_Pow_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_u8() {
        println!(
            "{:032b}", Instruction::S_Pow_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25695814u32);
        assert_eq!(
            Instruction::S_Pow_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25695814u32
        );
    }
    #[test]
    fn decode_S_Pow_u16() {
        assert_eq!(
            Instruction::decode(294131270u32), Instruction::S_Pow_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_u16() {
        println!(
            "{:032b}", Instruction::S_Pow_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131270u32);
        assert_eq!(
            Instruction::S_Pow_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131270u32
        );
    }
    #[test]
    fn decode_S_Pow_u32() {
        assert_eq!(
            Instruction::decode(562566726u32), Instruction::S_Pow_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_u32() {
        println!(
            "{:032b}", Instruction::S_Pow_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562566726u32);
        assert_eq!(
            Instruction::S_Pow_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562566726u32
        );
    }
    #[test]
    fn decode_S_Pow_u64() {
        assert_eq!(
            Instruction::decode(831002182u32), Instruction::S_Pow_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_u64() {
        println!(
            "{:032b}", Instruction::S_Pow_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002182u32);
        assert_eq!(
            Instruction::S_Pow_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002182u32
        );
    }
    #[test]
    fn decode_S_Sub_i8() {
        assert_eq!(
            Instruction::decode(2173179782u32), Instruction::S_Sub_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_i8() {
        println!(
            "{:032b}", Instruction::S_Sub_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179782u32);
        assert_eq!(
            Instruction::S_Sub_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179782u32
        );
    }
    #[test]
    fn decode_S_Sub_i16() {
        assert_eq!(
            Instruction::decode(2441615238u32), Instruction::S_Sub_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_i16() {
        println!(
            "{:032b}", Instruction::S_Sub_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2441615238u32);
        assert_eq!(
            Instruction::S_Sub_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441615238u32
        );
    }
    #[test]
    fn decode_S_Sub_i32() {
        assert_eq!(
            Instruction::decode(2710050694u32), Instruction::S_Sub_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_i32() {
        println!(
            "{:032b}", Instruction::S_Sub_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2710050694u32);
        assert_eq!(
            Instruction::S_Sub_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050694u32
        );
    }
    #[test]
    fn decode_S_Sub_i64() {
        assert_eq!(
            Instruction::decode(2978486150u32), Instruction::S_Sub_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_i64() {
        println!(
            "{:032b}", Instruction::S_Sub_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2978486150u32);
        assert_eq!(
            Instruction::S_Sub_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978486150u32
        );
    }
    #[test]
    fn decode_S_Sub_u8() {
        assert_eq!(
            Instruction::decode(25696134u32), Instruction::S_Sub_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_u8() {
        println!(
            "{:032b}", Instruction::S_Sub_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 25696134u32);
        assert_eq!(
            Instruction::S_Sub_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 25696134u32
        );
    }
    #[test]
    fn decode_S_Sub_u16() {
        assert_eq!(
            Instruction::decode(294131590u32), Instruction::S_Sub_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_u16() {
        println!(
            "{:032b}", Instruction::S_Sub_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 294131590u32);
        assert_eq!(
            Instruction::S_Sub_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 294131590u32
        );
    }
    #[test]
    fn decode_S_Sub_u32() {
        assert_eq!(
            Instruction::decode(562567046u32), Instruction::S_Sub_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_u32() {
        println!(
            "{:032b}", Instruction::S_Sub_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 562567046u32);
        assert_eq!(
            Instruction::S_Sub_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 562567046u32
        );
    }
    #[test]
    fn decode_S_Sub_u64() {
        assert_eq!(
            Instruction::decode(831002502u32), Instruction::S_Sub_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_u64() {
        println!(
            "{:032b}", Instruction::S_Sub_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 831002502u32);
        assert_eq!(
            Instruction::S_Sub_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 831002502u32
        );
    }
    #[test]
    fn decode_S_Sub_U_i8() {
        assert_eq!(
            Instruction::decode(2173179846u32), Instruction::S_Sub_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_U_i8() {
        println!(
            "{:032b}", Instruction::S_Sub_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 2173179846u32);
        assert_eq!(
            Instruction::S_Sub_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2173179846u32
        );
    }
    #[test]
    fn decode_S_Sub_U_i16() {
        assert_eq!(
            Instruction::decode(2441615302u32), Instruction::S_Sub_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_U_i16() {
        println!(
            "{:032b}", Instruction::S_Sub_U_i16 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2441615302u32);
        assert_eq!(
            Instruction::S_Sub_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2441615302u32
        );
    }
    #[test]
    fn decode_S_Sub_U_i32() {
        assert_eq!(
            Instruction::decode(2710050758u32), Instruction::S_Sub_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_U_i32() {
        println!(
            "{:032b}", Instruction::S_Sub_U_i32 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2710050758u32);
        assert_eq!(
            Instruction::S_Sub_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2710050758u32
        );
    }
    #[test]
    fn decode_S_Sub_U_i64() {
        assert_eq!(
            Instruction::decode(2978486214u32), Instruction::S_Sub_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_U_i64() {
        println!(
            "{:032b}", Instruction::S_Sub_U_i64 { rd : Register::General_Purpose(5), rs1
            : Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode()
        );
        println!("{:032b}", 2978486214u32);
        assert_eq!(
            Instruction::S_Sub_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2978486214u32
        );
    }
    #[test]
    fn decode_Abs_f32() {
        assert_eq!(
            Instruction::decode(3758625799u32), Instruction::Abs_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Abs_f32() {
        println!(
            "{:032b}", Instruction::Abs_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 3758625799u32);
        assert_eq!(
            Instruction::Abs_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 3758625799u32
        );
    }
    #[test]
    fn decode_Abs_f64() {
        assert_eq!(
            Instruction::decode(4027061255u32), Instruction::Abs_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Abs_f64() {
        println!(
            "{:032b}", Instruction::Abs_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 4027061255u32);
        assert_eq!(
            Instruction::Abs_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 4027061255u32
        );
    }
    #[test]
    fn decode_Add_f32() {
        assert_eq!(
            Instruction::decode(3783791687u32), Instruction::Add_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Add_f32() {
        println!(
            "{:032b}", Instruction::Add_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 3783791687u32);
        assert_eq!(
            Instruction::Add_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3783791687u32
        );
    }
    #[test]
    fn decode_Add_f64() {
        assert_eq!(
            Instruction::decode(4052227143u32), Instruction::Add_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Add_f64() {
        println!(
            "{:032b}", Instruction::Add_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 4052227143u32);
        assert_eq!(
            Instruction::Add_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4052227143u32
        );
    }
    #[test]
    fn decode_Div_f32() {
        assert_eq!(
            Instruction::decode(3783791815u32), Instruction::Div_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Div_f32() {
        println!(
            "{:032b}", Instruction::Div_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 3783791815u32);
        assert_eq!(
            Instruction::Div_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3783791815u32
        );
    }
    #[test]
    fn decode_Div_f64() {
        assert_eq!(
            Instruction::decode(4052227271u32), Instruction::Div_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Div_f64() {
        println!(
            "{:032b}", Instruction::Div_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 4052227271u32);
        assert_eq!(
            Instruction::Div_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4052227271u32
        );
    }
    #[test]
    fn decode_Div_E_f32() {
        assert_eq!(
            Instruction::decode(3783791879u32), Instruction::Div_E_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Div_E_f32() {
        println!(
            "{:032b}", Instruction::Div_E_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 3783791879u32);
        assert_eq!(
            Instruction::Div_E_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3783791879u32
        );
    }
    #[test]
    fn decode_Div_E_f64() {
        assert_eq!(
            Instruction::decode(4052227335u32), Instruction::Div_E_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Div_E_f64() {
        println!(
            "{:032b}", Instruction::Div_E_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 4052227335u32);
        assert_eq!(
            Instruction::Div_E_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4052227335u32
        );
    }
    #[test]
    fn decode_Log_f32() {
        assert_eq!(
            Instruction::decode(3783791943u32), Instruction::Log_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Log_f32() {
        println!(
            "{:032b}", Instruction::Log_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 3783791943u32);
        assert_eq!(
            Instruction::Log_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3783791943u32
        );
    }
    #[test]
    fn decode_Log_f64() {
        assert_eq!(
            Instruction::decode(4052227399u32), Instruction::Log_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Log_f64() {
        println!(
            "{:032b}", Instruction::Log_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 4052227399u32);
        assert_eq!(
            Instruction::Log_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4052227399u32
        );
    }
    #[test]
    fn decode_Sqrt_f32() {
        assert_eq!(
            Instruction::decode(3758626183u32), Instruction::Sqrt_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Sqrt_f32() {
        println!(
            "{:032b}", Instruction::Sqrt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 3758626183u32);
        assert_eq!(
            Instruction::Sqrt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 3758626183u32
        );
    }
    #[test]
    fn decode_Sqrt_f64() {
        assert_eq!(
            Instruction::decode(4027061639u32), Instruction::Sqrt_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Sqrt_f64() {
        println!(
            "{:032b}", Instruction::Sqrt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 4027061639u32);
        assert_eq!(
            Instruction::Sqrt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 4027061639u32
        );
    }
    #[test]
    fn decode_Mul_f32() {
        assert_eq!(
            Instruction::decode(3783792071u32), Instruction::Mul_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Mul_f32() {
        println!(
            "{:032b}", Instruction::Mul_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 3783792071u32);
        assert_eq!(
            Instruction::Mul_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3783792071u32
        );
    }
    #[test]
    fn decode_Mul_f64() {
        assert_eq!(
            Instruction::decode(4052227527u32), Instruction::Mul_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Mul_f64() {
        println!(
            "{:032b}", Instruction::Mul_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 4052227527u32);
        assert_eq!(
            Instruction::Mul_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4052227527u32
        );
    }
    #[test]
    fn decode_Neg_f32() {
        assert_eq!(
            Instruction::decode(3758626311u32), Instruction::Neg_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Neg_f32() {
        println!(
            "{:032b}", Instruction::Neg_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 3758626311u32);
        assert_eq!(
            Instruction::Neg_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 3758626311u32
        );
    }
    #[test]
    fn decode_Neg_f64() {
        assert_eq!(
            Instruction::decode(4027061767u32), Instruction::Neg_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Neg_f64() {
        println!(
            "{:032b}", Instruction::Neg_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 4027061767u32);
        assert_eq!(
            Instruction::Neg_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 4027061767u32
        );
    }
    #[test]
    fn decode_Pow_f32() {
        assert_eq!(
            Instruction::decode(3783792199u32), Instruction::Pow_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Pow_f32() {
        println!(
            "{:032b}", Instruction::Pow_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 3783792199u32);
        assert_eq!(
            Instruction::Pow_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3783792199u32
        );
    }
    #[test]
    fn decode_Pow_f64() {
        assert_eq!(
            Instruction::decode(4052227655u32), Instruction::Pow_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Pow_f64() {
        println!(
            "{:032b}", Instruction::Pow_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 4052227655u32);
        assert_eq!(
            Instruction::Pow_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4052227655u32
        );
    }
    #[test]
    fn decode_Rem_f32() {
        assert_eq!(
            Instruction::decode(3783792263u32), Instruction::Rem_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rem_f32() {
        println!(
            "{:032b}", Instruction::Rem_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 3783792263u32);
        assert_eq!(
            Instruction::Rem_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3783792263u32
        );
    }
    #[test]
    fn decode_Rem_f64() {
        assert_eq!(
            Instruction::decode(4052227719u32), Instruction::Rem_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rem_f64() {
        println!(
            "{:032b}", Instruction::Rem_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 4052227719u32);
        assert_eq!(
            Instruction::Rem_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4052227719u32
        );
    }
    #[test]
    fn decode_Rem_E_f32() {
        assert_eq!(
            Instruction::decode(3783792327u32), Instruction::Rem_E_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rem_E_f32() {
        println!(
            "{:032b}", Instruction::Rem_E_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 3783792327u32);
        assert_eq!(
            Instruction::Rem_E_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3783792327u32
        );
    }
    #[test]
    fn decode_Rem_E_f64() {
        assert_eq!(
            Instruction::decode(4052227783u32), Instruction::Rem_E_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rem_E_f64() {
        println!(
            "{:032b}", Instruction::Rem_E_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 4052227783u32);
        assert_eq!(
            Instruction::Rem_E_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4052227783u32
        );
    }
    #[test]
    fn decode_Cbrt_f32() {
        assert_eq!(
            Instruction::decode(3758626631u32), Instruction::Cbrt_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Cbrt_f32() {
        println!(
            "{:032b}", Instruction::Cbrt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 3758626631u32);
        assert_eq!(
            Instruction::Cbrt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 3758626631u32
        );
    }
    #[test]
    fn decode_Cbrt_f64() {
        assert_eq!(
            Instruction::decode(4027062087u32), Instruction::Cbrt_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Cbrt_f64() {
        println!(
            "{:032b}", Instruction::Cbrt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode()
        );
        println!("{:032b}", 4027062087u32);
        assert_eq!(
            Instruction::Cbrt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 4027062087u32
        );
    }
    #[test]
    fn decode_Sub_f32() {
        assert_eq!(
            Instruction::decode(3783792519u32), Instruction::Sub_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Sub_f32() {
        println!(
            "{:032b}", Instruction::Sub_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 3783792519u32);
        assert_eq!(
            Instruction::Sub_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3783792519u32
        );
    }
    #[test]
    fn decode_Sub_f64() {
        assert_eq!(
            Instruction::decode(4052227975u32), Instruction::Sub_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Sub_f64() {
        println!(
            "{:032b}", Instruction::Sub_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), } .encode()
        );
        println!("{:032b}", 4052227975u32);
        assert_eq!(
            Instruction::Sub_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4052227975u32
        );
    }
}
