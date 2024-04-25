use crate::register::Register;
use crate::encoding::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Halt,
    Trap,
    Call { rs2: Register },
    Callr { rs2: Register },
    Calli { imm: i16 },
    Ret,
    Ecall { imm: i16 },
    Break,
    Jal { rs2: Register },
    Jalr { rs2: Register },
    Jali { imm: i16 },
    Jnz { rs1: Register, rs2: Register },
    Jnzr { rs1: Register, rs2: Register },
    Jnzi { rd: Register, imm: i16 },
    Jiz { rs1: Register, rs2: Register },
    Jizr { rs1: Register, rs2: Register },
    Jizi { rd: Register, imm: i16 },
    Load_8 { rd: Register, rs1: Register },
    Load_16 { rd: Register, rs1: Register },
    Load_32 { rd: Register, rs1: Register },
    Load_64 { rd: Register, rs1: Register },
    Loadi { rd: Register, imm: i16 },
    Loada_8 { rd: Register, imm: i16 },
    Loada_16 { rd: Register, imm: i16 },
    Loada_32 { rd: Register, imm: i16 },
    Loada_64 { rd: Register, imm: i16 },
    Store_8 { rd: Register, rs1: Register },
    Store_16 { rd: Register, rs1: Register },
    Store_32 { rd: Register, rs1: Register },
    Store_64 { rd: Register, rs1: Register },
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
            0u8 if funct == 2u8 && s == 1u8 => Instruction::Callr { rs2 },
            0u8 if funct == 3u8 => Instruction::Calli { imm },
            0u8 if funct == 4u8 => Instruction::Ret,
            0u8 if funct == 5u8 => Instruction::Ecall { imm },
            0u8 if funct == 6u8 => Instruction::Break,
            0u8 if funct == 7u8 && s == 0u8 => Instruction::Jal { rs2 },
            0u8 if funct == 7u8 && s == 1u8 => Instruction::Jalr { rs2 },
            0u8 if funct == 8u8 => Instruction::Jali { imm },
            0u8 if funct == 9u8 && s == 0u8 => Instruction::Jnz { rs1, rs2 },
            0u8 if funct == 9u8 && s == 1u8 => Instruction::Jnzr { rs1, rs2 },
            0u8 if funct == 10u8 && !rd.is_readonly() => Instruction::Jnzi { rd, imm },
            0u8 if funct == 11u8 && s == 0u8 => Instruction::Jiz { rs1, rs2 },
            0u8 if funct == 11u8 && s == 1u8 => Instruction::Jizr { rs1, rs2 },
            0u8 if funct == 12u8 && !rd.is_readonly() => Instruction::Jizi { rd, imm },
            1u8 if funct == 0u8 && size == 8u8 && !rd.is_readonly() => {
                Instruction::Load_8 { rd, rs1 }
            }
            1u8 if funct == 0u8 && size == 16u8 && !rd.is_readonly() => {
                Instruction::Load_16 { rd, rs1 }
            }
            1u8 if funct == 0u8 && size == 32u8 && !rd.is_readonly() => {
                Instruction::Load_32 { rd, rs1 }
            }
            1u8 if funct == 0u8 && size == 64u8 && !rd.is_readonly() => {
                Instruction::Load_64 { rd, rs1 }
            }
            1u8 if funct == 1u8 && !rd.is_readonly() => Instruction::Loadi { rd, imm },
            1u8 if funct == 2u8 && !rd.is_readonly() => Instruction::Loada_8 { rd, imm },
            1u8 if funct == 3u8 && !rd.is_readonly() => Instruction::Loada_16 { rd, imm },
            1u8 if funct == 4u8 && !rd.is_readonly() => Instruction::Loada_32 { rd, imm },
            1u8 if funct == 5u8 && !rd.is_readonly() => Instruction::Loada_64 { rd, imm },
            1u8 if funct == 6u8 && size == 8u8 && !rd.is_readonly() => {
                Instruction::Store_8 { rd, rs1 }
            }
            1u8 if funct == 6u8 && size == 16u8 && !rd.is_readonly() => {
                Instruction::Store_16 { rd, rs1 }
            }
            1u8 if funct == 6u8 && size == 32u8 && !rd.is_readonly() => {
                Instruction::Store_32 { rd, rs1 }
            }
            1u8 if funct == 6u8 && size == 64u8 && !rd.is_readonly() => {
                Instruction::Store_64 { rd, rs1 }
            }
            1u8 if funct == 7u8 && !rd.is_readonly() => Instruction::Storei { rd, imm },
            1u8 if funct == 8u8 && !rd.is_readonly() => Instruction::Move { rd, rs1 },
            1u8 if funct == 9u8 && !rd.is_readonly() => Instruction::Push { rd },
            1u8 if funct == 10u8 => Instruction::Pushi { imm },
            1u8 if funct == 11u8 && !rd.is_readonly() => Instruction::Pop { rd },
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
            Instruction::Callr { rs2 } => {
                encode_s(1u8) | 0 | 0 | encode_rs2(*rs2) | 0 | 0 | encode_funct(2u8)
                    | encode_op_code(0u8)
            }
            Instruction::Calli { imm } => {
                encode_imm(*imm) | 0 | encode_funct(3u8) | encode_op_code(0u8)
            }
            Instruction::Ret => 256u32,
            Instruction::Ecall { imm } => {
                encode_imm(*imm) | 0 | encode_funct(5u8) | encode_op_code(0u8)
            }
            Instruction::Break => 384u32,
            Instruction::Jal { rs2 } => {
                encode_s(0u8) | 0 | 0 | encode_rs2(*rs2) | 0 | 0 | encode_funct(7u8)
                    | encode_op_code(0u8)
            }
            Instruction::Jalr { rs2 } => {
                encode_s(1u8) | 0 | 0 | encode_rs2(*rs2) | 0 | 0 | encode_funct(7u8)
                    | encode_op_code(0u8)
            }
            Instruction::Jali { imm } => {
                encode_imm(*imm) | 0 | encode_funct(8u8) | encode_op_code(0u8)
            }
            Instruction::Jnz { rs1, rs2 } => {
                encode_s(0u8) | 0 | 0 | encode_rs2(*rs2) | encode_rs1(*rs1) | 0
                    | encode_funct(9u8) | encode_op_code(0u8)
            }
            Instruction::Jnzr { rs1, rs2 } => {
                encode_s(1u8) | 0 | 0 | encode_rs2(*rs2) | encode_rs1(*rs1) | 0
                    | encode_funct(9u8) | encode_op_code(0u8)
            }
            Instruction::Jnzi { rd, imm } => {
                encode_imm(*imm) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(0u8)
            }
            Instruction::Jiz { rs1, rs2 } => {
                encode_s(0u8) | 0 | 0 | encode_rs2(*rs2) | encode_rs1(*rs1) | 0
                    | encode_funct(11u8) | encode_op_code(0u8)
            }
            Instruction::Jizr { rs1, rs2 } => {
                encode_s(1u8) | 0 | 0 | encode_rs2(*rs2) | encode_rs1(*rs1) | 0
                    | encode_funct(11u8) | encode_op_code(0u8)
            }
            Instruction::Jizi { rd, imm } => {
                encode_imm(*imm) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(0u8)
            }
            Instruction::Load_8 { rd, rs1 } => {
                0 | 0 | encode_size(8u8) | 0 | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(0u8) | encode_op_code(1u8)
            }
            Instruction::Load_16 { rd, rs1 } => {
                0 | 0 | encode_size(16u8) | 0 | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(0u8) | encode_op_code(1u8)
            }
            Instruction::Load_32 { rd, rs1 } => {
                0 | 0 | encode_size(32u8) | 0 | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(0u8) | encode_op_code(1u8)
            }
            Instruction::Load_64 { rd, rs1 } => {
                0 | 0 | encode_size(64u8) | 0 | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(0u8) | encode_op_code(1u8)
            }
            Instruction::Loadi { rd, imm } => {
                encode_imm(*imm) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(1u8)
            }
            Instruction::Loada_8 { rd, imm } => {
                encode_imm(*imm) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(1u8)
            }
            Instruction::Loada_16 { rd, imm } => {
                encode_imm(*imm) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(1u8)
            }
            Instruction::Loada_32 { rd, imm } => {
                encode_imm(*imm) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(1u8)
            }
            Instruction::Loada_64 { rd, imm } => {
                encode_imm(*imm) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(1u8)
            }
            Instruction::Store_8 { rd, rs1 } => {
                0 | 0 | encode_size(8u8) | 0 | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(6u8) | encode_op_code(1u8)
            }
            Instruction::Store_16 { rd, rs1 } => {
                0 | 0 | encode_size(16u8) | 0 | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(6u8) | encode_op_code(1u8)
            }
            Instruction::Store_32 { rd, rs1 } => {
                0 | 0 | encode_size(32u8) | 0 | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(6u8) | encode_op_code(1u8)
            }
            Instruction::Store_64 { rd, rs1 } => {
                0 | 0 | encode_size(64u8) | 0 | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(6u8) | encode_op_code(1u8)
            }
            Instruction::Storei { rd, imm } => {
                encode_imm(*imm) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(1u8)
            }
            Instruction::Move { rd, rs1 } => {
                0 | 0 | 0 | 0 | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(8u8)
                    | encode_op_code(1u8)
            }
            Instruction::Push { rd } => {
                0 | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(1u8)
            }
            Instruction::Pushi { imm } => {
                encode_imm(*imm) | 0 | encode_funct(10u8) | encode_op_code(1u8)
            }
            Instruction::Pop { rd } => {
                0 | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(1u8)
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
        assert_eq!(Instruction::Halt.encode(), 0u32);
    }
    #[test]
    fn decode_Trap() {
        assert_eq!(Instruction::decode(64u32), Instruction::Trap);
    }
    #[test]
    fn encode_Trap() {
        assert_eq!(Instruction::Trap.encode(), 64u32);
    }
    #[test]
    fn decode_Call() {
        assert_eq!(
            Instruction::decode(50331776u32), Instruction::Call { rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Call() {
        assert_eq!(
            Instruction::Call { rs2 : Register::General_Purpose(6), } .encode(),
            50331776u32
        );
    }
    #[test]
    fn decode_Callr() {
        assert_eq!(
            Instruction::decode(2197815424u32), Instruction::Callr { rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Callr() {
        assert_eq!(
            Instruction::Callr { rs2 : Register::General_Purpose(6), } .encode(),
            2197815424u32
        );
    }
    #[test]
    fn decode_Calli() {
        assert_eq!(
            Instruction::decode(4293918912u32), Instruction::Calli { imm : - 16, }
        );
    }
    #[test]
    fn encode_Calli() {
        assert_eq!(Instruction::Calli { imm : - 16, } .encode(), 4293918912u32);
    }
    #[test]
    fn decode_Ret() {
        assert_eq!(Instruction::decode(256u32), Instruction::Ret);
    }
    #[test]
    fn encode_Ret() {
        assert_eq!(Instruction::Ret.encode(), 256u32);
    }
    #[test]
    fn decode_Ecall() {
        assert_eq!(
            Instruction::decode(4293919040u32), Instruction::Ecall { imm : - 16, }
        );
    }
    #[test]
    fn encode_Ecall() {
        assert_eq!(Instruction::Ecall { imm : - 16, } .encode(), 4293919040u32);
    }
    #[test]
    fn decode_Break() {
        assert_eq!(Instruction::decode(384u32), Instruction::Break);
    }
    #[test]
    fn encode_Break() {
        assert_eq!(Instruction::Break.encode(), 384u32);
    }
    #[test]
    fn decode_Jal() {
        assert_eq!(
            Instruction::decode(50332096u32), Instruction::Jal { rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Jal() {
        assert_eq!(
            Instruction::Jal { rs2 : Register::General_Purpose(6), } .encode(),
            50332096u32
        );
    }
    #[test]
    fn decode_Jalr() {
        assert_eq!(
            Instruction::decode(2197815744u32), Instruction::Jalr { rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Jalr() {
        assert_eq!(
            Instruction::Jalr { rs2 : Register::General_Purpose(6), } .encode(),
            2197815744u32
        );
    }
    #[test]
    fn decode_Jali() {
        assert_eq!(
            Instruction::decode(4293919232u32), Instruction::Jali { imm : - 16, }
        );
    }
    #[test]
    fn encode_Jali() {
        assert_eq!(Instruction::Jali { imm : - 16, } .encode(), 4293919232u32);
    }
    #[test]
    fn decode_Jnz() {
        assert_eq!(
            Instruction::decode(51249728u32), Instruction::Jnz { rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Jnz() {
        assert_eq!(
            Instruction::Jnz { rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), } .encode(), 51249728u32
        );
    }
    #[test]
    fn decode_Jnzr() {
        assert_eq!(
            Instruction::decode(2198733376u32), Instruction::Jnzr { rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Jnzr() {
        assert_eq!(
            Instruction::Jnzr { rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), } .encode(), 2198733376u32
        );
    }
    #[test]
    fn decode_Jnzi() {
        assert_eq!(
            Instruction::decode(4293930624u32), Instruction::Jnzi { rd :
            Register::General_Purpose(5), imm : - 16, }
        );
    }
    #[test]
    fn encode_Jnzi() {
        assert_eq!(
            Instruction::Jnzi { rd : Register::General_Purpose(5), imm : - 16, }
            .encode(), 4293930624u32
        );
    }
    #[test]
    fn decode_Jiz() {
        assert_eq!(
            Instruction::decode(51249856u32), Instruction::Jiz { rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Jiz() {
        assert_eq!(
            Instruction::Jiz { rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), } .encode(), 51249856u32
        );
    }
    #[test]
    fn decode_Jizr() {
        assert_eq!(
            Instruction::decode(2198733504u32), Instruction::Jizr { rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Jizr() {
        assert_eq!(
            Instruction::Jizr { rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), } .encode(), 2198733504u32
        );
    }
    #[test]
    fn decode_Jizi() {
        assert_eq!(
            Instruction::decode(4293930752u32), Instruction::Jizi { rd :
            Register::General_Purpose(5), imm : - 16, }
        );
    }
    #[test]
    fn encode_Jizi() {
        assert_eq!(
            Instruction::Jizi { rd : Register::General_Purpose(5), imm : - 16, }
            .encode(), 4293930752u32
        );
    }
    #[test]
    fn decode_Load_8() {
        assert_eq!(
            Instruction::decode(928769u32), Instruction::Load_8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Load_8() {
        assert_eq!(
            Instruction::Load_8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 928769u32
        );
    }
    #[test]
    fn decode_Load_16() {
        assert_eq!(
            Instruction::decode(269364225u32), Instruction::Load_16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Load_16() {
        assert_eq!(
            Instruction::Load_16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364225u32
        );
    }
    #[test]
    fn decode_Load_32() {
        assert_eq!(
            Instruction::decode(537799681u32), Instruction::Load_32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Load_32() {
        assert_eq!(
            Instruction::Load_32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537799681u32
        );
    }
    #[test]
    fn decode_Load_64() {
        assert_eq!(
            Instruction::decode(806235137u32), Instruction::Load_64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Load_64() {
        assert_eq!(
            Instruction::Load_64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235137u32
        );
    }
    #[test]
    fn decode_Loadi() {
        assert_eq!(
            Instruction::decode(4293930049u32), Instruction::Loadi { rd :
            Register::General_Purpose(5), imm : - 16, }
        );
    }
    #[test]
    fn encode_Loadi() {
        assert_eq!(
            Instruction::Loadi { rd : Register::General_Purpose(5), imm : - 16, }
            .encode(), 4293930049u32
        );
    }
    #[test]
    fn decode_Loada_8() {
        assert_eq!(
            Instruction::decode(4293930113u32), Instruction::Loada_8 { rd :
            Register::General_Purpose(5), imm : - 16, }
        );
    }
    #[test]
    fn encode_Loada_8() {
        assert_eq!(
            Instruction::Loada_8 { rd : Register::General_Purpose(5), imm : - 16, }
            .encode(), 4293930113u32
        );
    }
    #[test]
    fn decode_Loada_16() {
        assert_eq!(
            Instruction::decode(4293930177u32), Instruction::Loada_16 { rd :
            Register::General_Purpose(5), imm : - 16, }
        );
    }
    #[test]
    fn encode_Loada_16() {
        assert_eq!(
            Instruction::Loada_16 { rd : Register::General_Purpose(5), imm : - 16, }
            .encode(), 4293930177u32
        );
    }
    #[test]
    fn decode_Loada_32() {
        assert_eq!(
            Instruction::decode(4293930241u32), Instruction::Loada_32 { rd :
            Register::General_Purpose(5), imm : - 16, }
        );
    }
    #[test]
    fn encode_Loada_32() {
        assert_eq!(
            Instruction::Loada_32 { rd : Register::General_Purpose(5), imm : - 16, }
            .encode(), 4293930241u32
        );
    }
    #[test]
    fn decode_Loada_64() {
        assert_eq!(
            Instruction::decode(4293930305u32), Instruction::Loada_64 { rd :
            Register::General_Purpose(5), imm : - 16, }
        );
    }
    #[test]
    fn encode_Loada_64() {
        assert_eq!(
            Instruction::Loada_64 { rd : Register::General_Purpose(5), imm : - 16, }
            .encode(), 4293930305u32
        );
    }
    #[test]
    fn decode_Store_8() {
        assert_eq!(
            Instruction::decode(929153u32), Instruction::Store_8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Store_8() {
        assert_eq!(
            Instruction::Store_8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929153u32
        );
    }
    #[test]
    fn decode_Store_16() {
        assert_eq!(
            Instruction::decode(269364609u32), Instruction::Store_16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Store_16() {
        assert_eq!(
            Instruction::Store_16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364609u32
        );
    }
    #[test]
    fn decode_Store_32() {
        assert_eq!(
            Instruction::decode(537800065u32), Instruction::Store_32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Store_32() {
        assert_eq!(
            Instruction::Store_32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800065u32
        );
    }
    #[test]
    fn decode_Store_64() {
        assert_eq!(
            Instruction::decode(806235521u32), Instruction::Store_64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Store_64() {
        assert_eq!(
            Instruction::Store_64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235521u32
        );
    }
    #[test]
    fn decode_Storei() {
        assert_eq!(
            Instruction::decode(4293930433u32), Instruction::Storei { rd :
            Register::General_Purpose(5), imm : - 16, }
        );
    }
    #[test]
    fn encode_Storei() {
        assert_eq!(
            Instruction::Storei { rd : Register::General_Purpose(5), imm : - 16, }
            .encode(), 4293930433u32
        );
    }
    #[test]
    fn decode_Move() {
        assert_eq!(
            Instruction::decode(929281u32), Instruction::Move { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Move() {
        assert_eq!(
            Instruction::Move { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929281u32
        );
    }
    #[test]
    fn decode_Push() {
        assert_eq!(
            Instruction::decode(11841u32), Instruction::Push { rd :
            Register::General_Purpose(5), }
        );
    }
    #[test]
    fn encode_Push() {
        assert_eq!(
            Instruction::Push { rd : Register::General_Purpose(5), } .encode(), 11841u32
        );
    }
    #[test]
    fn decode_Pushi() {
        assert_eq!(
            Instruction::decode(4293919361u32), Instruction::Pushi { imm : - 16, }
        );
    }
    #[test]
    fn encode_Pushi() {
        assert_eq!(Instruction::Pushi { imm : - 16, } .encode(), 4293919361u32);
    }
    #[test]
    fn decode_Pop() {
        assert_eq!(
            Instruction::decode(11969u32), Instruction::Pop { rd :
            Register::General_Purpose(5), }
        );
    }
    #[test]
    fn encode_Pop() {
        assert_eq!(
            Instruction::Pop { rd : Register::General_Purpose(5), } .encode(), 11969u32
        );
    }
    #[test]
    fn decode_Ie() {
        assert_eq!(
            Instruction::decode(51260418u32), Instruction::Ie { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ie() {
        assert_eq!(
            Instruction::Ie { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260418u32
        );
    }
    #[test]
    fn decode_Ie_f32() {
        assert_eq!(
            Instruction::decode(1661873154u32), Instruction::Ie_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ie_f32() {
        assert_eq!(
            Instruction::Ie_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1661873154u32
        );
    }
    #[test]
    fn decode_Ie_f64() {
        assert_eq!(
            Instruction::decode(1930308610u32), Instruction::Ie_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ie_f64() {
        assert_eq!(
            Instruction::Ie_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1930308610u32
        );
    }
    #[test]
    fn decode_Ne() {
        assert_eq!(
            Instruction::decode(51260482u32), Instruction::Ne { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ne() {
        assert_eq!(
            Instruction::Ne { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260482u32
        );
    }
    #[test]
    fn decode_Ne_f32() {
        assert_eq!(
            Instruction::decode(1661873218u32), Instruction::Ne_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ne_f32() {
        assert_eq!(
            Instruction::Ne_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1661873218u32
        );
    }
    #[test]
    fn decode_Ne_f64() {
        assert_eq!(
            Instruction::decode(1930308674u32), Instruction::Ne_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ne_f64() {
        assert_eq!(
            Instruction::Ne_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1930308674u32
        );
    }
    #[test]
    fn decode_Lt() {
        assert_eq!(
            Instruction::decode(51260546u32), Instruction::Lt { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Lt() {
        assert_eq!(
            Instruction::Lt { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260546u32
        );
    }
    #[test]
    fn decode_Lt_f32() {
        assert_eq!(
            Instruction::decode(1661873282u32), Instruction::Lt_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Lt_f32() {
        assert_eq!(
            Instruction::Lt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1661873282u32
        );
    }
    #[test]
    fn decode_Lt_f64() {
        assert_eq!(
            Instruction::decode(1930308738u32), Instruction::Lt_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Lt_f64() {
        assert_eq!(
            Instruction::Lt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1930308738u32
        );
    }
    #[test]
    fn decode_Le() {
        assert_eq!(
            Instruction::decode(51260610u32), Instruction::Le { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Le() {
        assert_eq!(
            Instruction::Le { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260610u32
        );
    }
    #[test]
    fn decode_Le_f32() {
        assert_eq!(
            Instruction::decode(1661873346u32), Instruction::Le_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Le_f32() {
        assert_eq!(
            Instruction::Le_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1661873346u32
        );
    }
    #[test]
    fn decode_Le_f64() {
        assert_eq!(
            Instruction::decode(1930308802u32), Instruction::Le_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Le_f64() {
        assert_eq!(
            Instruction::Le_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1930308802u32
        );
    }
    #[test]
    fn decode_Gt() {
        assert_eq!(
            Instruction::decode(51260674u32), Instruction::Gt { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Gt() {
        assert_eq!(
            Instruction::Gt { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260674u32
        );
    }
    #[test]
    fn decode_Gt_f32() {
        assert_eq!(
            Instruction::decode(1661873410u32), Instruction::Gt_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Gt_f32() {
        assert_eq!(
            Instruction::Gt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1661873410u32
        );
    }
    #[test]
    fn decode_Gt_f64() {
        assert_eq!(
            Instruction::decode(1930308866u32), Instruction::Gt_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Gt_f64() {
        assert_eq!(
            Instruction::Gt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1930308866u32
        );
    }
    #[test]
    fn decode_Ge() {
        assert_eq!(
            Instruction::decode(51260738u32), Instruction::Ge { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ge() {
        assert_eq!(
            Instruction::Ge { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260738u32
        );
    }
    #[test]
    fn decode_Ge_f32() {
        assert_eq!(
            Instruction::decode(1661873474u32), Instruction::Ge_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ge_f32() {
        assert_eq!(
            Instruction::Ge_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1661873474u32
        );
    }
    #[test]
    fn decode_Ge_f64() {
        assert_eq!(
            Instruction::decode(1930308930u32), Instruction::Ge_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Ge_f64() {
        assert_eq!(
            Instruction::Ge_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 1930308930u32
        );
    }
    #[test]
    fn decode_And_i8() {
        assert_eq!(
            Instruction::decode(2198744067u32), Instruction::And_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_i8() {
        assert_eq!(
            Instruction::And_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744067u32
        );
    }
    #[test]
    fn decode_And_i16() {
        assert_eq!(
            Instruction::decode(2467179523u32), Instruction::And_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_i16() {
        assert_eq!(
            Instruction::And_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179523u32
        );
    }
    #[test]
    fn decode_And_i32() {
        assert_eq!(
            Instruction::decode(2735614979u32), Instruction::And_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_i32() {
        assert_eq!(
            Instruction::And_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735614979u32
        );
    }
    #[test]
    fn decode_And_i64() {
        assert_eq!(
            Instruction::decode(3004050435u32), Instruction::And_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_i64() {
        assert_eq!(
            Instruction::And_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050435u32
        );
    }
    #[test]
    fn decode_And_u8() {
        assert_eq!(
            Instruction::decode(51260419u32), Instruction::And_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_u8() {
        assert_eq!(
            Instruction::And_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260419u32
        );
    }
    #[test]
    fn decode_And_u16() {
        assert_eq!(
            Instruction::decode(319695875u32), Instruction::And_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_u16() {
        assert_eq!(
            Instruction::And_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319695875u32
        );
    }
    #[test]
    fn decode_And_u32() {
        assert_eq!(
            Instruction::decode(588131331u32), Instruction::And_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_u32() {
        assert_eq!(
            Instruction::And_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131331u32
        );
    }
    #[test]
    fn decode_And_u64() {
        assert_eq!(
            Instruction::decode(856566787u32), Instruction::And_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_And_u64() {
        assert_eq!(
            Instruction::And_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856566787u32
        );
    }
    #[test]
    fn decode_Or_i8() {
        assert_eq!(
            Instruction::decode(2198744131u32), Instruction::Or_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_i8() {
        assert_eq!(
            Instruction::Or_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744131u32
        );
    }
    #[test]
    fn decode_Or_i16() {
        assert_eq!(
            Instruction::decode(2467179587u32), Instruction::Or_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_i16() {
        assert_eq!(
            Instruction::Or_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179587u32
        );
    }
    #[test]
    fn decode_Or_i32() {
        assert_eq!(
            Instruction::decode(2735615043u32), Instruction::Or_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_i32() {
        assert_eq!(
            Instruction::Or_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615043u32
        );
    }
    #[test]
    fn decode_Or_i64() {
        assert_eq!(
            Instruction::decode(3004050499u32), Instruction::Or_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_i64() {
        assert_eq!(
            Instruction::Or_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050499u32
        );
    }
    #[test]
    fn decode_Or_u8() {
        assert_eq!(
            Instruction::decode(51260483u32), Instruction::Or_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_u8() {
        assert_eq!(
            Instruction::Or_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260483u32
        );
    }
    #[test]
    fn decode_Or_u16() {
        assert_eq!(
            Instruction::decode(319695939u32), Instruction::Or_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_u16() {
        assert_eq!(
            Instruction::Or_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319695939u32
        );
    }
    #[test]
    fn decode_Or_u32() {
        assert_eq!(
            Instruction::decode(588131395u32), Instruction::Or_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_u32() {
        assert_eq!(
            Instruction::Or_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131395u32
        );
    }
    #[test]
    fn decode_Or_u64() {
        assert_eq!(
            Instruction::decode(856566851u32), Instruction::Or_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Or_u64() {
        assert_eq!(
            Instruction::Or_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856566851u32
        );
    }
    #[test]
    fn decode_Xor_i8() {
        assert_eq!(
            Instruction::decode(2198744195u32), Instruction::Xor_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_i8() {
        assert_eq!(
            Instruction::Xor_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744195u32
        );
    }
    #[test]
    fn decode_Xor_i16() {
        assert_eq!(
            Instruction::decode(2467179651u32), Instruction::Xor_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_i16() {
        assert_eq!(
            Instruction::Xor_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179651u32
        );
    }
    #[test]
    fn decode_Xor_i32() {
        assert_eq!(
            Instruction::decode(2735615107u32), Instruction::Xor_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_i32() {
        assert_eq!(
            Instruction::Xor_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615107u32
        );
    }
    #[test]
    fn decode_Xor_i64() {
        assert_eq!(
            Instruction::decode(3004050563u32), Instruction::Xor_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_i64() {
        assert_eq!(
            Instruction::Xor_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050563u32
        );
    }
    #[test]
    fn decode_Xor_u8() {
        assert_eq!(
            Instruction::decode(51260547u32), Instruction::Xor_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_u8() {
        assert_eq!(
            Instruction::Xor_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260547u32
        );
    }
    #[test]
    fn decode_Xor_u16() {
        assert_eq!(
            Instruction::decode(319696003u32), Instruction::Xor_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_u16() {
        assert_eq!(
            Instruction::Xor_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696003u32
        );
    }
    #[test]
    fn decode_Xor_u32() {
        assert_eq!(
            Instruction::decode(588131459u32), Instruction::Xor_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_u32() {
        assert_eq!(
            Instruction::Xor_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131459u32
        );
    }
    #[test]
    fn decode_Xor_u64() {
        assert_eq!(
            Instruction::decode(856566915u32), Instruction::Xor_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Xor_u64() {
        assert_eq!(
            Instruction::Xor_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856566915u32
        );
    }
    #[test]
    fn decode_Not_i8() {
        assert_eq!(
            Instruction::decode(2148412611u32), Instruction::Not_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_i8() {
        assert_eq!(
            Instruction::Not_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412611u32
        );
    }
    #[test]
    fn decode_Not_i16() {
        assert_eq!(
            Instruction::decode(2416848067u32), Instruction::Not_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_i16() {
        assert_eq!(
            Instruction::Not_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848067u32
        );
    }
    #[test]
    fn decode_Not_i32() {
        assert_eq!(
            Instruction::decode(2685283523u32), Instruction::Not_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_i32() {
        assert_eq!(
            Instruction::Not_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283523u32
        );
    }
    #[test]
    fn decode_Not_i64() {
        assert_eq!(
            Instruction::decode(2953718979u32), Instruction::Not_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_i64() {
        assert_eq!(
            Instruction::Not_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953718979u32
        );
    }
    #[test]
    fn decode_Not_u8() {
        assert_eq!(
            Instruction::decode(928963u32), Instruction::Not_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_u8() {
        assert_eq!(
            Instruction::Not_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 928963u32
        );
    }
    #[test]
    fn decode_Not_u16() {
        assert_eq!(
            Instruction::decode(269364419u32), Instruction::Not_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_u16() {
        assert_eq!(
            Instruction::Not_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364419u32
        );
    }
    #[test]
    fn decode_Not_u32() {
        assert_eq!(
            Instruction::decode(537799875u32), Instruction::Not_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_u32() {
        assert_eq!(
            Instruction::Not_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537799875u32
        );
    }
    #[test]
    fn decode_Not_u64() {
        assert_eq!(
            Instruction::decode(806235331u32), Instruction::Not_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Not_u64() {
        assert_eq!(
            Instruction::Not_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235331u32
        );
    }
    #[test]
    fn decode_Shl_i8() {
        assert_eq!(
            Instruction::decode(2198744323u32), Instruction::Shl_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_i8() {
        assert_eq!(
            Instruction::Shl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744323u32
        );
    }
    #[test]
    fn decode_Shl_i16() {
        assert_eq!(
            Instruction::decode(2467179779u32), Instruction::Shl_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_i16() {
        assert_eq!(
            Instruction::Shl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179779u32
        );
    }
    #[test]
    fn decode_Shl_i32() {
        assert_eq!(
            Instruction::decode(2735615235u32), Instruction::Shl_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_i32() {
        assert_eq!(
            Instruction::Shl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615235u32
        );
    }
    #[test]
    fn decode_Shl_i64() {
        assert_eq!(
            Instruction::decode(3004050691u32), Instruction::Shl_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_i64() {
        assert_eq!(
            Instruction::Shl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050691u32
        );
    }
    #[test]
    fn decode_Shl_u8() {
        assert_eq!(
            Instruction::decode(51260675u32), Instruction::Shl_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_u8() {
        assert_eq!(
            Instruction::Shl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260675u32
        );
    }
    #[test]
    fn decode_Shl_u16() {
        assert_eq!(
            Instruction::decode(319696131u32), Instruction::Shl_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_u16() {
        assert_eq!(
            Instruction::Shl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696131u32
        );
    }
    #[test]
    fn decode_Shl_u32() {
        assert_eq!(
            Instruction::decode(588131587u32), Instruction::Shl_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_u32() {
        assert_eq!(
            Instruction::Shl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131587u32
        );
    }
    #[test]
    fn decode_Shl_u64() {
        assert_eq!(
            Instruction::decode(856567043u32), Instruction::Shl_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shl_u64() {
        assert_eq!(
            Instruction::Shl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567043u32
        );
    }
    #[test]
    fn decode_Shr_i8() {
        assert_eq!(
            Instruction::decode(2198744387u32), Instruction::Shr_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_i8() {
        assert_eq!(
            Instruction::Shr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744387u32
        );
    }
    #[test]
    fn decode_Shr_i16() {
        assert_eq!(
            Instruction::decode(2467179843u32), Instruction::Shr_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_i16() {
        assert_eq!(
            Instruction::Shr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179843u32
        );
    }
    #[test]
    fn decode_Shr_i32() {
        assert_eq!(
            Instruction::decode(2735615299u32), Instruction::Shr_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_i32() {
        assert_eq!(
            Instruction::Shr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615299u32
        );
    }
    #[test]
    fn decode_Shr_i64() {
        assert_eq!(
            Instruction::decode(3004050755u32), Instruction::Shr_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_i64() {
        assert_eq!(
            Instruction::Shr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050755u32
        );
    }
    #[test]
    fn decode_Shr_u8() {
        assert_eq!(
            Instruction::decode(51260739u32), Instruction::Shr_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_u8() {
        assert_eq!(
            Instruction::Shr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260739u32
        );
    }
    #[test]
    fn decode_Shr_u16() {
        assert_eq!(
            Instruction::decode(319696195u32), Instruction::Shr_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_u16() {
        assert_eq!(
            Instruction::Shr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696195u32
        );
    }
    #[test]
    fn decode_Shr_u32() {
        assert_eq!(
            Instruction::decode(588131651u32), Instruction::Shr_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_u32() {
        assert_eq!(
            Instruction::Shr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131651u32
        );
    }
    #[test]
    fn decode_Shr_u64() {
        assert_eq!(
            Instruction::decode(856567107u32), Instruction::Shr_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Shr_u64() {
        assert_eq!(
            Instruction::Shr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567107u32
        );
    }
    #[test]
    fn decode_Rotl_i8() {
        assert_eq!(
            Instruction::decode(2198744451u32), Instruction::Rotl_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_i8() {
        assert_eq!(
            Instruction::Rotl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744451u32
        );
    }
    #[test]
    fn decode_Rotl_i16() {
        assert_eq!(
            Instruction::decode(2467179907u32), Instruction::Rotl_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_i16() {
        assert_eq!(
            Instruction::Rotl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179907u32
        );
    }
    #[test]
    fn decode_Rotl_i32() {
        assert_eq!(
            Instruction::decode(2735615363u32), Instruction::Rotl_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_i32() {
        assert_eq!(
            Instruction::Rotl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615363u32
        );
    }
    #[test]
    fn decode_Rotl_i64() {
        assert_eq!(
            Instruction::decode(3004050819u32), Instruction::Rotl_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_i64() {
        assert_eq!(
            Instruction::Rotl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050819u32
        );
    }
    #[test]
    fn decode_Rotl_u8() {
        assert_eq!(
            Instruction::decode(51260803u32), Instruction::Rotl_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_u8() {
        assert_eq!(
            Instruction::Rotl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260803u32
        );
    }
    #[test]
    fn decode_Rotl_u16() {
        assert_eq!(
            Instruction::decode(319696259u32), Instruction::Rotl_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_u16() {
        assert_eq!(
            Instruction::Rotl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696259u32
        );
    }
    #[test]
    fn decode_Rotl_u32() {
        assert_eq!(
            Instruction::decode(588131715u32), Instruction::Rotl_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_u32() {
        assert_eq!(
            Instruction::Rotl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131715u32
        );
    }
    #[test]
    fn decode_Rotl_u64() {
        assert_eq!(
            Instruction::decode(856567171u32), Instruction::Rotl_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotl_u64() {
        assert_eq!(
            Instruction::Rotl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567171u32
        );
    }
    #[test]
    fn decode_Rotr_i8() {
        assert_eq!(
            Instruction::decode(2198744515u32), Instruction::Rotr_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_i8() {
        assert_eq!(
            Instruction::Rotr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744515u32
        );
    }
    #[test]
    fn decode_Rotr_i16() {
        assert_eq!(
            Instruction::decode(2467179971u32), Instruction::Rotr_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_i16() {
        assert_eq!(
            Instruction::Rotr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179971u32
        );
    }
    #[test]
    fn decode_Rotr_i32() {
        assert_eq!(
            Instruction::decode(2735615427u32), Instruction::Rotr_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_i32() {
        assert_eq!(
            Instruction::Rotr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615427u32
        );
    }
    #[test]
    fn decode_Rotr_i64() {
        assert_eq!(
            Instruction::decode(3004050883u32), Instruction::Rotr_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_i64() {
        assert_eq!(
            Instruction::Rotr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050883u32
        );
    }
    #[test]
    fn decode_Rotr_u8() {
        assert_eq!(
            Instruction::decode(51260867u32), Instruction::Rotr_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_u8() {
        assert_eq!(
            Instruction::Rotr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260867u32
        );
    }
    #[test]
    fn decode_Rotr_u16() {
        assert_eq!(
            Instruction::decode(319696323u32), Instruction::Rotr_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_u16() {
        assert_eq!(
            Instruction::Rotr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696323u32
        );
    }
    #[test]
    fn decode_Rotr_u32() {
        assert_eq!(
            Instruction::decode(588131779u32), Instruction::Rotr_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_u32() {
        assert_eq!(
            Instruction::Rotr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131779u32
        );
    }
    #[test]
    fn decode_Rotr_u64() {
        assert_eq!(
            Instruction::decode(856567235u32), Instruction::Rotr_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rotr_u64() {
        assert_eq!(
            Instruction::Rotr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567235u32
        );
    }
    #[test]
    fn decode_Count_Ones_i8() {
        assert_eq!(
            Instruction::decode(2148412931u32), Instruction::Count_Ones_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_i8() {
        assert_eq!(
            Instruction::Count_Ones_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412931u32
        );
    }
    #[test]
    fn decode_Count_Ones_i16() {
        assert_eq!(
            Instruction::decode(2416848387u32), Instruction::Count_Ones_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_i16() {
        assert_eq!(
            Instruction::Count_Ones_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848387u32
        );
    }
    #[test]
    fn decode_Count_Ones_i32() {
        assert_eq!(
            Instruction::decode(2685283843u32), Instruction::Count_Ones_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_i32() {
        assert_eq!(
            Instruction::Count_Ones_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283843u32
        );
    }
    #[test]
    fn decode_Count_Ones_i64() {
        assert_eq!(
            Instruction::decode(2953719299u32), Instruction::Count_Ones_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_i64() {
        assert_eq!(
            Instruction::Count_Ones_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719299u32
        );
    }
    #[test]
    fn decode_Count_Ones_u8() {
        assert_eq!(
            Instruction::decode(929283u32), Instruction::Count_Ones_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_u8() {
        assert_eq!(
            Instruction::Count_Ones_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929283u32
        );
    }
    #[test]
    fn decode_Count_Ones_u16() {
        assert_eq!(
            Instruction::decode(269364739u32), Instruction::Count_Ones_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_u16() {
        assert_eq!(
            Instruction::Count_Ones_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364739u32
        );
    }
    #[test]
    fn decode_Count_Ones_u32() {
        assert_eq!(
            Instruction::decode(537800195u32), Instruction::Count_Ones_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_u32() {
        assert_eq!(
            Instruction::Count_Ones_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800195u32
        );
    }
    #[test]
    fn decode_Count_Ones_u64() {
        assert_eq!(
            Instruction::decode(806235651u32), Instruction::Count_Ones_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Ones_u64() {
        assert_eq!(
            Instruction::Count_Ones_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235651u32
        );
    }
    #[test]
    fn decode_Leading_Ones_i8() {
        assert_eq!(
            Instruction::decode(2148412995u32), Instruction::Leading_Ones_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_i8() {
        assert_eq!(
            Instruction::Leading_Ones_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412995u32
        );
    }
    #[test]
    fn decode_Leading_Ones_i16() {
        assert_eq!(
            Instruction::decode(2416848451u32), Instruction::Leading_Ones_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_i16() {
        assert_eq!(
            Instruction::Leading_Ones_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848451u32
        );
    }
    #[test]
    fn decode_Leading_Ones_i32() {
        assert_eq!(
            Instruction::decode(2685283907u32), Instruction::Leading_Ones_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_i32() {
        assert_eq!(
            Instruction::Leading_Ones_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283907u32
        );
    }
    #[test]
    fn decode_Leading_Ones_i64() {
        assert_eq!(
            Instruction::decode(2953719363u32), Instruction::Leading_Ones_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_i64() {
        assert_eq!(
            Instruction::Leading_Ones_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719363u32
        );
    }
    #[test]
    fn decode_Leading_Ones_u8() {
        assert_eq!(
            Instruction::decode(929347u32), Instruction::Leading_Ones_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_u8() {
        assert_eq!(
            Instruction::Leading_Ones_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929347u32
        );
    }
    #[test]
    fn decode_Leading_Ones_u16() {
        assert_eq!(
            Instruction::decode(269364803u32), Instruction::Leading_Ones_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_u16() {
        assert_eq!(
            Instruction::Leading_Ones_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364803u32
        );
    }
    #[test]
    fn decode_Leading_Ones_u32() {
        assert_eq!(
            Instruction::decode(537800259u32), Instruction::Leading_Ones_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_u32() {
        assert_eq!(
            Instruction::Leading_Ones_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800259u32
        );
    }
    #[test]
    fn decode_Leading_Ones_u64() {
        assert_eq!(
            Instruction::decode(806235715u32), Instruction::Leading_Ones_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Ones_u64() {
        assert_eq!(
            Instruction::Leading_Ones_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235715u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_i8() {
        assert_eq!(
            Instruction::decode(2148413059u32), Instruction::Trailing_Ones_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_i8() {
        assert_eq!(
            Instruction::Trailing_Ones_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148413059u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_i16() {
        assert_eq!(
            Instruction::decode(2416848515u32), Instruction::Trailing_Ones_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_i16() {
        assert_eq!(
            Instruction::Trailing_Ones_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848515u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_i32() {
        assert_eq!(
            Instruction::decode(2685283971u32), Instruction::Trailing_Ones_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_i32() {
        assert_eq!(
            Instruction::Trailing_Ones_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283971u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_i64() {
        assert_eq!(
            Instruction::decode(2953719427u32), Instruction::Trailing_Ones_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_i64() {
        assert_eq!(
            Instruction::Trailing_Ones_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719427u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_u8() {
        assert_eq!(
            Instruction::decode(929411u32), Instruction::Trailing_Ones_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_u8() {
        assert_eq!(
            Instruction::Trailing_Ones_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929411u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_u16() {
        assert_eq!(
            Instruction::decode(269364867u32), Instruction::Trailing_Ones_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_u16() {
        assert_eq!(
            Instruction::Trailing_Ones_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364867u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_u32() {
        assert_eq!(
            Instruction::decode(537800323u32), Instruction::Trailing_Ones_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_u32() {
        assert_eq!(
            Instruction::Trailing_Ones_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800323u32
        );
    }
    #[test]
    fn decode_Trailing_Ones_u64() {
        assert_eq!(
            Instruction::decode(806235779u32), Instruction::Trailing_Ones_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Ones_u64() {
        assert_eq!(
            Instruction::Trailing_Ones_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235779u32
        );
    }
    #[test]
    fn decode_Count_Zeros_i8() {
        assert_eq!(
            Instruction::decode(2148413123u32), Instruction::Count_Zeros_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_i8() {
        assert_eq!(
            Instruction::Count_Zeros_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148413123u32
        );
    }
    #[test]
    fn decode_Count_Zeros_i16() {
        assert_eq!(
            Instruction::decode(2416848579u32), Instruction::Count_Zeros_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_i16() {
        assert_eq!(
            Instruction::Count_Zeros_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848579u32
        );
    }
    #[test]
    fn decode_Count_Zeros_i32() {
        assert_eq!(
            Instruction::decode(2685284035u32), Instruction::Count_Zeros_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_i32() {
        assert_eq!(
            Instruction::Count_Zeros_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685284035u32
        );
    }
    #[test]
    fn decode_Count_Zeros_i64() {
        assert_eq!(
            Instruction::decode(2953719491u32), Instruction::Count_Zeros_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_i64() {
        assert_eq!(
            Instruction::Count_Zeros_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719491u32
        );
    }
    #[test]
    fn decode_Count_Zeros_u8() {
        assert_eq!(
            Instruction::decode(929475u32), Instruction::Count_Zeros_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_u8() {
        assert_eq!(
            Instruction::Count_Zeros_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929475u32
        );
    }
    #[test]
    fn decode_Count_Zeros_u16() {
        assert_eq!(
            Instruction::decode(269364931u32), Instruction::Count_Zeros_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_u16() {
        assert_eq!(
            Instruction::Count_Zeros_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364931u32
        );
    }
    #[test]
    fn decode_Count_Zeros_u32() {
        assert_eq!(
            Instruction::decode(537800387u32), Instruction::Count_Zeros_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_u32() {
        assert_eq!(
            Instruction::Count_Zeros_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800387u32
        );
    }
    #[test]
    fn decode_Count_Zeros_u64() {
        assert_eq!(
            Instruction::decode(806235843u32), Instruction::Count_Zeros_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Count_Zeros_u64() {
        assert_eq!(
            Instruction::Count_Zeros_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235843u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_i8() {
        assert_eq!(
            Instruction::decode(2148413187u32), Instruction::Leading_Zeros_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_i8() {
        assert_eq!(
            Instruction::Leading_Zeros_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148413187u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_i16() {
        assert_eq!(
            Instruction::decode(2416848643u32), Instruction::Leading_Zeros_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_i16() {
        assert_eq!(
            Instruction::Leading_Zeros_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848643u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_i32() {
        assert_eq!(
            Instruction::decode(2685284099u32), Instruction::Leading_Zeros_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_i32() {
        assert_eq!(
            Instruction::Leading_Zeros_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685284099u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_i64() {
        assert_eq!(
            Instruction::decode(2953719555u32), Instruction::Leading_Zeros_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_i64() {
        assert_eq!(
            Instruction::Leading_Zeros_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719555u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_u8() {
        assert_eq!(
            Instruction::decode(929539u32), Instruction::Leading_Zeros_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_u8() {
        assert_eq!(
            Instruction::Leading_Zeros_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929539u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_u16() {
        assert_eq!(
            Instruction::decode(269364995u32), Instruction::Leading_Zeros_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_u16() {
        assert_eq!(
            Instruction::Leading_Zeros_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364995u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_u32() {
        assert_eq!(
            Instruction::decode(537800451u32), Instruction::Leading_Zeros_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_u32() {
        assert_eq!(
            Instruction::Leading_Zeros_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800451u32
        );
    }
    #[test]
    fn decode_Leading_Zeros_u64() {
        assert_eq!(
            Instruction::decode(806235907u32), Instruction::Leading_Zeros_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Leading_Zeros_u64() {
        assert_eq!(
            Instruction::Leading_Zeros_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235907u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_i8() {
        assert_eq!(
            Instruction::decode(2148413251u32), Instruction::Trailing_Zeros_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_i8() {
        assert_eq!(
            Instruction::Trailing_Zeros_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148413251u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_i16() {
        assert_eq!(
            Instruction::decode(2416848707u32), Instruction::Trailing_Zeros_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_i16() {
        assert_eq!(
            Instruction::Trailing_Zeros_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848707u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_i32() {
        assert_eq!(
            Instruction::decode(2685284163u32), Instruction::Trailing_Zeros_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_i32() {
        assert_eq!(
            Instruction::Trailing_Zeros_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685284163u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_i64() {
        assert_eq!(
            Instruction::decode(2953719619u32), Instruction::Trailing_Zeros_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_i64() {
        assert_eq!(
            Instruction::Trailing_Zeros_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719619u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_u8() {
        assert_eq!(
            Instruction::decode(929603u32), Instruction::Trailing_Zeros_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_u8() {
        assert_eq!(
            Instruction::Trailing_Zeros_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929603u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_u16() {
        assert_eq!(
            Instruction::decode(269365059u32), Instruction::Trailing_Zeros_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_u16() {
        assert_eq!(
            Instruction::Trailing_Zeros_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269365059u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_u32() {
        assert_eq!(
            Instruction::decode(537800515u32), Instruction::Trailing_Zeros_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_u32() {
        assert_eq!(
            Instruction::Trailing_Zeros_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800515u32
        );
    }
    #[test]
    fn decode_Trailing_Zeros_u64() {
        assert_eq!(
            Instruction::decode(806235971u32), Instruction::Trailing_Zeros_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Trailing_Zeros_u64() {
        assert_eq!(
            Instruction::Trailing_Zeros_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235971u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_i8() {
        assert_eq!(
            Instruction::decode(2148413315u32), Instruction::Reverse_Bytes_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_i8() {
        assert_eq!(
            Instruction::Reverse_Bytes_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148413315u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_i16() {
        assert_eq!(
            Instruction::decode(2416848771u32), Instruction::Reverse_Bytes_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_i16() {
        assert_eq!(
            Instruction::Reverse_Bytes_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848771u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_i32() {
        assert_eq!(
            Instruction::decode(2685284227u32), Instruction::Reverse_Bytes_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_i32() {
        assert_eq!(
            Instruction::Reverse_Bytes_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685284227u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_i64() {
        assert_eq!(
            Instruction::decode(2953719683u32), Instruction::Reverse_Bytes_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_i64() {
        assert_eq!(
            Instruction::Reverse_Bytes_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719683u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_u8() {
        assert_eq!(
            Instruction::decode(929667u32), Instruction::Reverse_Bytes_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_u8() {
        assert_eq!(
            Instruction::Reverse_Bytes_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929667u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_u16() {
        assert_eq!(
            Instruction::decode(269365123u32), Instruction::Reverse_Bytes_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_u16() {
        assert_eq!(
            Instruction::Reverse_Bytes_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269365123u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_u32() {
        assert_eq!(
            Instruction::decode(537800579u32), Instruction::Reverse_Bytes_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_u32() {
        assert_eq!(
            Instruction::Reverse_Bytes_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800579u32
        );
    }
    #[test]
    fn decode_Reverse_Bytes_u64() {
        assert_eq!(
            Instruction::decode(806236035u32), Instruction::Reverse_Bytes_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bytes_u64() {
        assert_eq!(
            Instruction::Reverse_Bytes_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806236035u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_i8() {
        assert_eq!(
            Instruction::decode(2148413379u32), Instruction::Reverse_Bits_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_i8() {
        assert_eq!(
            Instruction::Reverse_Bits_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148413379u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_i16() {
        assert_eq!(
            Instruction::decode(2416848835u32), Instruction::Reverse_Bits_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_i16() {
        assert_eq!(
            Instruction::Reverse_Bits_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848835u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_i32() {
        assert_eq!(
            Instruction::decode(2685284291u32), Instruction::Reverse_Bits_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_i32() {
        assert_eq!(
            Instruction::Reverse_Bits_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685284291u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_i64() {
        assert_eq!(
            Instruction::decode(2953719747u32), Instruction::Reverse_Bits_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_i64() {
        assert_eq!(
            Instruction::Reverse_Bits_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719747u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_u8() {
        assert_eq!(
            Instruction::decode(929731u32), Instruction::Reverse_Bits_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_u8() {
        assert_eq!(
            Instruction::Reverse_Bits_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929731u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_u16() {
        assert_eq!(
            Instruction::decode(269365187u32), Instruction::Reverse_Bits_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_u16() {
        assert_eq!(
            Instruction::Reverse_Bits_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269365187u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_u32() {
        assert_eq!(
            Instruction::decode(537800643u32), Instruction::Reverse_Bits_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_u32() {
        assert_eq!(
            Instruction::Reverse_Bits_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800643u32
        );
    }
    #[test]
    fn decode_Reverse_Bits_u64() {
        assert_eq!(
            Instruction::decode(806236099u32), Instruction::Reverse_Bits_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Reverse_Bits_u64() {
        assert_eq!(
            Instruction::Reverse_Bits_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806236099u32
        );
    }
    #[test]
    fn decode_C_Abs_i8() {
        assert_eq!(
            Instruction::decode(2148412420u32), Instruction::C_Abs_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Abs_i8() {
        assert_eq!(
            Instruction::C_Abs_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412420u32
        );
    }
    #[test]
    fn decode_C_Abs_i16() {
        assert_eq!(
            Instruction::decode(2416847876u32), Instruction::C_Abs_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Abs_i16() {
        assert_eq!(
            Instruction::C_Abs_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416847876u32
        );
    }
    #[test]
    fn decode_C_Abs_i32() {
        assert_eq!(
            Instruction::decode(2685283332u32), Instruction::C_Abs_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Abs_i32() {
        assert_eq!(
            Instruction::C_Abs_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283332u32
        );
    }
    #[test]
    fn decode_C_Abs_i64() {
        assert_eq!(
            Instruction::decode(2953718788u32), Instruction::C_Abs_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Abs_i64() {
        assert_eq!(
            Instruction::C_Abs_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953718788u32
        );
    }
    #[test]
    fn decode_C_Add_i8() {
        assert_eq!(
            Instruction::decode(2198744132u32), Instruction::C_Add_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_i8() {
        assert_eq!(
            Instruction::C_Add_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744132u32
        );
    }
    #[test]
    fn decode_C_Add_i16() {
        assert_eq!(
            Instruction::decode(2467179588u32), Instruction::C_Add_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_i16() {
        assert_eq!(
            Instruction::C_Add_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179588u32
        );
    }
    #[test]
    fn decode_C_Add_i32() {
        assert_eq!(
            Instruction::decode(2735615044u32), Instruction::C_Add_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_i32() {
        assert_eq!(
            Instruction::C_Add_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615044u32
        );
    }
    #[test]
    fn decode_C_Add_i64() {
        assert_eq!(
            Instruction::decode(3004050500u32), Instruction::C_Add_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_i64() {
        assert_eq!(
            Instruction::C_Add_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050500u32
        );
    }
    #[test]
    fn decode_C_Add_u8() {
        assert_eq!(
            Instruction::decode(51260484u32), Instruction::C_Add_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_u8() {
        assert_eq!(
            Instruction::C_Add_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260484u32
        );
    }
    #[test]
    fn decode_C_Add_u16() {
        assert_eq!(
            Instruction::decode(319695940u32), Instruction::C_Add_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_u16() {
        assert_eq!(
            Instruction::C_Add_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319695940u32
        );
    }
    #[test]
    fn decode_C_Add_u32() {
        assert_eq!(
            Instruction::decode(588131396u32), Instruction::C_Add_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_u32() {
        assert_eq!(
            Instruction::C_Add_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131396u32
        );
    }
    #[test]
    fn decode_C_Add_u64() {
        assert_eq!(
            Instruction::decode(856566852u32), Instruction::C_Add_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_u64() {
        assert_eq!(
            Instruction::C_Add_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856566852u32
        );
    }
    #[test]
    fn decode_C_Add_U_i8() {
        assert_eq!(
            Instruction::decode(2198744196u32), Instruction::C_Add_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_U_i8() {
        assert_eq!(
            Instruction::C_Add_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744196u32
        );
    }
    #[test]
    fn decode_C_Add_U_i16() {
        assert_eq!(
            Instruction::decode(2467179652u32), Instruction::C_Add_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_U_i16() {
        assert_eq!(
            Instruction::C_Add_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179652u32
        );
    }
    #[test]
    fn decode_C_Add_U_i32() {
        assert_eq!(
            Instruction::decode(2735615108u32), Instruction::C_Add_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_U_i32() {
        assert_eq!(
            Instruction::C_Add_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615108u32
        );
    }
    #[test]
    fn decode_C_Add_U_i64() {
        assert_eq!(
            Instruction::decode(3004050564u32), Instruction::C_Add_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_U_i64() {
        assert_eq!(
            Instruction::C_Add_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050564u32
        );
    }
    #[test]
    fn decode_C_Add_S_u8() {
        assert_eq!(
            Instruction::decode(51260548u32), Instruction::C_Add_S_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_S_u8() {
        assert_eq!(
            Instruction::C_Add_S_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260548u32
        );
    }
    #[test]
    fn decode_C_Add_S_u16() {
        assert_eq!(
            Instruction::decode(319696004u32), Instruction::C_Add_S_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_S_u16() {
        assert_eq!(
            Instruction::C_Add_S_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696004u32
        );
    }
    #[test]
    fn decode_C_Add_S_u32() {
        assert_eq!(
            Instruction::decode(588131460u32), Instruction::C_Add_S_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_S_u32() {
        assert_eq!(
            Instruction::C_Add_S_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131460u32
        );
    }
    #[test]
    fn decode_C_Add_S_u64() {
        assert_eq!(
            Instruction::decode(856566916u32), Instruction::C_Add_S_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Add_S_u64() {
        assert_eq!(
            Instruction::C_Add_S_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856566916u32
        );
    }
    #[test]
    fn decode_C_Div_i8() {
        assert_eq!(
            Instruction::decode(2198744260u32), Instruction::C_Div_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_i8() {
        assert_eq!(
            Instruction::C_Div_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744260u32
        );
    }
    #[test]
    fn decode_C_Div_i16() {
        assert_eq!(
            Instruction::decode(2467179716u32), Instruction::C_Div_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_i16() {
        assert_eq!(
            Instruction::C_Div_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179716u32
        );
    }
    #[test]
    fn decode_C_Div_i32() {
        assert_eq!(
            Instruction::decode(2735615172u32), Instruction::C_Div_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_i32() {
        assert_eq!(
            Instruction::C_Div_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615172u32
        );
    }
    #[test]
    fn decode_C_Div_i64() {
        assert_eq!(
            Instruction::decode(3004050628u32), Instruction::C_Div_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_i64() {
        assert_eq!(
            Instruction::C_Div_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050628u32
        );
    }
    #[test]
    fn decode_C_Div_u8() {
        assert_eq!(
            Instruction::decode(51260612u32), Instruction::C_Div_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_u8() {
        assert_eq!(
            Instruction::C_Div_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260612u32
        );
    }
    #[test]
    fn decode_C_Div_u16() {
        assert_eq!(
            Instruction::decode(319696068u32), Instruction::C_Div_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_u16() {
        assert_eq!(
            Instruction::C_Div_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696068u32
        );
    }
    #[test]
    fn decode_C_Div_u32() {
        assert_eq!(
            Instruction::decode(588131524u32), Instruction::C_Div_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_u32() {
        assert_eq!(
            Instruction::C_Div_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131524u32
        );
    }
    #[test]
    fn decode_C_Div_u64() {
        assert_eq!(
            Instruction::decode(856566980u32), Instruction::C_Div_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_u64() {
        assert_eq!(
            Instruction::C_Div_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856566980u32
        );
    }
    #[test]
    fn decode_C_Div_E_i8() {
        assert_eq!(
            Instruction::decode(2198744324u32), Instruction::C_Div_E_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_i8() {
        assert_eq!(
            Instruction::C_Div_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744324u32
        );
    }
    #[test]
    fn decode_C_Div_E_i16() {
        assert_eq!(
            Instruction::decode(2467179780u32), Instruction::C_Div_E_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_i16() {
        assert_eq!(
            Instruction::C_Div_E_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179780u32
        );
    }
    #[test]
    fn decode_C_Div_E_i32() {
        assert_eq!(
            Instruction::decode(2735615236u32), Instruction::C_Div_E_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_i32() {
        assert_eq!(
            Instruction::C_Div_E_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615236u32
        );
    }
    #[test]
    fn decode_C_Div_E_i64() {
        assert_eq!(
            Instruction::decode(3004050692u32), Instruction::C_Div_E_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_i64() {
        assert_eq!(
            Instruction::C_Div_E_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050692u32
        );
    }
    #[test]
    fn decode_C_Div_E_u8() {
        assert_eq!(
            Instruction::decode(51260676u32), Instruction::C_Div_E_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_u8() {
        assert_eq!(
            Instruction::C_Div_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260676u32
        );
    }
    #[test]
    fn decode_C_Div_E_u16() {
        assert_eq!(
            Instruction::decode(319696132u32), Instruction::C_Div_E_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_u16() {
        assert_eq!(
            Instruction::C_Div_E_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696132u32
        );
    }
    #[test]
    fn decode_C_Div_E_u32() {
        assert_eq!(
            Instruction::decode(588131588u32), Instruction::C_Div_E_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_u32() {
        assert_eq!(
            Instruction::C_Div_E_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131588u32
        );
    }
    #[test]
    fn decode_C_Div_E_u64() {
        assert_eq!(
            Instruction::decode(856567044u32), Instruction::C_Div_E_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Div_E_u64() {
        assert_eq!(
            Instruction::C_Div_E_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567044u32
        );
    }
    #[test]
    fn decode_C_Log_i8() {
        assert_eq!(
            Instruction::decode(2198744388u32), Instruction::C_Log_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_i8() {
        assert_eq!(
            Instruction::C_Log_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744388u32
        );
    }
    #[test]
    fn decode_C_Log_i16() {
        assert_eq!(
            Instruction::decode(2467179844u32), Instruction::C_Log_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_i16() {
        assert_eq!(
            Instruction::C_Log_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179844u32
        );
    }
    #[test]
    fn decode_C_Log_i32() {
        assert_eq!(
            Instruction::decode(2735615300u32), Instruction::C_Log_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_i32() {
        assert_eq!(
            Instruction::C_Log_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615300u32
        );
    }
    #[test]
    fn decode_C_Log_i64() {
        assert_eq!(
            Instruction::decode(3004050756u32), Instruction::C_Log_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_i64() {
        assert_eq!(
            Instruction::C_Log_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050756u32
        );
    }
    #[test]
    fn decode_C_Log_u8() {
        assert_eq!(
            Instruction::decode(51260740u32), Instruction::C_Log_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_u8() {
        assert_eq!(
            Instruction::C_Log_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260740u32
        );
    }
    #[test]
    fn decode_C_Log_u16() {
        assert_eq!(
            Instruction::decode(319696196u32), Instruction::C_Log_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_u16() {
        assert_eq!(
            Instruction::C_Log_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696196u32
        );
    }
    #[test]
    fn decode_C_Log_u32() {
        assert_eq!(
            Instruction::decode(588131652u32), Instruction::C_Log_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_u32() {
        assert_eq!(
            Instruction::C_Log_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131652u32
        );
    }
    #[test]
    fn decode_C_Log_u64() {
        assert_eq!(
            Instruction::decode(856567108u32), Instruction::C_Log_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Log_u64() {
        assert_eq!(
            Instruction::C_Log_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567108u32
        );
    }
    #[test]
    fn decode_C_Sqrt_i8() {
        assert_eq!(
            Instruction::decode(2148412804u32), Instruction::C_Sqrt_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_i8() {
        assert_eq!(
            Instruction::C_Sqrt_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412804u32
        );
    }
    #[test]
    fn decode_C_Sqrt_i16() {
        assert_eq!(
            Instruction::decode(2416848260u32), Instruction::C_Sqrt_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_i16() {
        assert_eq!(
            Instruction::C_Sqrt_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848260u32
        );
    }
    #[test]
    fn decode_C_Sqrt_i32() {
        assert_eq!(
            Instruction::decode(2685283716u32), Instruction::C_Sqrt_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_i32() {
        assert_eq!(
            Instruction::C_Sqrt_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283716u32
        );
    }
    #[test]
    fn decode_C_Sqrt_i64() {
        assert_eq!(
            Instruction::decode(2953719172u32), Instruction::C_Sqrt_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_i64() {
        assert_eq!(
            Instruction::C_Sqrt_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719172u32
        );
    }
    #[test]
    fn decode_C_Sqrt_u8() {
        assert_eq!(
            Instruction::decode(929156u32), Instruction::C_Sqrt_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_u8() {
        assert_eq!(
            Instruction::C_Sqrt_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929156u32
        );
    }
    #[test]
    fn decode_C_Sqrt_u16() {
        assert_eq!(
            Instruction::decode(269364612u32), Instruction::C_Sqrt_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_u16() {
        assert_eq!(
            Instruction::C_Sqrt_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364612u32
        );
    }
    #[test]
    fn decode_C_Sqrt_u32() {
        assert_eq!(
            Instruction::decode(537800068u32), Instruction::C_Sqrt_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_u32() {
        assert_eq!(
            Instruction::C_Sqrt_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800068u32
        );
    }
    #[test]
    fn decode_C_Sqrt_u64() {
        assert_eq!(
            Instruction::decode(806235524u32), Instruction::C_Sqrt_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Sqrt_u64() {
        assert_eq!(
            Instruction::C_Sqrt_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235524u32
        );
    }
    #[test]
    fn decode_C_Mul_i8() {
        assert_eq!(
            Instruction::decode(2198744516u32), Instruction::C_Mul_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_i8() {
        assert_eq!(
            Instruction::C_Mul_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744516u32
        );
    }
    #[test]
    fn decode_C_Mul_i16() {
        assert_eq!(
            Instruction::decode(2467179972u32), Instruction::C_Mul_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_i16() {
        assert_eq!(
            Instruction::C_Mul_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179972u32
        );
    }
    #[test]
    fn decode_C_Mul_i32() {
        assert_eq!(
            Instruction::decode(2735615428u32), Instruction::C_Mul_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_i32() {
        assert_eq!(
            Instruction::C_Mul_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615428u32
        );
    }
    #[test]
    fn decode_C_Mul_i64() {
        assert_eq!(
            Instruction::decode(3004050884u32), Instruction::C_Mul_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_i64() {
        assert_eq!(
            Instruction::C_Mul_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050884u32
        );
    }
    #[test]
    fn decode_C_Mul_u8() {
        assert_eq!(
            Instruction::decode(51260868u32), Instruction::C_Mul_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_u8() {
        assert_eq!(
            Instruction::C_Mul_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260868u32
        );
    }
    #[test]
    fn decode_C_Mul_u16() {
        assert_eq!(
            Instruction::decode(319696324u32), Instruction::C_Mul_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_u16() {
        assert_eq!(
            Instruction::C_Mul_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696324u32
        );
    }
    #[test]
    fn decode_C_Mul_u32() {
        assert_eq!(
            Instruction::decode(588131780u32), Instruction::C_Mul_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_u32() {
        assert_eq!(
            Instruction::C_Mul_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131780u32
        );
    }
    #[test]
    fn decode_C_Mul_u64() {
        assert_eq!(
            Instruction::decode(856567236u32), Instruction::C_Mul_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Mul_u64() {
        assert_eq!(
            Instruction::C_Mul_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567236u32
        );
    }
    #[test]
    fn decode_C_Neg_i8() {
        assert_eq!(
            Instruction::decode(2148412932u32), Instruction::C_Neg_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Neg_i8() {
        assert_eq!(
            Instruction::C_Neg_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412932u32
        );
    }
    #[test]
    fn decode_C_Neg_i16() {
        assert_eq!(
            Instruction::decode(2416848388u32), Instruction::C_Neg_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Neg_i16() {
        assert_eq!(
            Instruction::C_Neg_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848388u32
        );
    }
    #[test]
    fn decode_C_Neg_i32() {
        assert_eq!(
            Instruction::decode(2685283844u32), Instruction::C_Neg_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Neg_i32() {
        assert_eq!(
            Instruction::C_Neg_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283844u32
        );
    }
    #[test]
    fn decode_C_Neg_i64() {
        assert_eq!(
            Instruction::decode(2953719300u32), Instruction::C_Neg_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_C_Neg_i64() {
        assert_eq!(
            Instruction::C_Neg_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719300u32
        );
    }
    #[test]
    fn decode_C_Pow_i8() {
        assert_eq!(
            Instruction::decode(2198744644u32), Instruction::C_Pow_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_i8() {
        assert_eq!(
            Instruction::C_Pow_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744644u32
        );
    }
    #[test]
    fn decode_C_Pow_i16() {
        assert_eq!(
            Instruction::decode(2467180100u32), Instruction::C_Pow_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_i16() {
        assert_eq!(
            Instruction::C_Pow_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180100u32
        );
    }
    #[test]
    fn decode_C_Pow_i32() {
        assert_eq!(
            Instruction::decode(2735615556u32), Instruction::C_Pow_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_i32() {
        assert_eq!(
            Instruction::C_Pow_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615556u32
        );
    }
    #[test]
    fn decode_C_Pow_i64() {
        assert_eq!(
            Instruction::decode(3004051012u32), Instruction::C_Pow_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_i64() {
        assert_eq!(
            Instruction::C_Pow_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051012u32
        );
    }
    #[test]
    fn decode_C_Pow_u8() {
        assert_eq!(
            Instruction::decode(51260996u32), Instruction::C_Pow_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_u8() {
        assert_eq!(
            Instruction::C_Pow_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260996u32
        );
    }
    #[test]
    fn decode_C_Pow_u16() {
        assert_eq!(
            Instruction::decode(319696452u32), Instruction::C_Pow_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_u16() {
        assert_eq!(
            Instruction::C_Pow_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696452u32
        );
    }
    #[test]
    fn decode_C_Pow_u32() {
        assert_eq!(
            Instruction::decode(588131908u32), Instruction::C_Pow_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_u32() {
        assert_eq!(
            Instruction::C_Pow_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131908u32
        );
    }
    #[test]
    fn decode_C_Pow_u64() {
        assert_eq!(
            Instruction::decode(856567364u32), Instruction::C_Pow_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Pow_u64() {
        assert_eq!(
            Instruction::C_Pow_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567364u32
        );
    }
    #[test]
    fn decode_C_Rem_i8() {
        assert_eq!(
            Instruction::decode(2198744708u32), Instruction::C_Rem_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_i8() {
        assert_eq!(
            Instruction::C_Rem_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744708u32
        );
    }
    #[test]
    fn decode_C_Rem_i16() {
        assert_eq!(
            Instruction::decode(2467180164u32), Instruction::C_Rem_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_i16() {
        assert_eq!(
            Instruction::C_Rem_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180164u32
        );
    }
    #[test]
    fn decode_C_Rem_i32() {
        assert_eq!(
            Instruction::decode(2735615620u32), Instruction::C_Rem_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_i32() {
        assert_eq!(
            Instruction::C_Rem_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615620u32
        );
    }
    #[test]
    fn decode_C_Rem_i64() {
        assert_eq!(
            Instruction::decode(3004051076u32), Instruction::C_Rem_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_i64() {
        assert_eq!(
            Instruction::C_Rem_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051076u32
        );
    }
    #[test]
    fn decode_C_Rem_u8() {
        assert_eq!(
            Instruction::decode(51261060u32), Instruction::C_Rem_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_u8() {
        assert_eq!(
            Instruction::C_Rem_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51261060u32
        );
    }
    #[test]
    fn decode_C_Rem_u16() {
        assert_eq!(
            Instruction::decode(319696516u32), Instruction::C_Rem_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_u16() {
        assert_eq!(
            Instruction::C_Rem_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696516u32
        );
    }
    #[test]
    fn decode_C_Rem_u32() {
        assert_eq!(
            Instruction::decode(588131972u32), Instruction::C_Rem_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_u32() {
        assert_eq!(
            Instruction::C_Rem_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131972u32
        );
    }
    #[test]
    fn decode_C_Rem_u64() {
        assert_eq!(
            Instruction::decode(856567428u32), Instruction::C_Rem_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_u64() {
        assert_eq!(
            Instruction::C_Rem_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567428u32
        );
    }
    #[test]
    fn decode_C_Rem_E_i8() {
        assert_eq!(
            Instruction::decode(2198744772u32), Instruction::C_Rem_E_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_i8() {
        assert_eq!(
            Instruction::C_Rem_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744772u32
        );
    }
    #[test]
    fn decode_C_Rem_E_i16() {
        assert_eq!(
            Instruction::decode(2467180228u32), Instruction::C_Rem_E_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_i16() {
        assert_eq!(
            Instruction::C_Rem_E_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180228u32
        );
    }
    #[test]
    fn decode_C_Rem_E_i32() {
        assert_eq!(
            Instruction::decode(2735615684u32), Instruction::C_Rem_E_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_i32() {
        assert_eq!(
            Instruction::C_Rem_E_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615684u32
        );
    }
    #[test]
    fn decode_C_Rem_E_i64() {
        assert_eq!(
            Instruction::decode(3004051140u32), Instruction::C_Rem_E_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_i64() {
        assert_eq!(
            Instruction::C_Rem_E_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051140u32
        );
    }
    #[test]
    fn decode_C_Rem_E_u8() {
        assert_eq!(
            Instruction::decode(51261124u32), Instruction::C_Rem_E_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_u8() {
        assert_eq!(
            Instruction::C_Rem_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51261124u32
        );
    }
    #[test]
    fn decode_C_Rem_E_u16() {
        assert_eq!(
            Instruction::decode(319696580u32), Instruction::C_Rem_E_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_u16() {
        assert_eq!(
            Instruction::C_Rem_E_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696580u32
        );
    }
    #[test]
    fn decode_C_Rem_E_u32() {
        assert_eq!(
            Instruction::decode(588132036u32), Instruction::C_Rem_E_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_u32() {
        assert_eq!(
            Instruction::C_Rem_E_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588132036u32
        );
    }
    #[test]
    fn decode_C_Rem_E_u64() {
        assert_eq!(
            Instruction::decode(856567492u32), Instruction::C_Rem_E_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Rem_E_u64() {
        assert_eq!(
            Instruction::C_Rem_E_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567492u32
        );
    }
    #[test]
    fn decode_C_Shl_i8() {
        assert_eq!(
            Instruction::decode(2198744836u32), Instruction::C_Shl_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_i8() {
        assert_eq!(
            Instruction::C_Shl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744836u32
        );
    }
    #[test]
    fn decode_C_Shl_i16() {
        assert_eq!(
            Instruction::decode(2467180292u32), Instruction::C_Shl_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_i16() {
        assert_eq!(
            Instruction::C_Shl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180292u32
        );
    }
    #[test]
    fn decode_C_Shl_i32() {
        assert_eq!(
            Instruction::decode(2735615748u32), Instruction::C_Shl_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_i32() {
        assert_eq!(
            Instruction::C_Shl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615748u32
        );
    }
    #[test]
    fn decode_C_Shl_i64() {
        assert_eq!(
            Instruction::decode(3004051204u32), Instruction::C_Shl_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_i64() {
        assert_eq!(
            Instruction::C_Shl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051204u32
        );
    }
    #[test]
    fn decode_C_Shl_u8() {
        assert_eq!(
            Instruction::decode(51261188u32), Instruction::C_Shl_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_u8() {
        assert_eq!(
            Instruction::C_Shl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51261188u32
        );
    }
    #[test]
    fn decode_C_Shl_u16() {
        assert_eq!(
            Instruction::decode(319696644u32), Instruction::C_Shl_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_u16() {
        assert_eq!(
            Instruction::C_Shl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696644u32
        );
    }
    #[test]
    fn decode_C_Shl_u32() {
        assert_eq!(
            Instruction::decode(588132100u32), Instruction::C_Shl_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_u32() {
        assert_eq!(
            Instruction::C_Shl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588132100u32
        );
    }
    #[test]
    fn decode_C_Shl_u64() {
        assert_eq!(
            Instruction::decode(856567556u32), Instruction::C_Shl_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shl_u64() {
        assert_eq!(
            Instruction::C_Shl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567556u32
        );
    }
    #[test]
    fn decode_C_Shr_i8() {
        assert_eq!(
            Instruction::decode(2198744900u32), Instruction::C_Shr_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_i8() {
        assert_eq!(
            Instruction::C_Shr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744900u32
        );
    }
    #[test]
    fn decode_C_Shr_i16() {
        assert_eq!(
            Instruction::decode(2467180356u32), Instruction::C_Shr_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_i16() {
        assert_eq!(
            Instruction::C_Shr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180356u32
        );
    }
    #[test]
    fn decode_C_Shr_i32() {
        assert_eq!(
            Instruction::decode(2735615812u32), Instruction::C_Shr_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_i32() {
        assert_eq!(
            Instruction::C_Shr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615812u32
        );
    }
    #[test]
    fn decode_C_Shr_i64() {
        assert_eq!(
            Instruction::decode(3004051268u32), Instruction::C_Shr_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_i64() {
        assert_eq!(
            Instruction::C_Shr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051268u32
        );
    }
    #[test]
    fn decode_C_Shr_u8() {
        assert_eq!(
            Instruction::decode(51261252u32), Instruction::C_Shr_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_u8() {
        assert_eq!(
            Instruction::C_Shr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51261252u32
        );
    }
    #[test]
    fn decode_C_Shr_u16() {
        assert_eq!(
            Instruction::decode(319696708u32), Instruction::C_Shr_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_u16() {
        assert_eq!(
            Instruction::C_Shr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696708u32
        );
    }
    #[test]
    fn decode_C_Shr_u32() {
        assert_eq!(
            Instruction::decode(588132164u32), Instruction::C_Shr_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_u32() {
        assert_eq!(
            Instruction::C_Shr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588132164u32
        );
    }
    #[test]
    fn decode_C_Shr_u64() {
        assert_eq!(
            Instruction::decode(856567620u32), Instruction::C_Shr_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Shr_u64() {
        assert_eq!(
            Instruction::C_Shr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567620u32
        );
    }
    #[test]
    fn decode_C_Sub_i8() {
        assert_eq!(
            Instruction::decode(2198744964u32), Instruction::C_Sub_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_i8() {
        assert_eq!(
            Instruction::C_Sub_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744964u32
        );
    }
    #[test]
    fn decode_C_Sub_i16() {
        assert_eq!(
            Instruction::decode(2467180420u32), Instruction::C_Sub_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_i16() {
        assert_eq!(
            Instruction::C_Sub_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180420u32
        );
    }
    #[test]
    fn decode_C_Sub_i32() {
        assert_eq!(
            Instruction::decode(2735615876u32), Instruction::C_Sub_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_i32() {
        assert_eq!(
            Instruction::C_Sub_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615876u32
        );
    }
    #[test]
    fn decode_C_Sub_i64() {
        assert_eq!(
            Instruction::decode(3004051332u32), Instruction::C_Sub_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_i64() {
        assert_eq!(
            Instruction::C_Sub_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051332u32
        );
    }
    #[test]
    fn decode_C_Sub_u8() {
        assert_eq!(
            Instruction::decode(51261316u32), Instruction::C_Sub_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_u8() {
        assert_eq!(
            Instruction::C_Sub_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51261316u32
        );
    }
    #[test]
    fn decode_C_Sub_u16() {
        assert_eq!(
            Instruction::decode(319696772u32), Instruction::C_Sub_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_u16() {
        assert_eq!(
            Instruction::C_Sub_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696772u32
        );
    }
    #[test]
    fn decode_C_Sub_u32() {
        assert_eq!(
            Instruction::decode(588132228u32), Instruction::C_Sub_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_u32() {
        assert_eq!(
            Instruction::C_Sub_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588132228u32
        );
    }
    #[test]
    fn decode_C_Sub_u64() {
        assert_eq!(
            Instruction::decode(856567684u32), Instruction::C_Sub_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_u64() {
        assert_eq!(
            Instruction::C_Sub_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567684u32
        );
    }
    #[test]
    fn decode_C_Sub_U_i8() {
        assert_eq!(
            Instruction::decode(2198745028u32), Instruction::C_Sub_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_U_i8() {
        assert_eq!(
            Instruction::C_Sub_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198745028u32
        );
    }
    #[test]
    fn decode_C_Sub_U_i16() {
        assert_eq!(
            Instruction::decode(2467180484u32), Instruction::C_Sub_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_U_i16() {
        assert_eq!(
            Instruction::C_Sub_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180484u32
        );
    }
    #[test]
    fn decode_C_Sub_U_i32() {
        assert_eq!(
            Instruction::decode(2735615940u32), Instruction::C_Sub_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_U_i32() {
        assert_eq!(
            Instruction::C_Sub_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615940u32
        );
    }
    #[test]
    fn decode_C_Sub_U_i64() {
        assert_eq!(
            Instruction::decode(3004051396u32), Instruction::C_Sub_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_C_Sub_U_i64() {
        assert_eq!(
            Instruction::C_Sub_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051396u32
        );
    }
    #[test]
    fn decode_O_Abs_i8() {
        assert_eq!(
            Instruction::decode(2148412421u32), Instruction::O_Abs_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Abs_i8() {
        assert_eq!(
            Instruction::O_Abs_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412421u32
        );
    }
    #[test]
    fn decode_O_Abs_i16() {
        assert_eq!(
            Instruction::decode(2416847877u32), Instruction::O_Abs_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Abs_i16() {
        assert_eq!(
            Instruction::O_Abs_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416847877u32
        );
    }
    #[test]
    fn decode_O_Abs_i32() {
        assert_eq!(
            Instruction::decode(2685283333u32), Instruction::O_Abs_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Abs_i32() {
        assert_eq!(
            Instruction::O_Abs_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283333u32
        );
    }
    #[test]
    fn decode_O_Abs_i64() {
        assert_eq!(
            Instruction::decode(2953718789u32), Instruction::O_Abs_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Abs_i64() {
        assert_eq!(
            Instruction::O_Abs_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953718789u32
        );
    }
    #[test]
    fn decode_O_Add_i8() {
        assert_eq!(
            Instruction::decode(2198744133u32), Instruction::O_Add_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_i8() {
        assert_eq!(
            Instruction::O_Add_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744133u32
        );
    }
    #[test]
    fn decode_O_Add_i16() {
        assert_eq!(
            Instruction::decode(2467179589u32), Instruction::O_Add_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_i16() {
        assert_eq!(
            Instruction::O_Add_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179589u32
        );
    }
    #[test]
    fn decode_O_Add_i32() {
        assert_eq!(
            Instruction::decode(2735615045u32), Instruction::O_Add_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_i32() {
        assert_eq!(
            Instruction::O_Add_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615045u32
        );
    }
    #[test]
    fn decode_O_Add_i64() {
        assert_eq!(
            Instruction::decode(3004050501u32), Instruction::O_Add_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_i64() {
        assert_eq!(
            Instruction::O_Add_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050501u32
        );
    }
    #[test]
    fn decode_O_Add_u8() {
        assert_eq!(
            Instruction::decode(51260485u32), Instruction::O_Add_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_u8() {
        assert_eq!(
            Instruction::O_Add_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260485u32
        );
    }
    #[test]
    fn decode_O_Add_u16() {
        assert_eq!(
            Instruction::decode(319695941u32), Instruction::O_Add_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_u16() {
        assert_eq!(
            Instruction::O_Add_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319695941u32
        );
    }
    #[test]
    fn decode_O_Add_u32() {
        assert_eq!(
            Instruction::decode(588131397u32), Instruction::O_Add_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_u32() {
        assert_eq!(
            Instruction::O_Add_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131397u32
        );
    }
    #[test]
    fn decode_O_Add_u64() {
        assert_eq!(
            Instruction::decode(856566853u32), Instruction::O_Add_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_u64() {
        assert_eq!(
            Instruction::O_Add_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856566853u32
        );
    }
    #[test]
    fn decode_O_Add_U_i8() {
        assert_eq!(
            Instruction::decode(2198744197u32), Instruction::O_Add_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_U_i8() {
        assert_eq!(
            Instruction::O_Add_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744197u32
        );
    }
    #[test]
    fn decode_O_Add_U_i16() {
        assert_eq!(
            Instruction::decode(2467179653u32), Instruction::O_Add_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_U_i16() {
        assert_eq!(
            Instruction::O_Add_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179653u32
        );
    }
    #[test]
    fn decode_O_Add_U_i32() {
        assert_eq!(
            Instruction::decode(2735615109u32), Instruction::O_Add_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_U_i32() {
        assert_eq!(
            Instruction::O_Add_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615109u32
        );
    }
    #[test]
    fn decode_O_Add_U_i64() {
        assert_eq!(
            Instruction::decode(3004050565u32), Instruction::O_Add_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_U_i64() {
        assert_eq!(
            Instruction::O_Add_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050565u32
        );
    }
    #[test]
    fn decode_O_Add_S_u8() {
        assert_eq!(
            Instruction::decode(51260549u32), Instruction::O_Add_S_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_S_u8() {
        assert_eq!(
            Instruction::O_Add_S_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260549u32
        );
    }
    #[test]
    fn decode_O_Add_S_u16() {
        assert_eq!(
            Instruction::decode(319696005u32), Instruction::O_Add_S_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_S_u16() {
        assert_eq!(
            Instruction::O_Add_S_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696005u32
        );
    }
    #[test]
    fn decode_O_Add_S_u32() {
        assert_eq!(
            Instruction::decode(588131461u32), Instruction::O_Add_S_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_S_u32() {
        assert_eq!(
            Instruction::O_Add_S_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131461u32
        );
    }
    #[test]
    fn decode_O_Add_S_u64() {
        assert_eq!(
            Instruction::decode(856566917u32), Instruction::O_Add_S_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Add_S_u64() {
        assert_eq!(
            Instruction::O_Add_S_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856566917u32
        );
    }
    #[test]
    fn decode_O_Div_i8() {
        assert_eq!(
            Instruction::decode(2198744261u32), Instruction::O_Div_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_i8() {
        assert_eq!(
            Instruction::O_Div_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744261u32
        );
    }
    #[test]
    fn decode_O_Div_i16() {
        assert_eq!(
            Instruction::decode(2467179717u32), Instruction::O_Div_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_i16() {
        assert_eq!(
            Instruction::O_Div_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179717u32
        );
    }
    #[test]
    fn decode_O_Div_i32() {
        assert_eq!(
            Instruction::decode(2735615173u32), Instruction::O_Div_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_i32() {
        assert_eq!(
            Instruction::O_Div_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615173u32
        );
    }
    #[test]
    fn decode_O_Div_i64() {
        assert_eq!(
            Instruction::decode(3004050629u32), Instruction::O_Div_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_i64() {
        assert_eq!(
            Instruction::O_Div_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050629u32
        );
    }
    #[test]
    fn decode_O_Div_u8() {
        assert_eq!(
            Instruction::decode(51260613u32), Instruction::O_Div_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_u8() {
        assert_eq!(
            Instruction::O_Div_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260613u32
        );
    }
    #[test]
    fn decode_O_Div_u16() {
        assert_eq!(
            Instruction::decode(319696069u32), Instruction::O_Div_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_u16() {
        assert_eq!(
            Instruction::O_Div_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696069u32
        );
    }
    #[test]
    fn decode_O_Div_u32() {
        assert_eq!(
            Instruction::decode(588131525u32), Instruction::O_Div_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_u32() {
        assert_eq!(
            Instruction::O_Div_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131525u32
        );
    }
    #[test]
    fn decode_O_Div_u64() {
        assert_eq!(
            Instruction::decode(856566981u32), Instruction::O_Div_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_u64() {
        assert_eq!(
            Instruction::O_Div_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856566981u32
        );
    }
    #[test]
    fn decode_O_Div_E_i8() {
        assert_eq!(
            Instruction::decode(2198744325u32), Instruction::O_Div_E_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_i8() {
        assert_eq!(
            Instruction::O_Div_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744325u32
        );
    }
    #[test]
    fn decode_O_Div_E_i16() {
        assert_eq!(
            Instruction::decode(2467179781u32), Instruction::O_Div_E_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_i16() {
        assert_eq!(
            Instruction::O_Div_E_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179781u32
        );
    }
    #[test]
    fn decode_O_Div_E_i32() {
        assert_eq!(
            Instruction::decode(2735615237u32), Instruction::O_Div_E_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_i32() {
        assert_eq!(
            Instruction::O_Div_E_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615237u32
        );
    }
    #[test]
    fn decode_O_Div_E_i64() {
        assert_eq!(
            Instruction::decode(3004050693u32), Instruction::O_Div_E_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_i64() {
        assert_eq!(
            Instruction::O_Div_E_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050693u32
        );
    }
    #[test]
    fn decode_O_Div_E_u8() {
        assert_eq!(
            Instruction::decode(51260677u32), Instruction::O_Div_E_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_u8() {
        assert_eq!(
            Instruction::O_Div_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260677u32
        );
    }
    #[test]
    fn decode_O_Div_E_u16() {
        assert_eq!(
            Instruction::decode(319696133u32), Instruction::O_Div_E_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_u16() {
        assert_eq!(
            Instruction::O_Div_E_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696133u32
        );
    }
    #[test]
    fn decode_O_Div_E_u32() {
        assert_eq!(
            Instruction::decode(588131589u32), Instruction::O_Div_E_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_u32() {
        assert_eq!(
            Instruction::O_Div_E_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131589u32
        );
    }
    #[test]
    fn decode_O_Div_E_u64() {
        assert_eq!(
            Instruction::decode(856567045u32), Instruction::O_Div_E_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Div_E_u64() {
        assert_eq!(
            Instruction::O_Div_E_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567045u32
        );
    }
    #[test]
    fn decode_O_Mul_i8() {
        assert_eq!(
            Instruction::decode(2198744517u32), Instruction::O_Mul_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_i8() {
        assert_eq!(
            Instruction::O_Mul_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744517u32
        );
    }
    #[test]
    fn decode_O_Mul_i16() {
        assert_eq!(
            Instruction::decode(2467179973u32), Instruction::O_Mul_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_i16() {
        assert_eq!(
            Instruction::O_Mul_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179973u32
        );
    }
    #[test]
    fn decode_O_Mul_i32() {
        assert_eq!(
            Instruction::decode(2735615429u32), Instruction::O_Mul_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_i32() {
        assert_eq!(
            Instruction::O_Mul_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615429u32
        );
    }
    #[test]
    fn decode_O_Mul_i64() {
        assert_eq!(
            Instruction::decode(3004050885u32), Instruction::O_Mul_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_i64() {
        assert_eq!(
            Instruction::O_Mul_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050885u32
        );
    }
    #[test]
    fn decode_O_Mul_u8() {
        assert_eq!(
            Instruction::decode(51260869u32), Instruction::O_Mul_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_u8() {
        assert_eq!(
            Instruction::O_Mul_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260869u32
        );
    }
    #[test]
    fn decode_O_Mul_u16() {
        assert_eq!(
            Instruction::decode(319696325u32), Instruction::O_Mul_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_u16() {
        assert_eq!(
            Instruction::O_Mul_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696325u32
        );
    }
    #[test]
    fn decode_O_Mul_u32() {
        assert_eq!(
            Instruction::decode(588131781u32), Instruction::O_Mul_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_u32() {
        assert_eq!(
            Instruction::O_Mul_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131781u32
        );
    }
    #[test]
    fn decode_O_Mul_u64() {
        assert_eq!(
            Instruction::decode(856567237u32), Instruction::O_Mul_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Mul_u64() {
        assert_eq!(
            Instruction::O_Mul_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567237u32
        );
    }
    #[test]
    fn decode_O_Neg_i8() {
        assert_eq!(
            Instruction::decode(2148412933u32), Instruction::O_Neg_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Neg_i8() {
        assert_eq!(
            Instruction::O_Neg_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412933u32
        );
    }
    #[test]
    fn decode_O_Neg_i16() {
        assert_eq!(
            Instruction::decode(2416848389u32), Instruction::O_Neg_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Neg_i16() {
        assert_eq!(
            Instruction::O_Neg_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848389u32
        );
    }
    #[test]
    fn decode_O_Neg_i32() {
        assert_eq!(
            Instruction::decode(2685283845u32), Instruction::O_Neg_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Neg_i32() {
        assert_eq!(
            Instruction::O_Neg_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283845u32
        );
    }
    #[test]
    fn decode_O_Neg_i64() {
        assert_eq!(
            Instruction::decode(2953719301u32), Instruction::O_Neg_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_O_Neg_i64() {
        assert_eq!(
            Instruction::O_Neg_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719301u32
        );
    }
    #[test]
    fn decode_O_Pow_i8() {
        assert_eq!(
            Instruction::decode(2198744645u32), Instruction::O_Pow_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_i8() {
        assert_eq!(
            Instruction::O_Pow_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744645u32
        );
    }
    #[test]
    fn decode_O_Pow_i16() {
        assert_eq!(
            Instruction::decode(2467180101u32), Instruction::O_Pow_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_i16() {
        assert_eq!(
            Instruction::O_Pow_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180101u32
        );
    }
    #[test]
    fn decode_O_Pow_i32() {
        assert_eq!(
            Instruction::decode(2735615557u32), Instruction::O_Pow_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_i32() {
        assert_eq!(
            Instruction::O_Pow_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615557u32
        );
    }
    #[test]
    fn decode_O_Pow_i64() {
        assert_eq!(
            Instruction::decode(3004051013u32), Instruction::O_Pow_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_i64() {
        assert_eq!(
            Instruction::O_Pow_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051013u32
        );
    }
    #[test]
    fn decode_O_Pow_u8() {
        assert_eq!(
            Instruction::decode(51260997u32), Instruction::O_Pow_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_u8() {
        assert_eq!(
            Instruction::O_Pow_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260997u32
        );
    }
    #[test]
    fn decode_O_Pow_u16() {
        assert_eq!(
            Instruction::decode(319696453u32), Instruction::O_Pow_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_u16() {
        assert_eq!(
            Instruction::O_Pow_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696453u32
        );
    }
    #[test]
    fn decode_O_Pow_u32() {
        assert_eq!(
            Instruction::decode(588131909u32), Instruction::O_Pow_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_u32() {
        assert_eq!(
            Instruction::O_Pow_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131909u32
        );
    }
    #[test]
    fn decode_O_Pow_u64() {
        assert_eq!(
            Instruction::decode(856567365u32), Instruction::O_Pow_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Pow_u64() {
        assert_eq!(
            Instruction::O_Pow_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567365u32
        );
    }
    #[test]
    fn decode_O_Rem_i8() {
        assert_eq!(
            Instruction::decode(2198744709u32), Instruction::O_Rem_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_i8() {
        assert_eq!(
            Instruction::O_Rem_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744709u32
        );
    }
    #[test]
    fn decode_O_Rem_i16() {
        assert_eq!(
            Instruction::decode(2467180165u32), Instruction::O_Rem_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_i16() {
        assert_eq!(
            Instruction::O_Rem_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180165u32
        );
    }
    #[test]
    fn decode_O_Rem_i32() {
        assert_eq!(
            Instruction::decode(2735615621u32), Instruction::O_Rem_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_i32() {
        assert_eq!(
            Instruction::O_Rem_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615621u32
        );
    }
    #[test]
    fn decode_O_Rem_i64() {
        assert_eq!(
            Instruction::decode(3004051077u32), Instruction::O_Rem_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_i64() {
        assert_eq!(
            Instruction::O_Rem_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051077u32
        );
    }
    #[test]
    fn decode_O_Rem_u8() {
        assert_eq!(
            Instruction::decode(51261061u32), Instruction::O_Rem_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_u8() {
        assert_eq!(
            Instruction::O_Rem_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51261061u32
        );
    }
    #[test]
    fn decode_O_Rem_u16() {
        assert_eq!(
            Instruction::decode(319696517u32), Instruction::O_Rem_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_u16() {
        assert_eq!(
            Instruction::O_Rem_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696517u32
        );
    }
    #[test]
    fn decode_O_Rem_u32() {
        assert_eq!(
            Instruction::decode(588131973u32), Instruction::O_Rem_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_u32() {
        assert_eq!(
            Instruction::O_Rem_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131973u32
        );
    }
    #[test]
    fn decode_O_Rem_u64() {
        assert_eq!(
            Instruction::decode(856567429u32), Instruction::O_Rem_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_u64() {
        assert_eq!(
            Instruction::O_Rem_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567429u32
        );
    }
    #[test]
    fn decode_O_Rem_E_i8() {
        assert_eq!(
            Instruction::decode(2198744773u32), Instruction::O_Rem_E_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_i8() {
        assert_eq!(
            Instruction::O_Rem_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744773u32
        );
    }
    #[test]
    fn decode_O_Rem_E_i16() {
        assert_eq!(
            Instruction::decode(2467180229u32), Instruction::O_Rem_E_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_i16() {
        assert_eq!(
            Instruction::O_Rem_E_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180229u32
        );
    }
    #[test]
    fn decode_O_Rem_E_i32() {
        assert_eq!(
            Instruction::decode(2735615685u32), Instruction::O_Rem_E_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_i32() {
        assert_eq!(
            Instruction::O_Rem_E_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615685u32
        );
    }
    #[test]
    fn decode_O_Rem_E_i64() {
        assert_eq!(
            Instruction::decode(3004051141u32), Instruction::O_Rem_E_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_i64() {
        assert_eq!(
            Instruction::O_Rem_E_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051141u32
        );
    }
    #[test]
    fn decode_O_Rem_E_u8() {
        assert_eq!(
            Instruction::decode(51261125u32), Instruction::O_Rem_E_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_u8() {
        assert_eq!(
            Instruction::O_Rem_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51261125u32
        );
    }
    #[test]
    fn decode_O_Rem_E_u16() {
        assert_eq!(
            Instruction::decode(319696581u32), Instruction::O_Rem_E_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_u16() {
        assert_eq!(
            Instruction::O_Rem_E_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696581u32
        );
    }
    #[test]
    fn decode_O_Rem_E_u32() {
        assert_eq!(
            Instruction::decode(588132037u32), Instruction::O_Rem_E_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_u32() {
        assert_eq!(
            Instruction::O_Rem_E_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588132037u32
        );
    }
    #[test]
    fn decode_O_Rem_E_u64() {
        assert_eq!(
            Instruction::decode(856567493u32), Instruction::O_Rem_E_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Rem_E_u64() {
        assert_eq!(
            Instruction::O_Rem_E_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567493u32
        );
    }
    #[test]
    fn decode_O_Shl_i8() {
        assert_eq!(
            Instruction::decode(2198744837u32), Instruction::O_Shl_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_i8() {
        assert_eq!(
            Instruction::O_Shl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744837u32
        );
    }
    #[test]
    fn decode_O_Shl_i16() {
        assert_eq!(
            Instruction::decode(2467180293u32), Instruction::O_Shl_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_i16() {
        assert_eq!(
            Instruction::O_Shl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180293u32
        );
    }
    #[test]
    fn decode_O_Shl_i32() {
        assert_eq!(
            Instruction::decode(2735615749u32), Instruction::O_Shl_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_i32() {
        assert_eq!(
            Instruction::O_Shl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615749u32
        );
    }
    #[test]
    fn decode_O_Shl_i64() {
        assert_eq!(
            Instruction::decode(3004051205u32), Instruction::O_Shl_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_i64() {
        assert_eq!(
            Instruction::O_Shl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051205u32
        );
    }
    #[test]
    fn decode_O_Shl_u8() {
        assert_eq!(
            Instruction::decode(51261189u32), Instruction::O_Shl_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_u8() {
        assert_eq!(
            Instruction::O_Shl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51261189u32
        );
    }
    #[test]
    fn decode_O_Shl_u16() {
        assert_eq!(
            Instruction::decode(319696645u32), Instruction::O_Shl_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_u16() {
        assert_eq!(
            Instruction::O_Shl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696645u32
        );
    }
    #[test]
    fn decode_O_Shl_u32() {
        assert_eq!(
            Instruction::decode(588132101u32), Instruction::O_Shl_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_u32() {
        assert_eq!(
            Instruction::O_Shl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588132101u32
        );
    }
    #[test]
    fn decode_O_Shl_u64() {
        assert_eq!(
            Instruction::decode(856567557u32), Instruction::O_Shl_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shl_u64() {
        assert_eq!(
            Instruction::O_Shl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567557u32
        );
    }
    #[test]
    fn decode_O_Shr_i8() {
        assert_eq!(
            Instruction::decode(2198744901u32), Instruction::O_Shr_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_i8() {
        assert_eq!(
            Instruction::O_Shr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744901u32
        );
    }
    #[test]
    fn decode_O_Shr_i16() {
        assert_eq!(
            Instruction::decode(2467180357u32), Instruction::O_Shr_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_i16() {
        assert_eq!(
            Instruction::O_Shr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180357u32
        );
    }
    #[test]
    fn decode_O_Shr_i32() {
        assert_eq!(
            Instruction::decode(2735615813u32), Instruction::O_Shr_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_i32() {
        assert_eq!(
            Instruction::O_Shr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615813u32
        );
    }
    #[test]
    fn decode_O_Shr_i64() {
        assert_eq!(
            Instruction::decode(3004051269u32), Instruction::O_Shr_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_i64() {
        assert_eq!(
            Instruction::O_Shr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051269u32
        );
    }
    #[test]
    fn decode_O_Shr_u8() {
        assert_eq!(
            Instruction::decode(51261253u32), Instruction::O_Shr_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_u8() {
        assert_eq!(
            Instruction::O_Shr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51261253u32
        );
    }
    #[test]
    fn decode_O_Shr_u16() {
        assert_eq!(
            Instruction::decode(319696709u32), Instruction::O_Shr_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_u16() {
        assert_eq!(
            Instruction::O_Shr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696709u32
        );
    }
    #[test]
    fn decode_O_Shr_u32() {
        assert_eq!(
            Instruction::decode(588132165u32), Instruction::O_Shr_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_u32() {
        assert_eq!(
            Instruction::O_Shr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588132165u32
        );
    }
    #[test]
    fn decode_O_Shr_u64() {
        assert_eq!(
            Instruction::decode(856567621u32), Instruction::O_Shr_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Shr_u64() {
        assert_eq!(
            Instruction::O_Shr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567621u32
        );
    }
    #[test]
    fn decode_O_Sub_i8() {
        assert_eq!(
            Instruction::decode(2198744965u32), Instruction::O_Sub_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_i8() {
        assert_eq!(
            Instruction::O_Sub_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744965u32
        );
    }
    #[test]
    fn decode_O_Sub_i16() {
        assert_eq!(
            Instruction::decode(2467180421u32), Instruction::O_Sub_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_i16() {
        assert_eq!(
            Instruction::O_Sub_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180421u32
        );
    }
    #[test]
    fn decode_O_Sub_i32() {
        assert_eq!(
            Instruction::decode(2735615877u32), Instruction::O_Sub_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_i32() {
        assert_eq!(
            Instruction::O_Sub_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615877u32
        );
    }
    #[test]
    fn decode_O_Sub_i64() {
        assert_eq!(
            Instruction::decode(3004051333u32), Instruction::O_Sub_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_i64() {
        assert_eq!(
            Instruction::O_Sub_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051333u32
        );
    }
    #[test]
    fn decode_O_Sub_u8() {
        assert_eq!(
            Instruction::decode(51261317u32), Instruction::O_Sub_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_u8() {
        assert_eq!(
            Instruction::O_Sub_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51261317u32
        );
    }
    #[test]
    fn decode_O_Sub_u16() {
        assert_eq!(
            Instruction::decode(319696773u32), Instruction::O_Sub_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_u16() {
        assert_eq!(
            Instruction::O_Sub_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696773u32
        );
    }
    #[test]
    fn decode_O_Sub_u32() {
        assert_eq!(
            Instruction::decode(588132229u32), Instruction::O_Sub_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_u32() {
        assert_eq!(
            Instruction::O_Sub_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588132229u32
        );
    }
    #[test]
    fn decode_O_Sub_u64() {
        assert_eq!(
            Instruction::decode(856567685u32), Instruction::O_Sub_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_u64() {
        assert_eq!(
            Instruction::O_Sub_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567685u32
        );
    }
    #[test]
    fn decode_O_Sub_U_i8() {
        assert_eq!(
            Instruction::decode(2198745029u32), Instruction::O_Sub_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_U_i8() {
        assert_eq!(
            Instruction::O_Sub_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198745029u32
        );
    }
    #[test]
    fn decode_O_Sub_U_i16() {
        assert_eq!(
            Instruction::decode(2467180485u32), Instruction::O_Sub_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_U_i16() {
        assert_eq!(
            Instruction::O_Sub_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180485u32
        );
    }
    #[test]
    fn decode_O_Sub_U_i32() {
        assert_eq!(
            Instruction::decode(2735615941u32), Instruction::O_Sub_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_U_i32() {
        assert_eq!(
            Instruction::O_Sub_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615941u32
        );
    }
    #[test]
    fn decode_O_Sub_U_i64() {
        assert_eq!(
            Instruction::decode(3004051397u32), Instruction::O_Sub_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_O_Sub_U_i64() {
        assert_eq!(
            Instruction::O_Sub_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051397u32
        );
    }
    #[test]
    fn decode_S_Abs_i8() {
        assert_eq!(
            Instruction::decode(2148412422u32), Instruction::S_Abs_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Abs_i8() {
        assert_eq!(
            Instruction::S_Abs_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412422u32
        );
    }
    #[test]
    fn decode_S_Abs_i16() {
        assert_eq!(
            Instruction::decode(2416847878u32), Instruction::S_Abs_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Abs_i16() {
        assert_eq!(
            Instruction::S_Abs_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416847878u32
        );
    }
    #[test]
    fn decode_S_Abs_i32() {
        assert_eq!(
            Instruction::decode(2685283334u32), Instruction::S_Abs_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Abs_i32() {
        assert_eq!(
            Instruction::S_Abs_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283334u32
        );
    }
    #[test]
    fn decode_S_Abs_i64() {
        assert_eq!(
            Instruction::decode(2953718790u32), Instruction::S_Abs_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Abs_i64() {
        assert_eq!(
            Instruction::S_Abs_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953718790u32
        );
    }
    #[test]
    fn decode_S_Add_i8() {
        assert_eq!(
            Instruction::decode(2198744134u32), Instruction::S_Add_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_i8() {
        assert_eq!(
            Instruction::S_Add_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744134u32
        );
    }
    #[test]
    fn decode_S_Add_i16() {
        assert_eq!(
            Instruction::decode(2467179590u32), Instruction::S_Add_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_i16() {
        assert_eq!(
            Instruction::S_Add_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179590u32
        );
    }
    #[test]
    fn decode_S_Add_i32() {
        assert_eq!(
            Instruction::decode(2735615046u32), Instruction::S_Add_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_i32() {
        assert_eq!(
            Instruction::S_Add_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615046u32
        );
    }
    #[test]
    fn decode_S_Add_i64() {
        assert_eq!(
            Instruction::decode(3004050502u32), Instruction::S_Add_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_i64() {
        assert_eq!(
            Instruction::S_Add_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050502u32
        );
    }
    #[test]
    fn decode_S_Add_u8() {
        assert_eq!(
            Instruction::decode(51260486u32), Instruction::S_Add_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_u8() {
        assert_eq!(
            Instruction::S_Add_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260486u32
        );
    }
    #[test]
    fn decode_S_Add_u16() {
        assert_eq!(
            Instruction::decode(319695942u32), Instruction::S_Add_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_u16() {
        assert_eq!(
            Instruction::S_Add_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319695942u32
        );
    }
    #[test]
    fn decode_S_Add_u32() {
        assert_eq!(
            Instruction::decode(588131398u32), Instruction::S_Add_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_u32() {
        assert_eq!(
            Instruction::S_Add_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131398u32
        );
    }
    #[test]
    fn decode_S_Add_u64() {
        assert_eq!(
            Instruction::decode(856566854u32), Instruction::S_Add_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_u64() {
        assert_eq!(
            Instruction::S_Add_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856566854u32
        );
    }
    #[test]
    fn decode_S_Add_U_i8() {
        assert_eq!(
            Instruction::decode(2198744198u32), Instruction::S_Add_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_U_i8() {
        assert_eq!(
            Instruction::S_Add_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744198u32
        );
    }
    #[test]
    fn decode_S_Add_U_i16() {
        assert_eq!(
            Instruction::decode(2467179654u32), Instruction::S_Add_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_U_i16() {
        assert_eq!(
            Instruction::S_Add_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179654u32
        );
    }
    #[test]
    fn decode_S_Add_U_i32() {
        assert_eq!(
            Instruction::decode(2735615110u32), Instruction::S_Add_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_U_i32() {
        assert_eq!(
            Instruction::S_Add_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615110u32
        );
    }
    #[test]
    fn decode_S_Add_U_i64() {
        assert_eq!(
            Instruction::decode(3004050566u32), Instruction::S_Add_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_U_i64() {
        assert_eq!(
            Instruction::S_Add_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050566u32
        );
    }
    #[test]
    fn decode_S_Add_S_u8() {
        assert_eq!(
            Instruction::decode(51260550u32), Instruction::S_Add_S_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_S_u8() {
        assert_eq!(
            Instruction::S_Add_S_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260550u32
        );
    }
    #[test]
    fn decode_S_Add_S_u16() {
        assert_eq!(
            Instruction::decode(319696006u32), Instruction::S_Add_S_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_S_u16() {
        assert_eq!(
            Instruction::S_Add_S_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696006u32
        );
    }
    #[test]
    fn decode_S_Add_S_u32() {
        assert_eq!(
            Instruction::decode(588131462u32), Instruction::S_Add_S_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_S_u32() {
        assert_eq!(
            Instruction::S_Add_S_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131462u32
        );
    }
    #[test]
    fn decode_S_Add_S_u64() {
        assert_eq!(
            Instruction::decode(856566918u32), Instruction::S_Add_S_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Add_S_u64() {
        assert_eq!(
            Instruction::S_Add_S_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856566918u32
        );
    }
    #[test]
    fn decode_S_Div_i8() {
        assert_eq!(
            Instruction::decode(2198744262u32), Instruction::S_Div_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_i8() {
        assert_eq!(
            Instruction::S_Div_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744262u32
        );
    }
    #[test]
    fn decode_S_Div_i16() {
        assert_eq!(
            Instruction::decode(2467179718u32), Instruction::S_Div_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_i16() {
        assert_eq!(
            Instruction::S_Div_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179718u32
        );
    }
    #[test]
    fn decode_S_Div_i32() {
        assert_eq!(
            Instruction::decode(2735615174u32), Instruction::S_Div_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_i32() {
        assert_eq!(
            Instruction::S_Div_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615174u32
        );
    }
    #[test]
    fn decode_S_Div_i64() {
        assert_eq!(
            Instruction::decode(3004050630u32), Instruction::S_Div_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_i64() {
        assert_eq!(
            Instruction::S_Div_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050630u32
        );
    }
    #[test]
    fn decode_S_Div_u8() {
        assert_eq!(
            Instruction::decode(51260614u32), Instruction::S_Div_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_u8() {
        assert_eq!(
            Instruction::S_Div_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260614u32
        );
    }
    #[test]
    fn decode_S_Div_u16() {
        assert_eq!(
            Instruction::decode(319696070u32), Instruction::S_Div_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_u16() {
        assert_eq!(
            Instruction::S_Div_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696070u32
        );
    }
    #[test]
    fn decode_S_Div_u32() {
        assert_eq!(
            Instruction::decode(588131526u32), Instruction::S_Div_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_u32() {
        assert_eq!(
            Instruction::S_Div_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131526u32
        );
    }
    #[test]
    fn decode_S_Div_u64() {
        assert_eq!(
            Instruction::decode(856566982u32), Instruction::S_Div_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Div_u64() {
        assert_eq!(
            Instruction::S_Div_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856566982u32
        );
    }
    #[test]
    fn decode_S_Mul_i8() {
        assert_eq!(
            Instruction::decode(2198744518u32), Instruction::S_Mul_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_i8() {
        assert_eq!(
            Instruction::S_Mul_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744518u32
        );
    }
    #[test]
    fn decode_S_Mul_i16() {
        assert_eq!(
            Instruction::decode(2467179974u32), Instruction::S_Mul_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_i16() {
        assert_eq!(
            Instruction::S_Mul_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467179974u32
        );
    }
    #[test]
    fn decode_S_Mul_i32() {
        assert_eq!(
            Instruction::decode(2735615430u32), Instruction::S_Mul_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_i32() {
        assert_eq!(
            Instruction::S_Mul_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615430u32
        );
    }
    #[test]
    fn decode_S_Mul_i64() {
        assert_eq!(
            Instruction::decode(3004050886u32), Instruction::S_Mul_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_i64() {
        assert_eq!(
            Instruction::S_Mul_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004050886u32
        );
    }
    #[test]
    fn decode_S_Mul_u8() {
        assert_eq!(
            Instruction::decode(51260870u32), Instruction::S_Mul_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_u8() {
        assert_eq!(
            Instruction::S_Mul_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260870u32
        );
    }
    #[test]
    fn decode_S_Mul_u16() {
        assert_eq!(
            Instruction::decode(319696326u32), Instruction::S_Mul_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_u16() {
        assert_eq!(
            Instruction::S_Mul_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696326u32
        );
    }
    #[test]
    fn decode_S_Mul_u32() {
        assert_eq!(
            Instruction::decode(588131782u32), Instruction::S_Mul_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_u32() {
        assert_eq!(
            Instruction::S_Mul_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131782u32
        );
    }
    #[test]
    fn decode_S_Mul_u64() {
        assert_eq!(
            Instruction::decode(856567238u32), Instruction::S_Mul_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Mul_u64() {
        assert_eq!(
            Instruction::S_Mul_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567238u32
        );
    }
    #[test]
    fn decode_S_Neg_i8() {
        assert_eq!(
            Instruction::decode(2148412934u32), Instruction::S_Neg_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Neg_i8() {
        assert_eq!(
            Instruction::S_Neg_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412934u32
        );
    }
    #[test]
    fn decode_S_Neg_i16() {
        assert_eq!(
            Instruction::decode(2416848390u32), Instruction::S_Neg_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Neg_i16() {
        assert_eq!(
            Instruction::S_Neg_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848390u32
        );
    }
    #[test]
    fn decode_S_Neg_i32() {
        assert_eq!(
            Instruction::decode(2685283846u32), Instruction::S_Neg_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Neg_i32() {
        assert_eq!(
            Instruction::S_Neg_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283846u32
        );
    }
    #[test]
    fn decode_S_Neg_i64() {
        assert_eq!(
            Instruction::decode(2953719302u32), Instruction::S_Neg_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_S_Neg_i64() {
        assert_eq!(
            Instruction::S_Neg_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719302u32
        );
    }
    #[test]
    fn decode_S_Pow_i8() {
        assert_eq!(
            Instruction::decode(2198744646u32), Instruction::S_Pow_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_i8() {
        assert_eq!(
            Instruction::S_Pow_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744646u32
        );
    }
    #[test]
    fn decode_S_Pow_i16() {
        assert_eq!(
            Instruction::decode(2467180102u32), Instruction::S_Pow_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_i16() {
        assert_eq!(
            Instruction::S_Pow_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180102u32
        );
    }
    #[test]
    fn decode_S_Pow_i32() {
        assert_eq!(
            Instruction::decode(2735615558u32), Instruction::S_Pow_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_i32() {
        assert_eq!(
            Instruction::S_Pow_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615558u32
        );
    }
    #[test]
    fn decode_S_Pow_i64() {
        assert_eq!(
            Instruction::decode(3004051014u32), Instruction::S_Pow_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_i64() {
        assert_eq!(
            Instruction::S_Pow_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051014u32
        );
    }
    #[test]
    fn decode_S_Pow_u8() {
        assert_eq!(
            Instruction::decode(51260998u32), Instruction::S_Pow_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_u8() {
        assert_eq!(
            Instruction::S_Pow_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51260998u32
        );
    }
    #[test]
    fn decode_S_Pow_u16() {
        assert_eq!(
            Instruction::decode(319696454u32), Instruction::S_Pow_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_u16() {
        assert_eq!(
            Instruction::S_Pow_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696454u32
        );
    }
    #[test]
    fn decode_S_Pow_u32() {
        assert_eq!(
            Instruction::decode(588131910u32), Instruction::S_Pow_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_u32() {
        assert_eq!(
            Instruction::S_Pow_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588131910u32
        );
    }
    #[test]
    fn decode_S_Pow_u64() {
        assert_eq!(
            Instruction::decode(856567366u32), Instruction::S_Pow_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Pow_u64() {
        assert_eq!(
            Instruction::S_Pow_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567366u32
        );
    }
    #[test]
    fn decode_S_Sub_i8() {
        assert_eq!(
            Instruction::decode(2198744966u32), Instruction::S_Sub_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_i8() {
        assert_eq!(
            Instruction::S_Sub_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198744966u32
        );
    }
    #[test]
    fn decode_S_Sub_i16() {
        assert_eq!(
            Instruction::decode(2467180422u32), Instruction::S_Sub_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_i16() {
        assert_eq!(
            Instruction::S_Sub_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180422u32
        );
    }
    #[test]
    fn decode_S_Sub_i32() {
        assert_eq!(
            Instruction::decode(2735615878u32), Instruction::S_Sub_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_i32() {
        assert_eq!(
            Instruction::S_Sub_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615878u32
        );
    }
    #[test]
    fn decode_S_Sub_i64() {
        assert_eq!(
            Instruction::decode(3004051334u32), Instruction::S_Sub_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_i64() {
        assert_eq!(
            Instruction::S_Sub_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051334u32
        );
    }
    #[test]
    fn decode_S_Sub_u8() {
        assert_eq!(
            Instruction::decode(51261318u32), Instruction::S_Sub_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_u8() {
        assert_eq!(
            Instruction::S_Sub_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 51261318u32
        );
    }
    #[test]
    fn decode_S_Sub_u16() {
        assert_eq!(
            Instruction::decode(319696774u32), Instruction::S_Sub_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_u16() {
        assert_eq!(
            Instruction::S_Sub_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 319696774u32
        );
    }
    #[test]
    fn decode_S_Sub_u32() {
        assert_eq!(
            Instruction::decode(588132230u32), Instruction::S_Sub_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_u32() {
        assert_eq!(
            Instruction::S_Sub_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 588132230u32
        );
    }
    #[test]
    fn decode_S_Sub_u64() {
        assert_eq!(
            Instruction::decode(856567686u32), Instruction::S_Sub_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_u64() {
        assert_eq!(
            Instruction::S_Sub_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 856567686u32
        );
    }
    #[test]
    fn decode_S_Sub_U_i8() {
        assert_eq!(
            Instruction::decode(2198745030u32), Instruction::S_Sub_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_U_i8() {
        assert_eq!(
            Instruction::S_Sub_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2198745030u32
        );
    }
    #[test]
    fn decode_S_Sub_U_i16() {
        assert_eq!(
            Instruction::decode(2467180486u32), Instruction::S_Sub_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_U_i16() {
        assert_eq!(
            Instruction::S_Sub_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2467180486u32
        );
    }
    #[test]
    fn decode_S_Sub_U_i32() {
        assert_eq!(
            Instruction::decode(2735615942u32), Instruction::S_Sub_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_U_i32() {
        assert_eq!(
            Instruction::S_Sub_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 2735615942u32
        );
    }
    #[test]
    fn decode_S_Sub_U_i64() {
        assert_eq!(
            Instruction::decode(3004051398u32), Instruction::S_Sub_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_S_Sub_U_i64() {
        assert_eq!(
            Instruction::S_Sub_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3004051398u32
        );
    }
    #[test]
    fn decode_Abs_f32() {
        assert_eq!(
            Instruction::decode(3759025159u32), Instruction::Abs_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Abs_f32() {
        assert_eq!(
            Instruction::Abs_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 3759025159u32
        );
    }
    #[test]
    fn decode_Abs_f64() {
        assert_eq!(
            Instruction::decode(4027460615u32), Instruction::Abs_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Abs_f64() {
        assert_eq!(
            Instruction::Abs_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 4027460615u32
        );
    }
    #[test]
    fn decode_Add_f32() {
        assert_eq!(
            Instruction::decode(3809356871u32), Instruction::Add_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Add_f32() {
        assert_eq!(
            Instruction::Add_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3809356871u32
        );
    }
    #[test]
    fn decode_Add_f64() {
        assert_eq!(
            Instruction::decode(4077792327u32), Instruction::Add_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Add_f64() {
        assert_eq!(
            Instruction::Add_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4077792327u32
        );
    }
    #[test]
    fn decode_Div_f32() {
        assert_eq!(
            Instruction::decode(3809356999u32), Instruction::Div_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Div_f32() {
        assert_eq!(
            Instruction::Div_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3809356999u32
        );
    }
    #[test]
    fn decode_Div_f64() {
        assert_eq!(
            Instruction::decode(4077792455u32), Instruction::Div_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Div_f64() {
        assert_eq!(
            Instruction::Div_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4077792455u32
        );
    }
    #[test]
    fn decode_Div_E_f32() {
        assert_eq!(
            Instruction::decode(3809357063u32), Instruction::Div_E_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Div_E_f32() {
        assert_eq!(
            Instruction::Div_E_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3809357063u32
        );
    }
    #[test]
    fn decode_Div_E_f64() {
        assert_eq!(
            Instruction::decode(4077792519u32), Instruction::Div_E_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Div_E_f64() {
        assert_eq!(
            Instruction::Div_E_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4077792519u32
        );
    }
    #[test]
    fn decode_Log_f32() {
        assert_eq!(
            Instruction::decode(3809357127u32), Instruction::Log_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Log_f32() {
        assert_eq!(
            Instruction::Log_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3809357127u32
        );
    }
    #[test]
    fn decode_Log_f64() {
        assert_eq!(
            Instruction::decode(4077792583u32), Instruction::Log_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Log_f64() {
        assert_eq!(
            Instruction::Log_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4077792583u32
        );
    }
    #[test]
    fn decode_Sqrt_f32() {
        assert_eq!(
            Instruction::decode(3759025543u32), Instruction::Sqrt_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Sqrt_f32() {
        assert_eq!(
            Instruction::Sqrt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 3759025543u32
        );
    }
    #[test]
    fn decode_Sqrt_f64() {
        assert_eq!(
            Instruction::decode(4027460999u32), Instruction::Sqrt_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Sqrt_f64() {
        assert_eq!(
            Instruction::Sqrt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 4027460999u32
        );
    }
    #[test]
    fn decode_Mul_f32() {
        assert_eq!(
            Instruction::decode(3809357255u32), Instruction::Mul_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Mul_f32() {
        assert_eq!(
            Instruction::Mul_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3809357255u32
        );
    }
    #[test]
    fn decode_Mul_f64() {
        assert_eq!(
            Instruction::decode(4077792711u32), Instruction::Mul_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Mul_f64() {
        assert_eq!(
            Instruction::Mul_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4077792711u32
        );
    }
    #[test]
    fn decode_Neg_f32() {
        assert_eq!(
            Instruction::decode(3759025671u32), Instruction::Neg_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Neg_f32() {
        assert_eq!(
            Instruction::Neg_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 3759025671u32
        );
    }
    #[test]
    fn decode_Neg_f64() {
        assert_eq!(
            Instruction::decode(4027461127u32), Instruction::Neg_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Neg_f64() {
        assert_eq!(
            Instruction::Neg_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 4027461127u32
        );
    }
    #[test]
    fn decode_Pow_f32() {
        assert_eq!(
            Instruction::decode(3809357383u32), Instruction::Pow_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Pow_f32() {
        assert_eq!(
            Instruction::Pow_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3809357383u32
        );
    }
    #[test]
    fn decode_Pow_f64() {
        assert_eq!(
            Instruction::decode(4077792839u32), Instruction::Pow_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Pow_f64() {
        assert_eq!(
            Instruction::Pow_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4077792839u32
        );
    }
    #[test]
    fn decode_Rem_f32() {
        assert_eq!(
            Instruction::decode(3809357447u32), Instruction::Rem_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rem_f32() {
        assert_eq!(
            Instruction::Rem_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3809357447u32
        );
    }
    #[test]
    fn decode_Rem_f64() {
        assert_eq!(
            Instruction::decode(4077792903u32), Instruction::Rem_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rem_f64() {
        assert_eq!(
            Instruction::Rem_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4077792903u32
        );
    }
    #[test]
    fn decode_Rem_E_f32() {
        assert_eq!(
            Instruction::decode(3809357511u32), Instruction::Rem_E_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rem_E_f32() {
        assert_eq!(
            Instruction::Rem_E_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3809357511u32
        );
    }
    #[test]
    fn decode_Rem_E_f64() {
        assert_eq!(
            Instruction::decode(4077792967u32), Instruction::Rem_E_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Rem_E_f64() {
        assert_eq!(
            Instruction::Rem_E_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4077792967u32
        );
    }
    #[test]
    fn decode_Cbrt_f32() {
        assert_eq!(
            Instruction::decode(3759025991u32), Instruction::Cbrt_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Cbrt_f32() {
        assert_eq!(
            Instruction::Cbrt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 3759025991u32
        );
    }
    #[test]
    fn decode_Cbrt_f64() {
        assert_eq!(
            Instruction::decode(4027461447u32), Instruction::Cbrt_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_Cbrt_f64() {
        assert_eq!(
            Instruction::Cbrt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 4027461447u32
        );
    }
    #[test]
    fn decode_Sub_f32() {
        assert_eq!(
            Instruction::decode(3809357703u32), Instruction::Sub_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Sub_f32() {
        assert_eq!(
            Instruction::Sub_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 3809357703u32
        );
    }
    #[test]
    fn decode_Sub_f64() {
        assert_eq!(
            Instruction::decode(4077793159u32), Instruction::Sub_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(6), }
        );
    }
    #[test]
    fn encode_Sub_f64() {
        assert_eq!(
            Instruction::Sub_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(6), }
            .encode(), 4077793159u32
        );
    }
}
