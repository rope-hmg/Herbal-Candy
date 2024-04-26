use crate::register::Register;
use crate::encoding::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Halt,
    Trap,
    Call { rs2: Register },
    Call_R { rs2: Register },
    Call_I { imm: i16 },
    Ret,
    Ecall { imm: i16 },
    Jal { rs2: Register },
    Jal_R { rs2: Register },
    Jal_I { imm: i16 },
    Jnz { rs1: Register, rs2: Register },
    Jnz_R { rs1: Register, rs2: Register },
    Jnz_I { rd: Register, imm: i16 },
    Jiz { rs1: Register, rs2: Register },
    Jiz_R { rs1: Register, rs2: Register },
    Jiz_I { rd: Register, imm: i16 },
    Ld_8 { rd: Register, rs1: Register },
    Ld_16 { rd: Register, rs1: Register },
    Ld_32 { rd: Register, rs1: Register },
    Ld_64 { rd: Register, rs1: Register },
    Ld_I { rd: Register, imm: i16 },
    Ld_A_8 { rd: Register, imm: u16 },
    Ld_A_16 { rd: Register, imm: u16 },
    Ld_A_32 { rd: Register, imm: u16 },
    Ld_A_64 { rd: Register, imm: u16 },
    St_8 { rd: Register, rs1: Register },
    St_16 { rd: Register, rs1: Register },
    St_32 { rd: Register, rs1: Register },
    St_64 { rd: Register, rs1: Register },
    St_I { rd: Register, imm: i16 },
    Mov { rd: Register, rs1: Register },
    Psh { rd: Register },
    Psh_I { imm: i16 },
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
    Rot_L_i8 { rd: Register, rs1: Register, rs2: Register },
    Rot_L_i16 { rd: Register, rs1: Register, rs2: Register },
    Rot_L_i32 { rd: Register, rs1: Register, rs2: Register },
    Rot_L_i64 { rd: Register, rs1: Register, rs2: Register },
    Rot_L_u8 { rd: Register, rs1: Register, rs2: Register },
    Rot_L_u16 { rd: Register, rs1: Register, rs2: Register },
    Rot_L_u32 { rd: Register, rs1: Register, rs2: Register },
    Rot_L_u64 { rd: Register, rs1: Register, rs2: Register },
    Rot_R_i8 { rd: Register, rs1: Register, rs2: Register },
    Rot_R_i16 { rd: Register, rs1: Register, rs2: Register },
    Rot_R_i32 { rd: Register, rs1: Register, rs2: Register },
    Rot_R_i64 { rd: Register, rs1: Register, rs2: Register },
    Rot_R_u8 { rd: Register, rs1: Register, rs2: Register },
    Rot_R_u16 { rd: Register, rs1: Register, rs2: Register },
    Rot_R_u32 { rd: Register, rs1: Register, rs2: Register },
    Rot_R_u64 { rd: Register, rs1: Register, rs2: Register },
    C_Ones_i8 { rd: Register, rs1: Register },
    C_Ones_i16 { rd: Register, rs1: Register },
    C_Ones_i32 { rd: Register, rs1: Register },
    C_Ones_i64 { rd: Register, rs1: Register },
    C_Ones_u8 { rd: Register, rs1: Register },
    C_Ones_u16 { rd: Register, rs1: Register },
    C_Ones_u32 { rd: Register, rs1: Register },
    C_Ones_u64 { rd: Register, rs1: Register },
    L_Ones_i8 { rd: Register, rs1: Register },
    L_Ones_i16 { rd: Register, rs1: Register },
    L_Ones_i32 { rd: Register, rs1: Register },
    L_Ones_i64 { rd: Register, rs1: Register },
    L_Ones_u8 { rd: Register, rs1: Register },
    L_Ones_u16 { rd: Register, rs1: Register },
    L_Ones_u32 { rd: Register, rs1: Register },
    L_Ones_u64 { rd: Register, rs1: Register },
    T_Ones_i8 { rd: Register, rs1: Register },
    T_Ones_i16 { rd: Register, rs1: Register },
    T_Ones_i32 { rd: Register, rs1: Register },
    T_Ones_i64 { rd: Register, rs1: Register },
    T_Ones_u8 { rd: Register, rs1: Register },
    T_Ones_u16 { rd: Register, rs1: Register },
    T_Ones_u32 { rd: Register, rs1: Register },
    T_Ones_u64 { rd: Register, rs1: Register },
    C_Zeros_i8 { rd: Register, rs1: Register },
    C_Zeros_i16 { rd: Register, rs1: Register },
    C_Zeros_i32 { rd: Register, rs1: Register },
    C_Zeros_i64 { rd: Register, rs1: Register },
    C_Zeros_u8 { rd: Register, rs1: Register },
    C_Zeros_u16 { rd: Register, rs1: Register },
    C_Zeros_u32 { rd: Register, rs1: Register },
    C_Zeros_u64 { rd: Register, rs1: Register },
    L_Zeros_i8 { rd: Register, rs1: Register },
    L_Zeros_i16 { rd: Register, rs1: Register },
    L_Zeros_i32 { rd: Register, rs1: Register },
    L_Zeros_i64 { rd: Register, rs1: Register },
    L_Zeros_u8 { rd: Register, rs1: Register },
    L_Zeros_u16 { rd: Register, rs1: Register },
    L_Zeros_u32 { rd: Register, rs1: Register },
    L_Zeros_u64 { rd: Register, rs1: Register },
    T_Zeros_i8 { rd: Register, rs1: Register },
    T_Zeros_i16 { rd: Register, rs1: Register },
    T_Zeros_i32 { rd: Register, rs1: Register },
    T_Zeros_i64 { rd: Register, rs1: Register },
    T_Zeros_u8 { rd: Register, rs1: Register },
    T_Zeros_u16 { rd: Register, rs1: Register },
    T_Zeros_u32 { rd: Register, rs1: Register },
    T_Zeros_u64 { rd: Register, rs1: Register },
    R_Bytes_i8 { rd: Register, rs1: Register },
    R_Bytes_i16 { rd: Register, rs1: Register },
    R_Bytes_i32 { rd: Register, rs1: Register },
    R_Bytes_i64 { rd: Register, rs1: Register },
    R_Bytes_u8 { rd: Register, rs1: Register },
    R_Bytes_u16 { rd: Register, rs1: Register },
    R_Bytes_u32 { rd: Register, rs1: Register },
    R_Bytes_u64 { rd: Register, rs1: Register },
    R_Bits_i8 { rd: Register, rs1: Register },
    R_Bits_i16 { rd: Register, rs1: Register },
    R_Bits_i32 { rd: Register, rs1: Register },
    R_Bits_i64 { rd: Register, rs1: Register },
    R_Bits_u8 { rd: Register, rs1: Register },
    R_Bits_u16 { rd: Register, rs1: Register },
    R_Bits_u32 { rd: Register, rs1: Register },
    R_Bits_u64 { rd: Register, rs1: Register },
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
    pub fn decode(instr: u32) -> Self {
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
            0u8 if funct == 0u8 => Self::Halt,
            0u8 if funct == 1u8 => Self::Trap,
            0u8 if funct == 2u8 && s == 0u8 => Self::Call { rs2 },
            0u8 if funct == 2u8 && s == 1u8 => Self::Call_R { rs2 },
            0u8 if funct == 3u8 => Self::Call_I { imm },
            0u8 if funct == 4u8 => Self::Ret,
            0u8 if funct == 5u8 => Self::Ecall { imm },
            0u8 if funct == 6u8 && s == 0u8 => Self::Jal { rs2 },
            0u8 if funct == 6u8 && s == 1u8 => Self::Jal_R { rs2 },
            0u8 if funct == 7u8 => Self::Jal_I { imm },
            0u8 if funct == 8u8 && s == 0u8 => Self::Jnz { rs1, rs2 },
            0u8 if funct == 8u8 && s == 1u8 => Self::Jnz_R { rs1, rs2 },
            0u8 if funct == 9u8 && !rd.is_readonly() => Self::Jnz_I { rd, imm },
            0u8 if funct == 10u8 && s == 0u8 => Self::Jiz { rs1, rs2 },
            0u8 if funct == 10u8 && s == 1u8 => Self::Jiz_R { rs1, rs2 },
            0u8 if funct == 11u8 && !rd.is_readonly() => Self::Jiz_I { rd, imm },
            1u8 if funct == 0u8 && size == 8u8 && !rd.is_readonly() => {
                Self::Ld_8 { rd, rs1 }
            }
            1u8 if funct == 0u8 && size == 16u8 && !rd.is_readonly() => {
                Self::Ld_16 { rd, rs1 }
            }
            1u8 if funct == 0u8 && size == 32u8 && !rd.is_readonly() => {
                Self::Ld_32 { rd, rs1 }
            }
            1u8 if funct == 0u8 && size == 64u8 && !rd.is_readonly() => {
                Self::Ld_64 { rd, rs1 }
            }
            1u8 if funct == 1u8 && !rd.is_readonly() => Self::Ld_I { rd, imm },
            1u8 if funct == 2u8 && !rd.is_readonly() => {
                Self::Ld_A_8 {
                    rd,
                    imm: imm as u16,
                }
            }
            1u8 if funct == 3u8 && !rd.is_readonly() => {
                Self::Ld_A_16 {
                    rd,
                    imm: imm as u16,
                }
            }
            1u8 if funct == 4u8 && !rd.is_readonly() => {
                Self::Ld_A_32 {
                    rd,
                    imm: imm as u16,
                }
            }
            1u8 if funct == 5u8 && !rd.is_readonly() => {
                Self::Ld_A_64 {
                    rd,
                    imm: imm as u16,
                }
            }
            1u8 if funct == 6u8 && size == 8u8 && !rd.is_readonly() => {
                Self::St_8 { rd, rs1 }
            }
            1u8 if funct == 6u8 && size == 16u8 && !rd.is_readonly() => {
                Self::St_16 { rd, rs1 }
            }
            1u8 if funct == 6u8 && size == 32u8 && !rd.is_readonly() => {
                Self::St_32 { rd, rs1 }
            }
            1u8 if funct == 6u8 && size == 64u8 && !rd.is_readonly() => {
                Self::St_64 { rd, rs1 }
            }
            1u8 if funct == 7u8 && !rd.is_readonly() => Self::St_I { rd, imm },
            1u8 if funct == 8u8 && !rd.is_readonly() => Self::Mov { rd, rs1 },
            1u8 if funct == 9u8 && !rd.is_readonly() => Self::Psh { rd },
            1u8 if funct == 10u8 => Self::Psh_I { imm },
            1u8 if funct == 11u8 && !rd.is_readonly() => Self::Pop { rd },
            2u8 if funct == 0u8 && f == 0u8 && !rd.is_readonly() => {
                Self::Ie { rd, rs1, rs2 }
            }
            2u8 if funct == 0u8 && size == 32u8 && f == 1u8 && !rd.is_readonly() => {
                Self::Ie_f32 { rd, rs1, rs2 }
            }
            2u8 if funct == 0u8 && size == 64u8 && f == 1u8 && !rd.is_readonly() => {
                Self::Ie_f64 { rd, rs1, rs2 }
            }
            2u8 if funct == 1u8 && f == 0u8 && !rd.is_readonly() => {
                Self::Ne { rd, rs1, rs2 }
            }
            2u8 if funct == 1u8 && size == 32u8 && f == 1u8 && !rd.is_readonly() => {
                Self::Ne_f32 { rd, rs1, rs2 }
            }
            2u8 if funct == 1u8 && size == 64u8 && f == 1u8 && !rd.is_readonly() => {
                Self::Ne_f64 { rd, rs1, rs2 }
            }
            2u8 if funct == 2u8 && f == 0u8 && !rd.is_readonly() => {
                Self::Lt { rd, rs1, rs2 }
            }
            2u8 if funct == 2u8 && size == 32u8 && f == 1u8 && !rd.is_readonly() => {
                Self::Lt_f32 { rd, rs1, rs2 }
            }
            2u8 if funct == 2u8 && size == 64u8 && f == 1u8 && !rd.is_readonly() => {
                Self::Lt_f64 { rd, rs1, rs2 }
            }
            2u8 if funct == 3u8 && f == 0u8 && !rd.is_readonly() => {
                Self::Le { rd, rs1, rs2 }
            }
            2u8 if funct == 3u8 && size == 32u8 && f == 1u8 && !rd.is_readonly() => {
                Self::Le_f32 { rd, rs1, rs2 }
            }
            2u8 if funct == 3u8 && size == 64u8 && f == 1u8 && !rd.is_readonly() => {
                Self::Le_f64 { rd, rs1, rs2 }
            }
            2u8 if funct == 4u8 && f == 0u8 && !rd.is_readonly() => {
                Self::Gt { rd, rs1, rs2 }
            }
            2u8 if funct == 4u8 && size == 32u8 && f == 1u8 && !rd.is_readonly() => {
                Self::Gt_f32 { rd, rs1, rs2 }
            }
            2u8 if funct == 4u8 && size == 64u8 && f == 1u8 && !rd.is_readonly() => {
                Self::Gt_f64 { rd, rs1, rs2 }
            }
            2u8 if funct == 5u8 && f == 0u8 && !rd.is_readonly() => {
                Self::Ge { rd, rs1, rs2 }
            }
            2u8 if funct == 5u8 && size == 32u8 && f == 1u8 && !rd.is_readonly() => {
                Self::Ge_f32 { rd, rs1, rs2 }
            }
            2u8 if funct == 5u8 && size == 64u8 && f == 1u8 && !rd.is_readonly() => {
                Self::Ge_f64 { rd, rs1, rs2 }
            }
            3u8 if funct == 0u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::And_i8 { rd, rs1, rs2 },
            3u8 if funct == 0u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::And_i16 { rd, rs1, rs2 },
            3u8 if funct == 0u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::And_i32 { rd, rs1, rs2 },
            3u8 if funct == 0u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::And_i64 { rd, rs1, rs2 },
            3u8 if funct == 0u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::And_u8 { rd, rs1, rs2 },
            3u8 if funct == 0u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::And_u16 { rd, rs1, rs2 },
            3u8 if funct == 0u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::And_u32 { rd, rs1, rs2 },
            3u8 if funct == 0u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::And_u64 { rd, rs1, rs2 },
            3u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Or_i8 { rd, rs1, rs2 },
            3u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Or_i16 { rd, rs1, rs2 },
            3u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Or_i32 { rd, rs1, rs2 },
            3u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Or_i64 { rd, rs1, rs2 },
            3u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Or_u8 { rd, rs1, rs2 },
            3u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Or_u16 { rd, rs1, rs2 },
            3u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Or_u32 { rd, rs1, rs2 },
            3u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Or_u64 { rd, rs1, rs2 },
            3u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Xor_i8 { rd, rs1, rs2 },
            3u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Xor_i16 { rd, rs1, rs2 },
            3u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Xor_i32 { rd, rs1, rs2 },
            3u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Xor_i64 { rd, rs1, rs2 },
            3u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Xor_u8 { rd, rs1, rs2 },
            3u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Xor_u16 { rd, rs1, rs2 },
            3u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Xor_u32 { rd, rs1, rs2 },
            3u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Xor_u64 { rd, rs1, rs2 },
            3u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Not_i8 { rd, rs1 },
            3u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Not_i16 { rd, rs1 },
            3u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Not_i32 { rd, rs1 },
            3u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Not_i64 { rd, rs1 },
            3u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Not_u8 { rd, rs1 },
            3u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Not_u16 { rd, rs1 },
            3u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Not_u32 { rd, rs1 },
            3u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Not_u64 { rd, rs1 },
            3u8 if funct == 4u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Shl_i8 { rd, rs1, rs2 },
            3u8 if funct == 4u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Shl_i16 { rd, rs1, rs2 },
            3u8 if funct == 4u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Shl_i32 { rd, rs1, rs2 },
            3u8 if funct == 4u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Shl_i64 { rd, rs1, rs2 },
            3u8 if funct == 4u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Shl_u8 { rd, rs1, rs2 },
            3u8 if funct == 4u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Shl_u16 { rd, rs1, rs2 },
            3u8 if funct == 4u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Shl_u32 { rd, rs1, rs2 },
            3u8 if funct == 4u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Shl_u64 { rd, rs1, rs2 },
            3u8 if funct == 5u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Shr_i8 { rd, rs1, rs2 },
            3u8 if funct == 5u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Shr_i16 { rd, rs1, rs2 },
            3u8 if funct == 5u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Shr_i32 { rd, rs1, rs2 },
            3u8 if funct == 5u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Shr_i64 { rd, rs1, rs2 },
            3u8 if funct == 5u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Shr_u8 { rd, rs1, rs2 },
            3u8 if funct == 5u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Shr_u16 { rd, rs1, rs2 },
            3u8 if funct == 5u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Shr_u32 { rd, rs1, rs2 },
            3u8 if funct == 5u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Shr_u64 { rd, rs1, rs2 },
            3u8 if funct == 6u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Rot_L_i8 { rd, rs1, rs2 },
            3u8 if funct == 6u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Rot_L_i16 { rd, rs1, rs2 },
            3u8 if funct == 6u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Rot_L_i32 { rd, rs1, rs2 },
            3u8 if funct == 6u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Rot_L_i64 { rd, rs1, rs2 },
            3u8 if funct == 6u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Rot_L_u8 { rd, rs1, rs2 },
            3u8 if funct == 6u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Rot_L_u16 { rd, rs1, rs2 },
            3u8 if funct == 6u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Rot_L_u32 { rd, rs1, rs2 },
            3u8 if funct == 6u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Rot_L_u64 { rd, rs1, rs2 },
            3u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Rot_R_i8 { rd, rs1, rs2 },
            3u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Rot_R_i16 { rd, rs1, rs2 },
            3u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Rot_R_i32 { rd, rs1, rs2 },
            3u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::Rot_R_i64 { rd, rs1, rs2 },
            3u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Rot_R_u8 { rd, rs1, rs2 },
            3u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Rot_R_u16 { rd, rs1, rs2 },
            3u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Rot_R_u32 { rd, rs1, rs2 },
            3u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::Rot_R_u64 { rd, rs1, rs2 },
            3u8 if funct == 8u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Ones_i8 { rd, rs1 },
            3u8 if funct == 8u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Ones_i16 { rd, rs1 },
            3u8 if funct == 8u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Ones_i32 { rd, rs1 },
            3u8 if funct == 8u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Ones_i64 { rd, rs1 },
            3u8 if funct == 8u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Ones_u8 { rd, rs1 },
            3u8 if funct == 8u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Ones_u16 { rd, rs1 },
            3u8 if funct == 8u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Ones_u32 { rd, rs1 },
            3u8 if funct == 8u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Ones_u64 { rd, rs1 },
            3u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::L_Ones_i8 { rd, rs1 },
            3u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::L_Ones_i16 { rd, rs1 },
            3u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::L_Ones_i32 { rd, rs1 },
            3u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::L_Ones_i64 { rd, rs1 },
            3u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::L_Ones_u8 { rd, rs1 },
            3u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::L_Ones_u16 { rd, rs1 },
            3u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::L_Ones_u32 { rd, rs1 },
            3u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::L_Ones_u64 { rd, rs1 },
            3u8 if funct == 10u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::T_Ones_i8 { rd, rs1 },
            3u8 if funct == 10u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::T_Ones_i16 { rd, rs1 },
            3u8 if funct == 10u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::T_Ones_i32 { rd, rs1 },
            3u8 if funct == 10u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::T_Ones_i64 { rd, rs1 },
            3u8 if funct == 10u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::T_Ones_u8 { rd, rs1 },
            3u8 if funct == 10u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::T_Ones_u16 { rd, rs1 },
            3u8 if funct == 10u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::T_Ones_u32 { rd, rs1 },
            3u8 if funct == 10u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::T_Ones_u64 { rd, rs1 },
            3u8 if funct == 11u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Zeros_i8 { rd, rs1 },
            3u8 if funct == 11u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Zeros_i16 { rd, rs1 },
            3u8 if funct == 11u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Zeros_i32 { rd, rs1 },
            3u8 if funct == 11u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Zeros_i64 { rd, rs1 },
            3u8 if funct == 11u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Zeros_u8 { rd, rs1 },
            3u8 if funct == 11u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Zeros_u16 { rd, rs1 },
            3u8 if funct == 11u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Zeros_u32 { rd, rs1 },
            3u8 if funct == 11u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Zeros_u64 { rd, rs1 },
            3u8 if funct == 12u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::L_Zeros_i8 { rd, rs1 },
            3u8 if funct == 12u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::L_Zeros_i16 { rd, rs1 },
            3u8 if funct == 12u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::L_Zeros_i32 { rd, rs1 },
            3u8 if funct == 12u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::L_Zeros_i64 { rd, rs1 },
            3u8 if funct == 12u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::L_Zeros_u8 { rd, rs1 },
            3u8 if funct == 12u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::L_Zeros_u16 { rd, rs1 },
            3u8 if funct == 12u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::L_Zeros_u32 { rd, rs1 },
            3u8 if funct == 12u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::L_Zeros_u64 { rd, rs1 },
            3u8 if funct == 13u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::T_Zeros_i8 { rd, rs1 },
            3u8 if funct == 13u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::T_Zeros_i16 { rd, rs1 },
            3u8 if funct == 13u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::T_Zeros_i32 { rd, rs1 },
            3u8 if funct == 13u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::T_Zeros_i64 { rd, rs1 },
            3u8 if funct == 13u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::T_Zeros_u8 { rd, rs1 },
            3u8 if funct == 13u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::T_Zeros_u16 { rd, rs1 },
            3u8 if funct == 13u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::T_Zeros_u32 { rd, rs1 },
            3u8 if funct == 13u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::T_Zeros_u64 { rd, rs1 },
            3u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::R_Bytes_i8 { rd, rs1 },
            3u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::R_Bytes_i16 { rd, rs1 },
            3u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::R_Bytes_i32 { rd, rs1 },
            3u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::R_Bytes_i64 { rd, rs1 },
            3u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::R_Bytes_u8 { rd, rs1 },
            3u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::R_Bytes_u16 { rd, rs1 },
            3u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::R_Bytes_u32 { rd, rs1 },
            3u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::R_Bytes_u64 { rd, rs1 },
            3u8 if funct == 15u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::R_Bits_i8 { rd, rs1 },
            3u8 if funct == 15u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::R_Bits_i16 { rd, rs1 },
            3u8 if funct == 15u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::R_Bits_i32 { rd, rs1 },
            3u8 if funct == 15u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::R_Bits_i64 { rd, rs1 },
            3u8 if funct == 15u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::R_Bits_u8 { rd, rs1 },
            3u8 if funct == 15u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::R_Bits_u16 { rd, rs1 },
            3u8 if funct == 15u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::R_Bits_u32 { rd, rs1 },
            3u8 if funct == 15u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::R_Bits_u64 { rd, rs1 },
            4u8 if funct == 0u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Abs_i8 { rd, rs1 },
            4u8 if funct == 0u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Abs_i16 { rd, rs1 },
            4u8 if funct == 0u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Abs_i32 { rd, rs1 },
            4u8 if funct == 0u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Abs_i64 { rd, rs1 },
            4u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Add_i8 { rd, rs1, rs2 },
            4u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Add_i16 { rd, rs1, rs2 },
            4u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Add_i32 { rd, rs1, rs2 },
            4u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Add_i64 { rd, rs1, rs2 },
            4u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Add_u8 { rd, rs1, rs2 },
            4u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Add_u16 { rd, rs1, rs2 },
            4u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Add_u32 { rd, rs1, rs2 },
            4u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Add_u64 { rd, rs1, rs2 },
            4u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Add_U_i8 { rd, rs1, rs2 },
            4u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Add_U_i16 { rd, rs1, rs2 },
            4u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Add_U_i32 { rd, rs1, rs2 },
            4u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Add_U_i64 { rd, rs1, rs2 },
            4u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Add_S_u8 { rd, rs1, rs2 },
            4u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Add_S_u16 { rd, rs1, rs2 },
            4u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Add_S_u32 { rd, rs1, rs2 },
            4u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Add_S_u64 { rd, rs1, rs2 },
            4u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Div_i8 { rd, rs1, rs2 },
            4u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Div_i16 { rd, rs1, rs2 },
            4u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Div_i32 { rd, rs1, rs2 },
            4u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Div_i64 { rd, rs1, rs2 },
            4u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Div_u8 { rd, rs1, rs2 },
            4u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Div_u16 { rd, rs1, rs2 },
            4u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Div_u32 { rd, rs1, rs2 },
            4u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Div_u64 { rd, rs1, rs2 },
            4u8 if funct == 4u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Div_E_i8 { rd, rs1, rs2 },
            4u8 if funct == 4u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Div_E_i16 { rd, rs1, rs2 },
            4u8 if funct == 4u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Div_E_i32 { rd, rs1, rs2 },
            4u8 if funct == 4u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Div_E_i64 { rd, rs1, rs2 },
            4u8 if funct == 4u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Div_E_u8 { rd, rs1, rs2 },
            4u8 if funct == 4u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Div_E_u16 { rd, rs1, rs2 },
            4u8 if funct == 4u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Div_E_u32 { rd, rs1, rs2 },
            4u8 if funct == 4u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Div_E_u64 { rd, rs1, rs2 },
            4u8 if funct == 5u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Log_i8 { rd, rs1, rs2 },
            4u8 if funct == 5u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Log_i16 { rd, rs1, rs2 },
            4u8 if funct == 5u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Log_i32 { rd, rs1, rs2 },
            4u8 if funct == 5u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Log_i64 { rd, rs1, rs2 },
            4u8 if funct == 5u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Log_u8 { rd, rs1, rs2 },
            4u8 if funct == 5u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Log_u16 { rd, rs1, rs2 },
            4u8 if funct == 5u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Log_u32 { rd, rs1, rs2 },
            4u8 if funct == 5u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Log_u64 { rd, rs1, rs2 },
            4u8 if funct == 6u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Sqrt_i8 { rd, rs1 },
            4u8 if funct == 6u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Sqrt_i16 { rd, rs1 },
            4u8 if funct == 6u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Sqrt_i32 { rd, rs1 },
            4u8 if funct == 6u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Sqrt_i64 { rd, rs1 },
            4u8 if funct == 6u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Sqrt_u8 { rd, rs1 },
            4u8 if funct == 6u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Sqrt_u16 { rd, rs1 },
            4u8 if funct == 6u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Sqrt_u32 { rd, rs1 },
            4u8 if funct == 6u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Sqrt_u64 { rd, rs1 },
            4u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Mul_i8 { rd, rs1, rs2 },
            4u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Mul_i16 { rd, rs1, rs2 },
            4u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Mul_i32 { rd, rs1, rs2 },
            4u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Mul_i64 { rd, rs1, rs2 },
            4u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Mul_u8 { rd, rs1, rs2 },
            4u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Mul_u16 { rd, rs1, rs2 },
            4u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Mul_u32 { rd, rs1, rs2 },
            4u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Mul_u64 { rd, rs1, rs2 },
            4u8 if funct == 8u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Neg_i8 { rd, rs1 },
            4u8 if funct == 8u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Neg_i16 { rd, rs1 },
            4u8 if funct == 8u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Neg_i32 { rd, rs1 },
            4u8 if funct == 8u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Neg_i64 { rd, rs1 },
            4u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Pow_i8 { rd, rs1, rs2 },
            4u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Pow_i16 { rd, rs1, rs2 },
            4u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Pow_i32 { rd, rs1, rs2 },
            4u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Pow_i64 { rd, rs1, rs2 },
            4u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Pow_u8 { rd, rs1, rs2 },
            4u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Pow_u16 { rd, rs1, rs2 },
            4u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Pow_u32 { rd, rs1, rs2 },
            4u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Pow_u64 { rd, rs1, rs2 },
            4u8 if funct == 10u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Rem_i8 { rd, rs1, rs2 },
            4u8 if funct == 10u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Rem_i16 { rd, rs1, rs2 },
            4u8 if funct == 10u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Rem_i32 { rd, rs1, rs2 },
            4u8 if funct == 10u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Rem_i64 { rd, rs1, rs2 },
            4u8 if funct == 10u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Rem_u8 { rd, rs1, rs2 },
            4u8 if funct == 10u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Rem_u16 { rd, rs1, rs2 },
            4u8 if funct == 10u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Rem_u32 { rd, rs1, rs2 },
            4u8 if funct == 10u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Rem_u64 { rd, rs1, rs2 },
            4u8 if funct == 11u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Rem_E_i8 { rd, rs1, rs2 },
            4u8 if funct == 11u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Rem_E_i16 { rd, rs1, rs2 },
            4u8 if funct == 11u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Rem_E_i32 { rd, rs1, rs2 },
            4u8 if funct == 11u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Rem_E_i64 { rd, rs1, rs2 },
            4u8 if funct == 11u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Rem_E_u8 { rd, rs1, rs2 },
            4u8 if funct == 11u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Rem_E_u16 { rd, rs1, rs2 },
            4u8 if funct == 11u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Rem_E_u32 { rd, rs1, rs2 },
            4u8 if funct == 11u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Rem_E_u64 { rd, rs1, rs2 },
            4u8 if funct == 12u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Shl_i8 { rd, rs1, rs2 },
            4u8 if funct == 12u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Shl_i16 { rd, rs1, rs2 },
            4u8 if funct == 12u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Shl_i32 { rd, rs1, rs2 },
            4u8 if funct == 12u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Shl_i64 { rd, rs1, rs2 },
            4u8 if funct == 12u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Shl_u8 { rd, rs1, rs2 },
            4u8 if funct == 12u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Shl_u16 { rd, rs1, rs2 },
            4u8 if funct == 12u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Shl_u32 { rd, rs1, rs2 },
            4u8 if funct == 12u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Shl_u64 { rd, rs1, rs2 },
            4u8 if funct == 13u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Shr_i8 { rd, rs1, rs2 },
            4u8 if funct == 13u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Shr_i16 { rd, rs1, rs2 },
            4u8 if funct == 13u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Shr_i32 { rd, rs1, rs2 },
            4u8 if funct == 13u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Shr_i64 { rd, rs1, rs2 },
            4u8 if funct == 13u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Shr_u8 { rd, rs1, rs2 },
            4u8 if funct == 13u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Shr_u16 { rd, rs1, rs2 },
            4u8 if funct == 13u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Shr_u32 { rd, rs1, rs2 },
            4u8 if funct == 13u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Shr_u64 { rd, rs1, rs2 },
            4u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Sub_i8 { rd, rs1, rs2 },
            4u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Sub_i16 { rd, rs1, rs2 },
            4u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Sub_i32 { rd, rs1, rs2 },
            4u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Sub_i64 { rd, rs1, rs2 },
            4u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Sub_u8 { rd, rs1, rs2 },
            4u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Sub_u16 { rd, rs1, rs2 },
            4u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Sub_u32 { rd, rs1, rs2 },
            4u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::C_Sub_u64 { rd, rs1, rs2 },
            4u8 if funct == 15u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Sub_U_i8 { rd, rs1, rs2 },
            4u8 if funct == 15u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Sub_U_i16 { rd, rs1, rs2 },
            4u8 if funct == 15u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Sub_U_i32 { rd, rs1, rs2 },
            4u8 if funct == 15u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::C_Sub_U_i64 { rd, rs1, rs2 },
            5u8 if funct == 0u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Abs_i8 { rd, rs1 },
            5u8 if funct == 0u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Abs_i16 { rd, rs1 },
            5u8 if funct == 0u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Abs_i32 { rd, rs1 },
            5u8 if funct == 0u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Abs_i64 { rd, rs1 },
            5u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Add_i8 { rd, rs1, rs2 },
            5u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Add_i16 { rd, rs1, rs2 },
            5u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Add_i32 { rd, rs1, rs2 },
            5u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Add_i64 { rd, rs1, rs2 },
            5u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Add_u8 { rd, rs1, rs2 },
            5u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Add_u16 { rd, rs1, rs2 },
            5u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Add_u32 { rd, rs1, rs2 },
            5u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Add_u64 { rd, rs1, rs2 },
            5u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Add_U_i8 { rd, rs1, rs2 },
            5u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Add_U_i16 { rd, rs1, rs2 },
            5u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Add_U_i32 { rd, rs1, rs2 },
            5u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Add_U_i64 { rd, rs1, rs2 },
            5u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Add_S_u8 { rd, rs1, rs2 },
            5u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Add_S_u16 { rd, rs1, rs2 },
            5u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Add_S_u32 { rd, rs1, rs2 },
            5u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Add_S_u64 { rd, rs1, rs2 },
            5u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Div_i8 { rd, rs1, rs2 },
            5u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Div_i16 { rd, rs1, rs2 },
            5u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Div_i32 { rd, rs1, rs2 },
            5u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Div_i64 { rd, rs1, rs2 },
            5u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Div_u8 { rd, rs1, rs2 },
            5u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Div_u16 { rd, rs1, rs2 },
            5u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Div_u32 { rd, rs1, rs2 },
            5u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Div_u64 { rd, rs1, rs2 },
            5u8 if funct == 4u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Div_E_i8 { rd, rs1, rs2 },
            5u8 if funct == 4u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Div_E_i16 { rd, rs1, rs2 },
            5u8 if funct == 4u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Div_E_i32 { rd, rs1, rs2 },
            5u8 if funct == 4u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Div_E_i64 { rd, rs1, rs2 },
            5u8 if funct == 4u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Div_E_u8 { rd, rs1, rs2 },
            5u8 if funct == 4u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Div_E_u16 { rd, rs1, rs2 },
            5u8 if funct == 4u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Div_E_u32 { rd, rs1, rs2 },
            5u8 if funct == 4u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Div_E_u64 { rd, rs1, rs2 },
            5u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Mul_i8 { rd, rs1, rs2 },
            5u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Mul_i16 { rd, rs1, rs2 },
            5u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Mul_i32 { rd, rs1, rs2 },
            5u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Mul_i64 { rd, rs1, rs2 },
            5u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Mul_u8 { rd, rs1, rs2 },
            5u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Mul_u16 { rd, rs1, rs2 },
            5u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Mul_u32 { rd, rs1, rs2 },
            5u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Mul_u64 { rd, rs1, rs2 },
            5u8 if funct == 8u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Neg_i8 { rd, rs1 },
            5u8 if funct == 8u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Neg_i16 { rd, rs1 },
            5u8 if funct == 8u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Neg_i32 { rd, rs1 },
            5u8 if funct == 8u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Neg_i64 { rd, rs1 },
            5u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Pow_i8 { rd, rs1, rs2 },
            5u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Pow_i16 { rd, rs1, rs2 },
            5u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Pow_i32 { rd, rs1, rs2 },
            5u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Pow_i64 { rd, rs1, rs2 },
            5u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Pow_u8 { rd, rs1, rs2 },
            5u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Pow_u16 { rd, rs1, rs2 },
            5u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Pow_u32 { rd, rs1, rs2 },
            5u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Pow_u64 { rd, rs1, rs2 },
            5u8 if funct == 10u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Rem_i8 { rd, rs1, rs2 },
            5u8 if funct == 10u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Rem_i16 { rd, rs1, rs2 },
            5u8 if funct == 10u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Rem_i32 { rd, rs1, rs2 },
            5u8 if funct == 10u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Rem_i64 { rd, rs1, rs2 },
            5u8 if funct == 10u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Rem_u8 { rd, rs1, rs2 },
            5u8 if funct == 10u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Rem_u16 { rd, rs1, rs2 },
            5u8 if funct == 10u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Rem_u32 { rd, rs1, rs2 },
            5u8 if funct == 10u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Rem_u64 { rd, rs1, rs2 },
            5u8 if funct == 11u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Rem_E_i8 { rd, rs1, rs2 },
            5u8 if funct == 11u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Rem_E_i16 { rd, rs1, rs2 },
            5u8 if funct == 11u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Rem_E_i32 { rd, rs1, rs2 },
            5u8 if funct == 11u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Rem_E_i64 { rd, rs1, rs2 },
            5u8 if funct == 11u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Rem_E_u8 { rd, rs1, rs2 },
            5u8 if funct == 11u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Rem_E_u16 { rd, rs1, rs2 },
            5u8 if funct == 11u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Rem_E_u32 { rd, rs1, rs2 },
            5u8 if funct == 11u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Rem_E_u64 { rd, rs1, rs2 },
            5u8 if funct == 12u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Shl_i8 { rd, rs1, rs2 },
            5u8 if funct == 12u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Shl_i16 { rd, rs1, rs2 },
            5u8 if funct == 12u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Shl_i32 { rd, rs1, rs2 },
            5u8 if funct == 12u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Shl_i64 { rd, rs1, rs2 },
            5u8 if funct == 12u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Shl_u8 { rd, rs1, rs2 },
            5u8 if funct == 12u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Shl_u16 { rd, rs1, rs2 },
            5u8 if funct == 12u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Shl_u32 { rd, rs1, rs2 },
            5u8 if funct == 12u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Shl_u64 { rd, rs1, rs2 },
            5u8 if funct == 13u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Shr_i8 { rd, rs1, rs2 },
            5u8 if funct == 13u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Shr_i16 { rd, rs1, rs2 },
            5u8 if funct == 13u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Shr_i32 { rd, rs1, rs2 },
            5u8 if funct == 13u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Shr_i64 { rd, rs1, rs2 },
            5u8 if funct == 13u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Shr_u8 { rd, rs1, rs2 },
            5u8 if funct == 13u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Shr_u16 { rd, rs1, rs2 },
            5u8 if funct == 13u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Shr_u32 { rd, rs1, rs2 },
            5u8 if funct == 13u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Shr_u64 { rd, rs1, rs2 },
            5u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Sub_i8 { rd, rs1, rs2 },
            5u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Sub_i16 { rd, rs1, rs2 },
            5u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Sub_i32 { rd, rs1, rs2 },
            5u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Sub_i64 { rd, rs1, rs2 },
            5u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Sub_u8 { rd, rs1, rs2 },
            5u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Sub_u16 { rd, rs1, rs2 },
            5u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Sub_u32 { rd, rs1, rs2 },
            5u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::O_Sub_u64 { rd, rs1, rs2 },
            5u8 if funct == 15u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Sub_U_i8 { rd, rs1, rs2 },
            5u8 if funct == 15u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Sub_U_i16 { rd, rs1, rs2 },
            5u8 if funct == 15u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Sub_U_i32 { rd, rs1, rs2 },
            5u8 if funct == 15u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::O_Sub_U_i64 { rd, rs1, rs2 },
            6u8 if funct == 0u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Abs_i8 { rd, rs1 },
            6u8 if funct == 0u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Abs_i16 { rd, rs1 },
            6u8 if funct == 0u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Abs_i32 { rd, rs1 },
            6u8 if funct == 0u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Abs_i64 { rd, rs1 },
            6u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Add_i8 { rd, rs1, rs2 },
            6u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Add_i16 { rd, rs1, rs2 },
            6u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Add_i32 { rd, rs1, rs2 },
            6u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Add_i64 { rd, rs1, rs2 },
            6u8 if funct == 1u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Add_u8 { rd, rs1, rs2 },
            6u8 if funct == 1u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Add_u16 { rd, rs1, rs2 },
            6u8 if funct == 1u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Add_u32 { rd, rs1, rs2 },
            6u8 if funct == 1u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Add_u64 { rd, rs1, rs2 },
            6u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Add_U_i8 { rd, rs1, rs2 },
            6u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Add_U_i16 { rd, rs1, rs2 },
            6u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Add_U_i32 { rd, rs1, rs2 },
            6u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Add_U_i64 { rd, rs1, rs2 },
            6u8 if funct == 2u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Add_S_u8 { rd, rs1, rs2 },
            6u8 if funct == 2u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Add_S_u16 { rd, rs1, rs2 },
            6u8 if funct == 2u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Add_S_u32 { rd, rs1, rs2 },
            6u8 if funct == 2u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Add_S_u64 { rd, rs1, rs2 },
            6u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Div_i8 { rd, rs1, rs2 },
            6u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Div_i16 { rd, rs1, rs2 },
            6u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Div_i32 { rd, rs1, rs2 },
            6u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Div_i64 { rd, rs1, rs2 },
            6u8 if funct == 3u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Div_u8 { rd, rs1, rs2 },
            6u8 if funct == 3u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Div_u16 { rd, rs1, rs2 },
            6u8 if funct == 3u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Div_u32 { rd, rs1, rs2 },
            6u8 if funct == 3u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Div_u64 { rd, rs1, rs2 },
            6u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Mul_i8 { rd, rs1, rs2 },
            6u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Mul_i16 { rd, rs1, rs2 },
            6u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Mul_i32 { rd, rs1, rs2 },
            6u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Mul_i64 { rd, rs1, rs2 },
            6u8 if funct == 7u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Mul_u8 { rd, rs1, rs2 },
            6u8 if funct == 7u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Mul_u16 { rd, rs1, rs2 },
            6u8 if funct == 7u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Mul_u32 { rd, rs1, rs2 },
            6u8 if funct == 7u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Mul_u64 { rd, rs1, rs2 },
            6u8 if funct == 8u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Neg_i8 { rd, rs1 },
            6u8 if funct == 8u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Neg_i16 { rd, rs1 },
            6u8 if funct == 8u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Neg_i32 { rd, rs1 },
            6u8 if funct == 8u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Neg_i64 { rd, rs1 },
            6u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Pow_i8 { rd, rs1, rs2 },
            6u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Pow_i16 { rd, rs1, rs2 },
            6u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Pow_i32 { rd, rs1, rs2 },
            6u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Pow_i64 { rd, rs1, rs2 },
            6u8 if funct == 9u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Pow_u8 { rd, rs1, rs2 },
            6u8 if funct == 9u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Pow_u16 { rd, rs1, rs2 },
            6u8 if funct == 9u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Pow_u32 { rd, rs1, rs2 },
            6u8 if funct == 9u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Pow_u64 { rd, rs1, rs2 },
            6u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Sub_i8 { rd, rs1, rs2 },
            6u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Sub_i16 { rd, rs1, rs2 },
            6u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Sub_i32 { rd, rs1, rs2 },
            6u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Sub_i64 { rd, rs1, rs2 },
            6u8 if funct == 14u8 && size == 8u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Sub_u8 { rd, rs1, rs2 },
            6u8 if funct == 14u8 && size == 16u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Sub_u16 { rd, rs1, rs2 },
            6u8 if funct == 14u8 && size == 32u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Sub_u32 { rd, rs1, rs2 },
            6u8 if funct == 14u8 && size == 64u8 && f == 0u8 && s == 0u8
                && !rd.is_readonly() => Self::S_Sub_u64 { rd, rs1, rs2 },
            6u8 if funct == 15u8 && size == 8u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Sub_U_i8 { rd, rs1, rs2 },
            6u8 if funct == 15u8 && size == 16u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Sub_U_i16 { rd, rs1, rs2 },
            6u8 if funct == 15u8 && size == 32u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Sub_U_i32 { rd, rs1, rs2 },
            6u8 if funct == 15u8 && size == 64u8 && f == 0u8 && s == 1u8
                && !rd.is_readonly() => Self::S_Sub_U_i64 { rd, rs1, rs2 },
            7u8 if funct == 0u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Abs_f32 { rd, rs1 },
            7u8 if funct == 0u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Abs_f64 { rd, rs1 },
            7u8 if funct == 1u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Add_f32 { rd, rs1, rs2 },
            7u8 if funct == 1u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Add_f64 { rd, rs1, rs2 },
            7u8 if funct == 3u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Div_f32 { rd, rs1, rs2 },
            7u8 if funct == 3u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Div_f64 { rd, rs1, rs2 },
            7u8 if funct == 4u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Div_E_f32 { rd, rs1, rs2 },
            7u8 if funct == 4u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Div_E_f64 { rd, rs1, rs2 },
            7u8 if funct == 5u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Log_f32 { rd, rs1, rs2 },
            7u8 if funct == 5u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Log_f64 { rd, rs1, rs2 },
            7u8 if funct == 6u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Sqrt_f32 { rd, rs1 },
            7u8 if funct == 6u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Sqrt_f64 { rd, rs1 },
            7u8 if funct == 7u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Mul_f32 { rd, rs1, rs2 },
            7u8 if funct == 7u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Mul_f64 { rd, rs1, rs2 },
            7u8 if funct == 8u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Neg_f32 { rd, rs1 },
            7u8 if funct == 8u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Neg_f64 { rd, rs1 },
            7u8 if funct == 9u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Pow_f32 { rd, rs1, rs2 },
            7u8 if funct == 9u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Pow_f64 { rd, rs1, rs2 },
            7u8 if funct == 10u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Rem_f32 { rd, rs1, rs2 },
            7u8 if funct == 10u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Rem_f64 { rd, rs1, rs2 },
            7u8 if funct == 11u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Rem_E_f32 { rd, rs1, rs2 },
            7u8 if funct == 11u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Rem_E_f64 { rd, rs1, rs2 },
            7u8 if funct == 13u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Cbrt_f32 { rd, rs1 },
            7u8 if funct == 13u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Cbrt_f64 { rd, rs1 },
            7u8 if funct == 14u8 && size == 32u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Sub_f32 { rd, rs1, rs2 },
            7u8 if funct == 14u8 && size == 64u8 && f == 1u8 && s == 1u8
                && !rd.is_readonly() => Self::Sub_f64 { rd, rs1, rs2 },
            _ => Self::Invalid(instr),
        }
    }
    pub fn encode(&self) -> u32 {
        match self {
            Self::Halt => encode_funct(0u8) | encode_op_code(0u8),
            Self::Trap => encode_funct(1u8) | encode_op_code(0u8),
            Self::Call { rs2 } => {
                encode_s(0u8) | encode_rs2(*rs2) | encode_funct(2u8)
                    | encode_op_code(0u8)
            }
            Self::Call_R { rs2 } => {
                encode_s(1u8) | encode_rs2(*rs2) | encode_funct(2u8)
                    | encode_op_code(0u8)
            }
            Self::Call_I { imm } => {
                encode_imm(*imm) | encode_funct(3u8) | encode_op_code(0u8)
            }
            Self::Ret => encode_funct(4u8) | encode_op_code(0u8),
            Self::Ecall { imm } => {
                encode_imm(*imm) | encode_funct(5u8) | encode_op_code(0u8)
            }
            Self::Jal { rs2 } => {
                encode_s(0u8) | encode_rs2(*rs2) | encode_funct(6u8)
                    | encode_op_code(0u8)
            }
            Self::Jal_R { rs2 } => {
                encode_s(1u8) | encode_rs2(*rs2) | encode_funct(6u8)
                    | encode_op_code(0u8)
            }
            Self::Jal_I { imm } => {
                encode_imm(*imm) | encode_funct(7u8) | encode_op_code(0u8)
            }
            Self::Jnz { rs1, rs2 } => {
                encode_s(0u8) | encode_rs2(*rs2) | encode_rs1(*rs1) | encode_funct(8u8)
                    | encode_op_code(0u8)
            }
            Self::Jnz_R { rs1, rs2 } => {
                encode_s(1u8) | encode_rs2(*rs2) | encode_rs1(*rs1) | encode_funct(8u8)
                    | encode_op_code(0u8)
            }
            Self::Jnz_I { rd, imm } => {
                encode_imm(*imm) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(0u8)
            }
            Self::Jiz { rs1, rs2 } => {
                encode_s(0u8) | encode_rs2(*rs2) | encode_rs1(*rs1) | encode_funct(10u8)
                    | encode_op_code(0u8)
            }
            Self::Jiz_R { rs1, rs2 } => {
                encode_s(1u8) | encode_rs2(*rs2) | encode_rs1(*rs1) | encode_funct(10u8)
                    | encode_op_code(0u8)
            }
            Self::Jiz_I { rd, imm } => {
                encode_imm(*imm) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(0u8)
            }
            Self::Ld_8 { rd, rs1 } => {
                encode_size(8u8) | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(1u8)
            }
            Self::Ld_16 { rd, rs1 } => {
                encode_size(16u8) | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(1u8)
            }
            Self::Ld_32 { rd, rs1 } => {
                encode_size(32u8) | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(1u8)
            }
            Self::Ld_64 { rd, rs1 } => {
                encode_size(64u8) | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(1u8)
            }
            Self::Ld_I { rd, imm } => {
                encode_imm(*imm) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(1u8)
            }
            Self::Ld_A_8 { rd, imm } => {
                encode_imm(*imm as i16) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(1u8)
            }
            Self::Ld_A_16 { rd, imm } => {
                encode_imm(*imm as i16) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(1u8)
            }
            Self::Ld_A_32 { rd, imm } => {
                encode_imm(*imm as i16) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(1u8)
            }
            Self::Ld_A_64 { rd, imm } => {
                encode_imm(*imm as i16) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(1u8)
            }
            Self::St_8 { rd, rs1 } => {
                encode_size(8u8) | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(1u8)
            }
            Self::St_16 { rd, rs1 } => {
                encode_size(16u8) | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(1u8)
            }
            Self::St_32 { rd, rs1 } => {
                encode_size(32u8) | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(1u8)
            }
            Self::St_64 { rd, rs1 } => {
                encode_size(64u8) | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(1u8)
            }
            Self::St_I { rd, imm } => {
                encode_imm(*imm) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(1u8)
            }
            Self::Mov { rd, rs1 } => {
                encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(8u8)
                    | encode_op_code(1u8)
            }
            Self::Psh { rd } => encode_rd(*rd) | encode_funct(9u8) | encode_op_code(1u8),
            Self::Psh_I { imm } => {
                encode_imm(*imm) | encode_funct(10u8) | encode_op_code(1u8)
            }
            Self::Pop { rd } => encode_rd(*rd) | encode_funct(11u8) | encode_op_code(1u8),
            Self::Ie { rd, rs1, rs2 } => {
                encode_f(0u8) | encode_rs2(*rs2) | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(0u8) | encode_op_code(2u8)
            }
            Self::Ie_f32 { rd, rs1, rs2 } => {
                encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(2u8)
            }
            Self::Ie_f64 { rd, rs1, rs2 } => {
                encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(2u8)
            }
            Self::Ne { rd, rs1, rs2 } => {
                encode_f(0u8) | encode_rs2(*rs2) | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(1u8) | encode_op_code(2u8)
            }
            Self::Ne_f32 { rd, rs1, rs2 } => {
                encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(1u8) | encode_op_code(2u8)
            }
            Self::Ne_f64 { rd, rs1, rs2 } => {
                encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(1u8) | encode_op_code(2u8)
            }
            Self::Lt { rd, rs1, rs2 } => {
                encode_f(0u8) | encode_rs2(*rs2) | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(2u8) | encode_op_code(2u8)
            }
            Self::Lt_f32 { rd, rs1, rs2 } => {
                encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(2u8) | encode_op_code(2u8)
            }
            Self::Lt_f64 { rd, rs1, rs2 } => {
                encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(2u8) | encode_op_code(2u8)
            }
            Self::Le { rd, rs1, rs2 } => {
                encode_f(0u8) | encode_rs2(*rs2) | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(3u8) | encode_op_code(2u8)
            }
            Self::Le_f32 { rd, rs1, rs2 } => {
                encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(2u8)
            }
            Self::Le_f64 { rd, rs1, rs2 } => {
                encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(2u8)
            }
            Self::Gt { rd, rs1, rs2 } => {
                encode_f(0u8) | encode_rs2(*rs2) | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(4u8) | encode_op_code(2u8)
            }
            Self::Gt_f32 { rd, rs1, rs2 } => {
                encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(4u8) | encode_op_code(2u8)
            }
            Self::Gt_f64 { rd, rs1, rs2 } => {
                encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(4u8) | encode_op_code(2u8)
            }
            Self::Ge { rd, rs1, rs2 } => {
                encode_f(0u8) | encode_rs2(*rs2) | encode_rs1(*rs1) | encode_rd(*rd)
                    | encode_funct(5u8) | encode_op_code(2u8)
            }
            Self::Ge_f32 { rd, rs1, rs2 } => {
                encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(5u8) | encode_op_code(2u8)
            }
            Self::Ge_f64 { rd, rs1, rs2 } => {
                encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(5u8) | encode_op_code(2u8)
            }
            Self::And_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Self::And_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Self::And_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Self::And_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Self::And_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Self::And_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Self::And_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Self::And_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(0u8)
                    | encode_op_code(3u8)
            }
            Self::Or_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Self::Or_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Self::Or_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Self::Or_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Self::Or_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Self::Or_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Self::Or_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Self::Or_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(3u8)
            }
            Self::Xor_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Self::Xor_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Self::Xor_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Self::Xor_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Self::Xor_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Self::Xor_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Self::Xor_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Self::Xor_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(3u8)
            }
            Self::Not_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Self::Not_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Self::Not_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Self::Not_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Self::Not_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Self::Not_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Self::Not_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Self::Not_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(3u8) | encode_op_code(3u8)
            }
            Self::Shl_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Self::Shl_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Self::Shl_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Self::Shl_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Self::Shl_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Self::Shl_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Self::Shl_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Self::Shl_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(3u8)
            }
            Self::Shr_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Self::Shr_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Self::Shr_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Self::Shr_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Self::Shr_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Self::Shr_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Self::Shr_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Self::Shr_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_L_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_L_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_L_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_L_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_L_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_L_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_L_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_L_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(6u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_R_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_R_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_R_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_R_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_R_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_R_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_R_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Self::Rot_R_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(3u8)
            }
            Self::C_Ones_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Self::C_Ones_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Self::C_Ones_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Self::C_Ones_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Self::C_Ones_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Self::C_Ones_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Self::C_Ones_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Self::C_Ones_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(3u8)
            }
            Self::L_Ones_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Self::L_Ones_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Self::L_Ones_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Self::L_Ones_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Self::L_Ones_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Self::L_Ones_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Self::L_Ones_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Self::L_Ones_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(9u8) | encode_op_code(3u8)
            }
            Self::T_Ones_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Self::T_Ones_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Self::T_Ones_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Self::T_Ones_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Self::T_Ones_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Self::T_Ones_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Self::T_Ones_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Self::T_Ones_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(10u8) | encode_op_code(3u8)
            }
            Self::C_Zeros_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Self::C_Zeros_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Self::C_Zeros_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Self::C_Zeros_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Self::C_Zeros_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Self::C_Zeros_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Self::C_Zeros_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Self::C_Zeros_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(11u8) | encode_op_code(3u8)
            }
            Self::L_Zeros_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Self::L_Zeros_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Self::L_Zeros_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Self::L_Zeros_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Self::L_Zeros_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Self::L_Zeros_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Self::L_Zeros_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Self::L_Zeros_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(12u8) | encode_op_code(3u8)
            }
            Self::T_Zeros_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Self::T_Zeros_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Self::T_Zeros_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Self::T_Zeros_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Self::T_Zeros_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Self::T_Zeros_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Self::T_Zeros_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Self::T_Zeros_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(3u8)
            }
            Self::R_Bytes_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Self::R_Bytes_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Self::R_Bytes_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Self::R_Bytes_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Self::R_Bytes_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Self::R_Bytes_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Self::R_Bytes_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Self::R_Bytes_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(14u8) | encode_op_code(3u8)
            }
            Self::R_Bits_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Self::R_Bits_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Self::R_Bits_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Self::R_Bits_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Self::R_Bits_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Self::R_Bits_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Self::R_Bits_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Self::R_Bits_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(15u8) | encode_op_code(3u8)
            }
            Self::C_Abs_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(4u8)
            }
            Self::C_Abs_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(4u8)
            }
            Self::C_Abs_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(4u8)
            }
            Self::C_Abs_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(4u8)
            }
            Self::C_Add_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_U_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_U_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_U_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_U_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_S_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_S_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_S_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Self::C_Add_S_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_E_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_E_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_E_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_E_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_E_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_E_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_E_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Self::C_Div_E_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(4u8)
            }
            Self::C_Log_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Self::C_Log_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Self::C_Log_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Self::C_Log_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Self::C_Log_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Self::C_Log_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Self::C_Log_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Self::C_Log_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(4u8)
            }
            Self::C_Sqrt_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Self::C_Sqrt_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Self::C_Sqrt_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Self::C_Sqrt_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Self::C_Sqrt_u8 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Self::C_Sqrt_u16 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Self::C_Sqrt_u32 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Self::C_Sqrt_u64 { rd, rs1 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(4u8)
            }
            Self::C_Mul_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Self::C_Mul_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Self::C_Mul_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Self::C_Mul_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Self::C_Mul_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Self::C_Mul_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Self::C_Mul_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Self::C_Mul_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(4u8)
            }
            Self::C_Neg_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(4u8)
            }
            Self::C_Neg_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(4u8)
            }
            Self::C_Neg_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(4u8)
            }
            Self::C_Neg_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(4u8)
            }
            Self::C_Pow_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Self::C_Pow_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Self::C_Pow_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Self::C_Pow_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Self::C_Pow_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Self::C_Pow_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Self::C_Pow_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Self::C_Pow_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_E_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_E_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_E_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_E_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_E_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_E_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_E_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Self::C_Rem_E_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shl_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shl_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shl_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shl_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shl_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shl_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shl_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shl_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shr_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shr_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shr_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shr_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shr_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shr_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shr_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Self::C_Shr_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(4u8)
            }
            Self::C_Sub_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Self::C_Sub_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Self::C_Sub_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Self::C_Sub_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Self::C_Sub_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Self::C_Sub_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Self::C_Sub_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Self::C_Sub_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(4u8)
            }
            Self::C_Sub_U_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(4u8)
            }
            Self::C_Sub_U_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(4u8)
            }
            Self::C_Sub_U_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(4u8)
            }
            Self::C_Sub_U_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(4u8)
            }
            Self::O_Abs_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(5u8)
            }
            Self::O_Abs_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(5u8)
            }
            Self::O_Abs_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(5u8)
            }
            Self::O_Abs_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(5u8)
            }
            Self::O_Add_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_U_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_U_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_U_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_U_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_S_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_S_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_S_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Self::O_Add_S_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_E_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_E_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_E_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_E_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_E_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_E_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_E_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Self::O_Div_E_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(5u8)
            }
            Self::O_Mul_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Self::O_Mul_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Self::O_Mul_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Self::O_Mul_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Self::O_Mul_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Self::O_Mul_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Self::O_Mul_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Self::O_Mul_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(5u8)
            }
            Self::O_Neg_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(5u8)
            }
            Self::O_Neg_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(5u8)
            }
            Self::O_Neg_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(5u8)
            }
            Self::O_Neg_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(5u8)
            }
            Self::O_Pow_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Self::O_Pow_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Self::O_Pow_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Self::O_Pow_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Self::O_Pow_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Self::O_Pow_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Self::O_Pow_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Self::O_Pow_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_E_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_E_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_E_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_E_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_E_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_E_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_E_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Self::O_Rem_E_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shl_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shl_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shl_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shl_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shl_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shl_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shl_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shl_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(12u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shr_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shr_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shr_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shr_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shr_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shr_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shr_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Self::O_Shr_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(13u8)
                    | encode_op_code(5u8)
            }
            Self::O_Sub_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Self::O_Sub_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Self::O_Sub_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Self::O_Sub_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Self::O_Sub_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Self::O_Sub_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Self::O_Sub_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Self::O_Sub_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(5u8)
            }
            Self::O_Sub_U_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(5u8)
            }
            Self::O_Sub_U_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(5u8)
            }
            Self::O_Sub_U_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(5u8)
            }
            Self::O_Sub_U_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(5u8)
            }
            Self::S_Abs_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(6u8)
            }
            Self::S_Abs_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(6u8)
            }
            Self::S_Abs_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(6u8)
            }
            Self::S_Abs_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(6u8)
            }
            Self::S_Add_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_U_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_U_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_U_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_U_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_S_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_S_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_S_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Self::S_Add_S_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(2u8)
                    | encode_op_code(6u8)
            }
            Self::S_Div_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Self::S_Div_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Self::S_Div_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Self::S_Div_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Self::S_Div_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Self::S_Div_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Self::S_Div_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Self::S_Div_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(6u8)
            }
            Self::S_Mul_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Self::S_Mul_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Self::S_Mul_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Self::S_Mul_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Self::S_Mul_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Self::S_Mul_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Self::S_Mul_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Self::S_Mul_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(6u8)
            }
            Self::S_Neg_i8 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(6u8)
            }
            Self::S_Neg_i16 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(6u8)
            }
            Self::S_Neg_i32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(6u8)
            }
            Self::S_Neg_i64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(6u8)
            }
            Self::S_Pow_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Self::S_Pow_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Self::S_Pow_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Self::S_Pow_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Self::S_Pow_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Self::S_Pow_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Self::S_Pow_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Self::S_Pow_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(6u8)
            }
            Self::S_Sub_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Self::S_Sub_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Self::S_Sub_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Self::S_Sub_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Self::S_Sub_u8 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Self::S_Sub_u16 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Self::S_Sub_u32 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Self::S_Sub_u64 { rd, rs1, rs2 } => {
                encode_s(0u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(6u8)
            }
            Self::S_Sub_U_i8 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(8u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(6u8)
            }
            Self::S_Sub_U_i16 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(16u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(6u8)
            }
            Self::S_Sub_U_i32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(6u8)
            }
            Self::S_Sub_U_i64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(0u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(15u8)
                    | encode_op_code(6u8)
            }
            Self::Abs_f32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(7u8)
            }
            Self::Abs_f64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(0u8) | encode_op_code(7u8)
            }
            Self::Add_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(7u8)
            }
            Self::Add_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(1u8)
                    | encode_op_code(7u8)
            }
            Self::Div_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(7u8)
            }
            Self::Div_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(3u8)
                    | encode_op_code(7u8)
            }
            Self::Div_E_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(7u8)
            }
            Self::Div_E_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(4u8)
                    | encode_op_code(7u8)
            }
            Self::Log_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(7u8)
            }
            Self::Log_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(5u8)
                    | encode_op_code(7u8)
            }
            Self::Sqrt_f32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(7u8)
            }
            Self::Sqrt_f64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(6u8) | encode_op_code(7u8)
            }
            Self::Mul_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(7u8)
            }
            Self::Mul_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(7u8)
                    | encode_op_code(7u8)
            }
            Self::Neg_f32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(7u8)
            }
            Self::Neg_f64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(8u8) | encode_op_code(7u8)
            }
            Self::Pow_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(7u8)
            }
            Self::Pow_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(9u8)
                    | encode_op_code(7u8)
            }
            Self::Rem_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(7u8)
            }
            Self::Rem_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(10u8)
                    | encode_op_code(7u8)
            }
            Self::Rem_E_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(7u8)
            }
            Self::Rem_E_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(11u8)
                    | encode_op_code(7u8)
            }
            Self::Cbrt_f32 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(7u8)
            }
            Self::Cbrt_f64 { rd, rs1 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs1(*rs1)
                    | encode_rd(*rd) | encode_funct(13u8) | encode_op_code(7u8)
            }
            Self::Sub_f32 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(32u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(7u8)
            }
            Self::Sub_f64 { rd, rs1, rs2 } => {
                encode_s(1u8) | encode_f(1u8) | encode_size(64u8) | encode_rs2(*rs2)
                    | encode_rs1(*rs1) | encode_rd(*rd) | encode_funct(14u8)
                    | encode_op_code(7u8)
            }
            Self::Invalid(instr) => *instr,
        }
    }
}
#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    #[test]
    fn decode_halt() {
        assert_eq!(Instruction::decode(0u32), Instruction::Halt);
    }
    #[test]
    fn encode_halt() {
        assert_eq!(Instruction::Halt.encode(), 0u32);
    }
    #[test]
    fn decode_trap() {
        assert_eq!(Instruction::decode(64u32), Instruction::Trap);
    }
    #[test]
    fn encode_trap() {
        assert_eq!(Instruction::Trap.encode(), 64u32);
    }
    #[test]
    fn decode_call() {
        assert_eq!(
            Instruction::decode(150995072u32), Instruction::Call { rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_call() {
        assert_eq!(
            Instruction::Call { rs2 : Register::General_Purpose(30), } .encode(),
            150995072u32
        );
    }
    #[test]
    fn decode_call_r() {
        assert_eq!(
            Instruction::decode(2298478720u32), Instruction::Call_R { rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_call_r() {
        assert_eq!(
            Instruction::Call_R { rs2 : Register::General_Purpose(30), } .encode(),
            2298478720u32
        );
    }
    #[test]
    fn decode_call_i() {
        assert_eq!(
            Instruction::decode(4293918912u32), Instruction::Call_I { imm : - 16, }
        );
    }
    #[test]
    fn encode_call_i() {
        assert_eq!(Instruction::Call_I { imm : - 16, } .encode(), 4293918912u32);
    }
    #[test]
    fn decode_ret() {
        assert_eq!(Instruction::decode(256u32), Instruction::Ret);
    }
    #[test]
    fn encode_ret() {
        assert_eq!(Instruction::Ret.encode(), 256u32);
    }
    #[test]
    fn decode_ecall() {
        assert_eq!(
            Instruction::decode(4293919040u32), Instruction::Ecall { imm : - 16, }
        );
    }
    #[test]
    fn encode_ecall() {
        assert_eq!(Instruction::Ecall { imm : - 16, } .encode(), 4293919040u32);
    }
    #[test]
    fn decode_jal() {
        assert_eq!(
            Instruction::decode(150995328u32), Instruction::Jal { rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_jal() {
        assert_eq!(
            Instruction::Jal { rs2 : Register::General_Purpose(30), } .encode(),
            150995328u32
        );
    }
    #[test]
    fn decode_jal_r() {
        assert_eq!(
            Instruction::decode(2298478976u32), Instruction::Jal_R { rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_jal_r() {
        assert_eq!(
            Instruction::Jal_R { rs2 : Register::General_Purpose(30), } .encode(),
            2298478976u32
        );
    }
    #[test]
    fn decode_jal_i() {
        assert_eq!(
            Instruction::decode(4293919168u32), Instruction::Jal_I { imm : - 16, }
        );
    }
    #[test]
    fn encode_jal_i() {
        assert_eq!(Instruction::Jal_I { imm : - 16, } .encode(), 4293919168u32);
    }
    #[test]
    fn decode_jnz() {
        assert_eq!(
            Instruction::decode(151912960u32), Instruction::Jnz { rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_jnz() {
        assert_eq!(
            Instruction::Jnz { rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), } .encode(), 151912960u32
        );
    }
    #[test]
    fn decode_jnz_r() {
        assert_eq!(
            Instruction::decode(2299396608u32), Instruction::Jnz_R { rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_jnz_r() {
        assert_eq!(
            Instruction::Jnz_R { rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), } .encode(), 2299396608u32
        );
    }
    #[test]
    fn decode_jnz_i() {
        assert_eq!(
            Instruction::decode(4293930560u32), Instruction::Jnz_I { rd :
            Register::General_Purpose(5), imm : - 16, }
        );
    }
    #[test]
    fn encode_jnz_i() {
        assert_eq!(
            Instruction::Jnz_I { rd : Register::General_Purpose(5), imm : - 16, }
            .encode(), 4293930560u32
        );
    }
    #[test]
    fn decode_jiz() {
        assert_eq!(
            Instruction::decode(151913088u32), Instruction::Jiz { rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_jiz() {
        assert_eq!(
            Instruction::Jiz { rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), } .encode(), 151913088u32
        );
    }
    #[test]
    fn decode_jiz_r() {
        assert_eq!(
            Instruction::decode(2299396736u32), Instruction::Jiz_R { rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_jiz_r() {
        assert_eq!(
            Instruction::Jiz_R { rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), } .encode(), 2299396736u32
        );
    }
    #[test]
    fn decode_jiz_i() {
        assert_eq!(
            Instruction::decode(4293930688u32), Instruction::Jiz_I { rd :
            Register::General_Purpose(5), imm : - 16, }
        );
    }
    #[test]
    fn encode_jiz_i() {
        assert_eq!(
            Instruction::Jiz_I { rd : Register::General_Purpose(5), imm : - 16, }
            .encode(), 4293930688u32
        );
    }
    #[test]
    fn decode_ld_8() {
        assert_eq!(
            Instruction::decode(928769u32), Instruction::Ld_8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_ld_8() {
        assert_eq!(
            Instruction::Ld_8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 928769u32
        );
    }
    #[test]
    fn decode_ld_16() {
        assert_eq!(
            Instruction::decode(269364225u32), Instruction::Ld_16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_ld_16() {
        assert_eq!(
            Instruction::Ld_16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364225u32
        );
    }
    #[test]
    fn decode_ld_32() {
        assert_eq!(
            Instruction::decode(537799681u32), Instruction::Ld_32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_ld_32() {
        assert_eq!(
            Instruction::Ld_32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537799681u32
        );
    }
    #[test]
    fn decode_ld_64() {
        assert_eq!(
            Instruction::decode(806235137u32), Instruction::Ld_64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_ld_64() {
        assert_eq!(
            Instruction::Ld_64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235137u32
        );
    }
    #[test]
    fn decode_ld_i() {
        assert_eq!(
            Instruction::decode(4293930049u32), Instruction::Ld_I { rd :
            Register::General_Purpose(5), imm : - 16, }
        );
    }
    #[test]
    fn encode_ld_i() {
        assert_eq!(
            Instruction::Ld_I { rd : Register::General_Purpose(5), imm : - 16, }
            .encode(), 4293930049u32
        );
    }
    #[test]
    fn decode_ld_a_8() {
        assert_eq!(
            Instruction::decode(1059969u32), Instruction::Ld_A_8 { rd :
            Register::General_Purpose(5), imm : 16, }
        );
    }
    #[test]
    fn encode_ld_a_8() {
        assert_eq!(
            Instruction::Ld_A_8 { rd : Register::General_Purpose(5), imm : 16, }
            .encode(), 1059969u32
        );
    }
    #[test]
    fn decode_ld_a_16() {
        assert_eq!(
            Instruction::decode(1060033u32), Instruction::Ld_A_16 { rd :
            Register::General_Purpose(5), imm : 16, }
        );
    }
    #[test]
    fn encode_ld_a_16() {
        assert_eq!(
            Instruction::Ld_A_16 { rd : Register::General_Purpose(5), imm : 16, }
            .encode(), 1060033u32
        );
    }
    #[test]
    fn decode_ld_a_32() {
        assert_eq!(
            Instruction::decode(1060097u32), Instruction::Ld_A_32 { rd :
            Register::General_Purpose(5), imm : 16, }
        );
    }
    #[test]
    fn encode_ld_a_32() {
        assert_eq!(
            Instruction::Ld_A_32 { rd : Register::General_Purpose(5), imm : 16, }
            .encode(), 1060097u32
        );
    }
    #[test]
    fn decode_ld_a_64() {
        assert_eq!(
            Instruction::decode(1060161u32), Instruction::Ld_A_64 { rd :
            Register::General_Purpose(5), imm : 16, }
        );
    }
    #[test]
    fn encode_ld_a_64() {
        assert_eq!(
            Instruction::Ld_A_64 { rd : Register::General_Purpose(5), imm : 16, }
            .encode(), 1060161u32
        );
    }
    #[test]
    fn decode_st_8() {
        assert_eq!(
            Instruction::decode(929153u32), Instruction::St_8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_st_8() {
        assert_eq!(
            Instruction::St_8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929153u32
        );
    }
    #[test]
    fn decode_st_16() {
        assert_eq!(
            Instruction::decode(269364609u32), Instruction::St_16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_st_16() {
        assert_eq!(
            Instruction::St_16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364609u32
        );
    }
    #[test]
    fn decode_st_32() {
        assert_eq!(
            Instruction::decode(537800065u32), Instruction::St_32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_st_32() {
        assert_eq!(
            Instruction::St_32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800065u32
        );
    }
    #[test]
    fn decode_st_64() {
        assert_eq!(
            Instruction::decode(806235521u32), Instruction::St_64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_st_64() {
        assert_eq!(
            Instruction::St_64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235521u32
        );
    }
    #[test]
    fn decode_st_i() {
        assert_eq!(
            Instruction::decode(4293930433u32), Instruction::St_I { rd :
            Register::General_Purpose(5), imm : - 16, }
        );
    }
    #[test]
    fn encode_st_i() {
        assert_eq!(
            Instruction::St_I { rd : Register::General_Purpose(5), imm : - 16, }
            .encode(), 4293930433u32
        );
    }
    #[test]
    fn decode_mov() {
        assert_eq!(
            Instruction::decode(929281u32), Instruction::Mov { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_mov() {
        assert_eq!(
            Instruction::Mov { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929281u32
        );
    }
    #[test]
    fn decode_psh() {
        assert_eq!(
            Instruction::decode(11841u32), Instruction::Psh { rd :
            Register::General_Purpose(5), }
        );
    }
    #[test]
    fn encode_psh() {
        assert_eq!(
            Instruction::Psh { rd : Register::General_Purpose(5), } .encode(), 11841u32
        );
    }
    #[test]
    fn decode_psh_i() {
        assert_eq!(
            Instruction::decode(4293919361u32), Instruction::Psh_I { imm : - 16, }
        );
    }
    #[test]
    fn encode_psh_i() {
        assert_eq!(Instruction::Psh_I { imm : - 16, } .encode(), 4293919361u32);
    }
    #[test]
    fn decode_pop() {
        assert_eq!(
            Instruction::decode(11969u32), Instruction::Pop { rd :
            Register::General_Purpose(5), }
        );
    }
    #[test]
    fn encode_pop() {
        assert_eq!(
            Instruction::Pop { rd : Register::General_Purpose(5), } .encode(), 11969u32
        );
    }
    #[test]
    fn decode_ie() {
        assert_eq!(
            Instruction::decode(151923714u32), Instruction::Ie { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_ie() {
        assert_eq!(
            Instruction::Ie { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923714u32
        );
    }
    #[test]
    fn decode_ie_f32() {
        assert_eq!(
            Instruction::decode(1762536450u32), Instruction::Ie_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_ie_f32() {
        assert_eq!(
            Instruction::Ie_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 1762536450u32
        );
    }
    #[test]
    fn decode_ie_f64() {
        assert_eq!(
            Instruction::decode(2030971906u32), Instruction::Ie_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_ie_f64() {
        assert_eq!(
            Instruction::Ie_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2030971906u32
        );
    }
    #[test]
    fn decode_ne() {
        assert_eq!(
            Instruction::decode(151923778u32), Instruction::Ne { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_ne() {
        assert_eq!(
            Instruction::Ne { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923778u32
        );
    }
    #[test]
    fn decode_ne_f32() {
        assert_eq!(
            Instruction::decode(1762536514u32), Instruction::Ne_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_ne_f32() {
        assert_eq!(
            Instruction::Ne_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 1762536514u32
        );
    }
    #[test]
    fn decode_ne_f64() {
        assert_eq!(
            Instruction::decode(2030971970u32), Instruction::Ne_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_ne_f64() {
        assert_eq!(
            Instruction::Ne_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2030971970u32
        );
    }
    #[test]
    fn decode_lt() {
        assert_eq!(
            Instruction::decode(151923842u32), Instruction::Lt { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_lt() {
        assert_eq!(
            Instruction::Lt { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923842u32
        );
    }
    #[test]
    fn decode_lt_f32() {
        assert_eq!(
            Instruction::decode(1762536578u32), Instruction::Lt_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_lt_f32() {
        assert_eq!(
            Instruction::Lt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 1762536578u32
        );
    }
    #[test]
    fn decode_lt_f64() {
        assert_eq!(
            Instruction::decode(2030972034u32), Instruction::Lt_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_lt_f64() {
        assert_eq!(
            Instruction::Lt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2030972034u32
        );
    }
    #[test]
    fn decode_le() {
        assert_eq!(
            Instruction::decode(151923906u32), Instruction::Le { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_le() {
        assert_eq!(
            Instruction::Le { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923906u32
        );
    }
    #[test]
    fn decode_le_f32() {
        assert_eq!(
            Instruction::decode(1762536642u32), Instruction::Le_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_le_f32() {
        assert_eq!(
            Instruction::Le_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 1762536642u32
        );
    }
    #[test]
    fn decode_le_f64() {
        assert_eq!(
            Instruction::decode(2030972098u32), Instruction::Le_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_le_f64() {
        assert_eq!(
            Instruction::Le_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2030972098u32
        );
    }
    #[test]
    fn decode_gt() {
        assert_eq!(
            Instruction::decode(151923970u32), Instruction::Gt { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_gt() {
        assert_eq!(
            Instruction::Gt { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923970u32
        );
    }
    #[test]
    fn decode_gt_f32() {
        assert_eq!(
            Instruction::decode(1762536706u32), Instruction::Gt_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_gt_f32() {
        assert_eq!(
            Instruction::Gt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 1762536706u32
        );
    }
    #[test]
    fn decode_gt_f64() {
        assert_eq!(
            Instruction::decode(2030972162u32), Instruction::Gt_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_gt_f64() {
        assert_eq!(
            Instruction::Gt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2030972162u32
        );
    }
    #[test]
    fn decode_ge() {
        assert_eq!(
            Instruction::decode(151924034u32), Instruction::Ge { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_ge() {
        assert_eq!(
            Instruction::Ge { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924034u32
        );
    }
    #[test]
    fn decode_ge_f32() {
        assert_eq!(
            Instruction::decode(1762536770u32), Instruction::Ge_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_ge_f32() {
        assert_eq!(
            Instruction::Ge_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 1762536770u32
        );
    }
    #[test]
    fn decode_ge_f64() {
        assert_eq!(
            Instruction::decode(2030972226u32), Instruction::Ge_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_ge_f64() {
        assert_eq!(
            Instruction::Ge_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2030972226u32
        );
    }
    #[test]
    fn decode_and_i8() {
        assert_eq!(
            Instruction::decode(2299407363u32), Instruction::And_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_and_i8() {
        assert_eq!(
            Instruction::And_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407363u32
        );
    }
    #[test]
    fn decode_and_i16() {
        assert_eq!(
            Instruction::decode(2567842819u32), Instruction::And_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_and_i16() {
        assert_eq!(
            Instruction::And_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567842819u32
        );
    }
    #[test]
    fn decode_and_i32() {
        assert_eq!(
            Instruction::decode(2836278275u32), Instruction::And_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_and_i32() {
        assert_eq!(
            Instruction::And_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278275u32
        );
    }
    #[test]
    fn decode_and_i64() {
        assert_eq!(
            Instruction::decode(3104713731u32), Instruction::And_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_and_i64() {
        assert_eq!(
            Instruction::And_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713731u32
        );
    }
    #[test]
    fn decode_and_u8() {
        assert_eq!(
            Instruction::decode(151923715u32), Instruction::And_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_and_u8() {
        assert_eq!(
            Instruction::And_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923715u32
        );
    }
    #[test]
    fn decode_and_u16() {
        assert_eq!(
            Instruction::decode(420359171u32), Instruction::And_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_and_u16() {
        assert_eq!(
            Instruction::And_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359171u32
        );
    }
    #[test]
    fn decode_and_u32() {
        assert_eq!(
            Instruction::decode(688794627u32), Instruction::And_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_and_u32() {
        assert_eq!(
            Instruction::And_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794627u32
        );
    }
    #[test]
    fn decode_and_u64() {
        assert_eq!(
            Instruction::decode(957230083u32), Instruction::And_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_and_u64() {
        assert_eq!(
            Instruction::And_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230083u32
        );
    }
    #[test]
    fn decode_or_i8() {
        assert_eq!(
            Instruction::decode(2299407427u32), Instruction::Or_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_or_i8() {
        assert_eq!(
            Instruction::Or_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407427u32
        );
    }
    #[test]
    fn decode_or_i16() {
        assert_eq!(
            Instruction::decode(2567842883u32), Instruction::Or_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_or_i16() {
        assert_eq!(
            Instruction::Or_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567842883u32
        );
    }
    #[test]
    fn decode_or_i32() {
        assert_eq!(
            Instruction::decode(2836278339u32), Instruction::Or_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_or_i32() {
        assert_eq!(
            Instruction::Or_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278339u32
        );
    }
    #[test]
    fn decode_or_i64() {
        assert_eq!(
            Instruction::decode(3104713795u32), Instruction::Or_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_or_i64() {
        assert_eq!(
            Instruction::Or_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713795u32
        );
    }
    #[test]
    fn decode_or_u8() {
        assert_eq!(
            Instruction::decode(151923779u32), Instruction::Or_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_or_u8() {
        assert_eq!(
            Instruction::Or_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923779u32
        );
    }
    #[test]
    fn decode_or_u16() {
        assert_eq!(
            Instruction::decode(420359235u32), Instruction::Or_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_or_u16() {
        assert_eq!(
            Instruction::Or_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359235u32
        );
    }
    #[test]
    fn decode_or_u32() {
        assert_eq!(
            Instruction::decode(688794691u32), Instruction::Or_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_or_u32() {
        assert_eq!(
            Instruction::Or_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794691u32
        );
    }
    #[test]
    fn decode_or_u64() {
        assert_eq!(
            Instruction::decode(957230147u32), Instruction::Or_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_or_u64() {
        assert_eq!(
            Instruction::Or_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230147u32
        );
    }
    #[test]
    fn decode_xor_i8() {
        assert_eq!(
            Instruction::decode(2299407491u32), Instruction::Xor_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_xor_i8() {
        assert_eq!(
            Instruction::Xor_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407491u32
        );
    }
    #[test]
    fn decode_xor_i16() {
        assert_eq!(
            Instruction::decode(2567842947u32), Instruction::Xor_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_xor_i16() {
        assert_eq!(
            Instruction::Xor_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567842947u32
        );
    }
    #[test]
    fn decode_xor_i32() {
        assert_eq!(
            Instruction::decode(2836278403u32), Instruction::Xor_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_xor_i32() {
        assert_eq!(
            Instruction::Xor_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278403u32
        );
    }
    #[test]
    fn decode_xor_i64() {
        assert_eq!(
            Instruction::decode(3104713859u32), Instruction::Xor_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_xor_i64() {
        assert_eq!(
            Instruction::Xor_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713859u32
        );
    }
    #[test]
    fn decode_xor_u8() {
        assert_eq!(
            Instruction::decode(151923843u32), Instruction::Xor_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_xor_u8() {
        assert_eq!(
            Instruction::Xor_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923843u32
        );
    }
    #[test]
    fn decode_xor_u16() {
        assert_eq!(
            Instruction::decode(420359299u32), Instruction::Xor_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_xor_u16() {
        assert_eq!(
            Instruction::Xor_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359299u32
        );
    }
    #[test]
    fn decode_xor_u32() {
        assert_eq!(
            Instruction::decode(688794755u32), Instruction::Xor_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_xor_u32() {
        assert_eq!(
            Instruction::Xor_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794755u32
        );
    }
    #[test]
    fn decode_xor_u64() {
        assert_eq!(
            Instruction::decode(957230211u32), Instruction::Xor_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_xor_u64() {
        assert_eq!(
            Instruction::Xor_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230211u32
        );
    }
    #[test]
    fn decode_not_i8() {
        assert_eq!(
            Instruction::decode(2148412611u32), Instruction::Not_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_not_i8() {
        assert_eq!(
            Instruction::Not_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412611u32
        );
    }
    #[test]
    fn decode_not_i16() {
        assert_eq!(
            Instruction::decode(2416848067u32), Instruction::Not_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_not_i16() {
        assert_eq!(
            Instruction::Not_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848067u32
        );
    }
    #[test]
    fn decode_not_i32() {
        assert_eq!(
            Instruction::decode(2685283523u32), Instruction::Not_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_not_i32() {
        assert_eq!(
            Instruction::Not_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283523u32
        );
    }
    #[test]
    fn decode_not_i64() {
        assert_eq!(
            Instruction::decode(2953718979u32), Instruction::Not_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_not_i64() {
        assert_eq!(
            Instruction::Not_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953718979u32
        );
    }
    #[test]
    fn decode_not_u8() {
        assert_eq!(
            Instruction::decode(928963u32), Instruction::Not_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_not_u8() {
        assert_eq!(
            Instruction::Not_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 928963u32
        );
    }
    #[test]
    fn decode_not_u16() {
        assert_eq!(
            Instruction::decode(269364419u32), Instruction::Not_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_not_u16() {
        assert_eq!(
            Instruction::Not_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364419u32
        );
    }
    #[test]
    fn decode_not_u32() {
        assert_eq!(
            Instruction::decode(537799875u32), Instruction::Not_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_not_u32() {
        assert_eq!(
            Instruction::Not_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537799875u32
        );
    }
    #[test]
    fn decode_not_u64() {
        assert_eq!(
            Instruction::decode(806235331u32), Instruction::Not_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_not_u64() {
        assert_eq!(
            Instruction::Not_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235331u32
        );
    }
    #[test]
    fn decode_shl_i8() {
        assert_eq!(
            Instruction::decode(2299407619u32), Instruction::Shl_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shl_i8() {
        assert_eq!(
            Instruction::Shl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407619u32
        );
    }
    #[test]
    fn decode_shl_i16() {
        assert_eq!(
            Instruction::decode(2567843075u32), Instruction::Shl_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shl_i16() {
        assert_eq!(
            Instruction::Shl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843075u32
        );
    }
    #[test]
    fn decode_shl_i32() {
        assert_eq!(
            Instruction::decode(2836278531u32), Instruction::Shl_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shl_i32() {
        assert_eq!(
            Instruction::Shl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278531u32
        );
    }
    #[test]
    fn decode_shl_i64() {
        assert_eq!(
            Instruction::decode(3104713987u32), Instruction::Shl_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shl_i64() {
        assert_eq!(
            Instruction::Shl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713987u32
        );
    }
    #[test]
    fn decode_shl_u8() {
        assert_eq!(
            Instruction::decode(151923971u32), Instruction::Shl_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shl_u8() {
        assert_eq!(
            Instruction::Shl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923971u32
        );
    }
    #[test]
    fn decode_shl_u16() {
        assert_eq!(
            Instruction::decode(420359427u32), Instruction::Shl_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shl_u16() {
        assert_eq!(
            Instruction::Shl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359427u32
        );
    }
    #[test]
    fn decode_shl_u32() {
        assert_eq!(
            Instruction::decode(688794883u32), Instruction::Shl_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shl_u32() {
        assert_eq!(
            Instruction::Shl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794883u32
        );
    }
    #[test]
    fn decode_shl_u64() {
        assert_eq!(
            Instruction::decode(957230339u32), Instruction::Shl_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shl_u64() {
        assert_eq!(
            Instruction::Shl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230339u32
        );
    }
    #[test]
    fn decode_shr_i8() {
        assert_eq!(
            Instruction::decode(2299407683u32), Instruction::Shr_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shr_i8() {
        assert_eq!(
            Instruction::Shr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407683u32
        );
    }
    #[test]
    fn decode_shr_i16() {
        assert_eq!(
            Instruction::decode(2567843139u32), Instruction::Shr_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shr_i16() {
        assert_eq!(
            Instruction::Shr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843139u32
        );
    }
    #[test]
    fn decode_shr_i32() {
        assert_eq!(
            Instruction::decode(2836278595u32), Instruction::Shr_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shr_i32() {
        assert_eq!(
            Instruction::Shr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278595u32
        );
    }
    #[test]
    fn decode_shr_i64() {
        assert_eq!(
            Instruction::decode(3104714051u32), Instruction::Shr_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shr_i64() {
        assert_eq!(
            Instruction::Shr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714051u32
        );
    }
    #[test]
    fn decode_shr_u8() {
        assert_eq!(
            Instruction::decode(151924035u32), Instruction::Shr_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shr_u8() {
        assert_eq!(
            Instruction::Shr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924035u32
        );
    }
    #[test]
    fn decode_shr_u16() {
        assert_eq!(
            Instruction::decode(420359491u32), Instruction::Shr_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shr_u16() {
        assert_eq!(
            Instruction::Shr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359491u32
        );
    }
    #[test]
    fn decode_shr_u32() {
        assert_eq!(
            Instruction::decode(688794947u32), Instruction::Shr_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shr_u32() {
        assert_eq!(
            Instruction::Shr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794947u32
        );
    }
    #[test]
    fn decode_shr_u64() {
        assert_eq!(
            Instruction::decode(957230403u32), Instruction::Shr_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_shr_u64() {
        assert_eq!(
            Instruction::Shr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230403u32
        );
    }
    #[test]
    fn decode_rot_l_i8() {
        assert_eq!(
            Instruction::decode(2299407747u32), Instruction::Rot_L_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_l_i8() {
        assert_eq!(
            Instruction::Rot_L_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407747u32
        );
    }
    #[test]
    fn decode_rot_l_i16() {
        assert_eq!(
            Instruction::decode(2567843203u32), Instruction::Rot_L_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_l_i16() {
        assert_eq!(
            Instruction::Rot_L_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843203u32
        );
    }
    #[test]
    fn decode_rot_l_i32() {
        assert_eq!(
            Instruction::decode(2836278659u32), Instruction::Rot_L_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_l_i32() {
        assert_eq!(
            Instruction::Rot_L_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278659u32
        );
    }
    #[test]
    fn decode_rot_l_i64() {
        assert_eq!(
            Instruction::decode(3104714115u32), Instruction::Rot_L_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_l_i64() {
        assert_eq!(
            Instruction::Rot_L_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714115u32
        );
    }
    #[test]
    fn decode_rot_l_u8() {
        assert_eq!(
            Instruction::decode(151924099u32), Instruction::Rot_L_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_l_u8() {
        assert_eq!(
            Instruction::Rot_L_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924099u32
        );
    }
    #[test]
    fn decode_rot_l_u16() {
        assert_eq!(
            Instruction::decode(420359555u32), Instruction::Rot_L_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_l_u16() {
        assert_eq!(
            Instruction::Rot_L_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359555u32
        );
    }
    #[test]
    fn decode_rot_l_u32() {
        assert_eq!(
            Instruction::decode(688795011u32), Instruction::Rot_L_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_l_u32() {
        assert_eq!(
            Instruction::Rot_L_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795011u32
        );
    }
    #[test]
    fn decode_rot_l_u64() {
        assert_eq!(
            Instruction::decode(957230467u32), Instruction::Rot_L_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_l_u64() {
        assert_eq!(
            Instruction::Rot_L_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230467u32
        );
    }
    #[test]
    fn decode_rot_r_i8() {
        assert_eq!(
            Instruction::decode(2299407811u32), Instruction::Rot_R_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_r_i8() {
        assert_eq!(
            Instruction::Rot_R_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407811u32
        );
    }
    #[test]
    fn decode_rot_r_i16() {
        assert_eq!(
            Instruction::decode(2567843267u32), Instruction::Rot_R_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_r_i16() {
        assert_eq!(
            Instruction::Rot_R_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843267u32
        );
    }
    #[test]
    fn decode_rot_r_i32() {
        assert_eq!(
            Instruction::decode(2836278723u32), Instruction::Rot_R_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_r_i32() {
        assert_eq!(
            Instruction::Rot_R_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278723u32
        );
    }
    #[test]
    fn decode_rot_r_i64() {
        assert_eq!(
            Instruction::decode(3104714179u32), Instruction::Rot_R_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_r_i64() {
        assert_eq!(
            Instruction::Rot_R_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714179u32
        );
    }
    #[test]
    fn decode_rot_r_u8() {
        assert_eq!(
            Instruction::decode(151924163u32), Instruction::Rot_R_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_r_u8() {
        assert_eq!(
            Instruction::Rot_R_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924163u32
        );
    }
    #[test]
    fn decode_rot_r_u16() {
        assert_eq!(
            Instruction::decode(420359619u32), Instruction::Rot_R_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_r_u16() {
        assert_eq!(
            Instruction::Rot_R_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359619u32
        );
    }
    #[test]
    fn decode_rot_r_u32() {
        assert_eq!(
            Instruction::decode(688795075u32), Instruction::Rot_R_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_r_u32() {
        assert_eq!(
            Instruction::Rot_R_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795075u32
        );
    }
    #[test]
    fn decode_rot_r_u64() {
        assert_eq!(
            Instruction::decode(957230531u32), Instruction::Rot_R_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rot_r_u64() {
        assert_eq!(
            Instruction::Rot_R_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230531u32
        );
    }
    #[test]
    fn decode_c_ones_i8() {
        assert_eq!(
            Instruction::decode(2148412931u32), Instruction::C_Ones_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_ones_i8() {
        assert_eq!(
            Instruction::C_Ones_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412931u32
        );
    }
    #[test]
    fn decode_c_ones_i16() {
        assert_eq!(
            Instruction::decode(2416848387u32), Instruction::C_Ones_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_ones_i16() {
        assert_eq!(
            Instruction::C_Ones_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848387u32
        );
    }
    #[test]
    fn decode_c_ones_i32() {
        assert_eq!(
            Instruction::decode(2685283843u32), Instruction::C_Ones_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_ones_i32() {
        assert_eq!(
            Instruction::C_Ones_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283843u32
        );
    }
    #[test]
    fn decode_c_ones_i64() {
        assert_eq!(
            Instruction::decode(2953719299u32), Instruction::C_Ones_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_ones_i64() {
        assert_eq!(
            Instruction::C_Ones_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719299u32
        );
    }
    #[test]
    fn decode_c_ones_u8() {
        assert_eq!(
            Instruction::decode(929283u32), Instruction::C_Ones_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_ones_u8() {
        assert_eq!(
            Instruction::C_Ones_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929283u32
        );
    }
    #[test]
    fn decode_c_ones_u16() {
        assert_eq!(
            Instruction::decode(269364739u32), Instruction::C_Ones_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_ones_u16() {
        assert_eq!(
            Instruction::C_Ones_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364739u32
        );
    }
    #[test]
    fn decode_c_ones_u32() {
        assert_eq!(
            Instruction::decode(537800195u32), Instruction::C_Ones_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_ones_u32() {
        assert_eq!(
            Instruction::C_Ones_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800195u32
        );
    }
    #[test]
    fn decode_c_ones_u64() {
        assert_eq!(
            Instruction::decode(806235651u32), Instruction::C_Ones_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_ones_u64() {
        assert_eq!(
            Instruction::C_Ones_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235651u32
        );
    }
    #[test]
    fn decode_l_ones_i8() {
        assert_eq!(
            Instruction::decode(2148412995u32), Instruction::L_Ones_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_ones_i8() {
        assert_eq!(
            Instruction::L_Ones_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412995u32
        );
    }
    #[test]
    fn decode_l_ones_i16() {
        assert_eq!(
            Instruction::decode(2416848451u32), Instruction::L_Ones_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_ones_i16() {
        assert_eq!(
            Instruction::L_Ones_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848451u32
        );
    }
    #[test]
    fn decode_l_ones_i32() {
        assert_eq!(
            Instruction::decode(2685283907u32), Instruction::L_Ones_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_ones_i32() {
        assert_eq!(
            Instruction::L_Ones_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283907u32
        );
    }
    #[test]
    fn decode_l_ones_i64() {
        assert_eq!(
            Instruction::decode(2953719363u32), Instruction::L_Ones_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_ones_i64() {
        assert_eq!(
            Instruction::L_Ones_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719363u32
        );
    }
    #[test]
    fn decode_l_ones_u8() {
        assert_eq!(
            Instruction::decode(929347u32), Instruction::L_Ones_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_ones_u8() {
        assert_eq!(
            Instruction::L_Ones_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929347u32
        );
    }
    #[test]
    fn decode_l_ones_u16() {
        assert_eq!(
            Instruction::decode(269364803u32), Instruction::L_Ones_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_ones_u16() {
        assert_eq!(
            Instruction::L_Ones_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364803u32
        );
    }
    #[test]
    fn decode_l_ones_u32() {
        assert_eq!(
            Instruction::decode(537800259u32), Instruction::L_Ones_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_ones_u32() {
        assert_eq!(
            Instruction::L_Ones_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800259u32
        );
    }
    #[test]
    fn decode_l_ones_u64() {
        assert_eq!(
            Instruction::decode(806235715u32), Instruction::L_Ones_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_ones_u64() {
        assert_eq!(
            Instruction::L_Ones_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235715u32
        );
    }
    #[test]
    fn decode_t_ones_i8() {
        assert_eq!(
            Instruction::decode(2148413059u32), Instruction::T_Ones_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_ones_i8() {
        assert_eq!(
            Instruction::T_Ones_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148413059u32
        );
    }
    #[test]
    fn decode_t_ones_i16() {
        assert_eq!(
            Instruction::decode(2416848515u32), Instruction::T_Ones_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_ones_i16() {
        assert_eq!(
            Instruction::T_Ones_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848515u32
        );
    }
    #[test]
    fn decode_t_ones_i32() {
        assert_eq!(
            Instruction::decode(2685283971u32), Instruction::T_Ones_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_ones_i32() {
        assert_eq!(
            Instruction::T_Ones_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283971u32
        );
    }
    #[test]
    fn decode_t_ones_i64() {
        assert_eq!(
            Instruction::decode(2953719427u32), Instruction::T_Ones_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_ones_i64() {
        assert_eq!(
            Instruction::T_Ones_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719427u32
        );
    }
    #[test]
    fn decode_t_ones_u8() {
        assert_eq!(
            Instruction::decode(929411u32), Instruction::T_Ones_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_ones_u8() {
        assert_eq!(
            Instruction::T_Ones_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929411u32
        );
    }
    #[test]
    fn decode_t_ones_u16() {
        assert_eq!(
            Instruction::decode(269364867u32), Instruction::T_Ones_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_ones_u16() {
        assert_eq!(
            Instruction::T_Ones_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364867u32
        );
    }
    #[test]
    fn decode_t_ones_u32() {
        assert_eq!(
            Instruction::decode(537800323u32), Instruction::T_Ones_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_ones_u32() {
        assert_eq!(
            Instruction::T_Ones_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800323u32
        );
    }
    #[test]
    fn decode_t_ones_u64() {
        assert_eq!(
            Instruction::decode(806235779u32), Instruction::T_Ones_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_ones_u64() {
        assert_eq!(
            Instruction::T_Ones_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235779u32
        );
    }
    #[test]
    fn decode_c_zeros_i8() {
        assert_eq!(
            Instruction::decode(2148413123u32), Instruction::C_Zeros_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_zeros_i8() {
        assert_eq!(
            Instruction::C_Zeros_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148413123u32
        );
    }
    #[test]
    fn decode_c_zeros_i16() {
        assert_eq!(
            Instruction::decode(2416848579u32), Instruction::C_Zeros_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_zeros_i16() {
        assert_eq!(
            Instruction::C_Zeros_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848579u32
        );
    }
    #[test]
    fn decode_c_zeros_i32() {
        assert_eq!(
            Instruction::decode(2685284035u32), Instruction::C_Zeros_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_zeros_i32() {
        assert_eq!(
            Instruction::C_Zeros_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685284035u32
        );
    }
    #[test]
    fn decode_c_zeros_i64() {
        assert_eq!(
            Instruction::decode(2953719491u32), Instruction::C_Zeros_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_zeros_i64() {
        assert_eq!(
            Instruction::C_Zeros_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719491u32
        );
    }
    #[test]
    fn decode_c_zeros_u8() {
        assert_eq!(
            Instruction::decode(929475u32), Instruction::C_Zeros_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_zeros_u8() {
        assert_eq!(
            Instruction::C_Zeros_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929475u32
        );
    }
    #[test]
    fn decode_c_zeros_u16() {
        assert_eq!(
            Instruction::decode(269364931u32), Instruction::C_Zeros_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_zeros_u16() {
        assert_eq!(
            Instruction::C_Zeros_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364931u32
        );
    }
    #[test]
    fn decode_c_zeros_u32() {
        assert_eq!(
            Instruction::decode(537800387u32), Instruction::C_Zeros_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_zeros_u32() {
        assert_eq!(
            Instruction::C_Zeros_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800387u32
        );
    }
    #[test]
    fn decode_c_zeros_u64() {
        assert_eq!(
            Instruction::decode(806235843u32), Instruction::C_Zeros_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_zeros_u64() {
        assert_eq!(
            Instruction::C_Zeros_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235843u32
        );
    }
    #[test]
    fn decode_l_zeros_i8() {
        assert_eq!(
            Instruction::decode(2148413187u32), Instruction::L_Zeros_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_zeros_i8() {
        assert_eq!(
            Instruction::L_Zeros_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148413187u32
        );
    }
    #[test]
    fn decode_l_zeros_i16() {
        assert_eq!(
            Instruction::decode(2416848643u32), Instruction::L_Zeros_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_zeros_i16() {
        assert_eq!(
            Instruction::L_Zeros_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848643u32
        );
    }
    #[test]
    fn decode_l_zeros_i32() {
        assert_eq!(
            Instruction::decode(2685284099u32), Instruction::L_Zeros_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_zeros_i32() {
        assert_eq!(
            Instruction::L_Zeros_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685284099u32
        );
    }
    #[test]
    fn decode_l_zeros_i64() {
        assert_eq!(
            Instruction::decode(2953719555u32), Instruction::L_Zeros_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_zeros_i64() {
        assert_eq!(
            Instruction::L_Zeros_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719555u32
        );
    }
    #[test]
    fn decode_l_zeros_u8() {
        assert_eq!(
            Instruction::decode(929539u32), Instruction::L_Zeros_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_zeros_u8() {
        assert_eq!(
            Instruction::L_Zeros_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929539u32
        );
    }
    #[test]
    fn decode_l_zeros_u16() {
        assert_eq!(
            Instruction::decode(269364995u32), Instruction::L_Zeros_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_zeros_u16() {
        assert_eq!(
            Instruction::L_Zeros_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364995u32
        );
    }
    #[test]
    fn decode_l_zeros_u32() {
        assert_eq!(
            Instruction::decode(537800451u32), Instruction::L_Zeros_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_zeros_u32() {
        assert_eq!(
            Instruction::L_Zeros_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800451u32
        );
    }
    #[test]
    fn decode_l_zeros_u64() {
        assert_eq!(
            Instruction::decode(806235907u32), Instruction::L_Zeros_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_l_zeros_u64() {
        assert_eq!(
            Instruction::L_Zeros_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235907u32
        );
    }
    #[test]
    fn decode_t_zeros_i8() {
        assert_eq!(
            Instruction::decode(2148413251u32), Instruction::T_Zeros_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_zeros_i8() {
        assert_eq!(
            Instruction::T_Zeros_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148413251u32
        );
    }
    #[test]
    fn decode_t_zeros_i16() {
        assert_eq!(
            Instruction::decode(2416848707u32), Instruction::T_Zeros_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_zeros_i16() {
        assert_eq!(
            Instruction::T_Zeros_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848707u32
        );
    }
    #[test]
    fn decode_t_zeros_i32() {
        assert_eq!(
            Instruction::decode(2685284163u32), Instruction::T_Zeros_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_zeros_i32() {
        assert_eq!(
            Instruction::T_Zeros_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685284163u32
        );
    }
    #[test]
    fn decode_t_zeros_i64() {
        assert_eq!(
            Instruction::decode(2953719619u32), Instruction::T_Zeros_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_zeros_i64() {
        assert_eq!(
            Instruction::T_Zeros_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719619u32
        );
    }
    #[test]
    fn decode_t_zeros_u8() {
        assert_eq!(
            Instruction::decode(929603u32), Instruction::T_Zeros_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_zeros_u8() {
        assert_eq!(
            Instruction::T_Zeros_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929603u32
        );
    }
    #[test]
    fn decode_t_zeros_u16() {
        assert_eq!(
            Instruction::decode(269365059u32), Instruction::T_Zeros_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_zeros_u16() {
        assert_eq!(
            Instruction::T_Zeros_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269365059u32
        );
    }
    #[test]
    fn decode_t_zeros_u32() {
        assert_eq!(
            Instruction::decode(537800515u32), Instruction::T_Zeros_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_zeros_u32() {
        assert_eq!(
            Instruction::T_Zeros_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800515u32
        );
    }
    #[test]
    fn decode_t_zeros_u64() {
        assert_eq!(
            Instruction::decode(806235971u32), Instruction::T_Zeros_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_t_zeros_u64() {
        assert_eq!(
            Instruction::T_Zeros_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235971u32
        );
    }
    #[test]
    fn decode_r_bytes_i8() {
        assert_eq!(
            Instruction::decode(2148413315u32), Instruction::R_Bytes_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bytes_i8() {
        assert_eq!(
            Instruction::R_Bytes_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148413315u32
        );
    }
    #[test]
    fn decode_r_bytes_i16() {
        assert_eq!(
            Instruction::decode(2416848771u32), Instruction::R_Bytes_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bytes_i16() {
        assert_eq!(
            Instruction::R_Bytes_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848771u32
        );
    }
    #[test]
    fn decode_r_bytes_i32() {
        assert_eq!(
            Instruction::decode(2685284227u32), Instruction::R_Bytes_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bytes_i32() {
        assert_eq!(
            Instruction::R_Bytes_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685284227u32
        );
    }
    #[test]
    fn decode_r_bytes_i64() {
        assert_eq!(
            Instruction::decode(2953719683u32), Instruction::R_Bytes_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bytes_i64() {
        assert_eq!(
            Instruction::R_Bytes_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719683u32
        );
    }
    #[test]
    fn decode_r_bytes_u8() {
        assert_eq!(
            Instruction::decode(929667u32), Instruction::R_Bytes_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bytes_u8() {
        assert_eq!(
            Instruction::R_Bytes_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929667u32
        );
    }
    #[test]
    fn decode_r_bytes_u16() {
        assert_eq!(
            Instruction::decode(269365123u32), Instruction::R_Bytes_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bytes_u16() {
        assert_eq!(
            Instruction::R_Bytes_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269365123u32
        );
    }
    #[test]
    fn decode_r_bytes_u32() {
        assert_eq!(
            Instruction::decode(537800579u32), Instruction::R_Bytes_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bytes_u32() {
        assert_eq!(
            Instruction::R_Bytes_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800579u32
        );
    }
    #[test]
    fn decode_r_bytes_u64() {
        assert_eq!(
            Instruction::decode(806236035u32), Instruction::R_Bytes_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bytes_u64() {
        assert_eq!(
            Instruction::R_Bytes_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806236035u32
        );
    }
    #[test]
    fn decode_r_bits_i8() {
        assert_eq!(
            Instruction::decode(2148413379u32), Instruction::R_Bits_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bits_i8() {
        assert_eq!(
            Instruction::R_Bits_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148413379u32
        );
    }
    #[test]
    fn decode_r_bits_i16() {
        assert_eq!(
            Instruction::decode(2416848835u32), Instruction::R_Bits_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bits_i16() {
        assert_eq!(
            Instruction::R_Bits_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848835u32
        );
    }
    #[test]
    fn decode_r_bits_i32() {
        assert_eq!(
            Instruction::decode(2685284291u32), Instruction::R_Bits_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bits_i32() {
        assert_eq!(
            Instruction::R_Bits_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685284291u32
        );
    }
    #[test]
    fn decode_r_bits_i64() {
        assert_eq!(
            Instruction::decode(2953719747u32), Instruction::R_Bits_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bits_i64() {
        assert_eq!(
            Instruction::R_Bits_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719747u32
        );
    }
    #[test]
    fn decode_r_bits_u8() {
        assert_eq!(
            Instruction::decode(929731u32), Instruction::R_Bits_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bits_u8() {
        assert_eq!(
            Instruction::R_Bits_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929731u32
        );
    }
    #[test]
    fn decode_r_bits_u16() {
        assert_eq!(
            Instruction::decode(269365187u32), Instruction::R_Bits_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bits_u16() {
        assert_eq!(
            Instruction::R_Bits_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269365187u32
        );
    }
    #[test]
    fn decode_r_bits_u32() {
        assert_eq!(
            Instruction::decode(537800643u32), Instruction::R_Bits_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bits_u32() {
        assert_eq!(
            Instruction::R_Bits_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800643u32
        );
    }
    #[test]
    fn decode_r_bits_u64() {
        assert_eq!(
            Instruction::decode(806236099u32), Instruction::R_Bits_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_r_bits_u64() {
        assert_eq!(
            Instruction::R_Bits_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806236099u32
        );
    }
    #[test]
    fn decode_c_abs_i8() {
        assert_eq!(
            Instruction::decode(2148412420u32), Instruction::C_Abs_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_abs_i8() {
        assert_eq!(
            Instruction::C_Abs_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412420u32
        );
    }
    #[test]
    fn decode_c_abs_i16() {
        assert_eq!(
            Instruction::decode(2416847876u32), Instruction::C_Abs_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_abs_i16() {
        assert_eq!(
            Instruction::C_Abs_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416847876u32
        );
    }
    #[test]
    fn decode_c_abs_i32() {
        assert_eq!(
            Instruction::decode(2685283332u32), Instruction::C_Abs_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_abs_i32() {
        assert_eq!(
            Instruction::C_Abs_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283332u32
        );
    }
    #[test]
    fn decode_c_abs_i64() {
        assert_eq!(
            Instruction::decode(2953718788u32), Instruction::C_Abs_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_abs_i64() {
        assert_eq!(
            Instruction::C_Abs_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953718788u32
        );
    }
    #[test]
    fn decode_c_add_i8() {
        assert_eq!(
            Instruction::decode(2299407428u32), Instruction::C_Add_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_i8() {
        assert_eq!(
            Instruction::C_Add_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407428u32
        );
    }
    #[test]
    fn decode_c_add_i16() {
        assert_eq!(
            Instruction::decode(2567842884u32), Instruction::C_Add_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_i16() {
        assert_eq!(
            Instruction::C_Add_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567842884u32
        );
    }
    #[test]
    fn decode_c_add_i32() {
        assert_eq!(
            Instruction::decode(2836278340u32), Instruction::C_Add_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_i32() {
        assert_eq!(
            Instruction::C_Add_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278340u32
        );
    }
    #[test]
    fn decode_c_add_i64() {
        assert_eq!(
            Instruction::decode(3104713796u32), Instruction::C_Add_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_i64() {
        assert_eq!(
            Instruction::C_Add_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713796u32
        );
    }
    #[test]
    fn decode_c_add_u8() {
        assert_eq!(
            Instruction::decode(151923780u32), Instruction::C_Add_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_u8() {
        assert_eq!(
            Instruction::C_Add_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923780u32
        );
    }
    #[test]
    fn decode_c_add_u16() {
        assert_eq!(
            Instruction::decode(420359236u32), Instruction::C_Add_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_u16() {
        assert_eq!(
            Instruction::C_Add_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359236u32
        );
    }
    #[test]
    fn decode_c_add_u32() {
        assert_eq!(
            Instruction::decode(688794692u32), Instruction::C_Add_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_u32() {
        assert_eq!(
            Instruction::C_Add_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794692u32
        );
    }
    #[test]
    fn decode_c_add_u64() {
        assert_eq!(
            Instruction::decode(957230148u32), Instruction::C_Add_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_u64() {
        assert_eq!(
            Instruction::C_Add_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230148u32
        );
    }
    #[test]
    fn decode_c_add_u_i8() {
        assert_eq!(
            Instruction::decode(2299407492u32), Instruction::C_Add_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_u_i8() {
        assert_eq!(
            Instruction::C_Add_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407492u32
        );
    }
    #[test]
    fn decode_c_add_u_i16() {
        assert_eq!(
            Instruction::decode(2567842948u32), Instruction::C_Add_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_u_i16() {
        assert_eq!(
            Instruction::C_Add_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567842948u32
        );
    }
    #[test]
    fn decode_c_add_u_i32() {
        assert_eq!(
            Instruction::decode(2836278404u32), Instruction::C_Add_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_u_i32() {
        assert_eq!(
            Instruction::C_Add_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278404u32
        );
    }
    #[test]
    fn decode_c_add_u_i64() {
        assert_eq!(
            Instruction::decode(3104713860u32), Instruction::C_Add_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_u_i64() {
        assert_eq!(
            Instruction::C_Add_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713860u32
        );
    }
    #[test]
    fn decode_c_add_s_u8() {
        assert_eq!(
            Instruction::decode(151923844u32), Instruction::C_Add_S_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_s_u8() {
        assert_eq!(
            Instruction::C_Add_S_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923844u32
        );
    }
    #[test]
    fn decode_c_add_s_u16() {
        assert_eq!(
            Instruction::decode(420359300u32), Instruction::C_Add_S_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_s_u16() {
        assert_eq!(
            Instruction::C_Add_S_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359300u32
        );
    }
    #[test]
    fn decode_c_add_s_u32() {
        assert_eq!(
            Instruction::decode(688794756u32), Instruction::C_Add_S_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_s_u32() {
        assert_eq!(
            Instruction::C_Add_S_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794756u32
        );
    }
    #[test]
    fn decode_c_add_s_u64() {
        assert_eq!(
            Instruction::decode(957230212u32), Instruction::C_Add_S_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_add_s_u64() {
        assert_eq!(
            Instruction::C_Add_S_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230212u32
        );
    }
    #[test]
    fn decode_c_div_i8() {
        assert_eq!(
            Instruction::decode(2299407556u32), Instruction::C_Div_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_i8() {
        assert_eq!(
            Instruction::C_Div_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407556u32
        );
    }
    #[test]
    fn decode_c_div_i16() {
        assert_eq!(
            Instruction::decode(2567843012u32), Instruction::C_Div_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_i16() {
        assert_eq!(
            Instruction::C_Div_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843012u32
        );
    }
    #[test]
    fn decode_c_div_i32() {
        assert_eq!(
            Instruction::decode(2836278468u32), Instruction::C_Div_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_i32() {
        assert_eq!(
            Instruction::C_Div_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278468u32
        );
    }
    #[test]
    fn decode_c_div_i64() {
        assert_eq!(
            Instruction::decode(3104713924u32), Instruction::C_Div_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_i64() {
        assert_eq!(
            Instruction::C_Div_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713924u32
        );
    }
    #[test]
    fn decode_c_div_u8() {
        assert_eq!(
            Instruction::decode(151923908u32), Instruction::C_Div_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_u8() {
        assert_eq!(
            Instruction::C_Div_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923908u32
        );
    }
    #[test]
    fn decode_c_div_u16() {
        assert_eq!(
            Instruction::decode(420359364u32), Instruction::C_Div_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_u16() {
        assert_eq!(
            Instruction::C_Div_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359364u32
        );
    }
    #[test]
    fn decode_c_div_u32() {
        assert_eq!(
            Instruction::decode(688794820u32), Instruction::C_Div_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_u32() {
        assert_eq!(
            Instruction::C_Div_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794820u32
        );
    }
    #[test]
    fn decode_c_div_u64() {
        assert_eq!(
            Instruction::decode(957230276u32), Instruction::C_Div_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_u64() {
        assert_eq!(
            Instruction::C_Div_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230276u32
        );
    }
    #[test]
    fn decode_c_div_e_i8() {
        assert_eq!(
            Instruction::decode(2299407620u32), Instruction::C_Div_E_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_e_i8() {
        assert_eq!(
            Instruction::C_Div_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407620u32
        );
    }
    #[test]
    fn decode_c_div_e_i16() {
        assert_eq!(
            Instruction::decode(2567843076u32), Instruction::C_Div_E_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_e_i16() {
        assert_eq!(
            Instruction::C_Div_E_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843076u32
        );
    }
    #[test]
    fn decode_c_div_e_i32() {
        assert_eq!(
            Instruction::decode(2836278532u32), Instruction::C_Div_E_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_e_i32() {
        assert_eq!(
            Instruction::C_Div_E_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278532u32
        );
    }
    #[test]
    fn decode_c_div_e_i64() {
        assert_eq!(
            Instruction::decode(3104713988u32), Instruction::C_Div_E_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_e_i64() {
        assert_eq!(
            Instruction::C_Div_E_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713988u32
        );
    }
    #[test]
    fn decode_c_div_e_u8() {
        assert_eq!(
            Instruction::decode(151923972u32), Instruction::C_Div_E_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_e_u8() {
        assert_eq!(
            Instruction::C_Div_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923972u32
        );
    }
    #[test]
    fn decode_c_div_e_u16() {
        assert_eq!(
            Instruction::decode(420359428u32), Instruction::C_Div_E_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_e_u16() {
        assert_eq!(
            Instruction::C_Div_E_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359428u32
        );
    }
    #[test]
    fn decode_c_div_e_u32() {
        assert_eq!(
            Instruction::decode(688794884u32), Instruction::C_Div_E_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_e_u32() {
        assert_eq!(
            Instruction::C_Div_E_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794884u32
        );
    }
    #[test]
    fn decode_c_div_e_u64() {
        assert_eq!(
            Instruction::decode(957230340u32), Instruction::C_Div_E_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_div_e_u64() {
        assert_eq!(
            Instruction::C_Div_E_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230340u32
        );
    }
    #[test]
    fn decode_c_log_i8() {
        assert_eq!(
            Instruction::decode(2299407684u32), Instruction::C_Log_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_log_i8() {
        assert_eq!(
            Instruction::C_Log_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407684u32
        );
    }
    #[test]
    fn decode_c_log_i16() {
        assert_eq!(
            Instruction::decode(2567843140u32), Instruction::C_Log_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_log_i16() {
        assert_eq!(
            Instruction::C_Log_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843140u32
        );
    }
    #[test]
    fn decode_c_log_i32() {
        assert_eq!(
            Instruction::decode(2836278596u32), Instruction::C_Log_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_log_i32() {
        assert_eq!(
            Instruction::C_Log_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278596u32
        );
    }
    #[test]
    fn decode_c_log_i64() {
        assert_eq!(
            Instruction::decode(3104714052u32), Instruction::C_Log_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_log_i64() {
        assert_eq!(
            Instruction::C_Log_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714052u32
        );
    }
    #[test]
    fn decode_c_log_u8() {
        assert_eq!(
            Instruction::decode(151924036u32), Instruction::C_Log_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_log_u8() {
        assert_eq!(
            Instruction::C_Log_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924036u32
        );
    }
    #[test]
    fn decode_c_log_u16() {
        assert_eq!(
            Instruction::decode(420359492u32), Instruction::C_Log_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_log_u16() {
        assert_eq!(
            Instruction::C_Log_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359492u32
        );
    }
    #[test]
    fn decode_c_log_u32() {
        assert_eq!(
            Instruction::decode(688794948u32), Instruction::C_Log_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_log_u32() {
        assert_eq!(
            Instruction::C_Log_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794948u32
        );
    }
    #[test]
    fn decode_c_log_u64() {
        assert_eq!(
            Instruction::decode(957230404u32), Instruction::C_Log_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_log_u64() {
        assert_eq!(
            Instruction::C_Log_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230404u32
        );
    }
    #[test]
    fn decode_c_sqrt_i8() {
        assert_eq!(
            Instruction::decode(2148412804u32), Instruction::C_Sqrt_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_sqrt_i8() {
        assert_eq!(
            Instruction::C_Sqrt_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412804u32
        );
    }
    #[test]
    fn decode_c_sqrt_i16() {
        assert_eq!(
            Instruction::decode(2416848260u32), Instruction::C_Sqrt_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_sqrt_i16() {
        assert_eq!(
            Instruction::C_Sqrt_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848260u32
        );
    }
    #[test]
    fn decode_c_sqrt_i32() {
        assert_eq!(
            Instruction::decode(2685283716u32), Instruction::C_Sqrt_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_sqrt_i32() {
        assert_eq!(
            Instruction::C_Sqrt_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283716u32
        );
    }
    #[test]
    fn decode_c_sqrt_i64() {
        assert_eq!(
            Instruction::decode(2953719172u32), Instruction::C_Sqrt_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_sqrt_i64() {
        assert_eq!(
            Instruction::C_Sqrt_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719172u32
        );
    }
    #[test]
    fn decode_c_sqrt_u8() {
        assert_eq!(
            Instruction::decode(929156u32), Instruction::C_Sqrt_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_sqrt_u8() {
        assert_eq!(
            Instruction::C_Sqrt_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 929156u32
        );
    }
    #[test]
    fn decode_c_sqrt_u16() {
        assert_eq!(
            Instruction::decode(269364612u32), Instruction::C_Sqrt_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_sqrt_u16() {
        assert_eq!(
            Instruction::C_Sqrt_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 269364612u32
        );
    }
    #[test]
    fn decode_c_sqrt_u32() {
        assert_eq!(
            Instruction::decode(537800068u32), Instruction::C_Sqrt_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_sqrt_u32() {
        assert_eq!(
            Instruction::C_Sqrt_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 537800068u32
        );
    }
    #[test]
    fn decode_c_sqrt_u64() {
        assert_eq!(
            Instruction::decode(806235524u32), Instruction::C_Sqrt_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_sqrt_u64() {
        assert_eq!(
            Instruction::C_Sqrt_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 806235524u32
        );
    }
    #[test]
    fn decode_c_mul_i8() {
        assert_eq!(
            Instruction::decode(2299407812u32), Instruction::C_Mul_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_mul_i8() {
        assert_eq!(
            Instruction::C_Mul_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407812u32
        );
    }
    #[test]
    fn decode_c_mul_i16() {
        assert_eq!(
            Instruction::decode(2567843268u32), Instruction::C_Mul_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_mul_i16() {
        assert_eq!(
            Instruction::C_Mul_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843268u32
        );
    }
    #[test]
    fn decode_c_mul_i32() {
        assert_eq!(
            Instruction::decode(2836278724u32), Instruction::C_Mul_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_mul_i32() {
        assert_eq!(
            Instruction::C_Mul_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278724u32
        );
    }
    #[test]
    fn decode_c_mul_i64() {
        assert_eq!(
            Instruction::decode(3104714180u32), Instruction::C_Mul_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_mul_i64() {
        assert_eq!(
            Instruction::C_Mul_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714180u32
        );
    }
    #[test]
    fn decode_c_mul_u8() {
        assert_eq!(
            Instruction::decode(151924164u32), Instruction::C_Mul_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_mul_u8() {
        assert_eq!(
            Instruction::C_Mul_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924164u32
        );
    }
    #[test]
    fn decode_c_mul_u16() {
        assert_eq!(
            Instruction::decode(420359620u32), Instruction::C_Mul_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_mul_u16() {
        assert_eq!(
            Instruction::C_Mul_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359620u32
        );
    }
    #[test]
    fn decode_c_mul_u32() {
        assert_eq!(
            Instruction::decode(688795076u32), Instruction::C_Mul_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_mul_u32() {
        assert_eq!(
            Instruction::C_Mul_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795076u32
        );
    }
    #[test]
    fn decode_c_mul_u64() {
        assert_eq!(
            Instruction::decode(957230532u32), Instruction::C_Mul_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_mul_u64() {
        assert_eq!(
            Instruction::C_Mul_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230532u32
        );
    }
    #[test]
    fn decode_c_neg_i8() {
        assert_eq!(
            Instruction::decode(2148412932u32), Instruction::C_Neg_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_neg_i8() {
        assert_eq!(
            Instruction::C_Neg_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412932u32
        );
    }
    #[test]
    fn decode_c_neg_i16() {
        assert_eq!(
            Instruction::decode(2416848388u32), Instruction::C_Neg_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_neg_i16() {
        assert_eq!(
            Instruction::C_Neg_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848388u32
        );
    }
    #[test]
    fn decode_c_neg_i32() {
        assert_eq!(
            Instruction::decode(2685283844u32), Instruction::C_Neg_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_neg_i32() {
        assert_eq!(
            Instruction::C_Neg_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283844u32
        );
    }
    #[test]
    fn decode_c_neg_i64() {
        assert_eq!(
            Instruction::decode(2953719300u32), Instruction::C_Neg_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_c_neg_i64() {
        assert_eq!(
            Instruction::C_Neg_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719300u32
        );
    }
    #[test]
    fn decode_c_pow_i8() {
        assert_eq!(
            Instruction::decode(2299407940u32), Instruction::C_Pow_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_pow_i8() {
        assert_eq!(
            Instruction::C_Pow_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407940u32
        );
    }
    #[test]
    fn decode_c_pow_i16() {
        assert_eq!(
            Instruction::decode(2567843396u32), Instruction::C_Pow_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_pow_i16() {
        assert_eq!(
            Instruction::C_Pow_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843396u32
        );
    }
    #[test]
    fn decode_c_pow_i32() {
        assert_eq!(
            Instruction::decode(2836278852u32), Instruction::C_Pow_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_pow_i32() {
        assert_eq!(
            Instruction::C_Pow_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278852u32
        );
    }
    #[test]
    fn decode_c_pow_i64() {
        assert_eq!(
            Instruction::decode(3104714308u32), Instruction::C_Pow_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_pow_i64() {
        assert_eq!(
            Instruction::C_Pow_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714308u32
        );
    }
    #[test]
    fn decode_c_pow_u8() {
        assert_eq!(
            Instruction::decode(151924292u32), Instruction::C_Pow_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_pow_u8() {
        assert_eq!(
            Instruction::C_Pow_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924292u32
        );
    }
    #[test]
    fn decode_c_pow_u16() {
        assert_eq!(
            Instruction::decode(420359748u32), Instruction::C_Pow_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_pow_u16() {
        assert_eq!(
            Instruction::C_Pow_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359748u32
        );
    }
    #[test]
    fn decode_c_pow_u32() {
        assert_eq!(
            Instruction::decode(688795204u32), Instruction::C_Pow_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_pow_u32() {
        assert_eq!(
            Instruction::C_Pow_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795204u32
        );
    }
    #[test]
    fn decode_c_pow_u64() {
        assert_eq!(
            Instruction::decode(957230660u32), Instruction::C_Pow_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_pow_u64() {
        assert_eq!(
            Instruction::C_Pow_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230660u32
        );
    }
    #[test]
    fn decode_c_rem_i8() {
        assert_eq!(
            Instruction::decode(2299408004u32), Instruction::C_Rem_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_i8() {
        assert_eq!(
            Instruction::C_Rem_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299408004u32
        );
    }
    #[test]
    fn decode_c_rem_i16() {
        assert_eq!(
            Instruction::decode(2567843460u32), Instruction::C_Rem_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_i16() {
        assert_eq!(
            Instruction::C_Rem_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843460u32
        );
    }
    #[test]
    fn decode_c_rem_i32() {
        assert_eq!(
            Instruction::decode(2836278916u32), Instruction::C_Rem_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_i32() {
        assert_eq!(
            Instruction::C_Rem_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278916u32
        );
    }
    #[test]
    fn decode_c_rem_i64() {
        assert_eq!(
            Instruction::decode(3104714372u32), Instruction::C_Rem_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_i64() {
        assert_eq!(
            Instruction::C_Rem_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714372u32
        );
    }
    #[test]
    fn decode_c_rem_u8() {
        assert_eq!(
            Instruction::decode(151924356u32), Instruction::C_Rem_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_u8() {
        assert_eq!(
            Instruction::C_Rem_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924356u32
        );
    }
    #[test]
    fn decode_c_rem_u16() {
        assert_eq!(
            Instruction::decode(420359812u32), Instruction::C_Rem_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_u16() {
        assert_eq!(
            Instruction::C_Rem_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359812u32
        );
    }
    #[test]
    fn decode_c_rem_u32() {
        assert_eq!(
            Instruction::decode(688795268u32), Instruction::C_Rem_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_u32() {
        assert_eq!(
            Instruction::C_Rem_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795268u32
        );
    }
    #[test]
    fn decode_c_rem_u64() {
        assert_eq!(
            Instruction::decode(957230724u32), Instruction::C_Rem_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_u64() {
        assert_eq!(
            Instruction::C_Rem_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230724u32
        );
    }
    #[test]
    fn decode_c_rem_e_i8() {
        assert_eq!(
            Instruction::decode(2299408068u32), Instruction::C_Rem_E_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_e_i8() {
        assert_eq!(
            Instruction::C_Rem_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299408068u32
        );
    }
    #[test]
    fn decode_c_rem_e_i16() {
        assert_eq!(
            Instruction::decode(2567843524u32), Instruction::C_Rem_E_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_e_i16() {
        assert_eq!(
            Instruction::C_Rem_E_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843524u32
        );
    }
    #[test]
    fn decode_c_rem_e_i32() {
        assert_eq!(
            Instruction::decode(2836278980u32), Instruction::C_Rem_E_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_e_i32() {
        assert_eq!(
            Instruction::C_Rem_E_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278980u32
        );
    }
    #[test]
    fn decode_c_rem_e_i64() {
        assert_eq!(
            Instruction::decode(3104714436u32), Instruction::C_Rem_E_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_e_i64() {
        assert_eq!(
            Instruction::C_Rem_E_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714436u32
        );
    }
    #[test]
    fn decode_c_rem_e_u8() {
        assert_eq!(
            Instruction::decode(151924420u32), Instruction::C_Rem_E_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_e_u8() {
        assert_eq!(
            Instruction::C_Rem_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924420u32
        );
    }
    #[test]
    fn decode_c_rem_e_u16() {
        assert_eq!(
            Instruction::decode(420359876u32), Instruction::C_Rem_E_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_e_u16() {
        assert_eq!(
            Instruction::C_Rem_E_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359876u32
        );
    }
    #[test]
    fn decode_c_rem_e_u32() {
        assert_eq!(
            Instruction::decode(688795332u32), Instruction::C_Rem_E_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_e_u32() {
        assert_eq!(
            Instruction::C_Rem_E_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795332u32
        );
    }
    #[test]
    fn decode_c_rem_e_u64() {
        assert_eq!(
            Instruction::decode(957230788u32), Instruction::C_Rem_E_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_rem_e_u64() {
        assert_eq!(
            Instruction::C_Rem_E_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230788u32
        );
    }
    #[test]
    fn decode_c_shl_i8() {
        assert_eq!(
            Instruction::decode(2299408132u32), Instruction::C_Shl_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shl_i8() {
        assert_eq!(
            Instruction::C_Shl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299408132u32
        );
    }
    #[test]
    fn decode_c_shl_i16() {
        assert_eq!(
            Instruction::decode(2567843588u32), Instruction::C_Shl_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shl_i16() {
        assert_eq!(
            Instruction::C_Shl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843588u32
        );
    }
    #[test]
    fn decode_c_shl_i32() {
        assert_eq!(
            Instruction::decode(2836279044u32), Instruction::C_Shl_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shl_i32() {
        assert_eq!(
            Instruction::C_Shl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836279044u32
        );
    }
    #[test]
    fn decode_c_shl_i64() {
        assert_eq!(
            Instruction::decode(3104714500u32), Instruction::C_Shl_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shl_i64() {
        assert_eq!(
            Instruction::C_Shl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714500u32
        );
    }
    #[test]
    fn decode_c_shl_u8() {
        assert_eq!(
            Instruction::decode(151924484u32), Instruction::C_Shl_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shl_u8() {
        assert_eq!(
            Instruction::C_Shl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924484u32
        );
    }
    #[test]
    fn decode_c_shl_u16() {
        assert_eq!(
            Instruction::decode(420359940u32), Instruction::C_Shl_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shl_u16() {
        assert_eq!(
            Instruction::C_Shl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359940u32
        );
    }
    #[test]
    fn decode_c_shl_u32() {
        assert_eq!(
            Instruction::decode(688795396u32), Instruction::C_Shl_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shl_u32() {
        assert_eq!(
            Instruction::C_Shl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795396u32
        );
    }
    #[test]
    fn decode_c_shl_u64() {
        assert_eq!(
            Instruction::decode(957230852u32), Instruction::C_Shl_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shl_u64() {
        assert_eq!(
            Instruction::C_Shl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230852u32
        );
    }
    #[test]
    fn decode_c_shr_i8() {
        assert_eq!(
            Instruction::decode(2299408196u32), Instruction::C_Shr_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shr_i8() {
        assert_eq!(
            Instruction::C_Shr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299408196u32
        );
    }
    #[test]
    fn decode_c_shr_i16() {
        assert_eq!(
            Instruction::decode(2567843652u32), Instruction::C_Shr_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shr_i16() {
        assert_eq!(
            Instruction::C_Shr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843652u32
        );
    }
    #[test]
    fn decode_c_shr_i32() {
        assert_eq!(
            Instruction::decode(2836279108u32), Instruction::C_Shr_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shr_i32() {
        assert_eq!(
            Instruction::C_Shr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836279108u32
        );
    }
    #[test]
    fn decode_c_shr_i64() {
        assert_eq!(
            Instruction::decode(3104714564u32), Instruction::C_Shr_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shr_i64() {
        assert_eq!(
            Instruction::C_Shr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714564u32
        );
    }
    #[test]
    fn decode_c_shr_u8() {
        assert_eq!(
            Instruction::decode(151924548u32), Instruction::C_Shr_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shr_u8() {
        assert_eq!(
            Instruction::C_Shr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924548u32
        );
    }
    #[test]
    fn decode_c_shr_u16() {
        assert_eq!(
            Instruction::decode(420360004u32), Instruction::C_Shr_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shr_u16() {
        assert_eq!(
            Instruction::C_Shr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420360004u32
        );
    }
    #[test]
    fn decode_c_shr_u32() {
        assert_eq!(
            Instruction::decode(688795460u32), Instruction::C_Shr_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shr_u32() {
        assert_eq!(
            Instruction::C_Shr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795460u32
        );
    }
    #[test]
    fn decode_c_shr_u64() {
        assert_eq!(
            Instruction::decode(957230916u32), Instruction::C_Shr_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_shr_u64() {
        assert_eq!(
            Instruction::C_Shr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230916u32
        );
    }
    #[test]
    fn decode_c_sub_i8() {
        assert_eq!(
            Instruction::decode(2299408260u32), Instruction::C_Sub_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_sub_i8() {
        assert_eq!(
            Instruction::C_Sub_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299408260u32
        );
    }
    #[test]
    fn decode_c_sub_i16() {
        assert_eq!(
            Instruction::decode(2567843716u32), Instruction::C_Sub_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_sub_i16() {
        assert_eq!(
            Instruction::C_Sub_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843716u32
        );
    }
    #[test]
    fn decode_c_sub_i32() {
        assert_eq!(
            Instruction::decode(2836279172u32), Instruction::C_Sub_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_sub_i32() {
        assert_eq!(
            Instruction::C_Sub_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836279172u32
        );
    }
    #[test]
    fn decode_c_sub_i64() {
        assert_eq!(
            Instruction::decode(3104714628u32), Instruction::C_Sub_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_sub_i64() {
        assert_eq!(
            Instruction::C_Sub_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714628u32
        );
    }
    #[test]
    fn decode_c_sub_u8() {
        assert_eq!(
            Instruction::decode(151924612u32), Instruction::C_Sub_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_sub_u8() {
        assert_eq!(
            Instruction::C_Sub_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924612u32
        );
    }
    #[test]
    fn decode_c_sub_u16() {
        assert_eq!(
            Instruction::decode(420360068u32), Instruction::C_Sub_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_sub_u16() {
        assert_eq!(
            Instruction::C_Sub_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420360068u32
        );
    }
    #[test]
    fn decode_c_sub_u32() {
        assert_eq!(
            Instruction::decode(688795524u32), Instruction::C_Sub_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_sub_u32() {
        assert_eq!(
            Instruction::C_Sub_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795524u32
        );
    }
    #[test]
    fn decode_c_sub_u64() {
        assert_eq!(
            Instruction::decode(957230980u32), Instruction::C_Sub_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_sub_u64() {
        assert_eq!(
            Instruction::C_Sub_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230980u32
        );
    }
    #[test]
    fn decode_c_sub_u_i8() {
        assert_eq!(
            Instruction::decode(2299408324u32), Instruction::C_Sub_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_sub_u_i8() {
        assert_eq!(
            Instruction::C_Sub_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299408324u32
        );
    }
    #[test]
    fn decode_c_sub_u_i16() {
        assert_eq!(
            Instruction::decode(2567843780u32), Instruction::C_Sub_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_sub_u_i16() {
        assert_eq!(
            Instruction::C_Sub_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843780u32
        );
    }
    #[test]
    fn decode_c_sub_u_i32() {
        assert_eq!(
            Instruction::decode(2836279236u32), Instruction::C_Sub_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_sub_u_i32() {
        assert_eq!(
            Instruction::C_Sub_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836279236u32
        );
    }
    #[test]
    fn decode_c_sub_u_i64() {
        assert_eq!(
            Instruction::decode(3104714692u32), Instruction::C_Sub_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_c_sub_u_i64() {
        assert_eq!(
            Instruction::C_Sub_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714692u32
        );
    }
    #[test]
    fn decode_o_abs_i8() {
        assert_eq!(
            Instruction::decode(2148412421u32), Instruction::O_Abs_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_o_abs_i8() {
        assert_eq!(
            Instruction::O_Abs_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412421u32
        );
    }
    #[test]
    fn decode_o_abs_i16() {
        assert_eq!(
            Instruction::decode(2416847877u32), Instruction::O_Abs_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_o_abs_i16() {
        assert_eq!(
            Instruction::O_Abs_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416847877u32
        );
    }
    #[test]
    fn decode_o_abs_i32() {
        assert_eq!(
            Instruction::decode(2685283333u32), Instruction::O_Abs_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_o_abs_i32() {
        assert_eq!(
            Instruction::O_Abs_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283333u32
        );
    }
    #[test]
    fn decode_o_abs_i64() {
        assert_eq!(
            Instruction::decode(2953718789u32), Instruction::O_Abs_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_o_abs_i64() {
        assert_eq!(
            Instruction::O_Abs_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953718789u32
        );
    }
    #[test]
    fn decode_o_add_i8() {
        assert_eq!(
            Instruction::decode(2299407429u32), Instruction::O_Add_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_i8() {
        assert_eq!(
            Instruction::O_Add_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407429u32
        );
    }
    #[test]
    fn decode_o_add_i16() {
        assert_eq!(
            Instruction::decode(2567842885u32), Instruction::O_Add_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_i16() {
        assert_eq!(
            Instruction::O_Add_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567842885u32
        );
    }
    #[test]
    fn decode_o_add_i32() {
        assert_eq!(
            Instruction::decode(2836278341u32), Instruction::O_Add_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_i32() {
        assert_eq!(
            Instruction::O_Add_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278341u32
        );
    }
    #[test]
    fn decode_o_add_i64() {
        assert_eq!(
            Instruction::decode(3104713797u32), Instruction::O_Add_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_i64() {
        assert_eq!(
            Instruction::O_Add_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713797u32
        );
    }
    #[test]
    fn decode_o_add_u8() {
        assert_eq!(
            Instruction::decode(151923781u32), Instruction::O_Add_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_u8() {
        assert_eq!(
            Instruction::O_Add_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923781u32
        );
    }
    #[test]
    fn decode_o_add_u16() {
        assert_eq!(
            Instruction::decode(420359237u32), Instruction::O_Add_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_u16() {
        assert_eq!(
            Instruction::O_Add_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359237u32
        );
    }
    #[test]
    fn decode_o_add_u32() {
        assert_eq!(
            Instruction::decode(688794693u32), Instruction::O_Add_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_u32() {
        assert_eq!(
            Instruction::O_Add_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794693u32
        );
    }
    #[test]
    fn decode_o_add_u64() {
        assert_eq!(
            Instruction::decode(957230149u32), Instruction::O_Add_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_u64() {
        assert_eq!(
            Instruction::O_Add_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230149u32
        );
    }
    #[test]
    fn decode_o_add_u_i8() {
        assert_eq!(
            Instruction::decode(2299407493u32), Instruction::O_Add_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_u_i8() {
        assert_eq!(
            Instruction::O_Add_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407493u32
        );
    }
    #[test]
    fn decode_o_add_u_i16() {
        assert_eq!(
            Instruction::decode(2567842949u32), Instruction::O_Add_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_u_i16() {
        assert_eq!(
            Instruction::O_Add_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567842949u32
        );
    }
    #[test]
    fn decode_o_add_u_i32() {
        assert_eq!(
            Instruction::decode(2836278405u32), Instruction::O_Add_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_u_i32() {
        assert_eq!(
            Instruction::O_Add_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278405u32
        );
    }
    #[test]
    fn decode_o_add_u_i64() {
        assert_eq!(
            Instruction::decode(3104713861u32), Instruction::O_Add_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_u_i64() {
        assert_eq!(
            Instruction::O_Add_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713861u32
        );
    }
    #[test]
    fn decode_o_add_s_u8() {
        assert_eq!(
            Instruction::decode(151923845u32), Instruction::O_Add_S_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_s_u8() {
        assert_eq!(
            Instruction::O_Add_S_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923845u32
        );
    }
    #[test]
    fn decode_o_add_s_u16() {
        assert_eq!(
            Instruction::decode(420359301u32), Instruction::O_Add_S_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_s_u16() {
        assert_eq!(
            Instruction::O_Add_S_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359301u32
        );
    }
    #[test]
    fn decode_o_add_s_u32() {
        assert_eq!(
            Instruction::decode(688794757u32), Instruction::O_Add_S_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_s_u32() {
        assert_eq!(
            Instruction::O_Add_S_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794757u32
        );
    }
    #[test]
    fn decode_o_add_s_u64() {
        assert_eq!(
            Instruction::decode(957230213u32), Instruction::O_Add_S_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_add_s_u64() {
        assert_eq!(
            Instruction::O_Add_S_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230213u32
        );
    }
    #[test]
    fn decode_o_div_i8() {
        assert_eq!(
            Instruction::decode(2299407557u32), Instruction::O_Div_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_i8() {
        assert_eq!(
            Instruction::O_Div_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407557u32
        );
    }
    #[test]
    fn decode_o_div_i16() {
        assert_eq!(
            Instruction::decode(2567843013u32), Instruction::O_Div_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_i16() {
        assert_eq!(
            Instruction::O_Div_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843013u32
        );
    }
    #[test]
    fn decode_o_div_i32() {
        assert_eq!(
            Instruction::decode(2836278469u32), Instruction::O_Div_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_i32() {
        assert_eq!(
            Instruction::O_Div_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278469u32
        );
    }
    #[test]
    fn decode_o_div_i64() {
        assert_eq!(
            Instruction::decode(3104713925u32), Instruction::O_Div_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_i64() {
        assert_eq!(
            Instruction::O_Div_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713925u32
        );
    }
    #[test]
    fn decode_o_div_u8() {
        assert_eq!(
            Instruction::decode(151923909u32), Instruction::O_Div_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_u8() {
        assert_eq!(
            Instruction::O_Div_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923909u32
        );
    }
    #[test]
    fn decode_o_div_u16() {
        assert_eq!(
            Instruction::decode(420359365u32), Instruction::O_Div_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_u16() {
        assert_eq!(
            Instruction::O_Div_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359365u32
        );
    }
    #[test]
    fn decode_o_div_u32() {
        assert_eq!(
            Instruction::decode(688794821u32), Instruction::O_Div_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_u32() {
        assert_eq!(
            Instruction::O_Div_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794821u32
        );
    }
    #[test]
    fn decode_o_div_u64() {
        assert_eq!(
            Instruction::decode(957230277u32), Instruction::O_Div_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_u64() {
        assert_eq!(
            Instruction::O_Div_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230277u32
        );
    }
    #[test]
    fn decode_o_div_e_i8() {
        assert_eq!(
            Instruction::decode(2299407621u32), Instruction::O_Div_E_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_e_i8() {
        assert_eq!(
            Instruction::O_Div_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407621u32
        );
    }
    #[test]
    fn decode_o_div_e_i16() {
        assert_eq!(
            Instruction::decode(2567843077u32), Instruction::O_Div_E_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_e_i16() {
        assert_eq!(
            Instruction::O_Div_E_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843077u32
        );
    }
    #[test]
    fn decode_o_div_e_i32() {
        assert_eq!(
            Instruction::decode(2836278533u32), Instruction::O_Div_E_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_e_i32() {
        assert_eq!(
            Instruction::O_Div_E_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278533u32
        );
    }
    #[test]
    fn decode_o_div_e_i64() {
        assert_eq!(
            Instruction::decode(3104713989u32), Instruction::O_Div_E_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_e_i64() {
        assert_eq!(
            Instruction::O_Div_E_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713989u32
        );
    }
    #[test]
    fn decode_o_div_e_u8() {
        assert_eq!(
            Instruction::decode(151923973u32), Instruction::O_Div_E_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_e_u8() {
        assert_eq!(
            Instruction::O_Div_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923973u32
        );
    }
    #[test]
    fn decode_o_div_e_u16() {
        assert_eq!(
            Instruction::decode(420359429u32), Instruction::O_Div_E_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_e_u16() {
        assert_eq!(
            Instruction::O_Div_E_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359429u32
        );
    }
    #[test]
    fn decode_o_div_e_u32() {
        assert_eq!(
            Instruction::decode(688794885u32), Instruction::O_Div_E_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_e_u32() {
        assert_eq!(
            Instruction::O_Div_E_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794885u32
        );
    }
    #[test]
    fn decode_o_div_e_u64() {
        assert_eq!(
            Instruction::decode(957230341u32), Instruction::O_Div_E_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_div_e_u64() {
        assert_eq!(
            Instruction::O_Div_E_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230341u32
        );
    }
    #[test]
    fn decode_o_mul_i8() {
        assert_eq!(
            Instruction::decode(2299407813u32), Instruction::O_Mul_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_mul_i8() {
        assert_eq!(
            Instruction::O_Mul_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407813u32
        );
    }
    #[test]
    fn decode_o_mul_i16() {
        assert_eq!(
            Instruction::decode(2567843269u32), Instruction::O_Mul_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_mul_i16() {
        assert_eq!(
            Instruction::O_Mul_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843269u32
        );
    }
    #[test]
    fn decode_o_mul_i32() {
        assert_eq!(
            Instruction::decode(2836278725u32), Instruction::O_Mul_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_mul_i32() {
        assert_eq!(
            Instruction::O_Mul_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278725u32
        );
    }
    #[test]
    fn decode_o_mul_i64() {
        assert_eq!(
            Instruction::decode(3104714181u32), Instruction::O_Mul_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_mul_i64() {
        assert_eq!(
            Instruction::O_Mul_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714181u32
        );
    }
    #[test]
    fn decode_o_mul_u8() {
        assert_eq!(
            Instruction::decode(151924165u32), Instruction::O_Mul_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_mul_u8() {
        assert_eq!(
            Instruction::O_Mul_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924165u32
        );
    }
    #[test]
    fn decode_o_mul_u16() {
        assert_eq!(
            Instruction::decode(420359621u32), Instruction::O_Mul_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_mul_u16() {
        assert_eq!(
            Instruction::O_Mul_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359621u32
        );
    }
    #[test]
    fn decode_o_mul_u32() {
        assert_eq!(
            Instruction::decode(688795077u32), Instruction::O_Mul_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_mul_u32() {
        assert_eq!(
            Instruction::O_Mul_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795077u32
        );
    }
    #[test]
    fn decode_o_mul_u64() {
        assert_eq!(
            Instruction::decode(957230533u32), Instruction::O_Mul_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_mul_u64() {
        assert_eq!(
            Instruction::O_Mul_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230533u32
        );
    }
    #[test]
    fn decode_o_neg_i8() {
        assert_eq!(
            Instruction::decode(2148412933u32), Instruction::O_Neg_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_o_neg_i8() {
        assert_eq!(
            Instruction::O_Neg_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412933u32
        );
    }
    #[test]
    fn decode_o_neg_i16() {
        assert_eq!(
            Instruction::decode(2416848389u32), Instruction::O_Neg_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_o_neg_i16() {
        assert_eq!(
            Instruction::O_Neg_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848389u32
        );
    }
    #[test]
    fn decode_o_neg_i32() {
        assert_eq!(
            Instruction::decode(2685283845u32), Instruction::O_Neg_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_o_neg_i32() {
        assert_eq!(
            Instruction::O_Neg_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283845u32
        );
    }
    #[test]
    fn decode_o_neg_i64() {
        assert_eq!(
            Instruction::decode(2953719301u32), Instruction::O_Neg_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_o_neg_i64() {
        assert_eq!(
            Instruction::O_Neg_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719301u32
        );
    }
    #[test]
    fn decode_o_pow_i8() {
        assert_eq!(
            Instruction::decode(2299407941u32), Instruction::O_Pow_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_pow_i8() {
        assert_eq!(
            Instruction::O_Pow_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407941u32
        );
    }
    #[test]
    fn decode_o_pow_i16() {
        assert_eq!(
            Instruction::decode(2567843397u32), Instruction::O_Pow_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_pow_i16() {
        assert_eq!(
            Instruction::O_Pow_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843397u32
        );
    }
    #[test]
    fn decode_o_pow_i32() {
        assert_eq!(
            Instruction::decode(2836278853u32), Instruction::O_Pow_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_pow_i32() {
        assert_eq!(
            Instruction::O_Pow_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278853u32
        );
    }
    #[test]
    fn decode_o_pow_i64() {
        assert_eq!(
            Instruction::decode(3104714309u32), Instruction::O_Pow_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_pow_i64() {
        assert_eq!(
            Instruction::O_Pow_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714309u32
        );
    }
    #[test]
    fn decode_o_pow_u8() {
        assert_eq!(
            Instruction::decode(151924293u32), Instruction::O_Pow_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_pow_u8() {
        assert_eq!(
            Instruction::O_Pow_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924293u32
        );
    }
    #[test]
    fn decode_o_pow_u16() {
        assert_eq!(
            Instruction::decode(420359749u32), Instruction::O_Pow_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_pow_u16() {
        assert_eq!(
            Instruction::O_Pow_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359749u32
        );
    }
    #[test]
    fn decode_o_pow_u32() {
        assert_eq!(
            Instruction::decode(688795205u32), Instruction::O_Pow_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_pow_u32() {
        assert_eq!(
            Instruction::O_Pow_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795205u32
        );
    }
    #[test]
    fn decode_o_pow_u64() {
        assert_eq!(
            Instruction::decode(957230661u32), Instruction::O_Pow_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_pow_u64() {
        assert_eq!(
            Instruction::O_Pow_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230661u32
        );
    }
    #[test]
    fn decode_o_rem_i8() {
        assert_eq!(
            Instruction::decode(2299408005u32), Instruction::O_Rem_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_i8() {
        assert_eq!(
            Instruction::O_Rem_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299408005u32
        );
    }
    #[test]
    fn decode_o_rem_i16() {
        assert_eq!(
            Instruction::decode(2567843461u32), Instruction::O_Rem_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_i16() {
        assert_eq!(
            Instruction::O_Rem_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843461u32
        );
    }
    #[test]
    fn decode_o_rem_i32() {
        assert_eq!(
            Instruction::decode(2836278917u32), Instruction::O_Rem_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_i32() {
        assert_eq!(
            Instruction::O_Rem_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278917u32
        );
    }
    #[test]
    fn decode_o_rem_i64() {
        assert_eq!(
            Instruction::decode(3104714373u32), Instruction::O_Rem_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_i64() {
        assert_eq!(
            Instruction::O_Rem_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714373u32
        );
    }
    #[test]
    fn decode_o_rem_u8() {
        assert_eq!(
            Instruction::decode(151924357u32), Instruction::O_Rem_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_u8() {
        assert_eq!(
            Instruction::O_Rem_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924357u32
        );
    }
    #[test]
    fn decode_o_rem_u16() {
        assert_eq!(
            Instruction::decode(420359813u32), Instruction::O_Rem_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_u16() {
        assert_eq!(
            Instruction::O_Rem_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359813u32
        );
    }
    #[test]
    fn decode_o_rem_u32() {
        assert_eq!(
            Instruction::decode(688795269u32), Instruction::O_Rem_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_u32() {
        assert_eq!(
            Instruction::O_Rem_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795269u32
        );
    }
    #[test]
    fn decode_o_rem_u64() {
        assert_eq!(
            Instruction::decode(957230725u32), Instruction::O_Rem_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_u64() {
        assert_eq!(
            Instruction::O_Rem_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230725u32
        );
    }
    #[test]
    fn decode_o_rem_e_i8() {
        assert_eq!(
            Instruction::decode(2299408069u32), Instruction::O_Rem_E_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_e_i8() {
        assert_eq!(
            Instruction::O_Rem_E_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299408069u32
        );
    }
    #[test]
    fn decode_o_rem_e_i16() {
        assert_eq!(
            Instruction::decode(2567843525u32), Instruction::O_Rem_E_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_e_i16() {
        assert_eq!(
            Instruction::O_Rem_E_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843525u32
        );
    }
    #[test]
    fn decode_o_rem_e_i32() {
        assert_eq!(
            Instruction::decode(2836278981u32), Instruction::O_Rem_E_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_e_i32() {
        assert_eq!(
            Instruction::O_Rem_E_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278981u32
        );
    }
    #[test]
    fn decode_o_rem_e_i64() {
        assert_eq!(
            Instruction::decode(3104714437u32), Instruction::O_Rem_E_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_e_i64() {
        assert_eq!(
            Instruction::O_Rem_E_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714437u32
        );
    }
    #[test]
    fn decode_o_rem_e_u8() {
        assert_eq!(
            Instruction::decode(151924421u32), Instruction::O_Rem_E_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_e_u8() {
        assert_eq!(
            Instruction::O_Rem_E_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924421u32
        );
    }
    #[test]
    fn decode_o_rem_e_u16() {
        assert_eq!(
            Instruction::decode(420359877u32), Instruction::O_Rem_E_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_e_u16() {
        assert_eq!(
            Instruction::O_Rem_E_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359877u32
        );
    }
    #[test]
    fn decode_o_rem_e_u32() {
        assert_eq!(
            Instruction::decode(688795333u32), Instruction::O_Rem_E_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_e_u32() {
        assert_eq!(
            Instruction::O_Rem_E_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795333u32
        );
    }
    #[test]
    fn decode_o_rem_e_u64() {
        assert_eq!(
            Instruction::decode(957230789u32), Instruction::O_Rem_E_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_rem_e_u64() {
        assert_eq!(
            Instruction::O_Rem_E_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230789u32
        );
    }
    #[test]
    fn decode_o_shl_i8() {
        assert_eq!(
            Instruction::decode(2299408133u32), Instruction::O_Shl_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shl_i8() {
        assert_eq!(
            Instruction::O_Shl_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299408133u32
        );
    }
    #[test]
    fn decode_o_shl_i16() {
        assert_eq!(
            Instruction::decode(2567843589u32), Instruction::O_Shl_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shl_i16() {
        assert_eq!(
            Instruction::O_Shl_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843589u32
        );
    }
    #[test]
    fn decode_o_shl_i32() {
        assert_eq!(
            Instruction::decode(2836279045u32), Instruction::O_Shl_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shl_i32() {
        assert_eq!(
            Instruction::O_Shl_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836279045u32
        );
    }
    #[test]
    fn decode_o_shl_i64() {
        assert_eq!(
            Instruction::decode(3104714501u32), Instruction::O_Shl_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shl_i64() {
        assert_eq!(
            Instruction::O_Shl_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714501u32
        );
    }
    #[test]
    fn decode_o_shl_u8() {
        assert_eq!(
            Instruction::decode(151924485u32), Instruction::O_Shl_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shl_u8() {
        assert_eq!(
            Instruction::O_Shl_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924485u32
        );
    }
    #[test]
    fn decode_o_shl_u16() {
        assert_eq!(
            Instruction::decode(420359941u32), Instruction::O_Shl_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shl_u16() {
        assert_eq!(
            Instruction::O_Shl_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359941u32
        );
    }
    #[test]
    fn decode_o_shl_u32() {
        assert_eq!(
            Instruction::decode(688795397u32), Instruction::O_Shl_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shl_u32() {
        assert_eq!(
            Instruction::O_Shl_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795397u32
        );
    }
    #[test]
    fn decode_o_shl_u64() {
        assert_eq!(
            Instruction::decode(957230853u32), Instruction::O_Shl_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shl_u64() {
        assert_eq!(
            Instruction::O_Shl_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230853u32
        );
    }
    #[test]
    fn decode_o_shr_i8() {
        assert_eq!(
            Instruction::decode(2299408197u32), Instruction::O_Shr_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shr_i8() {
        assert_eq!(
            Instruction::O_Shr_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299408197u32
        );
    }
    #[test]
    fn decode_o_shr_i16() {
        assert_eq!(
            Instruction::decode(2567843653u32), Instruction::O_Shr_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shr_i16() {
        assert_eq!(
            Instruction::O_Shr_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843653u32
        );
    }
    #[test]
    fn decode_o_shr_i32() {
        assert_eq!(
            Instruction::decode(2836279109u32), Instruction::O_Shr_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shr_i32() {
        assert_eq!(
            Instruction::O_Shr_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836279109u32
        );
    }
    #[test]
    fn decode_o_shr_i64() {
        assert_eq!(
            Instruction::decode(3104714565u32), Instruction::O_Shr_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shr_i64() {
        assert_eq!(
            Instruction::O_Shr_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714565u32
        );
    }
    #[test]
    fn decode_o_shr_u8() {
        assert_eq!(
            Instruction::decode(151924549u32), Instruction::O_Shr_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shr_u8() {
        assert_eq!(
            Instruction::O_Shr_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924549u32
        );
    }
    #[test]
    fn decode_o_shr_u16() {
        assert_eq!(
            Instruction::decode(420360005u32), Instruction::O_Shr_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shr_u16() {
        assert_eq!(
            Instruction::O_Shr_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420360005u32
        );
    }
    #[test]
    fn decode_o_shr_u32() {
        assert_eq!(
            Instruction::decode(688795461u32), Instruction::O_Shr_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shr_u32() {
        assert_eq!(
            Instruction::O_Shr_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795461u32
        );
    }
    #[test]
    fn decode_o_shr_u64() {
        assert_eq!(
            Instruction::decode(957230917u32), Instruction::O_Shr_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_shr_u64() {
        assert_eq!(
            Instruction::O_Shr_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230917u32
        );
    }
    #[test]
    fn decode_o_sub_i8() {
        assert_eq!(
            Instruction::decode(2299408261u32), Instruction::O_Sub_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_sub_i8() {
        assert_eq!(
            Instruction::O_Sub_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299408261u32
        );
    }
    #[test]
    fn decode_o_sub_i16() {
        assert_eq!(
            Instruction::decode(2567843717u32), Instruction::O_Sub_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_sub_i16() {
        assert_eq!(
            Instruction::O_Sub_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843717u32
        );
    }
    #[test]
    fn decode_o_sub_i32() {
        assert_eq!(
            Instruction::decode(2836279173u32), Instruction::O_Sub_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_sub_i32() {
        assert_eq!(
            Instruction::O_Sub_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836279173u32
        );
    }
    #[test]
    fn decode_o_sub_i64() {
        assert_eq!(
            Instruction::decode(3104714629u32), Instruction::O_Sub_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_sub_i64() {
        assert_eq!(
            Instruction::O_Sub_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714629u32
        );
    }
    #[test]
    fn decode_o_sub_u8() {
        assert_eq!(
            Instruction::decode(151924613u32), Instruction::O_Sub_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_sub_u8() {
        assert_eq!(
            Instruction::O_Sub_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924613u32
        );
    }
    #[test]
    fn decode_o_sub_u16() {
        assert_eq!(
            Instruction::decode(420360069u32), Instruction::O_Sub_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_sub_u16() {
        assert_eq!(
            Instruction::O_Sub_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420360069u32
        );
    }
    #[test]
    fn decode_o_sub_u32() {
        assert_eq!(
            Instruction::decode(688795525u32), Instruction::O_Sub_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_sub_u32() {
        assert_eq!(
            Instruction::O_Sub_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795525u32
        );
    }
    #[test]
    fn decode_o_sub_u64() {
        assert_eq!(
            Instruction::decode(957230981u32), Instruction::O_Sub_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_sub_u64() {
        assert_eq!(
            Instruction::O_Sub_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230981u32
        );
    }
    #[test]
    fn decode_o_sub_u_i8() {
        assert_eq!(
            Instruction::decode(2299408325u32), Instruction::O_Sub_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_sub_u_i8() {
        assert_eq!(
            Instruction::O_Sub_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299408325u32
        );
    }
    #[test]
    fn decode_o_sub_u_i16() {
        assert_eq!(
            Instruction::decode(2567843781u32), Instruction::O_Sub_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_sub_u_i16() {
        assert_eq!(
            Instruction::O_Sub_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843781u32
        );
    }
    #[test]
    fn decode_o_sub_u_i32() {
        assert_eq!(
            Instruction::decode(2836279237u32), Instruction::O_Sub_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_sub_u_i32() {
        assert_eq!(
            Instruction::O_Sub_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836279237u32
        );
    }
    #[test]
    fn decode_o_sub_u_i64() {
        assert_eq!(
            Instruction::decode(3104714693u32), Instruction::O_Sub_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_o_sub_u_i64() {
        assert_eq!(
            Instruction::O_Sub_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714693u32
        );
    }
    #[test]
    fn decode_s_abs_i8() {
        assert_eq!(
            Instruction::decode(2148412422u32), Instruction::S_Abs_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_s_abs_i8() {
        assert_eq!(
            Instruction::S_Abs_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412422u32
        );
    }
    #[test]
    fn decode_s_abs_i16() {
        assert_eq!(
            Instruction::decode(2416847878u32), Instruction::S_Abs_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_s_abs_i16() {
        assert_eq!(
            Instruction::S_Abs_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416847878u32
        );
    }
    #[test]
    fn decode_s_abs_i32() {
        assert_eq!(
            Instruction::decode(2685283334u32), Instruction::S_Abs_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_s_abs_i32() {
        assert_eq!(
            Instruction::S_Abs_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283334u32
        );
    }
    #[test]
    fn decode_s_abs_i64() {
        assert_eq!(
            Instruction::decode(2953718790u32), Instruction::S_Abs_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_s_abs_i64() {
        assert_eq!(
            Instruction::S_Abs_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953718790u32
        );
    }
    #[test]
    fn decode_s_add_i8() {
        assert_eq!(
            Instruction::decode(2299407430u32), Instruction::S_Add_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_i8() {
        assert_eq!(
            Instruction::S_Add_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407430u32
        );
    }
    #[test]
    fn decode_s_add_i16() {
        assert_eq!(
            Instruction::decode(2567842886u32), Instruction::S_Add_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_i16() {
        assert_eq!(
            Instruction::S_Add_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567842886u32
        );
    }
    #[test]
    fn decode_s_add_i32() {
        assert_eq!(
            Instruction::decode(2836278342u32), Instruction::S_Add_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_i32() {
        assert_eq!(
            Instruction::S_Add_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278342u32
        );
    }
    #[test]
    fn decode_s_add_i64() {
        assert_eq!(
            Instruction::decode(3104713798u32), Instruction::S_Add_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_i64() {
        assert_eq!(
            Instruction::S_Add_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713798u32
        );
    }
    #[test]
    fn decode_s_add_u8() {
        assert_eq!(
            Instruction::decode(151923782u32), Instruction::S_Add_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_u8() {
        assert_eq!(
            Instruction::S_Add_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923782u32
        );
    }
    #[test]
    fn decode_s_add_u16() {
        assert_eq!(
            Instruction::decode(420359238u32), Instruction::S_Add_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_u16() {
        assert_eq!(
            Instruction::S_Add_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359238u32
        );
    }
    #[test]
    fn decode_s_add_u32() {
        assert_eq!(
            Instruction::decode(688794694u32), Instruction::S_Add_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_u32() {
        assert_eq!(
            Instruction::S_Add_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794694u32
        );
    }
    #[test]
    fn decode_s_add_u64() {
        assert_eq!(
            Instruction::decode(957230150u32), Instruction::S_Add_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_u64() {
        assert_eq!(
            Instruction::S_Add_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230150u32
        );
    }
    #[test]
    fn decode_s_add_u_i8() {
        assert_eq!(
            Instruction::decode(2299407494u32), Instruction::S_Add_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_u_i8() {
        assert_eq!(
            Instruction::S_Add_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407494u32
        );
    }
    #[test]
    fn decode_s_add_u_i16() {
        assert_eq!(
            Instruction::decode(2567842950u32), Instruction::S_Add_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_u_i16() {
        assert_eq!(
            Instruction::S_Add_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567842950u32
        );
    }
    #[test]
    fn decode_s_add_u_i32() {
        assert_eq!(
            Instruction::decode(2836278406u32), Instruction::S_Add_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_u_i32() {
        assert_eq!(
            Instruction::S_Add_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278406u32
        );
    }
    #[test]
    fn decode_s_add_u_i64() {
        assert_eq!(
            Instruction::decode(3104713862u32), Instruction::S_Add_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_u_i64() {
        assert_eq!(
            Instruction::S_Add_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713862u32
        );
    }
    #[test]
    fn decode_s_add_s_u8() {
        assert_eq!(
            Instruction::decode(151923846u32), Instruction::S_Add_S_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_s_u8() {
        assert_eq!(
            Instruction::S_Add_S_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923846u32
        );
    }
    #[test]
    fn decode_s_add_s_u16() {
        assert_eq!(
            Instruction::decode(420359302u32), Instruction::S_Add_S_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_s_u16() {
        assert_eq!(
            Instruction::S_Add_S_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359302u32
        );
    }
    #[test]
    fn decode_s_add_s_u32() {
        assert_eq!(
            Instruction::decode(688794758u32), Instruction::S_Add_S_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_s_u32() {
        assert_eq!(
            Instruction::S_Add_S_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794758u32
        );
    }
    #[test]
    fn decode_s_add_s_u64() {
        assert_eq!(
            Instruction::decode(957230214u32), Instruction::S_Add_S_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_add_s_u64() {
        assert_eq!(
            Instruction::S_Add_S_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230214u32
        );
    }
    #[test]
    fn decode_s_div_i8() {
        assert_eq!(
            Instruction::decode(2299407558u32), Instruction::S_Div_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_div_i8() {
        assert_eq!(
            Instruction::S_Div_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407558u32
        );
    }
    #[test]
    fn decode_s_div_i16() {
        assert_eq!(
            Instruction::decode(2567843014u32), Instruction::S_Div_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_div_i16() {
        assert_eq!(
            Instruction::S_Div_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843014u32
        );
    }
    #[test]
    fn decode_s_div_i32() {
        assert_eq!(
            Instruction::decode(2836278470u32), Instruction::S_Div_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_div_i32() {
        assert_eq!(
            Instruction::S_Div_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278470u32
        );
    }
    #[test]
    fn decode_s_div_i64() {
        assert_eq!(
            Instruction::decode(3104713926u32), Instruction::S_Div_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_div_i64() {
        assert_eq!(
            Instruction::S_Div_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104713926u32
        );
    }
    #[test]
    fn decode_s_div_u8() {
        assert_eq!(
            Instruction::decode(151923910u32), Instruction::S_Div_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_div_u8() {
        assert_eq!(
            Instruction::S_Div_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151923910u32
        );
    }
    #[test]
    fn decode_s_div_u16() {
        assert_eq!(
            Instruction::decode(420359366u32), Instruction::S_Div_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_div_u16() {
        assert_eq!(
            Instruction::S_Div_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359366u32
        );
    }
    #[test]
    fn decode_s_div_u32() {
        assert_eq!(
            Instruction::decode(688794822u32), Instruction::S_Div_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_div_u32() {
        assert_eq!(
            Instruction::S_Div_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688794822u32
        );
    }
    #[test]
    fn decode_s_div_u64() {
        assert_eq!(
            Instruction::decode(957230278u32), Instruction::S_Div_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_div_u64() {
        assert_eq!(
            Instruction::S_Div_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230278u32
        );
    }
    #[test]
    fn decode_s_mul_i8() {
        assert_eq!(
            Instruction::decode(2299407814u32), Instruction::S_Mul_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_mul_i8() {
        assert_eq!(
            Instruction::S_Mul_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407814u32
        );
    }
    #[test]
    fn decode_s_mul_i16() {
        assert_eq!(
            Instruction::decode(2567843270u32), Instruction::S_Mul_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_mul_i16() {
        assert_eq!(
            Instruction::S_Mul_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843270u32
        );
    }
    #[test]
    fn decode_s_mul_i32() {
        assert_eq!(
            Instruction::decode(2836278726u32), Instruction::S_Mul_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_mul_i32() {
        assert_eq!(
            Instruction::S_Mul_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278726u32
        );
    }
    #[test]
    fn decode_s_mul_i64() {
        assert_eq!(
            Instruction::decode(3104714182u32), Instruction::S_Mul_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_mul_i64() {
        assert_eq!(
            Instruction::S_Mul_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714182u32
        );
    }
    #[test]
    fn decode_s_mul_u8() {
        assert_eq!(
            Instruction::decode(151924166u32), Instruction::S_Mul_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_mul_u8() {
        assert_eq!(
            Instruction::S_Mul_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924166u32
        );
    }
    #[test]
    fn decode_s_mul_u16() {
        assert_eq!(
            Instruction::decode(420359622u32), Instruction::S_Mul_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_mul_u16() {
        assert_eq!(
            Instruction::S_Mul_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359622u32
        );
    }
    #[test]
    fn decode_s_mul_u32() {
        assert_eq!(
            Instruction::decode(688795078u32), Instruction::S_Mul_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_mul_u32() {
        assert_eq!(
            Instruction::S_Mul_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795078u32
        );
    }
    #[test]
    fn decode_s_mul_u64() {
        assert_eq!(
            Instruction::decode(957230534u32), Instruction::S_Mul_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_mul_u64() {
        assert_eq!(
            Instruction::S_Mul_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230534u32
        );
    }
    #[test]
    fn decode_s_neg_i8() {
        assert_eq!(
            Instruction::decode(2148412934u32), Instruction::S_Neg_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_s_neg_i8() {
        assert_eq!(
            Instruction::S_Neg_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2148412934u32
        );
    }
    #[test]
    fn decode_s_neg_i16() {
        assert_eq!(
            Instruction::decode(2416848390u32), Instruction::S_Neg_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_s_neg_i16() {
        assert_eq!(
            Instruction::S_Neg_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2416848390u32
        );
    }
    #[test]
    fn decode_s_neg_i32() {
        assert_eq!(
            Instruction::decode(2685283846u32), Instruction::S_Neg_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_s_neg_i32() {
        assert_eq!(
            Instruction::S_Neg_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2685283846u32
        );
    }
    #[test]
    fn decode_s_neg_i64() {
        assert_eq!(
            Instruction::decode(2953719302u32), Instruction::S_Neg_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_s_neg_i64() {
        assert_eq!(
            Instruction::S_Neg_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 2953719302u32
        );
    }
    #[test]
    fn decode_s_pow_i8() {
        assert_eq!(
            Instruction::decode(2299407942u32), Instruction::S_Pow_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_pow_i8() {
        assert_eq!(
            Instruction::S_Pow_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299407942u32
        );
    }
    #[test]
    fn decode_s_pow_i16() {
        assert_eq!(
            Instruction::decode(2567843398u32), Instruction::S_Pow_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_pow_i16() {
        assert_eq!(
            Instruction::S_Pow_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843398u32
        );
    }
    #[test]
    fn decode_s_pow_i32() {
        assert_eq!(
            Instruction::decode(2836278854u32), Instruction::S_Pow_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_pow_i32() {
        assert_eq!(
            Instruction::S_Pow_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836278854u32
        );
    }
    #[test]
    fn decode_s_pow_i64() {
        assert_eq!(
            Instruction::decode(3104714310u32), Instruction::S_Pow_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_pow_i64() {
        assert_eq!(
            Instruction::S_Pow_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714310u32
        );
    }
    #[test]
    fn decode_s_pow_u8() {
        assert_eq!(
            Instruction::decode(151924294u32), Instruction::S_Pow_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_pow_u8() {
        assert_eq!(
            Instruction::S_Pow_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924294u32
        );
    }
    #[test]
    fn decode_s_pow_u16() {
        assert_eq!(
            Instruction::decode(420359750u32), Instruction::S_Pow_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_pow_u16() {
        assert_eq!(
            Instruction::S_Pow_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420359750u32
        );
    }
    #[test]
    fn decode_s_pow_u32() {
        assert_eq!(
            Instruction::decode(688795206u32), Instruction::S_Pow_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_pow_u32() {
        assert_eq!(
            Instruction::S_Pow_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795206u32
        );
    }
    #[test]
    fn decode_s_pow_u64() {
        assert_eq!(
            Instruction::decode(957230662u32), Instruction::S_Pow_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_pow_u64() {
        assert_eq!(
            Instruction::S_Pow_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230662u32
        );
    }
    #[test]
    fn decode_s_sub_i8() {
        assert_eq!(
            Instruction::decode(2299408262u32), Instruction::S_Sub_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_sub_i8() {
        assert_eq!(
            Instruction::S_Sub_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299408262u32
        );
    }
    #[test]
    fn decode_s_sub_i16() {
        assert_eq!(
            Instruction::decode(2567843718u32), Instruction::S_Sub_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_sub_i16() {
        assert_eq!(
            Instruction::S_Sub_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843718u32
        );
    }
    #[test]
    fn decode_s_sub_i32() {
        assert_eq!(
            Instruction::decode(2836279174u32), Instruction::S_Sub_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_sub_i32() {
        assert_eq!(
            Instruction::S_Sub_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836279174u32
        );
    }
    #[test]
    fn decode_s_sub_i64() {
        assert_eq!(
            Instruction::decode(3104714630u32), Instruction::S_Sub_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_sub_i64() {
        assert_eq!(
            Instruction::S_Sub_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714630u32
        );
    }
    #[test]
    fn decode_s_sub_u8() {
        assert_eq!(
            Instruction::decode(151924614u32), Instruction::S_Sub_u8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_sub_u8() {
        assert_eq!(
            Instruction::S_Sub_u8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 151924614u32
        );
    }
    #[test]
    fn decode_s_sub_u16() {
        assert_eq!(
            Instruction::decode(420360070u32), Instruction::S_Sub_u16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_sub_u16() {
        assert_eq!(
            Instruction::S_Sub_u16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 420360070u32
        );
    }
    #[test]
    fn decode_s_sub_u32() {
        assert_eq!(
            Instruction::decode(688795526u32), Instruction::S_Sub_u32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_sub_u32() {
        assert_eq!(
            Instruction::S_Sub_u32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 688795526u32
        );
    }
    #[test]
    fn decode_s_sub_u64() {
        assert_eq!(
            Instruction::decode(957230982u32), Instruction::S_Sub_u64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_sub_u64() {
        assert_eq!(
            Instruction::S_Sub_u64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 957230982u32
        );
    }
    #[test]
    fn decode_s_sub_u_i8() {
        assert_eq!(
            Instruction::decode(2299408326u32), Instruction::S_Sub_U_i8 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_sub_u_i8() {
        assert_eq!(
            Instruction::S_Sub_U_i8 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2299408326u32
        );
    }
    #[test]
    fn decode_s_sub_u_i16() {
        assert_eq!(
            Instruction::decode(2567843782u32), Instruction::S_Sub_U_i16 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_sub_u_i16() {
        assert_eq!(
            Instruction::S_Sub_U_i16 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2567843782u32
        );
    }
    #[test]
    fn decode_s_sub_u_i32() {
        assert_eq!(
            Instruction::decode(2836279238u32), Instruction::S_Sub_U_i32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_sub_u_i32() {
        assert_eq!(
            Instruction::S_Sub_U_i32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 2836279238u32
        );
    }
    #[test]
    fn decode_s_sub_u_i64() {
        assert_eq!(
            Instruction::decode(3104714694u32), Instruction::S_Sub_U_i64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_s_sub_u_i64() {
        assert_eq!(
            Instruction::S_Sub_U_i64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3104714694u32
        );
    }
    #[test]
    fn decode_abs_f32() {
        assert_eq!(
            Instruction::decode(3759025159u32), Instruction::Abs_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_abs_f32() {
        assert_eq!(
            Instruction::Abs_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 3759025159u32
        );
    }
    #[test]
    fn decode_abs_f64() {
        assert_eq!(
            Instruction::decode(4027460615u32), Instruction::Abs_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_abs_f64() {
        assert_eq!(
            Instruction::Abs_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 4027460615u32
        );
    }
    #[test]
    fn decode_add_f32() {
        assert_eq!(
            Instruction::decode(3910020167u32), Instruction::Add_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_add_f32() {
        assert_eq!(
            Instruction::Add_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3910020167u32
        );
    }
    #[test]
    fn decode_add_f64() {
        assert_eq!(
            Instruction::decode(4178455623u32), Instruction::Add_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_add_f64() {
        assert_eq!(
            Instruction::Add_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 4178455623u32
        );
    }
    #[test]
    fn decode_div_f32() {
        assert_eq!(
            Instruction::decode(3910020295u32), Instruction::Div_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_div_f32() {
        assert_eq!(
            Instruction::Div_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3910020295u32
        );
    }
    #[test]
    fn decode_div_f64() {
        assert_eq!(
            Instruction::decode(4178455751u32), Instruction::Div_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_div_f64() {
        assert_eq!(
            Instruction::Div_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 4178455751u32
        );
    }
    #[test]
    fn decode_div_e_f32() {
        assert_eq!(
            Instruction::decode(3910020359u32), Instruction::Div_E_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_div_e_f32() {
        assert_eq!(
            Instruction::Div_E_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3910020359u32
        );
    }
    #[test]
    fn decode_div_e_f64() {
        assert_eq!(
            Instruction::decode(4178455815u32), Instruction::Div_E_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_div_e_f64() {
        assert_eq!(
            Instruction::Div_E_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 4178455815u32
        );
    }
    #[test]
    fn decode_log_f32() {
        assert_eq!(
            Instruction::decode(3910020423u32), Instruction::Log_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_log_f32() {
        assert_eq!(
            Instruction::Log_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3910020423u32
        );
    }
    #[test]
    fn decode_log_f64() {
        assert_eq!(
            Instruction::decode(4178455879u32), Instruction::Log_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_log_f64() {
        assert_eq!(
            Instruction::Log_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 4178455879u32
        );
    }
    #[test]
    fn decode_sqrt_f32() {
        assert_eq!(
            Instruction::decode(3759025543u32), Instruction::Sqrt_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_sqrt_f32() {
        assert_eq!(
            Instruction::Sqrt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 3759025543u32
        );
    }
    #[test]
    fn decode_sqrt_f64() {
        assert_eq!(
            Instruction::decode(4027460999u32), Instruction::Sqrt_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_sqrt_f64() {
        assert_eq!(
            Instruction::Sqrt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 4027460999u32
        );
    }
    #[test]
    fn decode_mul_f32() {
        assert_eq!(
            Instruction::decode(3910020551u32), Instruction::Mul_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_mul_f32() {
        assert_eq!(
            Instruction::Mul_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3910020551u32
        );
    }
    #[test]
    fn decode_mul_f64() {
        assert_eq!(
            Instruction::decode(4178456007u32), Instruction::Mul_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_mul_f64() {
        assert_eq!(
            Instruction::Mul_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 4178456007u32
        );
    }
    #[test]
    fn decode_neg_f32() {
        assert_eq!(
            Instruction::decode(3759025671u32), Instruction::Neg_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_neg_f32() {
        assert_eq!(
            Instruction::Neg_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 3759025671u32
        );
    }
    #[test]
    fn decode_neg_f64() {
        assert_eq!(
            Instruction::decode(4027461127u32), Instruction::Neg_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_neg_f64() {
        assert_eq!(
            Instruction::Neg_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 4027461127u32
        );
    }
    #[test]
    fn decode_pow_f32() {
        assert_eq!(
            Instruction::decode(3910020679u32), Instruction::Pow_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_pow_f32() {
        assert_eq!(
            Instruction::Pow_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3910020679u32
        );
    }
    #[test]
    fn decode_pow_f64() {
        assert_eq!(
            Instruction::decode(4178456135u32), Instruction::Pow_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_pow_f64() {
        assert_eq!(
            Instruction::Pow_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 4178456135u32
        );
    }
    #[test]
    fn decode_rem_f32() {
        assert_eq!(
            Instruction::decode(3910020743u32), Instruction::Rem_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rem_f32() {
        assert_eq!(
            Instruction::Rem_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3910020743u32
        );
    }
    #[test]
    fn decode_rem_f64() {
        assert_eq!(
            Instruction::decode(4178456199u32), Instruction::Rem_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rem_f64() {
        assert_eq!(
            Instruction::Rem_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 4178456199u32
        );
    }
    #[test]
    fn decode_rem_e_f32() {
        assert_eq!(
            Instruction::decode(3910020807u32), Instruction::Rem_E_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rem_e_f32() {
        assert_eq!(
            Instruction::Rem_E_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3910020807u32
        );
    }
    #[test]
    fn decode_rem_e_f64() {
        assert_eq!(
            Instruction::decode(4178456263u32), Instruction::Rem_E_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_rem_e_f64() {
        assert_eq!(
            Instruction::Rem_E_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 4178456263u32
        );
    }
    #[test]
    fn decode_cbrt_f32() {
        assert_eq!(
            Instruction::decode(3759025991u32), Instruction::Cbrt_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_cbrt_f32() {
        assert_eq!(
            Instruction::Cbrt_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 3759025991u32
        );
    }
    #[test]
    fn decode_cbrt_f64() {
        assert_eq!(
            Instruction::decode(4027461447u32), Instruction::Cbrt_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), }
        );
    }
    #[test]
    fn encode_cbrt_f64() {
        assert_eq!(
            Instruction::Cbrt_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), } .encode(), 4027461447u32
        );
    }
    #[test]
    fn decode_sub_f32() {
        assert_eq!(
            Instruction::decode(3910020999u32), Instruction::Sub_f32 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_sub_f32() {
        assert_eq!(
            Instruction::Sub_f32 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 3910020999u32
        );
    }
    #[test]
    fn decode_sub_f64() {
        assert_eq!(
            Instruction::decode(4178456455u32), Instruction::Sub_f64 { rd :
            Register::General_Purpose(5), rs1 : Register::General_Purpose(8), rs2 :
            Register::General_Purpose(30), }
        );
    }
    #[test]
    fn encode_sub_f64() {
        assert_eq!(
            Instruction::Sub_f64 { rd : Register::General_Purpose(5), rs1 :
            Register::General_Purpose(8), rs2 : Register::General_Purpose(30), }
            .encode(), 4178456455u32
        );
    }
}
