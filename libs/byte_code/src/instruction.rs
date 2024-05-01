use crate::register::Register;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Halt,
    Trap,
    Call { imm: i32 },
    Ret,
    Ecall { imm: i32 },
    Jal { imm: i32 },
    Bie { imm: i16, rs1: Register, rs2: Register },
    Bne { imm: i16, rs1: Register, rs2: Register },
    Blts { imm: i16, rs1: Register, rs2: Register },
    Bltu { imm: i16, rs1: Register, rs2: Register },
    Bles { imm: i16, rs1: Register, rs2: Register },
    Bleu { imm: i16, rs1: Register, rs2: Register },
    Bgts { imm: i16, rs1: Register, rs2: Register },
    Bgtu { imm: i16, rs1: Register, rs2: Register },
    Bges { imm: i16, rs1: Register, rs2: Register },
    Bgeu { imm: i16, rs1: Register, rs2: Register },
    Bie_f32 { imm: i16, rs1: Register, rs2: Register },
    Bie_f64 { imm: i16, rs1: Register, rs2: Register },
    Bne_f32 { imm: i16, rs1: Register, rs2: Register },
    Bne_f64 { imm: i16, rs1: Register, rs2: Register },
    Blt_f32 { imm: i16, rs1: Register, rs2: Register },
    Blt_f64 { imm: i16, rs1: Register, rs2: Register },
    Ble_f32 { imm: i16, rs1: Register, rs2: Register },
    Ble_f64 { imm: i16, rs1: Register, rs2: Register },
    Bgt_f32 { imm: i16, rs1: Register, rs2: Register },
    Bgt_f64 { imm: i16, rs1: Register, rs2: Register },
    Bge_f32 { imm: i16, rs1: Register, rs2: Register },
    Bge_f64 { imm: i16, rs1: Register, rs2: Register },
    Cie { rd: Register, rs1: Register, rs2: Register },
    Cie_f32 { rd: Register, rs1: Register, rs2: Register },
    Cie_f64 { rd: Register, rs1: Register, rs2: Register },
    Cne { rd: Register, rs1: Register, rs2: Register },
    Cne_f32 { rd: Register, rs1: Register, rs2: Register },
    Cne_f64 { rd: Register, rs1: Register, rs2: Register },
    Clts { rd: Register, rs1: Register, rs2: Register },
    Cltu { rd: Register, rs1: Register, rs2: Register },
    Clt_f32 { rd: Register, rs1: Register, rs2: Register },
    Clt_f64 { rd: Register, rs1: Register, rs2: Register },
    Cles { rd: Register, rs1: Register, rs2: Register },
    Cleu { rd: Register, rs1: Register, rs2: Register },
    Cle_f32 { rd: Register, rs1: Register, rs2: Register },
    Cle_f64 { rd: Register, rs1: Register, rs2: Register },
    Cgts { rd: Register, rs1: Register, rs2: Register },
    Cgtu { rd: Register, rs1: Register, rs2: Register },
    Cgt_f32 { rd: Register, rs1: Register, rs2: Register },
    Cgt_f64 { rd: Register, rs1: Register, rs2: Register },
    Cges { rd: Register, rs1: Register, rs2: Register },
    Cgeu { rd: Register, rs1: Register, rs2: Register },
    Cge_f32 { rd: Register, rs1: Register, rs2: Register },
    Cge_f64 { rd: Register, rs1: Register, rs2: Register },
    Lra_8 { rd: Register, rs1: Register },
    Lra_16 { rd: Register, rs1: Register },
    Lra_32 { rd: Register, rs1: Register },
    Lra_64 { rd: Register, rs1: Register },
    Lsi { rd: Register, imm: i16 },
    Lui { rd: Register, imm: u16 },
    Lia_8 { rd: Register, imm: u16 },
    Lia_16 { rd: Register, imm: u16 },
    Lia_32 { rd: Register, imm: u16 },
    Lia_64 { rd: Register, imm: u16 },
    Sra_8 { rd: Register, rs1: Register },
    Sra_16 { rd: Register, rs1: Register },
    Sra_32 { rd: Register, rs1: Register },
    Sra_64 { rd: Register, rs1: Register },
    Ssi { rd: Register, imm: i16 },
    Sui { rd: Register, imm: u16 },
    Mov { rd: Register, rs1: Register },
    Pop { rd: Register },
    Psh { rd: Register },
    Psi { imm: i32 },
    Pui { imm: i32 },
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
    Fetch_Add_i8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Add_i16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Add_i32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Add_i64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Add_u8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Add_u16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Add_u32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Add_u64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Sub_i8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Sub_i16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Sub_i32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Sub_i64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Sub_u8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Sub_u16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Sub_u32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Sub_u64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Min_i8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Min_i16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Min_i32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Min_i64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Min_u8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Min_u16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Min_u32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Min_u64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Max_i8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Max_i16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Max_i32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Max_i64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Max_u8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Max_u16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Max_u32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Max_u64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_And_i8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_And_i16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_And_i32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_And_i64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_And_u8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_And_u16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_And_u32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_And_u64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Nand_i8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Nand_i16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Nand_i32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Nand_i64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Nand_u8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Nand_u16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Nand_u32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Nand_u64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Or_i8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Or_i16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Or_i32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Or_i64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Or_u8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Or_u16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Or_u32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Or_u64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Xor_i8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Xor_i16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Xor_i32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Xor_i64 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Xor_u8 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Xor_u16 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Xor_u32 { rd: Register, rs1: Register, rs2: Register },
    Fetch_Xor_u64 { rd: Register, rs1: Register, rs2: Register },
    Cmp_Exchg_i8 { rd: Register, rs1: Register, rs2: Register },
    Cmp_Exchg_i16 { rd: Register, rs1: Register, rs2: Register },
    Cmp_Exchg_i32 { rd: Register, rs1: Register, rs2: Register },
    Cmp_Exchg_i64 { rd: Register, rs1: Register, rs2: Register },
    Cmp_Exchg_u8 { rd: Register, rs1: Register, rs2: Register },
    Cmp_Exchg_u16 { rd: Register, rs1: Register, rs2: Register },
    Cmp_Exchg_u32 { rd: Register, rs1: Register, rs2: Register },
    Cmp_Exchg_u64 { rd: Register, rs1: Register, rs2: Register },
    Atomic_Ld_8 { rd: Register, rs1: Register },
    Atomic_Ld_16 { rd: Register, rs1: Register },
    Atomic_Ld_32 { rd: Register, rs1: Register },
    Atomic_Ld_64 { rd: Register, rs1: Register },
    Atomic_St_8 { rd: Register, rs1: Register },
    Atomic_St_16 { rd: Register, rs1: Register },
    Atomic_St_32 { rd: Register, rs1: Register },
    Atomic_St_64 { rd: Register, rs1: Register },
    Spawn { rd: Register, imm: u16 },
    Wait { rd: Register, rs1: Register },
    Notify { rd: Register, rs1: Register },
    Swap_8 { rd: Register, rs1: Register },
    Swap_16 { rd: Register, rs1: Register },
    Swap_32 { rd: Register, rs1: Register },
    Swap_64 { rd: Register, rs1: Register },
    Invalid(u32),
}
impl Instruction {
    pub fn decode(instr: u32) -> Self {
        let op_code = (instr & 15u32) >> 0u32;
        let funct_4 = (instr & 240u32) >> 4u32;
        let rd = (instr & 16128u32) >> 8u32;
        let imm_i16 = (instr & 2147450880u32) >> 15u32;
        let sign = (instr & 2147483648u32) >> 31u32;
        let imm_u16 = (instr & 2147450880u32) >> 15u32;
        let rs1 = (instr & 1032192u32) >> 14u32;
        let rs2 = (instr & 66060288u32) >> 20u32;
        let funct_2 = (instr & 201326592u32) >> 26u32;
        let size = (instr & 805306368u32) >> 28u32;
        let float = (instr & 1073741824u32) >> 30u32;
        let imm_24 = (instr & 4294967040u32) >> 8u32;
        let imm_l6 = (instr & 16128u32) >> 8u32;
        let imm_h6 = (instr & 4227858432u32) >> 26u32;
        match op_code {
            0u32 if funct_4 == 0u32 && imm_24 == 0u32 => Self::Halt,
            0u32 if funct_4 == 1u32 && imm_24 == 0u32 => Self::Trap,
            0u32 if funct_4 == 2u32 => Self::Call { imm: imm_24 as i32 },
            0u32 if funct_4 == 4u32 && imm_24 == 0u32 => Self::Ret,
            0u32 if funct_4 == 5u32 => Self::Ecall { imm: imm_24 as i32 },
            0u32 if funct_4 == 6u32 => Self::Jal { imm: imm_24 as i32 },
            1u32 if funct_4 == 0u32 => {
                Self::Bie {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            1u32 if funct_4 == 1u32 => {
                Self::Bne {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            1u32 if funct_4 == 2u32 => {
                Self::Blts {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            1u32 if funct_4 == 3u32 => {
                Self::Bltu {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            1u32 if funct_4 == 4u32 => {
                Self::Bles {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            1u32 if funct_4 == 5u32 => {
                Self::Bleu {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            1u32 if funct_4 == 6u32 => {
                Self::Bgts {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            1u32 if funct_4 == 7u32 => {
                Self::Bgtu {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            1u32 if funct_4 == 8u32 => {
                Self::Bges {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            1u32 if funct_4 == 9u32 => {
                Self::Bgeu {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            2u32 if funct_4 == 0u32 => {
                Self::Bie_f32 {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            2u32 if funct_4 == 1u32 => {
                Self::Bie_f64 {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            2u32 if funct_4 == 2u32 => {
                Self::Bne_f32 {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            2u32 if funct_4 == 3u32 => {
                Self::Bne_f64 {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            2u32 if funct_4 == 4u32 => {
                Self::Blt_f32 {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            2u32 if funct_4 == 5u32 => {
                Self::Blt_f64 {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            2u32 if funct_4 == 6u32 => {
                Self::Ble_f32 {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            2u32 if funct_4 == 7u32 => {
                Self::Ble_f64 {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            2u32 if funct_4 == 8u32 => {
                Self::Bgt_f32 {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            2u32 if funct_4 == 9u32 => {
                Self::Bgt_f64 {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            2u32 if funct_4 == 10u32 => {
                Self::Bge_f32 {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            2u32 if funct_4 == 11u32 => {
                Self::Bge_f64 {
                    imm: (imm_l6 << 6u32) as i16 | imm_h6 as i16,
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Cie {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Cie_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Cie_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Cne {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Cne_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Cne_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Clts {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Cltu {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Clt_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Clt_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Cles {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Cleu {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Cle_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Cle_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Cgts {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Cgtu {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Cgt_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Cgt_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Cges {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Cgeu {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Cge_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            3u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Cge_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            4u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::Lra_8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            4u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::Lra_16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            4u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::Lra_32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            4u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::Lra_64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            4u32 if funct_4 == 1u32 && sign == 1u32 => {
                Self::Lsi {
                    rd: Register::from_index(rd as u8),
                    imm: imm_i16 as i16,
                }
            }
            4u32 if funct_4 == 1u32 && sign == 0u32 => {
                Self::Lui {
                    rd: Register::from_index(rd as u8),
                    imm: imm_u16 as u16,
                }
            }
            4u32 if funct_4 == 2u32 && sign == 0u32 => {
                Self::Lia_8 {
                    rd: Register::from_index(rd as u8),
                    imm: imm_u16 as u16,
                }
            }
            4u32 if funct_4 == 3u32 && sign == 0u32 => {
                Self::Lia_16 {
                    rd: Register::from_index(rd as u8),
                    imm: imm_u16 as u16,
                }
            }
            4u32 if funct_4 == 4u32 && sign == 0u32 => {
                Self::Lia_32 {
                    rd: Register::from_index(rd as u8),
                    imm: imm_u16 as u16,
                }
            }
            4u32 if funct_4 == 5u32 && sign == 0u32 => {
                Self::Lia_64 {
                    rd: Register::from_index(rd as u8),
                    imm: imm_u16 as u16,
                }
            }
            4u32 if funct_4 == 6u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::Sra_8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            4u32 if funct_4 == 6u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::Sra_16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            4u32 if funct_4 == 6u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::Sra_32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            4u32 if funct_4 == 6u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::Sra_64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            4u32 if funct_4 == 7u32 && sign == 1u32 => {
                Self::Ssi {
                    rd: Register::from_index(rd as u8),
                    imm: imm_i16 as i16,
                }
            }
            4u32 if funct_4 == 7u32 && sign == 0u32 => {
                Self::Sui {
                    rd: Register::from_index(rd as u8),
                    imm: imm_u16 as u16,
                }
            }
            4u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::Mov {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            4u32 if funct_4 == 9u32 && imm_i16 == 0u32 && sign == 0u32 => {
                Self::Pop {
                    rd: Register::from_index(rd as u8),
                }
            }
            4u32 if funct_4 == 9u32 && imm_i16 == 0u32 && sign == 1u32 => {
                Self::Psh {
                    rd: Register::from_index(rd as u8),
                }
            }
            4u32 if funct_4 == 10u32 => Self::Psi { imm: imm_24 as i32 },
            4u32 if funct_4 == 11u32 => Self::Pui { imm: imm_24 as i32 },
            5u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::And_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::And_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::And_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::And_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::And_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::And_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::And_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::And_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Or_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Or_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Or_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Or_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Or_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Or_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Or_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Or_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Xor_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Xor_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Xor_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Xor_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Xor_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Xor_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Xor_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Xor_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 3u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::Not_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 3u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::Not_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 3u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::Not_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 3u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::Not_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 3u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::Not_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 3u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::Not_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 3u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::Not_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 3u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::Not_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Shl_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Shl_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Shl_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Shl_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Shl_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Shl_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Shl_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Shl_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Shr_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Shr_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Shr_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Shr_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Shr_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Shr_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Shr_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Shr_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Rot_L_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Rot_L_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Rot_L_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Rot_L_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Rot_L_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Rot_L_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Rot_L_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Rot_L_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Rot_R_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Rot_R_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Rot_R_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Rot_R_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Rot_R_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Rot_R_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Rot_R_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Rot_R_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            5u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Ones_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Ones_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Ones_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Ones_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::C_Ones_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::C_Ones_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::C_Ones_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::C_Ones_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 9u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::L_Ones_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 9u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::L_Ones_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 9u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::L_Ones_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 9u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::L_Ones_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 9u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::L_Ones_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 9u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::L_Ones_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 9u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::L_Ones_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 9u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::L_Ones_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 10u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::T_Ones_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 10u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::T_Ones_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 10u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::T_Ones_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 10u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::T_Ones_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 10u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::T_Ones_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 10u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::T_Ones_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 10u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::T_Ones_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 10u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::T_Ones_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 11u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Zeros_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 11u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Zeros_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 11u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Zeros_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 11u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Zeros_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 11u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::C_Zeros_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 11u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::C_Zeros_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 11u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::C_Zeros_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 11u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::C_Zeros_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 12u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::L_Zeros_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 12u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::L_Zeros_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 12u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::L_Zeros_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 12u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::L_Zeros_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 12u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::L_Zeros_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 12u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::L_Zeros_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 12u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::L_Zeros_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 12u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::L_Zeros_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 13u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::T_Zeros_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 13u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::T_Zeros_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 13u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::T_Zeros_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 13u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::T_Zeros_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 13u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::T_Zeros_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 13u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::T_Zeros_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 13u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::T_Zeros_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 13u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::T_Zeros_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 14u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::R_Bytes_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 14u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::R_Bytes_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 14u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::R_Bytes_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 14u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::R_Bytes_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 14u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::R_Bytes_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 14u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::R_Bytes_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 14u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::R_Bytes_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 14u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::R_Bytes_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 15u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::R_Bits_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 15u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::R_Bits_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 15u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::R_Bits_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 15u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::R_Bits_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 15u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::R_Bits_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 15u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::R_Bits_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 15u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::R_Bits_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            5u32 if funct_4 == 15u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::R_Bits_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Abs_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Abs_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Abs_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Abs_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Add_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Add_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Add_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Add_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Add_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Add_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Add_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Add_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Add_U_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Add_U_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Add_U_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Add_U_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Add_S_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Add_S_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Add_S_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Add_S_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Div_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Div_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Div_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Div_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Div_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Div_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Div_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Div_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Div_E_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Div_E_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Div_E_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Div_E_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Div_E_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Div_E_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Div_E_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Div_E_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Log_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Log_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Log_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Log_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Log_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Log_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Log_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Log_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 6u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Sqrt_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 6u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Sqrt_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 6u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Sqrt_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 6u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Sqrt_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 6u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::C_Sqrt_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 6u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::C_Sqrt_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 6u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::C_Sqrt_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 6u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::C_Sqrt_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Mul_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Mul_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Mul_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Mul_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Mul_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Mul_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Mul_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Mul_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Neg_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Neg_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Neg_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::C_Neg_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            6u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Pow_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Pow_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Pow_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Pow_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Pow_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Pow_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Pow_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Pow_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Rem_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Rem_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Rem_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Rem_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Rem_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Rem_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Rem_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Rem_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Rem_E_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Rem_E_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Rem_E_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Rem_E_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Rem_E_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Rem_E_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Rem_E_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Rem_E_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Shl_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Shl_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Shl_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Shl_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Shl_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Shl_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Shl_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Shl_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Shr_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Shr_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Shr_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Shr_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Shr_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Shr_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Shr_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Shr_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Sub_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Sub_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Sub_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Sub_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Sub_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Sub_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Sub_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::C_Sub_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 15u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Sub_U_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 15u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Sub_U_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 15u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Sub_U_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            6u32 if funct_4 == 15u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::C_Sub_U_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::O_Abs_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            7u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::O_Abs_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            7u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::O_Abs_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            7u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::O_Abs_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            7u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Add_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Add_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Add_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Add_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Add_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Add_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Add_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Add_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Add_U_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Add_U_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Add_U_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Add_U_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Add_S_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Add_S_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Add_S_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Add_S_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Div_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Div_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Div_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Div_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Div_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Div_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Div_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Div_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Div_E_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Div_E_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Div_E_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Div_E_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Div_E_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Div_E_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Div_E_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Div_E_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Mul_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Mul_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Mul_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Mul_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Mul_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Mul_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Mul_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Mul_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::O_Neg_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            7u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::O_Neg_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            7u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::O_Neg_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            7u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::O_Neg_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            7u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Pow_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Pow_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Pow_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Pow_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Pow_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Pow_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Pow_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Pow_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Rem_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Rem_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Rem_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Rem_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Rem_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Rem_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Rem_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Rem_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Rem_E_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Rem_E_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Rem_E_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Rem_E_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Rem_E_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Rem_E_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Rem_E_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Rem_E_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Shl_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Shl_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Shl_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Shl_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Shl_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Shl_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Shl_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 12u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Shl_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Shr_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Shr_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Shr_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Shr_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Shr_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Shr_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Shr_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 13u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Shr_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Sub_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Sub_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Sub_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Sub_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Sub_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Sub_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Sub_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::O_Sub_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 15u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Sub_U_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 15u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Sub_U_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 15u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Sub_U_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            7u32 if funct_4 == 15u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::O_Sub_U_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::S_Abs_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            8u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::S_Abs_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            8u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::S_Abs_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            8u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::S_Abs_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            8u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Add_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Add_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Add_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Add_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Add_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Add_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Add_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Add_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Add_U_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Add_U_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Add_U_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Add_U_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Add_S_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Add_S_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Add_S_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Add_S_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Div_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Div_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Div_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Div_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Div_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Div_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Div_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Div_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Mul_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Mul_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Mul_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Mul_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Mul_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Mul_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Mul_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Mul_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 1u32 => {
                Self::S_Neg_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            8u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 1u32 => {
                Self::S_Neg_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            8u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 1u32 => {
                Self::S_Neg_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            8u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 1u32 => {
                Self::S_Neg_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            8u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Pow_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Pow_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Pow_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Pow_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Pow_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Pow_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Pow_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Pow_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Sub_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Sub_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Sub_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Sub_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Sub_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Sub_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Sub_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::S_Sub_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 15u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Sub_U_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 15u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Sub_U_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 15u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Sub_U_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            8u32 if funct_4 == 15u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::S_Sub_U_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 1u32 && sign == 1u32 => {
                Self::Abs_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            9u32 if funct_4 == 0u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 1u32 && sign == 1u32 => {
                Self::Abs_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            9u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Add_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Add_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Div_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Div_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Div_E_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Div_E_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Log_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Log_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 6u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 1u32 && sign == 1u32 => {
                Self::Sqrt_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            9u32 if funct_4 == 6u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 1u32 && sign == 1u32 => {
                Self::Sqrt_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            9u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Mul_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Mul_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 1u32 && sign == 1u32 => {
                Self::Neg_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            9u32 if funct_4 == 8u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 1u32 && sign == 1u32 => {
                Self::Neg_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            9u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Pow_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 9u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Pow_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Rem_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 10u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Rem_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Rem_E_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 11u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Rem_E_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 13u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 1u32 && sign == 1u32 => {
                Self::Cbrt_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            9u32 if funct_4 == 13u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 1u32 && sign == 1u32 => {
                Self::Cbrt_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            9u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 2u32 && float == 1u32
                && sign == 1u32 => {
                Self::Sub_f32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            9u32 if funct_4 == 14u32 && funct_2 == 0u32 && size == 3u32 && float == 1u32
                && sign == 1u32 => {
                Self::Sub_f64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Add_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Add_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Add_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Add_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Add_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Add_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Add_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 0u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Add_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Sub_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Sub_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Sub_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Sub_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Sub_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Sub_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Sub_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 1u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Sub_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Min_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Min_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Min_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Min_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Min_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Min_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Min_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 2u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Min_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Max_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Max_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Max_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Max_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Max_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Max_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Max_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 3u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Max_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_And_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_And_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_And_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_And_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_And_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_And_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_And_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 4u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_And_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Nand_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Nand_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Nand_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Nand_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Nand_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Nand_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Nand_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 5u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Nand_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Or_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Or_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Or_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Or_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Or_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Or_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Or_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 6u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Or_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Xor_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Xor_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Xor_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Fetch_Xor_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Xor_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Xor_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Xor_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 7u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Fetch_Xor_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 8u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 1u32 => {
                Self::Cmp_Exchg_i8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 8u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 1u32 => {
                Self::Cmp_Exchg_i16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 8u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 1u32 => {
                Self::Cmp_Exchg_i32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 8u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 1u32 => {
                Self::Cmp_Exchg_i64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 8u32 && funct_2 == 0u32 && size == 0u32 && float == 0u32
                && sign == 0u32 => {
                Self::Cmp_Exchg_u8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 8u32 && funct_2 == 0u32 && size == 1u32 && float == 0u32
                && sign == 0u32 => {
                Self::Cmp_Exchg_u16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 8u32 && funct_2 == 0u32 && size == 2u32 && float == 0u32
                && sign == 0u32 => {
                Self::Cmp_Exchg_u32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 8u32 && funct_2 == 0u32 && size == 3u32 && float == 0u32
                && sign == 0u32 => {
                Self::Cmp_Exchg_u64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                    rs2: Register::from_index(rs2 as u8),
                }
            }
            10u32 if funct_4 == 9u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::Atomic_Ld_8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            10u32 if funct_4 == 9u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::Atomic_Ld_16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            10u32 if funct_4 == 9u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::Atomic_Ld_32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            10u32 if funct_4 == 9u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::Atomic_Ld_64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            10u32 if funct_4 == 10u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::Atomic_St_8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            10u32 if funct_4 == 10u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::Atomic_St_16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            10u32 if funct_4 == 10u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::Atomic_St_32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            10u32 if funct_4 == 10u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::Atomic_St_64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            10u32 if funct_4 == 11u32 && sign == 0u32 => {
                Self::Spawn {
                    rd: Register::from_index(rd as u8),
                    imm: imm_u16 as u16,
                }
            }
            10u32 if funct_4 == 12u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::Wait {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            10u32 if funct_4 == 13u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::Notify {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            10u32 if funct_4 == 14u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 0u32
                && float == 0u32 && sign == 0u32 => {
                Self::Swap_8 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            10u32 if funct_4 == 14u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 1u32
                && float == 0u32 && sign == 0u32 => {
                Self::Swap_16 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            10u32 if funct_4 == 14u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 2u32
                && float == 0u32 && sign == 0u32 => {
                Self::Swap_32 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            10u32 if funct_4 == 14u32 && rs2 == 0u32 && funct_2 == 0u32 && size == 3u32
                && float == 0u32 && sign == 0u32 => {
                Self::Swap_64 {
                    rd: Register::from_index(rd as u8),
                    rs1: Register::from_index(rs1 as u8),
                }
            }
            _ => Self::Invalid(instr),
        }
    }
    pub fn encode(&self) -> u32 {
        match self {
            Self::Halt => 0u32,
            Self::Trap => 16u32,
            Self::Call { imm } => 32u32 | (((*imm as u32) << 8u32) & 4294967040u32),
            Self::Ret => 64u32,
            Self::Ecall { imm } => 80u32 | (((*imm as u32) << 8u32) & 4294967040u32),
            Self::Jal { imm } => 96u32 | (((*imm as u32) << 8u32) & 4294967040u32),
            Self::Bie { imm, rs1, rs2 } => {
                1u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bne { imm, rs1, rs2 } => {
                17u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Blts { imm, rs1, rs2 } => {
                33u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bltu { imm, rs1, rs2 } => {
                49u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bles { imm, rs1, rs2 } => {
                65u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bleu { imm, rs1, rs2 } => {
                81u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bgts { imm, rs1, rs2 } => {
                97u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bgtu { imm, rs1, rs2 } => {
                113u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bges { imm, rs1, rs2 } => {
                129u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bgeu { imm, rs1, rs2 } => {
                145u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bie_f32 { imm, rs1, rs2 } => {
                2u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bie_f64 { imm, rs1, rs2 } => {
                18u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bne_f32 { imm, rs1, rs2 } => {
                34u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bne_f64 { imm, rs1, rs2 } => {
                50u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Blt_f32 { imm, rs1, rs2 } => {
                66u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Blt_f64 { imm, rs1, rs2 } => {
                82u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Ble_f32 { imm, rs1, rs2 } => {
                98u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Ble_f64 { imm, rs1, rs2 } => {
                114u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bgt_f32 { imm, rs1, rs2 } => {
                130u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bgt_f64 { imm, rs1, rs2 } => {
                146u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bge_f32 { imm, rs1, rs2 } => {
                162u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Bge_f64 { imm, rs1, rs2 } => {
                178u32 | (((*imm as u32) << 2u32) & 16128u32)
                    | (((*imm as u32) << 26u32) & 4227858432u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cie { rd, rs1, rs2 } => {
                2147483651u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cie_f32 { rd, rs1, rs2 } => {
                3758096387u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cie_f64 { rd, rs1, rs2 } => {
                4026531843u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cne { rd, rs1, rs2 } => {
                2147483667u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cne_f32 { rd, rs1, rs2 } => {
                3758096403u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cne_f64 { rd, rs1, rs2 } => {
                4026531859u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Clts { rd, rs1, rs2 } => {
                2147483683u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cltu { rd, rs1, rs2 } => {
                35u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Clt_f32 { rd, rs1, rs2 } => {
                3758096419u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Clt_f64 { rd, rs1, rs2 } => {
                4026531875u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cles { rd, rs1, rs2 } => {
                2147483699u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cleu { rd, rs1, rs2 } => {
                51u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cle_f32 { rd, rs1, rs2 } => {
                3758096435u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cle_f64 { rd, rs1, rs2 } => {
                4026531891u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cgts { rd, rs1, rs2 } => {
                2147483715u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cgtu { rd, rs1, rs2 } => {
                67u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cgt_f32 { rd, rs1, rs2 } => {
                3758096451u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cgt_f64 { rd, rs1, rs2 } => {
                4026531907u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cges { rd, rs1, rs2 } => {
                2147483731u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cgeu { rd, rs1, rs2 } => {
                83u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cge_f32 { rd, rs1, rs2 } => {
                3758096467u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cge_f64 { rd, rs1, rs2 } => {
                4026531923u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Lra_8 { rd, rs1 } => {
                4u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Lra_16 { rd, rs1 } => {
                268435460u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Lra_32 { rd, rs1 } => {
                536870916u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Lra_64 { rd, rs1 } => {
                805306372u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Lsi { rd, imm } => {
                2147483668u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((*imm as u32) << 15u32) & 2147450880u32)
            }
            Self::Lui { rd, imm } => {
                20u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((*imm as u32) << 15u32) & 2147450880u32)
            }
            Self::Lia_8 { rd, imm } => {
                36u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((*imm as u32) << 15u32) & 2147450880u32)
            }
            Self::Lia_16 { rd, imm } => {
                52u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((*imm as u32) << 15u32) & 2147450880u32)
            }
            Self::Lia_32 { rd, imm } => {
                68u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((*imm as u32) << 15u32) & 2147450880u32)
            }
            Self::Lia_64 { rd, imm } => {
                84u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((*imm as u32) << 15u32) & 2147450880u32)
            }
            Self::Sra_8 { rd, rs1 } => {
                100u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Sra_16 { rd, rs1 } => {
                268435556u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Sra_32 { rd, rs1 } => {
                536871012u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Sra_64 { rd, rs1 } => {
                805306468u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Ssi { rd, imm } => {
                2147483764u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((*imm as u32) << 15u32) & 2147450880u32)
            }
            Self::Sui { rd, imm } => {
                116u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((*imm as u32) << 15u32) & 2147450880u32)
            }
            Self::Mov { rd, rs1 } => {
                132u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Pop { rd } => 148u32 | (((rd.index() as u32) << 8u32) & 16128u32),
            Self::Psh { rd } => {
                2147483796u32 | (((rd.index() as u32) << 8u32) & 16128u32)
            }
            Self::Psi { imm } => 164u32 | (((*imm as u32) << 8u32) & 4294967040u32),
            Self::Pui { imm } => 180u32 | (((*imm as u32) << 8u32) & 4294967040u32),
            Self::And_i8 { rd, rs1, rs2 } => {
                2147483653u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::And_i16 { rd, rs1, rs2 } => {
                2415919109u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::And_i32 { rd, rs1, rs2 } => {
                2684354565u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::And_i64 { rd, rs1, rs2 } => {
                2952790021u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::And_u8 { rd, rs1, rs2 } => {
                5u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::And_u16 { rd, rs1, rs2 } => {
                268435461u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::And_u32 { rd, rs1, rs2 } => {
                536870917u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::And_u64 { rd, rs1, rs2 } => {
                805306373u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Or_i8 { rd, rs1, rs2 } => {
                2147483669u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Or_i16 { rd, rs1, rs2 } => {
                2415919125u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Or_i32 { rd, rs1, rs2 } => {
                2684354581u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Or_i64 { rd, rs1, rs2 } => {
                2952790037u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Or_u8 { rd, rs1, rs2 } => {
                21u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Or_u16 { rd, rs1, rs2 } => {
                268435477u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Or_u32 { rd, rs1, rs2 } => {
                536870933u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Or_u64 { rd, rs1, rs2 } => {
                805306389u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Xor_i8 { rd, rs1, rs2 } => {
                2147483685u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Xor_i16 { rd, rs1, rs2 } => {
                2415919141u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Xor_i32 { rd, rs1, rs2 } => {
                2684354597u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Xor_i64 { rd, rs1, rs2 } => {
                2952790053u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Xor_u8 { rd, rs1, rs2 } => {
                37u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Xor_u16 { rd, rs1, rs2 } => {
                268435493u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Xor_u32 { rd, rs1, rs2 } => {
                536870949u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Xor_u64 { rd, rs1, rs2 } => {
                805306405u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Not_i8 { rd, rs1 } => {
                2147483701u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Not_i16 { rd, rs1 } => {
                2415919157u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Not_i32 { rd, rs1 } => {
                2684354613u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Not_i64 { rd, rs1 } => {
                2952790069u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Not_u8 { rd, rs1 } => {
                53u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Not_u16 { rd, rs1 } => {
                268435509u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Not_u32 { rd, rs1 } => {
                536870965u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Not_u64 { rd, rs1 } => {
                805306421u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Shl_i8 { rd, rs1, rs2 } => {
                2147483717u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shl_i16 { rd, rs1, rs2 } => {
                2415919173u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shl_i32 { rd, rs1, rs2 } => {
                2684354629u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shl_i64 { rd, rs1, rs2 } => {
                2952790085u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shl_u8 { rd, rs1, rs2 } => {
                69u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shl_u16 { rd, rs1, rs2 } => {
                268435525u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shl_u32 { rd, rs1, rs2 } => {
                536870981u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shl_u64 { rd, rs1, rs2 } => {
                805306437u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shr_i8 { rd, rs1, rs2 } => {
                2147483733u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shr_i16 { rd, rs1, rs2 } => {
                2415919189u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shr_i32 { rd, rs1, rs2 } => {
                2684354645u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shr_i64 { rd, rs1, rs2 } => {
                2952790101u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shr_u8 { rd, rs1, rs2 } => {
                85u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shr_u16 { rd, rs1, rs2 } => {
                268435541u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shr_u32 { rd, rs1, rs2 } => {
                536870997u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Shr_u64 { rd, rs1, rs2 } => {
                805306453u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_L_i8 { rd, rs1, rs2 } => {
                2147483749u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_L_i16 { rd, rs1, rs2 } => {
                2415919205u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_L_i32 { rd, rs1, rs2 } => {
                2684354661u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_L_i64 { rd, rs1, rs2 } => {
                2952790117u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_L_u8 { rd, rs1, rs2 } => {
                101u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_L_u16 { rd, rs1, rs2 } => {
                268435557u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_L_u32 { rd, rs1, rs2 } => {
                536871013u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_L_u64 { rd, rs1, rs2 } => {
                805306469u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_R_i8 { rd, rs1, rs2 } => {
                2147483765u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_R_i16 { rd, rs1, rs2 } => {
                2415919221u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_R_i32 { rd, rs1, rs2 } => {
                2684354677u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_R_i64 { rd, rs1, rs2 } => {
                2952790133u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_R_u8 { rd, rs1, rs2 } => {
                117u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_R_u16 { rd, rs1, rs2 } => {
                268435573u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_R_u32 { rd, rs1, rs2 } => {
                536871029u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rot_R_u64 { rd, rs1, rs2 } => {
                805306485u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Ones_i8 { rd, rs1 } => {
                2147483781u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Ones_i16 { rd, rs1 } => {
                2415919237u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Ones_i32 { rd, rs1 } => {
                2684354693u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Ones_i64 { rd, rs1 } => {
                2952790149u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Ones_u8 { rd, rs1 } => {
                133u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Ones_u16 { rd, rs1 } => {
                268435589u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Ones_u32 { rd, rs1 } => {
                536871045u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Ones_u64 { rd, rs1 } => {
                805306501u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Ones_i8 { rd, rs1 } => {
                2147483797u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Ones_i16 { rd, rs1 } => {
                2415919253u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Ones_i32 { rd, rs1 } => {
                2684354709u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Ones_i64 { rd, rs1 } => {
                2952790165u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Ones_u8 { rd, rs1 } => {
                149u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Ones_u16 { rd, rs1 } => {
                268435605u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Ones_u32 { rd, rs1 } => {
                536871061u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Ones_u64 { rd, rs1 } => {
                805306517u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Ones_i8 { rd, rs1 } => {
                2147483813u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Ones_i16 { rd, rs1 } => {
                2415919269u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Ones_i32 { rd, rs1 } => {
                2684354725u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Ones_i64 { rd, rs1 } => {
                2952790181u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Ones_u8 { rd, rs1 } => {
                165u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Ones_u16 { rd, rs1 } => {
                268435621u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Ones_u32 { rd, rs1 } => {
                536871077u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Ones_u64 { rd, rs1 } => {
                805306533u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Zeros_i8 { rd, rs1 } => {
                2147483829u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Zeros_i16 { rd, rs1 } => {
                2415919285u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Zeros_i32 { rd, rs1 } => {
                2684354741u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Zeros_i64 { rd, rs1 } => {
                2952790197u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Zeros_u8 { rd, rs1 } => {
                181u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Zeros_u16 { rd, rs1 } => {
                268435637u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Zeros_u32 { rd, rs1 } => {
                536871093u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Zeros_u64 { rd, rs1 } => {
                805306549u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Zeros_i8 { rd, rs1 } => {
                2147483845u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Zeros_i16 { rd, rs1 } => {
                2415919301u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Zeros_i32 { rd, rs1 } => {
                2684354757u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Zeros_i64 { rd, rs1 } => {
                2952790213u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Zeros_u8 { rd, rs1 } => {
                197u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Zeros_u16 { rd, rs1 } => {
                268435653u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Zeros_u32 { rd, rs1 } => {
                536871109u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::L_Zeros_u64 { rd, rs1 } => {
                805306565u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Zeros_i8 { rd, rs1 } => {
                2147483861u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Zeros_i16 { rd, rs1 } => {
                2415919317u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Zeros_i32 { rd, rs1 } => {
                2684354773u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Zeros_i64 { rd, rs1 } => {
                2952790229u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Zeros_u8 { rd, rs1 } => {
                213u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Zeros_u16 { rd, rs1 } => {
                268435669u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Zeros_u32 { rd, rs1 } => {
                536871125u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::T_Zeros_u64 { rd, rs1 } => {
                805306581u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bytes_i8 { rd, rs1 } => {
                2147483877u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bytes_i16 { rd, rs1 } => {
                2415919333u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bytes_i32 { rd, rs1 } => {
                2684354789u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bytes_i64 { rd, rs1 } => {
                2952790245u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bytes_u8 { rd, rs1 } => {
                229u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bytes_u16 { rd, rs1 } => {
                268435685u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bytes_u32 { rd, rs1 } => {
                536871141u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bytes_u64 { rd, rs1 } => {
                805306597u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bits_i8 { rd, rs1 } => {
                2147483893u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bits_i16 { rd, rs1 } => {
                2415919349u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bits_i32 { rd, rs1 } => {
                2684354805u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bits_i64 { rd, rs1 } => {
                2952790261u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bits_u8 { rd, rs1 } => {
                245u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bits_u16 { rd, rs1 } => {
                268435701u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bits_u32 { rd, rs1 } => {
                536871157u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::R_Bits_u64 { rd, rs1 } => {
                805306613u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Abs_i8 { rd, rs1 } => {
                2147483654u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Abs_i16 { rd, rs1 } => {
                2415919110u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Abs_i32 { rd, rs1 } => {
                2684354566u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Abs_i64 { rd, rs1 } => {
                2952790022u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Add_i8 { rd, rs1, rs2 } => {
                2147483670u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_i16 { rd, rs1, rs2 } => {
                2415919126u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_i32 { rd, rs1, rs2 } => {
                2684354582u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_i64 { rd, rs1, rs2 } => {
                2952790038u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_u8 { rd, rs1, rs2 } => {
                22u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_u16 { rd, rs1, rs2 } => {
                268435478u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_u32 { rd, rs1, rs2 } => {
                536870934u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_u64 { rd, rs1, rs2 } => {
                805306390u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_U_i8 { rd, rs1, rs2 } => {
                2147483686u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_U_i16 { rd, rs1, rs2 } => {
                2415919142u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_U_i32 { rd, rs1, rs2 } => {
                2684354598u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_U_i64 { rd, rs1, rs2 } => {
                2952790054u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_S_u8 { rd, rs1, rs2 } => {
                38u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_S_u16 { rd, rs1, rs2 } => {
                268435494u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_S_u32 { rd, rs1, rs2 } => {
                536870950u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Add_S_u64 { rd, rs1, rs2 } => {
                805306406u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_i8 { rd, rs1, rs2 } => {
                2147483702u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_i16 { rd, rs1, rs2 } => {
                2415919158u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_i32 { rd, rs1, rs2 } => {
                2684354614u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_i64 { rd, rs1, rs2 } => {
                2952790070u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_u8 { rd, rs1, rs2 } => {
                54u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_u16 { rd, rs1, rs2 } => {
                268435510u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_u32 { rd, rs1, rs2 } => {
                536870966u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_u64 { rd, rs1, rs2 } => {
                805306422u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_E_i8 { rd, rs1, rs2 } => {
                2147483718u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_E_i16 { rd, rs1, rs2 } => {
                2415919174u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_E_i32 { rd, rs1, rs2 } => {
                2684354630u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_E_i64 { rd, rs1, rs2 } => {
                2952790086u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_E_u8 { rd, rs1, rs2 } => {
                70u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_E_u16 { rd, rs1, rs2 } => {
                268435526u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_E_u32 { rd, rs1, rs2 } => {
                536870982u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Div_E_u64 { rd, rs1, rs2 } => {
                805306438u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Log_i8 { rd, rs1, rs2 } => {
                2147483734u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Log_i16 { rd, rs1, rs2 } => {
                2415919190u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Log_i32 { rd, rs1, rs2 } => {
                2684354646u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Log_i64 { rd, rs1, rs2 } => {
                2952790102u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Log_u8 { rd, rs1, rs2 } => {
                86u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Log_u16 { rd, rs1, rs2 } => {
                268435542u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Log_u32 { rd, rs1, rs2 } => {
                536870998u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Log_u64 { rd, rs1, rs2 } => {
                805306454u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Sqrt_i8 { rd, rs1 } => {
                2147483750u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Sqrt_i16 { rd, rs1 } => {
                2415919206u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Sqrt_i32 { rd, rs1 } => {
                2684354662u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Sqrt_i64 { rd, rs1 } => {
                2952790118u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Sqrt_u8 { rd, rs1 } => {
                102u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Sqrt_u16 { rd, rs1 } => {
                268435558u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Sqrt_u32 { rd, rs1 } => {
                536871014u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Sqrt_u64 { rd, rs1 } => {
                805306470u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Mul_i8 { rd, rs1, rs2 } => {
                2147483766u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Mul_i16 { rd, rs1, rs2 } => {
                2415919222u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Mul_i32 { rd, rs1, rs2 } => {
                2684354678u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Mul_i64 { rd, rs1, rs2 } => {
                2952790134u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Mul_u8 { rd, rs1, rs2 } => {
                118u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Mul_u16 { rd, rs1, rs2 } => {
                268435574u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Mul_u32 { rd, rs1, rs2 } => {
                536871030u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Mul_u64 { rd, rs1, rs2 } => {
                805306486u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Neg_i8 { rd, rs1 } => {
                2147483782u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Neg_i16 { rd, rs1 } => {
                2415919238u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Neg_i32 { rd, rs1 } => {
                2684354694u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Neg_i64 { rd, rs1 } => {
                2952790150u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::C_Pow_i8 { rd, rs1, rs2 } => {
                2147483798u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Pow_i16 { rd, rs1, rs2 } => {
                2415919254u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Pow_i32 { rd, rs1, rs2 } => {
                2684354710u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Pow_i64 { rd, rs1, rs2 } => {
                2952790166u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Pow_u8 { rd, rs1, rs2 } => {
                150u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Pow_u16 { rd, rs1, rs2 } => {
                268435606u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Pow_u32 { rd, rs1, rs2 } => {
                536871062u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Pow_u64 { rd, rs1, rs2 } => {
                805306518u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_i8 { rd, rs1, rs2 } => {
                2147483814u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_i16 { rd, rs1, rs2 } => {
                2415919270u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_i32 { rd, rs1, rs2 } => {
                2684354726u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_i64 { rd, rs1, rs2 } => {
                2952790182u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_u8 { rd, rs1, rs2 } => {
                166u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_u16 { rd, rs1, rs2 } => {
                268435622u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_u32 { rd, rs1, rs2 } => {
                536871078u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_u64 { rd, rs1, rs2 } => {
                805306534u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_E_i8 { rd, rs1, rs2 } => {
                2147483830u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_E_i16 { rd, rs1, rs2 } => {
                2415919286u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_E_i32 { rd, rs1, rs2 } => {
                2684354742u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_E_i64 { rd, rs1, rs2 } => {
                2952790198u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_E_u8 { rd, rs1, rs2 } => {
                182u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_E_u16 { rd, rs1, rs2 } => {
                268435638u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_E_u32 { rd, rs1, rs2 } => {
                536871094u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Rem_E_u64 { rd, rs1, rs2 } => {
                805306550u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shl_i8 { rd, rs1, rs2 } => {
                2147483846u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shl_i16 { rd, rs1, rs2 } => {
                2415919302u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shl_i32 { rd, rs1, rs2 } => {
                2684354758u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shl_i64 { rd, rs1, rs2 } => {
                2952790214u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shl_u8 { rd, rs1, rs2 } => {
                198u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shl_u16 { rd, rs1, rs2 } => {
                268435654u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shl_u32 { rd, rs1, rs2 } => {
                536871110u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shl_u64 { rd, rs1, rs2 } => {
                805306566u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shr_i8 { rd, rs1, rs2 } => {
                2147483862u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shr_i16 { rd, rs1, rs2 } => {
                2415919318u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shr_i32 { rd, rs1, rs2 } => {
                2684354774u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shr_i64 { rd, rs1, rs2 } => {
                2952790230u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shr_u8 { rd, rs1, rs2 } => {
                214u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shr_u16 { rd, rs1, rs2 } => {
                268435670u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shr_u32 { rd, rs1, rs2 } => {
                536871126u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Shr_u64 { rd, rs1, rs2 } => {
                805306582u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Sub_i8 { rd, rs1, rs2 } => {
                2147483878u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Sub_i16 { rd, rs1, rs2 } => {
                2415919334u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Sub_i32 { rd, rs1, rs2 } => {
                2684354790u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Sub_i64 { rd, rs1, rs2 } => {
                2952790246u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Sub_u8 { rd, rs1, rs2 } => {
                230u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Sub_u16 { rd, rs1, rs2 } => {
                268435686u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Sub_u32 { rd, rs1, rs2 } => {
                536871142u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Sub_u64 { rd, rs1, rs2 } => {
                805306598u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Sub_U_i8 { rd, rs1, rs2 } => {
                2147483894u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Sub_U_i16 { rd, rs1, rs2 } => {
                2415919350u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Sub_U_i32 { rd, rs1, rs2 } => {
                2684354806u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::C_Sub_U_i64 { rd, rs1, rs2 } => {
                2952790262u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Abs_i8 { rd, rs1 } => {
                2147483655u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::O_Abs_i16 { rd, rs1 } => {
                2415919111u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::O_Abs_i32 { rd, rs1 } => {
                2684354567u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::O_Abs_i64 { rd, rs1 } => {
                2952790023u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::O_Add_i8 { rd, rs1, rs2 } => {
                2147483671u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_i16 { rd, rs1, rs2 } => {
                2415919127u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_i32 { rd, rs1, rs2 } => {
                2684354583u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_i64 { rd, rs1, rs2 } => {
                2952790039u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_u8 { rd, rs1, rs2 } => {
                23u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_u16 { rd, rs1, rs2 } => {
                268435479u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_u32 { rd, rs1, rs2 } => {
                536870935u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_u64 { rd, rs1, rs2 } => {
                805306391u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_U_i8 { rd, rs1, rs2 } => {
                2147483687u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_U_i16 { rd, rs1, rs2 } => {
                2415919143u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_U_i32 { rd, rs1, rs2 } => {
                2684354599u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_U_i64 { rd, rs1, rs2 } => {
                2952790055u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_S_u8 { rd, rs1, rs2 } => {
                39u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_S_u16 { rd, rs1, rs2 } => {
                268435495u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_S_u32 { rd, rs1, rs2 } => {
                536870951u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Add_S_u64 { rd, rs1, rs2 } => {
                805306407u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_i8 { rd, rs1, rs2 } => {
                2147483703u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_i16 { rd, rs1, rs2 } => {
                2415919159u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_i32 { rd, rs1, rs2 } => {
                2684354615u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_i64 { rd, rs1, rs2 } => {
                2952790071u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_u8 { rd, rs1, rs2 } => {
                55u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_u16 { rd, rs1, rs2 } => {
                268435511u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_u32 { rd, rs1, rs2 } => {
                536870967u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_u64 { rd, rs1, rs2 } => {
                805306423u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_E_i8 { rd, rs1, rs2 } => {
                2147483719u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_E_i16 { rd, rs1, rs2 } => {
                2415919175u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_E_i32 { rd, rs1, rs2 } => {
                2684354631u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_E_i64 { rd, rs1, rs2 } => {
                2952790087u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_E_u8 { rd, rs1, rs2 } => {
                71u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_E_u16 { rd, rs1, rs2 } => {
                268435527u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_E_u32 { rd, rs1, rs2 } => {
                536870983u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Div_E_u64 { rd, rs1, rs2 } => {
                805306439u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Mul_i8 { rd, rs1, rs2 } => {
                2147483767u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Mul_i16 { rd, rs1, rs2 } => {
                2415919223u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Mul_i32 { rd, rs1, rs2 } => {
                2684354679u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Mul_i64 { rd, rs1, rs2 } => {
                2952790135u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Mul_u8 { rd, rs1, rs2 } => {
                119u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Mul_u16 { rd, rs1, rs2 } => {
                268435575u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Mul_u32 { rd, rs1, rs2 } => {
                536871031u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Mul_u64 { rd, rs1, rs2 } => {
                805306487u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Neg_i8 { rd, rs1 } => {
                2147483783u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::O_Neg_i16 { rd, rs1 } => {
                2415919239u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::O_Neg_i32 { rd, rs1 } => {
                2684354695u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::O_Neg_i64 { rd, rs1 } => {
                2952790151u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::O_Pow_i8 { rd, rs1, rs2 } => {
                2147483799u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Pow_i16 { rd, rs1, rs2 } => {
                2415919255u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Pow_i32 { rd, rs1, rs2 } => {
                2684354711u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Pow_i64 { rd, rs1, rs2 } => {
                2952790167u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Pow_u8 { rd, rs1, rs2 } => {
                151u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Pow_u16 { rd, rs1, rs2 } => {
                268435607u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Pow_u32 { rd, rs1, rs2 } => {
                536871063u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Pow_u64 { rd, rs1, rs2 } => {
                805306519u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_i8 { rd, rs1, rs2 } => {
                2147483815u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_i16 { rd, rs1, rs2 } => {
                2415919271u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_i32 { rd, rs1, rs2 } => {
                2684354727u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_i64 { rd, rs1, rs2 } => {
                2952790183u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_u8 { rd, rs1, rs2 } => {
                167u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_u16 { rd, rs1, rs2 } => {
                268435623u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_u32 { rd, rs1, rs2 } => {
                536871079u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_u64 { rd, rs1, rs2 } => {
                805306535u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_E_i8 { rd, rs1, rs2 } => {
                2147483831u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_E_i16 { rd, rs1, rs2 } => {
                2415919287u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_E_i32 { rd, rs1, rs2 } => {
                2684354743u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_E_i64 { rd, rs1, rs2 } => {
                2952790199u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_E_u8 { rd, rs1, rs2 } => {
                183u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_E_u16 { rd, rs1, rs2 } => {
                268435639u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_E_u32 { rd, rs1, rs2 } => {
                536871095u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Rem_E_u64 { rd, rs1, rs2 } => {
                805306551u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shl_i8 { rd, rs1, rs2 } => {
                2147483847u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shl_i16 { rd, rs1, rs2 } => {
                2415919303u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shl_i32 { rd, rs1, rs2 } => {
                2684354759u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shl_i64 { rd, rs1, rs2 } => {
                2952790215u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shl_u8 { rd, rs1, rs2 } => {
                199u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shl_u16 { rd, rs1, rs2 } => {
                268435655u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shl_u32 { rd, rs1, rs2 } => {
                536871111u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shl_u64 { rd, rs1, rs2 } => {
                805306567u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shr_i8 { rd, rs1, rs2 } => {
                2147483863u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shr_i16 { rd, rs1, rs2 } => {
                2415919319u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shr_i32 { rd, rs1, rs2 } => {
                2684354775u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shr_i64 { rd, rs1, rs2 } => {
                2952790231u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shr_u8 { rd, rs1, rs2 } => {
                215u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shr_u16 { rd, rs1, rs2 } => {
                268435671u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shr_u32 { rd, rs1, rs2 } => {
                536871127u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Shr_u64 { rd, rs1, rs2 } => {
                805306583u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Sub_i8 { rd, rs1, rs2 } => {
                2147483879u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Sub_i16 { rd, rs1, rs2 } => {
                2415919335u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Sub_i32 { rd, rs1, rs2 } => {
                2684354791u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Sub_i64 { rd, rs1, rs2 } => {
                2952790247u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Sub_u8 { rd, rs1, rs2 } => {
                231u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Sub_u16 { rd, rs1, rs2 } => {
                268435687u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Sub_u32 { rd, rs1, rs2 } => {
                536871143u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Sub_u64 { rd, rs1, rs2 } => {
                805306599u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Sub_U_i8 { rd, rs1, rs2 } => {
                2147483895u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Sub_U_i16 { rd, rs1, rs2 } => {
                2415919351u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Sub_U_i32 { rd, rs1, rs2 } => {
                2684354807u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::O_Sub_U_i64 { rd, rs1, rs2 } => {
                2952790263u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Abs_i8 { rd, rs1 } => {
                2147483656u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::S_Abs_i16 { rd, rs1 } => {
                2415919112u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::S_Abs_i32 { rd, rs1 } => {
                2684354568u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::S_Abs_i64 { rd, rs1 } => {
                2952790024u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::S_Add_i8 { rd, rs1, rs2 } => {
                2147483672u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_i16 { rd, rs1, rs2 } => {
                2415919128u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_i32 { rd, rs1, rs2 } => {
                2684354584u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_i64 { rd, rs1, rs2 } => {
                2952790040u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_u8 { rd, rs1, rs2 } => {
                24u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_u16 { rd, rs1, rs2 } => {
                268435480u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_u32 { rd, rs1, rs2 } => {
                536870936u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_u64 { rd, rs1, rs2 } => {
                805306392u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_U_i8 { rd, rs1, rs2 } => {
                2147483688u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_U_i16 { rd, rs1, rs2 } => {
                2415919144u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_U_i32 { rd, rs1, rs2 } => {
                2684354600u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_U_i64 { rd, rs1, rs2 } => {
                2952790056u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_S_u8 { rd, rs1, rs2 } => {
                40u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_S_u16 { rd, rs1, rs2 } => {
                268435496u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_S_u32 { rd, rs1, rs2 } => {
                536870952u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Add_S_u64 { rd, rs1, rs2 } => {
                805306408u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Div_i8 { rd, rs1, rs2 } => {
                2147483704u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Div_i16 { rd, rs1, rs2 } => {
                2415919160u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Div_i32 { rd, rs1, rs2 } => {
                2684354616u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Div_i64 { rd, rs1, rs2 } => {
                2952790072u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Div_u8 { rd, rs1, rs2 } => {
                56u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Div_u16 { rd, rs1, rs2 } => {
                268435512u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Div_u32 { rd, rs1, rs2 } => {
                536870968u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Div_u64 { rd, rs1, rs2 } => {
                805306424u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Mul_i8 { rd, rs1, rs2 } => {
                2147483768u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Mul_i16 { rd, rs1, rs2 } => {
                2415919224u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Mul_i32 { rd, rs1, rs2 } => {
                2684354680u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Mul_i64 { rd, rs1, rs2 } => {
                2952790136u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Mul_u8 { rd, rs1, rs2 } => {
                120u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Mul_u16 { rd, rs1, rs2 } => {
                268435576u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Mul_u32 { rd, rs1, rs2 } => {
                536871032u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Mul_u64 { rd, rs1, rs2 } => {
                805306488u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Neg_i8 { rd, rs1 } => {
                2147483784u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::S_Neg_i16 { rd, rs1 } => {
                2415919240u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::S_Neg_i32 { rd, rs1 } => {
                2684354696u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::S_Neg_i64 { rd, rs1 } => {
                2952790152u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::S_Pow_i8 { rd, rs1, rs2 } => {
                2147483800u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Pow_i16 { rd, rs1, rs2 } => {
                2415919256u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Pow_i32 { rd, rs1, rs2 } => {
                2684354712u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Pow_i64 { rd, rs1, rs2 } => {
                2952790168u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Pow_u8 { rd, rs1, rs2 } => {
                152u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Pow_u16 { rd, rs1, rs2 } => {
                268435608u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Pow_u32 { rd, rs1, rs2 } => {
                536871064u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Pow_u64 { rd, rs1, rs2 } => {
                805306520u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Sub_i8 { rd, rs1, rs2 } => {
                2147483880u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Sub_i16 { rd, rs1, rs2 } => {
                2415919336u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Sub_i32 { rd, rs1, rs2 } => {
                2684354792u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Sub_i64 { rd, rs1, rs2 } => {
                2952790248u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Sub_u8 { rd, rs1, rs2 } => {
                232u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Sub_u16 { rd, rs1, rs2 } => {
                268435688u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Sub_u32 { rd, rs1, rs2 } => {
                536871144u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Sub_u64 { rd, rs1, rs2 } => {
                805306600u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Sub_U_i8 { rd, rs1, rs2 } => {
                2147483896u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Sub_U_i16 { rd, rs1, rs2 } => {
                2415919352u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Sub_U_i32 { rd, rs1, rs2 } => {
                2684354808u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::S_Sub_U_i64 { rd, rs1, rs2 } => {
                2952790264u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Abs_f32 { rd, rs1 } => {
                3758096393u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Abs_f64 { rd, rs1 } => {
                4026531849u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Add_f32 { rd, rs1, rs2 } => {
                3758096409u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Add_f64 { rd, rs1, rs2 } => {
                4026531865u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Div_f32 { rd, rs1, rs2 } => {
                3758096441u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Div_f64 { rd, rs1, rs2 } => {
                4026531897u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Div_E_f32 { rd, rs1, rs2 } => {
                3758096457u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Div_E_f64 { rd, rs1, rs2 } => {
                4026531913u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Log_f32 { rd, rs1, rs2 } => {
                3758096473u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Log_f64 { rd, rs1, rs2 } => {
                4026531929u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Sqrt_f32 { rd, rs1 } => {
                3758096489u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Sqrt_f64 { rd, rs1 } => {
                4026531945u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Mul_f32 { rd, rs1, rs2 } => {
                3758096505u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Mul_f64 { rd, rs1, rs2 } => {
                4026531961u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Neg_f32 { rd, rs1 } => {
                3758096521u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Neg_f64 { rd, rs1 } => {
                4026531977u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Pow_f32 { rd, rs1, rs2 } => {
                3758096537u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Pow_f64 { rd, rs1, rs2 } => {
                4026531993u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rem_f32 { rd, rs1, rs2 } => {
                3758096553u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rem_f64 { rd, rs1, rs2 } => {
                4026532009u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rem_E_f32 { rd, rs1, rs2 } => {
                3758096569u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Rem_E_f64 { rd, rs1, rs2 } => {
                4026532025u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cbrt_f32 { rd, rs1 } => {
                3758096601u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Cbrt_f64 { rd, rs1 } => {
                4026532057u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Sub_f32 { rd, rs1, rs2 } => {
                3758096617u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Sub_f64 { rd, rs1, rs2 } => {
                4026532073u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Add_i8 { rd, rs1, rs2 } => {
                2147483658u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Add_i16 { rd, rs1, rs2 } => {
                2415919114u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Add_i32 { rd, rs1, rs2 } => {
                2684354570u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Add_i64 { rd, rs1, rs2 } => {
                2952790026u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Add_u8 { rd, rs1, rs2 } => {
                10u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Add_u16 { rd, rs1, rs2 } => {
                268435466u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Add_u32 { rd, rs1, rs2 } => {
                536870922u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Add_u64 { rd, rs1, rs2 } => {
                805306378u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Sub_i8 { rd, rs1, rs2 } => {
                2147483674u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Sub_i16 { rd, rs1, rs2 } => {
                2415919130u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Sub_i32 { rd, rs1, rs2 } => {
                2684354586u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Sub_i64 { rd, rs1, rs2 } => {
                2952790042u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Sub_u8 { rd, rs1, rs2 } => {
                26u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Sub_u16 { rd, rs1, rs2 } => {
                268435482u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Sub_u32 { rd, rs1, rs2 } => {
                536870938u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Sub_u64 { rd, rs1, rs2 } => {
                805306394u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Min_i8 { rd, rs1, rs2 } => {
                2147483690u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Min_i16 { rd, rs1, rs2 } => {
                2415919146u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Min_i32 { rd, rs1, rs2 } => {
                2684354602u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Min_i64 { rd, rs1, rs2 } => {
                2952790058u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Min_u8 { rd, rs1, rs2 } => {
                42u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Min_u16 { rd, rs1, rs2 } => {
                268435498u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Min_u32 { rd, rs1, rs2 } => {
                536870954u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Min_u64 { rd, rs1, rs2 } => {
                805306410u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Max_i8 { rd, rs1, rs2 } => {
                2147483706u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Max_i16 { rd, rs1, rs2 } => {
                2415919162u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Max_i32 { rd, rs1, rs2 } => {
                2684354618u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Max_i64 { rd, rs1, rs2 } => {
                2952790074u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Max_u8 { rd, rs1, rs2 } => {
                58u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Max_u16 { rd, rs1, rs2 } => {
                268435514u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Max_u32 { rd, rs1, rs2 } => {
                536870970u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Max_u64 { rd, rs1, rs2 } => {
                805306426u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_And_i8 { rd, rs1, rs2 } => {
                2147483722u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_And_i16 { rd, rs1, rs2 } => {
                2415919178u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_And_i32 { rd, rs1, rs2 } => {
                2684354634u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_And_i64 { rd, rs1, rs2 } => {
                2952790090u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_And_u8 { rd, rs1, rs2 } => {
                74u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_And_u16 { rd, rs1, rs2 } => {
                268435530u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_And_u32 { rd, rs1, rs2 } => {
                536870986u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_And_u64 { rd, rs1, rs2 } => {
                805306442u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Nand_i8 { rd, rs1, rs2 } => {
                2147483738u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Nand_i16 { rd, rs1, rs2 } => {
                2415919194u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Nand_i32 { rd, rs1, rs2 } => {
                2684354650u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Nand_i64 { rd, rs1, rs2 } => {
                2952790106u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Nand_u8 { rd, rs1, rs2 } => {
                90u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Nand_u16 { rd, rs1, rs2 } => {
                268435546u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Nand_u32 { rd, rs1, rs2 } => {
                536871002u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Nand_u64 { rd, rs1, rs2 } => {
                805306458u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Or_i8 { rd, rs1, rs2 } => {
                2147483754u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Or_i16 { rd, rs1, rs2 } => {
                2415919210u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Or_i32 { rd, rs1, rs2 } => {
                2684354666u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Or_i64 { rd, rs1, rs2 } => {
                2952790122u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Or_u8 { rd, rs1, rs2 } => {
                106u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Or_u16 { rd, rs1, rs2 } => {
                268435562u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Or_u32 { rd, rs1, rs2 } => {
                536871018u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Or_u64 { rd, rs1, rs2 } => {
                805306474u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Xor_i8 { rd, rs1, rs2 } => {
                2147483770u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Xor_i16 { rd, rs1, rs2 } => {
                2415919226u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Xor_i32 { rd, rs1, rs2 } => {
                2684354682u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Xor_i64 { rd, rs1, rs2 } => {
                2952790138u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Xor_u8 { rd, rs1, rs2 } => {
                122u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Xor_u16 { rd, rs1, rs2 } => {
                268435578u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Xor_u32 { rd, rs1, rs2 } => {
                536871034u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Fetch_Xor_u64 { rd, rs1, rs2 } => {
                805306490u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cmp_Exchg_i8 { rd, rs1, rs2 } => {
                2147483786u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cmp_Exchg_i16 { rd, rs1, rs2 } => {
                2415919242u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cmp_Exchg_i32 { rd, rs1, rs2 } => {
                2684354698u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cmp_Exchg_i64 { rd, rs1, rs2 } => {
                2952790154u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cmp_Exchg_u8 { rd, rs1, rs2 } => {
                138u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cmp_Exchg_u16 { rd, rs1, rs2 } => {
                268435594u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cmp_Exchg_u32 { rd, rs1, rs2 } => {
                536871050u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Cmp_Exchg_u64 { rd, rs1, rs2 } => {
                805306506u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
                    | (((rs2.index() as u32) << 20u32) & 66060288u32)
            }
            Self::Atomic_Ld_8 { rd, rs1 } => {
                154u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Atomic_Ld_16 { rd, rs1 } => {
                268435610u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Atomic_Ld_32 { rd, rs1 } => {
                536871066u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Atomic_Ld_64 { rd, rs1 } => {
                805306522u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Atomic_St_8 { rd, rs1 } => {
                170u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Atomic_St_16 { rd, rs1 } => {
                268435626u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Atomic_St_32 { rd, rs1 } => {
                536871082u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Atomic_St_64 { rd, rs1 } => {
                805306538u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Spawn { rd, imm } => {
                186u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((*imm as u32) << 15u32) & 2147450880u32)
            }
            Self::Wait { rd, rs1 } => {
                202u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Notify { rd, rs1 } => {
                218u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Swap_8 { rd, rs1 } => {
                234u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Swap_16 { rd, rs1 } => {
                268435690u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Swap_32 { rd, rs1 } => {
                536871146u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
            }
            Self::Swap_64 { rd, rs1 } => {
                805306602u32 | (((rd.index() as u32) << 8u32) & 16128u32)
                    | (((rs1.index() as u32) << 14u32) & 1032192u32)
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
    fn Halt() {
        let instr = Instruction::decode(0u32);
        println!("{:032b}", 0u32);
        assert_eq!(instr, Instruction::Halt);
        assert_eq!(instr.encode(), 0u32);
    }
    #[test]
    fn Trap() {
        let instr = Instruction::decode(16u32);
        println!("{:032b}", 16u32);
        assert_eq!(instr, Instruction::Trap);
        assert_eq!(instr.encode(), 16u32);
    }
    #[test]
    fn Call() {
        let instr = Instruction::decode(2592u32);
        println!("{:032b}", 2592u32);
        assert_eq!(instr, Instruction::Call { imm : 10, });
        assert_eq!(instr.encode(), 2592u32);
    }
    #[test]
    fn Ret() {
        let instr = Instruction::decode(64u32);
        println!("{:032b}", 64u32);
        assert_eq!(instr, Instruction::Ret);
        assert_eq!(instr.encode(), 64u32);
    }
    #[test]
    fn Ecall() {
        let instr = Instruction::decode(2640u32);
        println!("{:032b}", 2640u32);
        assert_eq!(instr, Instruction::Ecall { imm : 10, });
        assert_eq!(instr.encode(), 2640u32);
    }
    #[test]
    fn Jal() {
        let instr = Instruction::decode(2656u32);
        println!("{:032b}", 2656u32);
        assert_eq!(instr, Instruction::Jal { imm : 10, });
        assert_eq!(instr.encode(), 2656u32);
    }
    #[test]
    fn Bie() {
        let instr = Instruction::decode(681738241u32);
        println!("{:032b}", 681738241u32);
        assert_eq!(
            instr, Instruction::Bie { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738241u32);
    }
    #[test]
    fn Bne() {
        let instr = Instruction::decode(681738257u32);
        println!("{:032b}", 681738257u32);
        assert_eq!(
            instr, Instruction::Bne { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738257u32);
    }
    #[test]
    fn Blts() {
        let instr = Instruction::decode(681738273u32);
        println!("{:032b}", 681738273u32);
        assert_eq!(
            instr, Instruction::Blts { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738273u32);
    }
    #[test]
    fn Bltu() {
        let instr = Instruction::decode(681738289u32);
        println!("{:032b}", 681738289u32);
        assert_eq!(
            instr, Instruction::Bltu { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738289u32);
    }
    #[test]
    fn Bles() {
        let instr = Instruction::decode(681738305u32);
        println!("{:032b}", 681738305u32);
        assert_eq!(
            instr, Instruction::Bles { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738305u32);
    }
    #[test]
    fn Bleu() {
        let instr = Instruction::decode(681738321u32);
        println!("{:032b}", 681738321u32);
        assert_eq!(
            instr, Instruction::Bleu { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738321u32);
    }
    #[test]
    fn Bgts() {
        let instr = Instruction::decode(681738337u32);
        println!("{:032b}", 681738337u32);
        assert_eq!(
            instr, Instruction::Bgts { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738337u32);
    }
    #[test]
    fn Bgtu() {
        let instr = Instruction::decode(681738353u32);
        println!("{:032b}", 681738353u32);
        assert_eq!(
            instr, Instruction::Bgtu { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738353u32);
    }
    #[test]
    fn Bges() {
        let instr = Instruction::decode(681738369u32);
        println!("{:032b}", 681738369u32);
        assert_eq!(
            instr, Instruction::Bges { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738369u32);
    }
    #[test]
    fn Bgeu() {
        let instr = Instruction::decode(681738385u32);
        println!("{:032b}", 681738385u32);
        assert_eq!(
            instr, Instruction::Bgeu { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738385u32);
    }
    #[test]
    fn Bie_f32() {
        let instr = Instruction::decode(681738242u32);
        println!("{:032b}", 681738242u32);
        assert_eq!(
            instr, Instruction::Bie_f32 { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738242u32);
    }
    #[test]
    fn Bie_f64() {
        let instr = Instruction::decode(681738258u32);
        println!("{:032b}", 681738258u32);
        assert_eq!(
            instr, Instruction::Bie_f64 { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738258u32);
    }
    #[test]
    fn Bne_f32() {
        let instr = Instruction::decode(681738274u32);
        println!("{:032b}", 681738274u32);
        assert_eq!(
            instr, Instruction::Bne_f32 { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738274u32);
    }
    #[test]
    fn Bne_f64() {
        let instr = Instruction::decode(681738290u32);
        println!("{:032b}", 681738290u32);
        assert_eq!(
            instr, Instruction::Bne_f64 { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738290u32);
    }
    #[test]
    fn Blt_f32() {
        let instr = Instruction::decode(681738306u32);
        println!("{:032b}", 681738306u32);
        assert_eq!(
            instr, Instruction::Blt_f32 { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738306u32);
    }
    #[test]
    fn Blt_f64() {
        let instr = Instruction::decode(681738322u32);
        println!("{:032b}", 681738322u32);
        assert_eq!(
            instr, Instruction::Blt_f64 { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738322u32);
    }
    #[test]
    fn Ble_f32() {
        let instr = Instruction::decode(681738338u32);
        println!("{:032b}", 681738338u32);
        assert_eq!(
            instr, Instruction::Ble_f32 { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738338u32);
    }
    #[test]
    fn Ble_f64() {
        let instr = Instruction::decode(681738354u32);
        println!("{:032b}", 681738354u32);
        assert_eq!(
            instr, Instruction::Ble_f64 { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738354u32);
    }
    #[test]
    fn Bgt_f32() {
        let instr = Instruction::decode(681738370u32);
        println!("{:032b}", 681738370u32);
        assert_eq!(
            instr, Instruction::Bgt_f32 { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738370u32);
    }
    #[test]
    fn Bgt_f64() {
        let instr = Instruction::decode(681738386u32);
        println!("{:032b}", 681738386u32);
        assert_eq!(
            instr, Instruction::Bgt_f64 { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738386u32);
    }
    #[test]
    fn Bge_f32() {
        let instr = Instruction::decode(681738402u32);
        println!("{:032b}", 681738402u32);
        assert_eq!(
            instr, Instruction::Bge_f32 { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738402u32);
    }
    #[test]
    fn Bge_f64() {
        let instr = Instruction::decode(681738418u32);
        println!("{:032b}", 681738418u32);
        assert_eq!(
            instr, Instruction::Bge_f64 { imm : 10, rs1 : Register::from_index(10), rs2 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 681738418u32);
    }
    #[test]
    fn Cie() {
        let instr = Instruction::decode(2158135811u32);
        println!("{:032b}", 2158135811u32);
        assert_eq!(
            instr, Instruction::Cie { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135811u32);
    }
    #[test]
    fn Cie_f32() {
        let instr = Instruction::decode(3768748547u32);
        println!("{:032b}", 3768748547u32);
        assert_eq!(
            instr, Instruction::Cie_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748547u32);
    }
    #[test]
    fn Cie_f64() {
        let instr = Instruction::decode(4037184003u32);
        println!("{:032b}", 4037184003u32);
        assert_eq!(
            instr, Instruction::Cie_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184003u32);
    }
    #[test]
    fn Cne() {
        let instr = Instruction::decode(2158135827u32);
        println!("{:032b}", 2158135827u32);
        assert_eq!(
            instr, Instruction::Cne { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135827u32);
    }
    #[test]
    fn Cne_f32() {
        let instr = Instruction::decode(3768748563u32);
        println!("{:032b}", 3768748563u32);
        assert_eq!(
            instr, Instruction::Cne_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748563u32);
    }
    #[test]
    fn Cne_f64() {
        let instr = Instruction::decode(4037184019u32);
        println!("{:032b}", 4037184019u32);
        assert_eq!(
            instr, Instruction::Cne_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184019u32);
    }
    #[test]
    fn Clts() {
        let instr = Instruction::decode(2158135843u32);
        println!("{:032b}", 2158135843u32);
        assert_eq!(
            instr, Instruction::Clts { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135843u32);
    }
    #[test]
    fn Cltu() {
        let instr = Instruction::decode(10652195u32);
        println!("{:032b}", 10652195u32);
        assert_eq!(
            instr, Instruction::Cltu { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652195u32);
    }
    #[test]
    fn Clt_f32() {
        let instr = Instruction::decode(3768748579u32);
        println!("{:032b}", 3768748579u32);
        assert_eq!(
            instr, Instruction::Clt_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748579u32);
    }
    #[test]
    fn Clt_f64() {
        let instr = Instruction::decode(4037184035u32);
        println!("{:032b}", 4037184035u32);
        assert_eq!(
            instr, Instruction::Clt_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184035u32);
    }
    #[test]
    fn Cles() {
        let instr = Instruction::decode(2158135859u32);
        println!("{:032b}", 2158135859u32);
        assert_eq!(
            instr, Instruction::Cles { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135859u32);
    }
    #[test]
    fn Cleu() {
        let instr = Instruction::decode(10652211u32);
        println!("{:032b}", 10652211u32);
        assert_eq!(
            instr, Instruction::Cleu { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652211u32);
    }
    #[test]
    fn Cle_f32() {
        let instr = Instruction::decode(3768748595u32);
        println!("{:032b}", 3768748595u32);
        assert_eq!(
            instr, Instruction::Cle_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748595u32);
    }
    #[test]
    fn Cle_f64() {
        let instr = Instruction::decode(4037184051u32);
        println!("{:032b}", 4037184051u32);
        assert_eq!(
            instr, Instruction::Cle_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184051u32);
    }
    #[test]
    fn Cgts() {
        let instr = Instruction::decode(2158135875u32);
        println!("{:032b}", 2158135875u32);
        assert_eq!(
            instr, Instruction::Cgts { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135875u32);
    }
    #[test]
    fn Cgtu() {
        let instr = Instruction::decode(10652227u32);
        println!("{:032b}", 10652227u32);
        assert_eq!(
            instr, Instruction::Cgtu { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652227u32);
    }
    #[test]
    fn Cgt_f32() {
        let instr = Instruction::decode(3768748611u32);
        println!("{:032b}", 3768748611u32);
        assert_eq!(
            instr, Instruction::Cgt_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748611u32);
    }
    #[test]
    fn Cgt_f64() {
        let instr = Instruction::decode(4037184067u32);
        println!("{:032b}", 4037184067u32);
        assert_eq!(
            instr, Instruction::Cgt_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184067u32);
    }
    #[test]
    fn Cges() {
        let instr = Instruction::decode(2158135891u32);
        println!("{:032b}", 2158135891u32);
        assert_eq!(
            instr, Instruction::Cges { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135891u32);
    }
    #[test]
    fn Cgeu() {
        let instr = Instruction::decode(10652243u32);
        println!("{:032b}", 10652243u32);
        assert_eq!(
            instr, Instruction::Cgeu { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652243u32);
    }
    #[test]
    fn Cge_f32() {
        let instr = Instruction::decode(3768748627u32);
        println!("{:032b}", 3768748627u32);
        assert_eq!(
            instr, Instruction::Cge_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748627u32);
    }
    #[test]
    fn Cge_f64() {
        let instr = Instruction::decode(4037184083u32);
        println!("{:032b}", 4037184083u32);
        assert_eq!(
            instr, Instruction::Cge_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184083u32);
    }
    #[test]
    fn Lra_8() {
        let instr = Instruction::decode(166404u32);
        println!("{:032b}", 166404u32);
        assert_eq!(
            instr, Instruction::Lra_8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166404u32);
    }
    #[test]
    fn Lra_16() {
        let instr = Instruction::decode(268601860u32);
        println!("{:032b}", 268601860u32);
        assert_eq!(
            instr, Instruction::Lra_16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268601860u32);
    }
    #[test]
    fn Lra_32() {
        let instr = Instruction::decode(537037316u32);
        println!("{:032b}", 537037316u32);
        assert_eq!(
            instr, Instruction::Lra_32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037316u32);
    }
    #[test]
    fn Lra_64() {
        let instr = Instruction::decode(805472772u32);
        println!("{:032b}", 805472772u32);
        assert_eq!(
            instr, Instruction::Lra_64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805472772u32);
    }
    #[test]
    fn Lsi() {
        let instr = Instruction::decode(2147813908u32);
        println!("{:032b}", 2147813908u32);
        assert_eq!(instr, Instruction::Lsi { rd : Register::from_index(10), imm : 10, });
        assert_eq!(instr.encode(), 2147813908u32);
    }
    #[test]
    fn Lui() {
        let instr = Instruction::decode(330260u32);
        println!("{:032b}", 330260u32);
        assert_eq!(instr, Instruction::Lui { rd : Register::from_index(10), imm : 10, });
        assert_eq!(instr.encode(), 330260u32);
    }
    #[test]
    fn Lia_8() {
        let instr = Instruction::decode(330276u32);
        println!("{:032b}", 330276u32);
        assert_eq!(
            instr, Instruction::Lia_8 { rd : Register::from_index(10), imm : 10, }
        );
        assert_eq!(instr.encode(), 330276u32);
    }
    #[test]
    fn Lia_16() {
        let instr = Instruction::decode(330292u32);
        println!("{:032b}", 330292u32);
        assert_eq!(
            instr, Instruction::Lia_16 { rd : Register::from_index(10), imm : 10, }
        );
        assert_eq!(instr.encode(), 330292u32);
    }
    #[test]
    fn Lia_32() {
        let instr = Instruction::decode(330308u32);
        println!("{:032b}", 330308u32);
        assert_eq!(
            instr, Instruction::Lia_32 { rd : Register::from_index(10), imm : 10, }
        );
        assert_eq!(instr.encode(), 330308u32);
    }
    #[test]
    fn Lia_64() {
        let instr = Instruction::decode(330324u32);
        println!("{:032b}", 330324u32);
        assert_eq!(
            instr, Instruction::Lia_64 { rd : Register::from_index(10), imm : 10, }
        );
        assert_eq!(instr.encode(), 330324u32);
    }
    #[test]
    fn Sra_8() {
        let instr = Instruction::decode(166500u32);
        println!("{:032b}", 166500u32);
        assert_eq!(
            instr, Instruction::Sra_8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166500u32);
    }
    #[test]
    fn Sra_16() {
        let instr = Instruction::decode(268601956u32);
        println!("{:032b}", 268601956u32);
        assert_eq!(
            instr, Instruction::Sra_16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268601956u32);
    }
    #[test]
    fn Sra_32() {
        let instr = Instruction::decode(537037412u32);
        println!("{:032b}", 537037412u32);
        assert_eq!(
            instr, Instruction::Sra_32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037412u32);
    }
    #[test]
    fn Sra_64() {
        let instr = Instruction::decode(805472868u32);
        println!("{:032b}", 805472868u32);
        assert_eq!(
            instr, Instruction::Sra_64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805472868u32);
    }
    #[test]
    fn Ssi() {
        let instr = Instruction::decode(2147814004u32);
        println!("{:032b}", 2147814004u32);
        assert_eq!(instr, Instruction::Ssi { rd : Register::from_index(10), imm : 10, });
        assert_eq!(instr.encode(), 2147814004u32);
    }
    #[test]
    fn Sui() {
        let instr = Instruction::decode(330356u32);
        println!("{:032b}", 330356u32);
        assert_eq!(instr, Instruction::Sui { rd : Register::from_index(10), imm : 10, });
        assert_eq!(instr.encode(), 330356u32);
    }
    #[test]
    fn Mov() {
        let instr = Instruction::decode(166532u32);
        println!("{:032b}", 166532u32);
        assert_eq!(
            instr, Instruction::Mov { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166532u32);
    }
    #[test]
    fn Pop() {
        let instr = Instruction::decode(2708u32);
        println!("{:032b}", 2708u32);
        assert_eq!(instr, Instruction::Pop { rd : Register::from_index(10), });
        assert_eq!(instr.encode(), 2708u32);
    }
    #[test]
    fn Psh() {
        let instr = Instruction::decode(2147486356u32);
        println!("{:032b}", 2147486356u32);
        assert_eq!(instr, Instruction::Psh { rd : Register::from_index(10), });
        assert_eq!(instr.encode(), 2147486356u32);
    }
    #[test]
    fn Psi() {
        let instr = Instruction::decode(2724u32);
        println!("{:032b}", 2724u32);
        assert_eq!(instr, Instruction::Psi { imm : 10, });
        assert_eq!(instr.encode(), 2724u32);
    }
    #[test]
    fn Pui() {
        let instr = Instruction::decode(2740u32);
        println!("{:032b}", 2740u32);
        assert_eq!(instr, Instruction::Pui { imm : 10, });
        assert_eq!(instr.encode(), 2740u32);
    }
    #[test]
    fn And_i8() {
        let instr = Instruction::decode(2158135813u32);
        println!("{:032b}", 2158135813u32);
        assert_eq!(
            instr, Instruction::And_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135813u32);
    }
    #[test]
    fn And_i16() {
        let instr = Instruction::decode(2426571269u32);
        println!("{:032b}", 2426571269u32);
        assert_eq!(
            instr, Instruction::And_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571269u32);
    }
    #[test]
    fn And_i32() {
        let instr = Instruction::decode(2695006725u32);
        println!("{:032b}", 2695006725u32);
        assert_eq!(
            instr, Instruction::And_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006725u32);
    }
    #[test]
    fn And_i64() {
        let instr = Instruction::decode(2963442181u32);
        println!("{:032b}", 2963442181u32);
        assert_eq!(
            instr, Instruction::And_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442181u32);
    }
    #[test]
    fn And_u8() {
        let instr = Instruction::decode(10652165u32);
        println!("{:032b}", 10652165u32);
        assert_eq!(
            instr, Instruction::And_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652165u32);
    }
    #[test]
    fn And_u16() {
        let instr = Instruction::decode(279087621u32);
        println!("{:032b}", 279087621u32);
        assert_eq!(
            instr, Instruction::And_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087621u32);
    }
    #[test]
    fn And_u32() {
        let instr = Instruction::decode(547523077u32);
        println!("{:032b}", 547523077u32);
        assert_eq!(
            instr, Instruction::And_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523077u32);
    }
    #[test]
    fn And_u64() {
        let instr = Instruction::decode(815958533u32);
        println!("{:032b}", 815958533u32);
        assert_eq!(
            instr, Instruction::And_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958533u32);
    }
    #[test]
    fn Or_i8() {
        let instr = Instruction::decode(2158135829u32);
        println!("{:032b}", 2158135829u32);
        assert_eq!(
            instr, Instruction::Or_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135829u32);
    }
    #[test]
    fn Or_i16() {
        let instr = Instruction::decode(2426571285u32);
        println!("{:032b}", 2426571285u32);
        assert_eq!(
            instr, Instruction::Or_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571285u32);
    }
    #[test]
    fn Or_i32() {
        let instr = Instruction::decode(2695006741u32);
        println!("{:032b}", 2695006741u32);
        assert_eq!(
            instr, Instruction::Or_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006741u32);
    }
    #[test]
    fn Or_i64() {
        let instr = Instruction::decode(2963442197u32);
        println!("{:032b}", 2963442197u32);
        assert_eq!(
            instr, Instruction::Or_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442197u32);
    }
    #[test]
    fn Or_u8() {
        let instr = Instruction::decode(10652181u32);
        println!("{:032b}", 10652181u32);
        assert_eq!(
            instr, Instruction::Or_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652181u32);
    }
    #[test]
    fn Or_u16() {
        let instr = Instruction::decode(279087637u32);
        println!("{:032b}", 279087637u32);
        assert_eq!(
            instr, Instruction::Or_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087637u32);
    }
    #[test]
    fn Or_u32() {
        let instr = Instruction::decode(547523093u32);
        println!("{:032b}", 547523093u32);
        assert_eq!(
            instr, Instruction::Or_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523093u32);
    }
    #[test]
    fn Or_u64() {
        let instr = Instruction::decode(815958549u32);
        println!("{:032b}", 815958549u32);
        assert_eq!(
            instr, Instruction::Or_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958549u32);
    }
    #[test]
    fn Xor_i8() {
        let instr = Instruction::decode(2158135845u32);
        println!("{:032b}", 2158135845u32);
        assert_eq!(
            instr, Instruction::Xor_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135845u32);
    }
    #[test]
    fn Xor_i16() {
        let instr = Instruction::decode(2426571301u32);
        println!("{:032b}", 2426571301u32);
        assert_eq!(
            instr, Instruction::Xor_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571301u32);
    }
    #[test]
    fn Xor_i32() {
        let instr = Instruction::decode(2695006757u32);
        println!("{:032b}", 2695006757u32);
        assert_eq!(
            instr, Instruction::Xor_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006757u32);
    }
    #[test]
    fn Xor_i64() {
        let instr = Instruction::decode(2963442213u32);
        println!("{:032b}", 2963442213u32);
        assert_eq!(
            instr, Instruction::Xor_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442213u32);
    }
    #[test]
    fn Xor_u8() {
        let instr = Instruction::decode(10652197u32);
        println!("{:032b}", 10652197u32);
        assert_eq!(
            instr, Instruction::Xor_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652197u32);
    }
    #[test]
    fn Xor_u16() {
        let instr = Instruction::decode(279087653u32);
        println!("{:032b}", 279087653u32);
        assert_eq!(
            instr, Instruction::Xor_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087653u32);
    }
    #[test]
    fn Xor_u32() {
        let instr = Instruction::decode(547523109u32);
        println!("{:032b}", 547523109u32);
        assert_eq!(
            instr, Instruction::Xor_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523109u32);
    }
    #[test]
    fn Xor_u64() {
        let instr = Instruction::decode(815958565u32);
        println!("{:032b}", 815958565u32);
        assert_eq!(
            instr, Instruction::Xor_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958565u32);
    }
    #[test]
    fn Not_i8() {
        let instr = Instruction::decode(2147650101u32);
        println!("{:032b}", 2147650101u32);
        assert_eq!(
            instr, Instruction::Not_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650101u32);
    }
    #[test]
    fn Not_i16() {
        let instr = Instruction::decode(2416085557u32);
        println!("{:032b}", 2416085557u32);
        assert_eq!(
            instr, Instruction::Not_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085557u32);
    }
    #[test]
    fn Not_i32() {
        let instr = Instruction::decode(2684521013u32);
        println!("{:032b}", 2684521013u32);
        assert_eq!(
            instr, Instruction::Not_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684521013u32);
    }
    #[test]
    fn Not_i64() {
        let instr = Instruction::decode(2952956469u32);
        println!("{:032b}", 2952956469u32);
        assert_eq!(
            instr, Instruction::Not_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956469u32);
    }
    #[test]
    fn Not_u8() {
        let instr = Instruction::decode(166453u32);
        println!("{:032b}", 166453u32);
        assert_eq!(
            instr, Instruction::Not_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166453u32);
    }
    #[test]
    fn Not_u16() {
        let instr = Instruction::decode(268601909u32);
        println!("{:032b}", 268601909u32);
        assert_eq!(
            instr, Instruction::Not_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268601909u32);
    }
    #[test]
    fn Not_u32() {
        let instr = Instruction::decode(537037365u32);
        println!("{:032b}", 537037365u32);
        assert_eq!(
            instr, Instruction::Not_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037365u32);
    }
    #[test]
    fn Not_u64() {
        let instr = Instruction::decode(805472821u32);
        println!("{:032b}", 805472821u32);
        assert_eq!(
            instr, Instruction::Not_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805472821u32);
    }
    #[test]
    fn Shl_i8() {
        let instr = Instruction::decode(2158135877u32);
        println!("{:032b}", 2158135877u32);
        assert_eq!(
            instr, Instruction::Shl_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135877u32);
    }
    #[test]
    fn Shl_i16() {
        let instr = Instruction::decode(2426571333u32);
        println!("{:032b}", 2426571333u32);
        assert_eq!(
            instr, Instruction::Shl_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571333u32);
    }
    #[test]
    fn Shl_i32() {
        let instr = Instruction::decode(2695006789u32);
        println!("{:032b}", 2695006789u32);
        assert_eq!(
            instr, Instruction::Shl_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006789u32);
    }
    #[test]
    fn Shl_i64() {
        let instr = Instruction::decode(2963442245u32);
        println!("{:032b}", 2963442245u32);
        assert_eq!(
            instr, Instruction::Shl_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442245u32);
    }
    #[test]
    fn Shl_u8() {
        let instr = Instruction::decode(10652229u32);
        println!("{:032b}", 10652229u32);
        assert_eq!(
            instr, Instruction::Shl_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652229u32);
    }
    #[test]
    fn Shl_u16() {
        let instr = Instruction::decode(279087685u32);
        println!("{:032b}", 279087685u32);
        assert_eq!(
            instr, Instruction::Shl_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087685u32);
    }
    #[test]
    fn Shl_u32() {
        let instr = Instruction::decode(547523141u32);
        println!("{:032b}", 547523141u32);
        assert_eq!(
            instr, Instruction::Shl_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523141u32);
    }
    #[test]
    fn Shl_u64() {
        let instr = Instruction::decode(815958597u32);
        println!("{:032b}", 815958597u32);
        assert_eq!(
            instr, Instruction::Shl_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958597u32);
    }
    #[test]
    fn Shr_i8() {
        let instr = Instruction::decode(2158135893u32);
        println!("{:032b}", 2158135893u32);
        assert_eq!(
            instr, Instruction::Shr_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135893u32);
    }
    #[test]
    fn Shr_i16() {
        let instr = Instruction::decode(2426571349u32);
        println!("{:032b}", 2426571349u32);
        assert_eq!(
            instr, Instruction::Shr_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571349u32);
    }
    #[test]
    fn Shr_i32() {
        let instr = Instruction::decode(2695006805u32);
        println!("{:032b}", 2695006805u32);
        assert_eq!(
            instr, Instruction::Shr_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006805u32);
    }
    #[test]
    fn Shr_i64() {
        let instr = Instruction::decode(2963442261u32);
        println!("{:032b}", 2963442261u32);
        assert_eq!(
            instr, Instruction::Shr_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442261u32);
    }
    #[test]
    fn Shr_u8() {
        let instr = Instruction::decode(10652245u32);
        println!("{:032b}", 10652245u32);
        assert_eq!(
            instr, Instruction::Shr_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652245u32);
    }
    #[test]
    fn Shr_u16() {
        let instr = Instruction::decode(279087701u32);
        println!("{:032b}", 279087701u32);
        assert_eq!(
            instr, Instruction::Shr_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087701u32);
    }
    #[test]
    fn Shr_u32() {
        let instr = Instruction::decode(547523157u32);
        println!("{:032b}", 547523157u32);
        assert_eq!(
            instr, Instruction::Shr_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523157u32);
    }
    #[test]
    fn Shr_u64() {
        let instr = Instruction::decode(815958613u32);
        println!("{:032b}", 815958613u32);
        assert_eq!(
            instr, Instruction::Shr_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958613u32);
    }
    #[test]
    fn Rot_L_i8() {
        let instr = Instruction::decode(2158135909u32);
        println!("{:032b}", 2158135909u32);
        assert_eq!(
            instr, Instruction::Rot_L_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135909u32);
    }
    #[test]
    fn Rot_L_i16() {
        let instr = Instruction::decode(2426571365u32);
        println!("{:032b}", 2426571365u32);
        assert_eq!(
            instr, Instruction::Rot_L_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571365u32);
    }
    #[test]
    fn Rot_L_i32() {
        let instr = Instruction::decode(2695006821u32);
        println!("{:032b}", 2695006821u32);
        assert_eq!(
            instr, Instruction::Rot_L_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006821u32);
    }
    #[test]
    fn Rot_L_i64() {
        let instr = Instruction::decode(2963442277u32);
        println!("{:032b}", 2963442277u32);
        assert_eq!(
            instr, Instruction::Rot_L_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442277u32);
    }
    #[test]
    fn Rot_L_u8() {
        let instr = Instruction::decode(10652261u32);
        println!("{:032b}", 10652261u32);
        assert_eq!(
            instr, Instruction::Rot_L_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652261u32);
    }
    #[test]
    fn Rot_L_u16() {
        let instr = Instruction::decode(279087717u32);
        println!("{:032b}", 279087717u32);
        assert_eq!(
            instr, Instruction::Rot_L_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087717u32);
    }
    #[test]
    fn Rot_L_u32() {
        let instr = Instruction::decode(547523173u32);
        println!("{:032b}", 547523173u32);
        assert_eq!(
            instr, Instruction::Rot_L_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523173u32);
    }
    #[test]
    fn Rot_L_u64() {
        let instr = Instruction::decode(815958629u32);
        println!("{:032b}", 815958629u32);
        assert_eq!(
            instr, Instruction::Rot_L_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958629u32);
    }
    #[test]
    fn Rot_R_i8() {
        let instr = Instruction::decode(2158135925u32);
        println!("{:032b}", 2158135925u32);
        assert_eq!(
            instr, Instruction::Rot_R_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135925u32);
    }
    #[test]
    fn Rot_R_i16() {
        let instr = Instruction::decode(2426571381u32);
        println!("{:032b}", 2426571381u32);
        assert_eq!(
            instr, Instruction::Rot_R_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571381u32);
    }
    #[test]
    fn Rot_R_i32() {
        let instr = Instruction::decode(2695006837u32);
        println!("{:032b}", 2695006837u32);
        assert_eq!(
            instr, Instruction::Rot_R_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006837u32);
    }
    #[test]
    fn Rot_R_i64() {
        let instr = Instruction::decode(2963442293u32);
        println!("{:032b}", 2963442293u32);
        assert_eq!(
            instr, Instruction::Rot_R_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442293u32);
    }
    #[test]
    fn Rot_R_u8() {
        let instr = Instruction::decode(10652277u32);
        println!("{:032b}", 10652277u32);
        assert_eq!(
            instr, Instruction::Rot_R_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652277u32);
    }
    #[test]
    fn Rot_R_u16() {
        let instr = Instruction::decode(279087733u32);
        println!("{:032b}", 279087733u32);
        assert_eq!(
            instr, Instruction::Rot_R_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087733u32);
    }
    #[test]
    fn Rot_R_u32() {
        let instr = Instruction::decode(547523189u32);
        println!("{:032b}", 547523189u32);
        assert_eq!(
            instr, Instruction::Rot_R_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523189u32);
    }
    #[test]
    fn Rot_R_u64() {
        let instr = Instruction::decode(815958645u32);
        println!("{:032b}", 815958645u32);
        assert_eq!(
            instr, Instruction::Rot_R_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958645u32);
    }
    #[test]
    fn C_Ones_i8() {
        let instr = Instruction::decode(2147650181u32);
        println!("{:032b}", 2147650181u32);
        assert_eq!(
            instr, Instruction::C_Ones_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650181u32);
    }
    #[test]
    fn C_Ones_i16() {
        let instr = Instruction::decode(2416085637u32);
        println!("{:032b}", 2416085637u32);
        assert_eq!(
            instr, Instruction::C_Ones_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085637u32);
    }
    #[test]
    fn C_Ones_i32() {
        let instr = Instruction::decode(2684521093u32);
        println!("{:032b}", 2684521093u32);
        assert_eq!(
            instr, Instruction::C_Ones_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684521093u32);
    }
    #[test]
    fn C_Ones_i64() {
        let instr = Instruction::decode(2952956549u32);
        println!("{:032b}", 2952956549u32);
        assert_eq!(
            instr, Instruction::C_Ones_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956549u32);
    }
    #[test]
    fn C_Ones_u8() {
        let instr = Instruction::decode(166533u32);
        println!("{:032b}", 166533u32);
        assert_eq!(
            instr, Instruction::C_Ones_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166533u32);
    }
    #[test]
    fn C_Ones_u16() {
        let instr = Instruction::decode(268601989u32);
        println!("{:032b}", 268601989u32);
        assert_eq!(
            instr, Instruction::C_Ones_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268601989u32);
    }
    #[test]
    fn C_Ones_u32() {
        let instr = Instruction::decode(537037445u32);
        println!("{:032b}", 537037445u32);
        assert_eq!(
            instr, Instruction::C_Ones_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037445u32);
    }
    #[test]
    fn C_Ones_u64() {
        let instr = Instruction::decode(805472901u32);
        println!("{:032b}", 805472901u32);
        assert_eq!(
            instr, Instruction::C_Ones_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805472901u32);
    }
    #[test]
    fn L_Ones_i8() {
        let instr = Instruction::decode(2147650197u32);
        println!("{:032b}", 2147650197u32);
        assert_eq!(
            instr, Instruction::L_Ones_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650197u32);
    }
    #[test]
    fn L_Ones_i16() {
        let instr = Instruction::decode(2416085653u32);
        println!("{:032b}", 2416085653u32);
        assert_eq!(
            instr, Instruction::L_Ones_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085653u32);
    }
    #[test]
    fn L_Ones_i32() {
        let instr = Instruction::decode(2684521109u32);
        println!("{:032b}", 2684521109u32);
        assert_eq!(
            instr, Instruction::L_Ones_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684521109u32);
    }
    #[test]
    fn L_Ones_i64() {
        let instr = Instruction::decode(2952956565u32);
        println!("{:032b}", 2952956565u32);
        assert_eq!(
            instr, Instruction::L_Ones_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956565u32);
    }
    #[test]
    fn L_Ones_u8() {
        let instr = Instruction::decode(166549u32);
        println!("{:032b}", 166549u32);
        assert_eq!(
            instr, Instruction::L_Ones_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166549u32);
    }
    #[test]
    fn L_Ones_u16() {
        let instr = Instruction::decode(268602005u32);
        println!("{:032b}", 268602005u32);
        assert_eq!(
            instr, Instruction::L_Ones_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268602005u32);
    }
    #[test]
    fn L_Ones_u32() {
        let instr = Instruction::decode(537037461u32);
        println!("{:032b}", 537037461u32);
        assert_eq!(
            instr, Instruction::L_Ones_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037461u32);
    }
    #[test]
    fn L_Ones_u64() {
        let instr = Instruction::decode(805472917u32);
        println!("{:032b}", 805472917u32);
        assert_eq!(
            instr, Instruction::L_Ones_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805472917u32);
    }
    #[test]
    fn T_Ones_i8() {
        let instr = Instruction::decode(2147650213u32);
        println!("{:032b}", 2147650213u32);
        assert_eq!(
            instr, Instruction::T_Ones_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650213u32);
    }
    #[test]
    fn T_Ones_i16() {
        let instr = Instruction::decode(2416085669u32);
        println!("{:032b}", 2416085669u32);
        assert_eq!(
            instr, Instruction::T_Ones_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085669u32);
    }
    #[test]
    fn T_Ones_i32() {
        let instr = Instruction::decode(2684521125u32);
        println!("{:032b}", 2684521125u32);
        assert_eq!(
            instr, Instruction::T_Ones_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684521125u32);
    }
    #[test]
    fn T_Ones_i64() {
        let instr = Instruction::decode(2952956581u32);
        println!("{:032b}", 2952956581u32);
        assert_eq!(
            instr, Instruction::T_Ones_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956581u32);
    }
    #[test]
    fn T_Ones_u8() {
        let instr = Instruction::decode(166565u32);
        println!("{:032b}", 166565u32);
        assert_eq!(
            instr, Instruction::T_Ones_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166565u32);
    }
    #[test]
    fn T_Ones_u16() {
        let instr = Instruction::decode(268602021u32);
        println!("{:032b}", 268602021u32);
        assert_eq!(
            instr, Instruction::T_Ones_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268602021u32);
    }
    #[test]
    fn T_Ones_u32() {
        let instr = Instruction::decode(537037477u32);
        println!("{:032b}", 537037477u32);
        assert_eq!(
            instr, Instruction::T_Ones_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037477u32);
    }
    #[test]
    fn T_Ones_u64() {
        let instr = Instruction::decode(805472933u32);
        println!("{:032b}", 805472933u32);
        assert_eq!(
            instr, Instruction::T_Ones_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805472933u32);
    }
    #[test]
    fn C_Zeros_i8() {
        let instr = Instruction::decode(2147650229u32);
        println!("{:032b}", 2147650229u32);
        assert_eq!(
            instr, Instruction::C_Zeros_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650229u32);
    }
    #[test]
    fn C_Zeros_i16() {
        let instr = Instruction::decode(2416085685u32);
        println!("{:032b}", 2416085685u32);
        assert_eq!(
            instr, Instruction::C_Zeros_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085685u32);
    }
    #[test]
    fn C_Zeros_i32() {
        let instr = Instruction::decode(2684521141u32);
        println!("{:032b}", 2684521141u32);
        assert_eq!(
            instr, Instruction::C_Zeros_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684521141u32);
    }
    #[test]
    fn C_Zeros_i64() {
        let instr = Instruction::decode(2952956597u32);
        println!("{:032b}", 2952956597u32);
        assert_eq!(
            instr, Instruction::C_Zeros_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956597u32);
    }
    #[test]
    fn C_Zeros_u8() {
        let instr = Instruction::decode(166581u32);
        println!("{:032b}", 166581u32);
        assert_eq!(
            instr, Instruction::C_Zeros_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166581u32);
    }
    #[test]
    fn C_Zeros_u16() {
        let instr = Instruction::decode(268602037u32);
        println!("{:032b}", 268602037u32);
        assert_eq!(
            instr, Instruction::C_Zeros_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268602037u32);
    }
    #[test]
    fn C_Zeros_u32() {
        let instr = Instruction::decode(537037493u32);
        println!("{:032b}", 537037493u32);
        assert_eq!(
            instr, Instruction::C_Zeros_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037493u32);
    }
    #[test]
    fn C_Zeros_u64() {
        let instr = Instruction::decode(805472949u32);
        println!("{:032b}", 805472949u32);
        assert_eq!(
            instr, Instruction::C_Zeros_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805472949u32);
    }
    #[test]
    fn L_Zeros_i8() {
        let instr = Instruction::decode(2147650245u32);
        println!("{:032b}", 2147650245u32);
        assert_eq!(
            instr, Instruction::L_Zeros_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650245u32);
    }
    #[test]
    fn L_Zeros_i16() {
        let instr = Instruction::decode(2416085701u32);
        println!("{:032b}", 2416085701u32);
        assert_eq!(
            instr, Instruction::L_Zeros_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085701u32);
    }
    #[test]
    fn L_Zeros_i32() {
        let instr = Instruction::decode(2684521157u32);
        println!("{:032b}", 2684521157u32);
        assert_eq!(
            instr, Instruction::L_Zeros_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684521157u32);
    }
    #[test]
    fn L_Zeros_i64() {
        let instr = Instruction::decode(2952956613u32);
        println!("{:032b}", 2952956613u32);
        assert_eq!(
            instr, Instruction::L_Zeros_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956613u32);
    }
    #[test]
    fn L_Zeros_u8() {
        let instr = Instruction::decode(166597u32);
        println!("{:032b}", 166597u32);
        assert_eq!(
            instr, Instruction::L_Zeros_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166597u32);
    }
    #[test]
    fn L_Zeros_u16() {
        let instr = Instruction::decode(268602053u32);
        println!("{:032b}", 268602053u32);
        assert_eq!(
            instr, Instruction::L_Zeros_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268602053u32);
    }
    #[test]
    fn L_Zeros_u32() {
        let instr = Instruction::decode(537037509u32);
        println!("{:032b}", 537037509u32);
        assert_eq!(
            instr, Instruction::L_Zeros_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037509u32);
    }
    #[test]
    fn L_Zeros_u64() {
        let instr = Instruction::decode(805472965u32);
        println!("{:032b}", 805472965u32);
        assert_eq!(
            instr, Instruction::L_Zeros_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805472965u32);
    }
    #[test]
    fn T_Zeros_i8() {
        let instr = Instruction::decode(2147650261u32);
        println!("{:032b}", 2147650261u32);
        assert_eq!(
            instr, Instruction::T_Zeros_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650261u32);
    }
    #[test]
    fn T_Zeros_i16() {
        let instr = Instruction::decode(2416085717u32);
        println!("{:032b}", 2416085717u32);
        assert_eq!(
            instr, Instruction::T_Zeros_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085717u32);
    }
    #[test]
    fn T_Zeros_i32() {
        let instr = Instruction::decode(2684521173u32);
        println!("{:032b}", 2684521173u32);
        assert_eq!(
            instr, Instruction::T_Zeros_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684521173u32);
    }
    #[test]
    fn T_Zeros_i64() {
        let instr = Instruction::decode(2952956629u32);
        println!("{:032b}", 2952956629u32);
        assert_eq!(
            instr, Instruction::T_Zeros_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956629u32);
    }
    #[test]
    fn T_Zeros_u8() {
        let instr = Instruction::decode(166613u32);
        println!("{:032b}", 166613u32);
        assert_eq!(
            instr, Instruction::T_Zeros_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166613u32);
    }
    #[test]
    fn T_Zeros_u16() {
        let instr = Instruction::decode(268602069u32);
        println!("{:032b}", 268602069u32);
        assert_eq!(
            instr, Instruction::T_Zeros_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268602069u32);
    }
    #[test]
    fn T_Zeros_u32() {
        let instr = Instruction::decode(537037525u32);
        println!("{:032b}", 537037525u32);
        assert_eq!(
            instr, Instruction::T_Zeros_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037525u32);
    }
    #[test]
    fn T_Zeros_u64() {
        let instr = Instruction::decode(805472981u32);
        println!("{:032b}", 805472981u32);
        assert_eq!(
            instr, Instruction::T_Zeros_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805472981u32);
    }
    #[test]
    fn R_Bytes_i8() {
        let instr = Instruction::decode(2147650277u32);
        println!("{:032b}", 2147650277u32);
        assert_eq!(
            instr, Instruction::R_Bytes_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650277u32);
    }
    #[test]
    fn R_Bytes_i16() {
        let instr = Instruction::decode(2416085733u32);
        println!("{:032b}", 2416085733u32);
        assert_eq!(
            instr, Instruction::R_Bytes_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085733u32);
    }
    #[test]
    fn R_Bytes_i32() {
        let instr = Instruction::decode(2684521189u32);
        println!("{:032b}", 2684521189u32);
        assert_eq!(
            instr, Instruction::R_Bytes_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684521189u32);
    }
    #[test]
    fn R_Bytes_i64() {
        let instr = Instruction::decode(2952956645u32);
        println!("{:032b}", 2952956645u32);
        assert_eq!(
            instr, Instruction::R_Bytes_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956645u32);
    }
    #[test]
    fn R_Bytes_u8() {
        let instr = Instruction::decode(166629u32);
        println!("{:032b}", 166629u32);
        assert_eq!(
            instr, Instruction::R_Bytes_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166629u32);
    }
    #[test]
    fn R_Bytes_u16() {
        let instr = Instruction::decode(268602085u32);
        println!("{:032b}", 268602085u32);
        assert_eq!(
            instr, Instruction::R_Bytes_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268602085u32);
    }
    #[test]
    fn R_Bytes_u32() {
        let instr = Instruction::decode(537037541u32);
        println!("{:032b}", 537037541u32);
        assert_eq!(
            instr, Instruction::R_Bytes_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037541u32);
    }
    #[test]
    fn R_Bytes_u64() {
        let instr = Instruction::decode(805472997u32);
        println!("{:032b}", 805472997u32);
        assert_eq!(
            instr, Instruction::R_Bytes_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805472997u32);
    }
    #[test]
    fn R_Bits_i8() {
        let instr = Instruction::decode(2147650293u32);
        println!("{:032b}", 2147650293u32);
        assert_eq!(
            instr, Instruction::R_Bits_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650293u32);
    }
    #[test]
    fn R_Bits_i16() {
        let instr = Instruction::decode(2416085749u32);
        println!("{:032b}", 2416085749u32);
        assert_eq!(
            instr, Instruction::R_Bits_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085749u32);
    }
    #[test]
    fn R_Bits_i32() {
        let instr = Instruction::decode(2684521205u32);
        println!("{:032b}", 2684521205u32);
        assert_eq!(
            instr, Instruction::R_Bits_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684521205u32);
    }
    #[test]
    fn R_Bits_i64() {
        let instr = Instruction::decode(2952956661u32);
        println!("{:032b}", 2952956661u32);
        assert_eq!(
            instr, Instruction::R_Bits_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956661u32);
    }
    #[test]
    fn R_Bits_u8() {
        let instr = Instruction::decode(166645u32);
        println!("{:032b}", 166645u32);
        assert_eq!(
            instr, Instruction::R_Bits_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166645u32);
    }
    #[test]
    fn R_Bits_u16() {
        let instr = Instruction::decode(268602101u32);
        println!("{:032b}", 268602101u32);
        assert_eq!(
            instr, Instruction::R_Bits_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268602101u32);
    }
    #[test]
    fn R_Bits_u32() {
        let instr = Instruction::decode(537037557u32);
        println!("{:032b}", 537037557u32);
        assert_eq!(
            instr, Instruction::R_Bits_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037557u32);
    }
    #[test]
    fn R_Bits_u64() {
        let instr = Instruction::decode(805473013u32);
        println!("{:032b}", 805473013u32);
        assert_eq!(
            instr, Instruction::R_Bits_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805473013u32);
    }
    #[test]
    fn C_Abs_i8() {
        let instr = Instruction::decode(2147650054u32);
        println!("{:032b}", 2147650054u32);
        assert_eq!(
            instr, Instruction::C_Abs_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650054u32);
    }
    #[test]
    fn C_Abs_i16() {
        let instr = Instruction::decode(2416085510u32);
        println!("{:032b}", 2416085510u32);
        assert_eq!(
            instr, Instruction::C_Abs_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085510u32);
    }
    #[test]
    fn C_Abs_i32() {
        let instr = Instruction::decode(2684520966u32);
        println!("{:032b}", 2684520966u32);
        assert_eq!(
            instr, Instruction::C_Abs_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684520966u32);
    }
    #[test]
    fn C_Abs_i64() {
        let instr = Instruction::decode(2952956422u32);
        println!("{:032b}", 2952956422u32);
        assert_eq!(
            instr, Instruction::C_Abs_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956422u32);
    }
    #[test]
    fn C_Add_i8() {
        let instr = Instruction::decode(2158135830u32);
        println!("{:032b}", 2158135830u32);
        assert_eq!(
            instr, Instruction::C_Add_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135830u32);
    }
    #[test]
    fn C_Add_i16() {
        let instr = Instruction::decode(2426571286u32);
        println!("{:032b}", 2426571286u32);
        assert_eq!(
            instr, Instruction::C_Add_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571286u32);
    }
    #[test]
    fn C_Add_i32() {
        let instr = Instruction::decode(2695006742u32);
        println!("{:032b}", 2695006742u32);
        assert_eq!(
            instr, Instruction::C_Add_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006742u32);
    }
    #[test]
    fn C_Add_i64() {
        let instr = Instruction::decode(2963442198u32);
        println!("{:032b}", 2963442198u32);
        assert_eq!(
            instr, Instruction::C_Add_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442198u32);
    }
    #[test]
    fn C_Add_u8() {
        let instr = Instruction::decode(10652182u32);
        println!("{:032b}", 10652182u32);
        assert_eq!(
            instr, Instruction::C_Add_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652182u32);
    }
    #[test]
    fn C_Add_u16() {
        let instr = Instruction::decode(279087638u32);
        println!("{:032b}", 279087638u32);
        assert_eq!(
            instr, Instruction::C_Add_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087638u32);
    }
    #[test]
    fn C_Add_u32() {
        let instr = Instruction::decode(547523094u32);
        println!("{:032b}", 547523094u32);
        assert_eq!(
            instr, Instruction::C_Add_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523094u32);
    }
    #[test]
    fn C_Add_u64() {
        let instr = Instruction::decode(815958550u32);
        println!("{:032b}", 815958550u32);
        assert_eq!(
            instr, Instruction::C_Add_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958550u32);
    }
    #[test]
    fn C_Add_U_i8() {
        let instr = Instruction::decode(2158135846u32);
        println!("{:032b}", 2158135846u32);
        assert_eq!(
            instr, Instruction::C_Add_U_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135846u32);
    }
    #[test]
    fn C_Add_U_i16() {
        let instr = Instruction::decode(2426571302u32);
        println!("{:032b}", 2426571302u32);
        assert_eq!(
            instr, Instruction::C_Add_U_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571302u32);
    }
    #[test]
    fn C_Add_U_i32() {
        let instr = Instruction::decode(2695006758u32);
        println!("{:032b}", 2695006758u32);
        assert_eq!(
            instr, Instruction::C_Add_U_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006758u32);
    }
    #[test]
    fn C_Add_U_i64() {
        let instr = Instruction::decode(2963442214u32);
        println!("{:032b}", 2963442214u32);
        assert_eq!(
            instr, Instruction::C_Add_U_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442214u32);
    }
    #[test]
    fn C_Add_S_u8() {
        let instr = Instruction::decode(10652198u32);
        println!("{:032b}", 10652198u32);
        assert_eq!(
            instr, Instruction::C_Add_S_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652198u32);
    }
    #[test]
    fn C_Add_S_u16() {
        let instr = Instruction::decode(279087654u32);
        println!("{:032b}", 279087654u32);
        assert_eq!(
            instr, Instruction::C_Add_S_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087654u32);
    }
    #[test]
    fn C_Add_S_u32() {
        let instr = Instruction::decode(547523110u32);
        println!("{:032b}", 547523110u32);
        assert_eq!(
            instr, Instruction::C_Add_S_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523110u32);
    }
    #[test]
    fn C_Add_S_u64() {
        let instr = Instruction::decode(815958566u32);
        println!("{:032b}", 815958566u32);
        assert_eq!(
            instr, Instruction::C_Add_S_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958566u32);
    }
    #[test]
    fn C_Div_i8() {
        let instr = Instruction::decode(2158135862u32);
        println!("{:032b}", 2158135862u32);
        assert_eq!(
            instr, Instruction::C_Div_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135862u32);
    }
    #[test]
    fn C_Div_i16() {
        let instr = Instruction::decode(2426571318u32);
        println!("{:032b}", 2426571318u32);
        assert_eq!(
            instr, Instruction::C_Div_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571318u32);
    }
    #[test]
    fn C_Div_i32() {
        let instr = Instruction::decode(2695006774u32);
        println!("{:032b}", 2695006774u32);
        assert_eq!(
            instr, Instruction::C_Div_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006774u32);
    }
    #[test]
    fn C_Div_i64() {
        let instr = Instruction::decode(2963442230u32);
        println!("{:032b}", 2963442230u32);
        assert_eq!(
            instr, Instruction::C_Div_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442230u32);
    }
    #[test]
    fn C_Div_u8() {
        let instr = Instruction::decode(10652214u32);
        println!("{:032b}", 10652214u32);
        assert_eq!(
            instr, Instruction::C_Div_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652214u32);
    }
    #[test]
    fn C_Div_u16() {
        let instr = Instruction::decode(279087670u32);
        println!("{:032b}", 279087670u32);
        assert_eq!(
            instr, Instruction::C_Div_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087670u32);
    }
    #[test]
    fn C_Div_u32() {
        let instr = Instruction::decode(547523126u32);
        println!("{:032b}", 547523126u32);
        assert_eq!(
            instr, Instruction::C_Div_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523126u32);
    }
    #[test]
    fn C_Div_u64() {
        let instr = Instruction::decode(815958582u32);
        println!("{:032b}", 815958582u32);
        assert_eq!(
            instr, Instruction::C_Div_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958582u32);
    }
    #[test]
    fn C_Div_E_i8() {
        let instr = Instruction::decode(2158135878u32);
        println!("{:032b}", 2158135878u32);
        assert_eq!(
            instr, Instruction::C_Div_E_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135878u32);
    }
    #[test]
    fn C_Div_E_i16() {
        let instr = Instruction::decode(2426571334u32);
        println!("{:032b}", 2426571334u32);
        assert_eq!(
            instr, Instruction::C_Div_E_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571334u32);
    }
    #[test]
    fn C_Div_E_i32() {
        let instr = Instruction::decode(2695006790u32);
        println!("{:032b}", 2695006790u32);
        assert_eq!(
            instr, Instruction::C_Div_E_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006790u32);
    }
    #[test]
    fn C_Div_E_i64() {
        let instr = Instruction::decode(2963442246u32);
        println!("{:032b}", 2963442246u32);
        assert_eq!(
            instr, Instruction::C_Div_E_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442246u32);
    }
    #[test]
    fn C_Div_E_u8() {
        let instr = Instruction::decode(10652230u32);
        println!("{:032b}", 10652230u32);
        assert_eq!(
            instr, Instruction::C_Div_E_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652230u32);
    }
    #[test]
    fn C_Div_E_u16() {
        let instr = Instruction::decode(279087686u32);
        println!("{:032b}", 279087686u32);
        assert_eq!(
            instr, Instruction::C_Div_E_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087686u32);
    }
    #[test]
    fn C_Div_E_u32() {
        let instr = Instruction::decode(547523142u32);
        println!("{:032b}", 547523142u32);
        assert_eq!(
            instr, Instruction::C_Div_E_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523142u32);
    }
    #[test]
    fn C_Div_E_u64() {
        let instr = Instruction::decode(815958598u32);
        println!("{:032b}", 815958598u32);
        assert_eq!(
            instr, Instruction::C_Div_E_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958598u32);
    }
    #[test]
    fn C_Log_i8() {
        let instr = Instruction::decode(2158135894u32);
        println!("{:032b}", 2158135894u32);
        assert_eq!(
            instr, Instruction::C_Log_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135894u32);
    }
    #[test]
    fn C_Log_i16() {
        let instr = Instruction::decode(2426571350u32);
        println!("{:032b}", 2426571350u32);
        assert_eq!(
            instr, Instruction::C_Log_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571350u32);
    }
    #[test]
    fn C_Log_i32() {
        let instr = Instruction::decode(2695006806u32);
        println!("{:032b}", 2695006806u32);
        assert_eq!(
            instr, Instruction::C_Log_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006806u32);
    }
    #[test]
    fn C_Log_i64() {
        let instr = Instruction::decode(2963442262u32);
        println!("{:032b}", 2963442262u32);
        assert_eq!(
            instr, Instruction::C_Log_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442262u32);
    }
    #[test]
    fn C_Log_u8() {
        let instr = Instruction::decode(10652246u32);
        println!("{:032b}", 10652246u32);
        assert_eq!(
            instr, Instruction::C_Log_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652246u32);
    }
    #[test]
    fn C_Log_u16() {
        let instr = Instruction::decode(279087702u32);
        println!("{:032b}", 279087702u32);
        assert_eq!(
            instr, Instruction::C_Log_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087702u32);
    }
    #[test]
    fn C_Log_u32() {
        let instr = Instruction::decode(547523158u32);
        println!("{:032b}", 547523158u32);
        assert_eq!(
            instr, Instruction::C_Log_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523158u32);
    }
    #[test]
    fn C_Log_u64() {
        let instr = Instruction::decode(815958614u32);
        println!("{:032b}", 815958614u32);
        assert_eq!(
            instr, Instruction::C_Log_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958614u32);
    }
    #[test]
    fn C_Sqrt_i8() {
        let instr = Instruction::decode(2147650150u32);
        println!("{:032b}", 2147650150u32);
        assert_eq!(
            instr, Instruction::C_Sqrt_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650150u32);
    }
    #[test]
    fn C_Sqrt_i16() {
        let instr = Instruction::decode(2416085606u32);
        println!("{:032b}", 2416085606u32);
        assert_eq!(
            instr, Instruction::C_Sqrt_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085606u32);
    }
    #[test]
    fn C_Sqrt_i32() {
        let instr = Instruction::decode(2684521062u32);
        println!("{:032b}", 2684521062u32);
        assert_eq!(
            instr, Instruction::C_Sqrt_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684521062u32);
    }
    #[test]
    fn C_Sqrt_i64() {
        let instr = Instruction::decode(2952956518u32);
        println!("{:032b}", 2952956518u32);
        assert_eq!(
            instr, Instruction::C_Sqrt_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956518u32);
    }
    #[test]
    fn C_Sqrt_u8() {
        let instr = Instruction::decode(166502u32);
        println!("{:032b}", 166502u32);
        assert_eq!(
            instr, Instruction::C_Sqrt_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166502u32);
    }
    #[test]
    fn C_Sqrt_u16() {
        let instr = Instruction::decode(268601958u32);
        println!("{:032b}", 268601958u32);
        assert_eq!(
            instr, Instruction::C_Sqrt_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268601958u32);
    }
    #[test]
    fn C_Sqrt_u32() {
        let instr = Instruction::decode(537037414u32);
        println!("{:032b}", 537037414u32);
        assert_eq!(
            instr, Instruction::C_Sqrt_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037414u32);
    }
    #[test]
    fn C_Sqrt_u64() {
        let instr = Instruction::decode(805472870u32);
        println!("{:032b}", 805472870u32);
        assert_eq!(
            instr, Instruction::C_Sqrt_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805472870u32);
    }
    #[test]
    fn C_Mul_i8() {
        let instr = Instruction::decode(2158135926u32);
        println!("{:032b}", 2158135926u32);
        assert_eq!(
            instr, Instruction::C_Mul_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135926u32);
    }
    #[test]
    fn C_Mul_i16() {
        let instr = Instruction::decode(2426571382u32);
        println!("{:032b}", 2426571382u32);
        assert_eq!(
            instr, Instruction::C_Mul_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571382u32);
    }
    #[test]
    fn C_Mul_i32() {
        let instr = Instruction::decode(2695006838u32);
        println!("{:032b}", 2695006838u32);
        assert_eq!(
            instr, Instruction::C_Mul_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006838u32);
    }
    #[test]
    fn C_Mul_i64() {
        let instr = Instruction::decode(2963442294u32);
        println!("{:032b}", 2963442294u32);
        assert_eq!(
            instr, Instruction::C_Mul_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442294u32);
    }
    #[test]
    fn C_Mul_u8() {
        let instr = Instruction::decode(10652278u32);
        println!("{:032b}", 10652278u32);
        assert_eq!(
            instr, Instruction::C_Mul_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652278u32);
    }
    #[test]
    fn C_Mul_u16() {
        let instr = Instruction::decode(279087734u32);
        println!("{:032b}", 279087734u32);
        assert_eq!(
            instr, Instruction::C_Mul_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087734u32);
    }
    #[test]
    fn C_Mul_u32() {
        let instr = Instruction::decode(547523190u32);
        println!("{:032b}", 547523190u32);
        assert_eq!(
            instr, Instruction::C_Mul_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523190u32);
    }
    #[test]
    fn C_Mul_u64() {
        let instr = Instruction::decode(815958646u32);
        println!("{:032b}", 815958646u32);
        assert_eq!(
            instr, Instruction::C_Mul_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958646u32);
    }
    #[test]
    fn C_Neg_i8() {
        let instr = Instruction::decode(2147650182u32);
        println!("{:032b}", 2147650182u32);
        assert_eq!(
            instr, Instruction::C_Neg_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650182u32);
    }
    #[test]
    fn C_Neg_i16() {
        let instr = Instruction::decode(2416085638u32);
        println!("{:032b}", 2416085638u32);
        assert_eq!(
            instr, Instruction::C_Neg_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085638u32);
    }
    #[test]
    fn C_Neg_i32() {
        let instr = Instruction::decode(2684521094u32);
        println!("{:032b}", 2684521094u32);
        assert_eq!(
            instr, Instruction::C_Neg_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684521094u32);
    }
    #[test]
    fn C_Neg_i64() {
        let instr = Instruction::decode(2952956550u32);
        println!("{:032b}", 2952956550u32);
        assert_eq!(
            instr, Instruction::C_Neg_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956550u32);
    }
    #[test]
    fn C_Pow_i8() {
        let instr = Instruction::decode(2158135958u32);
        println!("{:032b}", 2158135958u32);
        assert_eq!(
            instr, Instruction::C_Pow_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135958u32);
    }
    #[test]
    fn C_Pow_i16() {
        let instr = Instruction::decode(2426571414u32);
        println!("{:032b}", 2426571414u32);
        assert_eq!(
            instr, Instruction::C_Pow_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571414u32);
    }
    #[test]
    fn C_Pow_i32() {
        let instr = Instruction::decode(2695006870u32);
        println!("{:032b}", 2695006870u32);
        assert_eq!(
            instr, Instruction::C_Pow_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006870u32);
    }
    #[test]
    fn C_Pow_i64() {
        let instr = Instruction::decode(2963442326u32);
        println!("{:032b}", 2963442326u32);
        assert_eq!(
            instr, Instruction::C_Pow_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442326u32);
    }
    #[test]
    fn C_Pow_u8() {
        let instr = Instruction::decode(10652310u32);
        println!("{:032b}", 10652310u32);
        assert_eq!(
            instr, Instruction::C_Pow_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652310u32);
    }
    #[test]
    fn C_Pow_u16() {
        let instr = Instruction::decode(279087766u32);
        println!("{:032b}", 279087766u32);
        assert_eq!(
            instr, Instruction::C_Pow_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087766u32);
    }
    #[test]
    fn C_Pow_u32() {
        let instr = Instruction::decode(547523222u32);
        println!("{:032b}", 547523222u32);
        assert_eq!(
            instr, Instruction::C_Pow_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523222u32);
    }
    #[test]
    fn C_Pow_u64() {
        let instr = Instruction::decode(815958678u32);
        println!("{:032b}", 815958678u32);
        assert_eq!(
            instr, Instruction::C_Pow_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958678u32);
    }
    #[test]
    fn C_Rem_i8() {
        let instr = Instruction::decode(2158135974u32);
        println!("{:032b}", 2158135974u32);
        assert_eq!(
            instr, Instruction::C_Rem_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135974u32);
    }
    #[test]
    fn C_Rem_i16() {
        let instr = Instruction::decode(2426571430u32);
        println!("{:032b}", 2426571430u32);
        assert_eq!(
            instr, Instruction::C_Rem_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571430u32);
    }
    #[test]
    fn C_Rem_i32() {
        let instr = Instruction::decode(2695006886u32);
        println!("{:032b}", 2695006886u32);
        assert_eq!(
            instr, Instruction::C_Rem_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006886u32);
    }
    #[test]
    fn C_Rem_i64() {
        let instr = Instruction::decode(2963442342u32);
        println!("{:032b}", 2963442342u32);
        assert_eq!(
            instr, Instruction::C_Rem_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442342u32);
    }
    #[test]
    fn C_Rem_u8() {
        let instr = Instruction::decode(10652326u32);
        println!("{:032b}", 10652326u32);
        assert_eq!(
            instr, Instruction::C_Rem_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652326u32);
    }
    #[test]
    fn C_Rem_u16() {
        let instr = Instruction::decode(279087782u32);
        println!("{:032b}", 279087782u32);
        assert_eq!(
            instr, Instruction::C_Rem_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087782u32);
    }
    #[test]
    fn C_Rem_u32() {
        let instr = Instruction::decode(547523238u32);
        println!("{:032b}", 547523238u32);
        assert_eq!(
            instr, Instruction::C_Rem_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523238u32);
    }
    #[test]
    fn C_Rem_u64() {
        let instr = Instruction::decode(815958694u32);
        println!("{:032b}", 815958694u32);
        assert_eq!(
            instr, Instruction::C_Rem_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958694u32);
    }
    #[test]
    fn C_Rem_E_i8() {
        let instr = Instruction::decode(2158135990u32);
        println!("{:032b}", 2158135990u32);
        assert_eq!(
            instr, Instruction::C_Rem_E_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135990u32);
    }
    #[test]
    fn C_Rem_E_i16() {
        let instr = Instruction::decode(2426571446u32);
        println!("{:032b}", 2426571446u32);
        assert_eq!(
            instr, Instruction::C_Rem_E_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571446u32);
    }
    #[test]
    fn C_Rem_E_i32() {
        let instr = Instruction::decode(2695006902u32);
        println!("{:032b}", 2695006902u32);
        assert_eq!(
            instr, Instruction::C_Rem_E_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006902u32);
    }
    #[test]
    fn C_Rem_E_i64() {
        let instr = Instruction::decode(2963442358u32);
        println!("{:032b}", 2963442358u32);
        assert_eq!(
            instr, Instruction::C_Rem_E_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442358u32);
    }
    #[test]
    fn C_Rem_E_u8() {
        let instr = Instruction::decode(10652342u32);
        println!("{:032b}", 10652342u32);
        assert_eq!(
            instr, Instruction::C_Rem_E_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652342u32);
    }
    #[test]
    fn C_Rem_E_u16() {
        let instr = Instruction::decode(279087798u32);
        println!("{:032b}", 279087798u32);
        assert_eq!(
            instr, Instruction::C_Rem_E_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087798u32);
    }
    #[test]
    fn C_Rem_E_u32() {
        let instr = Instruction::decode(547523254u32);
        println!("{:032b}", 547523254u32);
        assert_eq!(
            instr, Instruction::C_Rem_E_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523254u32);
    }
    #[test]
    fn C_Rem_E_u64() {
        let instr = Instruction::decode(815958710u32);
        println!("{:032b}", 815958710u32);
        assert_eq!(
            instr, Instruction::C_Rem_E_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958710u32);
    }
    #[test]
    fn C_Shl_i8() {
        let instr = Instruction::decode(2158136006u32);
        println!("{:032b}", 2158136006u32);
        assert_eq!(
            instr, Instruction::C_Shl_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158136006u32);
    }
    #[test]
    fn C_Shl_i16() {
        let instr = Instruction::decode(2426571462u32);
        println!("{:032b}", 2426571462u32);
        assert_eq!(
            instr, Instruction::C_Shl_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571462u32);
    }
    #[test]
    fn C_Shl_i32() {
        let instr = Instruction::decode(2695006918u32);
        println!("{:032b}", 2695006918u32);
        assert_eq!(
            instr, Instruction::C_Shl_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006918u32);
    }
    #[test]
    fn C_Shl_i64() {
        let instr = Instruction::decode(2963442374u32);
        println!("{:032b}", 2963442374u32);
        assert_eq!(
            instr, Instruction::C_Shl_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442374u32);
    }
    #[test]
    fn C_Shl_u8() {
        let instr = Instruction::decode(10652358u32);
        println!("{:032b}", 10652358u32);
        assert_eq!(
            instr, Instruction::C_Shl_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652358u32);
    }
    #[test]
    fn C_Shl_u16() {
        let instr = Instruction::decode(279087814u32);
        println!("{:032b}", 279087814u32);
        assert_eq!(
            instr, Instruction::C_Shl_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087814u32);
    }
    #[test]
    fn C_Shl_u32() {
        let instr = Instruction::decode(547523270u32);
        println!("{:032b}", 547523270u32);
        assert_eq!(
            instr, Instruction::C_Shl_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523270u32);
    }
    #[test]
    fn C_Shl_u64() {
        let instr = Instruction::decode(815958726u32);
        println!("{:032b}", 815958726u32);
        assert_eq!(
            instr, Instruction::C_Shl_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958726u32);
    }
    #[test]
    fn C_Shr_i8() {
        let instr = Instruction::decode(2158136022u32);
        println!("{:032b}", 2158136022u32);
        assert_eq!(
            instr, Instruction::C_Shr_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158136022u32);
    }
    #[test]
    fn C_Shr_i16() {
        let instr = Instruction::decode(2426571478u32);
        println!("{:032b}", 2426571478u32);
        assert_eq!(
            instr, Instruction::C_Shr_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571478u32);
    }
    #[test]
    fn C_Shr_i32() {
        let instr = Instruction::decode(2695006934u32);
        println!("{:032b}", 2695006934u32);
        assert_eq!(
            instr, Instruction::C_Shr_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006934u32);
    }
    #[test]
    fn C_Shr_i64() {
        let instr = Instruction::decode(2963442390u32);
        println!("{:032b}", 2963442390u32);
        assert_eq!(
            instr, Instruction::C_Shr_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442390u32);
    }
    #[test]
    fn C_Shr_u8() {
        let instr = Instruction::decode(10652374u32);
        println!("{:032b}", 10652374u32);
        assert_eq!(
            instr, Instruction::C_Shr_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652374u32);
    }
    #[test]
    fn C_Shr_u16() {
        let instr = Instruction::decode(279087830u32);
        println!("{:032b}", 279087830u32);
        assert_eq!(
            instr, Instruction::C_Shr_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087830u32);
    }
    #[test]
    fn C_Shr_u32() {
        let instr = Instruction::decode(547523286u32);
        println!("{:032b}", 547523286u32);
        assert_eq!(
            instr, Instruction::C_Shr_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523286u32);
    }
    #[test]
    fn C_Shr_u64() {
        let instr = Instruction::decode(815958742u32);
        println!("{:032b}", 815958742u32);
        assert_eq!(
            instr, Instruction::C_Shr_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958742u32);
    }
    #[test]
    fn C_Sub_i8() {
        let instr = Instruction::decode(2158136038u32);
        println!("{:032b}", 2158136038u32);
        assert_eq!(
            instr, Instruction::C_Sub_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158136038u32);
    }
    #[test]
    fn C_Sub_i16() {
        let instr = Instruction::decode(2426571494u32);
        println!("{:032b}", 2426571494u32);
        assert_eq!(
            instr, Instruction::C_Sub_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571494u32);
    }
    #[test]
    fn C_Sub_i32() {
        let instr = Instruction::decode(2695006950u32);
        println!("{:032b}", 2695006950u32);
        assert_eq!(
            instr, Instruction::C_Sub_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006950u32);
    }
    #[test]
    fn C_Sub_i64() {
        let instr = Instruction::decode(2963442406u32);
        println!("{:032b}", 2963442406u32);
        assert_eq!(
            instr, Instruction::C_Sub_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442406u32);
    }
    #[test]
    fn C_Sub_u8() {
        let instr = Instruction::decode(10652390u32);
        println!("{:032b}", 10652390u32);
        assert_eq!(
            instr, Instruction::C_Sub_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652390u32);
    }
    #[test]
    fn C_Sub_u16() {
        let instr = Instruction::decode(279087846u32);
        println!("{:032b}", 279087846u32);
        assert_eq!(
            instr, Instruction::C_Sub_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087846u32);
    }
    #[test]
    fn C_Sub_u32() {
        let instr = Instruction::decode(547523302u32);
        println!("{:032b}", 547523302u32);
        assert_eq!(
            instr, Instruction::C_Sub_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523302u32);
    }
    #[test]
    fn C_Sub_u64() {
        let instr = Instruction::decode(815958758u32);
        println!("{:032b}", 815958758u32);
        assert_eq!(
            instr, Instruction::C_Sub_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958758u32);
    }
    #[test]
    fn C_Sub_U_i8() {
        let instr = Instruction::decode(2158136054u32);
        println!("{:032b}", 2158136054u32);
        assert_eq!(
            instr, Instruction::C_Sub_U_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158136054u32);
    }
    #[test]
    fn C_Sub_U_i16() {
        let instr = Instruction::decode(2426571510u32);
        println!("{:032b}", 2426571510u32);
        assert_eq!(
            instr, Instruction::C_Sub_U_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571510u32);
    }
    #[test]
    fn C_Sub_U_i32() {
        let instr = Instruction::decode(2695006966u32);
        println!("{:032b}", 2695006966u32);
        assert_eq!(
            instr, Instruction::C_Sub_U_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006966u32);
    }
    #[test]
    fn C_Sub_U_i64() {
        let instr = Instruction::decode(2963442422u32);
        println!("{:032b}", 2963442422u32);
        assert_eq!(
            instr, Instruction::C_Sub_U_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442422u32);
    }
    #[test]
    fn O_Abs_i8() {
        let instr = Instruction::decode(2147650055u32);
        println!("{:032b}", 2147650055u32);
        assert_eq!(
            instr, Instruction::O_Abs_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650055u32);
    }
    #[test]
    fn O_Abs_i16() {
        let instr = Instruction::decode(2416085511u32);
        println!("{:032b}", 2416085511u32);
        assert_eq!(
            instr, Instruction::O_Abs_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085511u32);
    }
    #[test]
    fn O_Abs_i32() {
        let instr = Instruction::decode(2684520967u32);
        println!("{:032b}", 2684520967u32);
        assert_eq!(
            instr, Instruction::O_Abs_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684520967u32);
    }
    #[test]
    fn O_Abs_i64() {
        let instr = Instruction::decode(2952956423u32);
        println!("{:032b}", 2952956423u32);
        assert_eq!(
            instr, Instruction::O_Abs_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956423u32);
    }
    #[test]
    fn O_Add_i8() {
        let instr = Instruction::decode(2158135831u32);
        println!("{:032b}", 2158135831u32);
        assert_eq!(
            instr, Instruction::O_Add_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135831u32);
    }
    #[test]
    fn O_Add_i16() {
        let instr = Instruction::decode(2426571287u32);
        println!("{:032b}", 2426571287u32);
        assert_eq!(
            instr, Instruction::O_Add_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571287u32);
    }
    #[test]
    fn O_Add_i32() {
        let instr = Instruction::decode(2695006743u32);
        println!("{:032b}", 2695006743u32);
        assert_eq!(
            instr, Instruction::O_Add_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006743u32);
    }
    #[test]
    fn O_Add_i64() {
        let instr = Instruction::decode(2963442199u32);
        println!("{:032b}", 2963442199u32);
        assert_eq!(
            instr, Instruction::O_Add_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442199u32);
    }
    #[test]
    fn O_Add_u8() {
        let instr = Instruction::decode(10652183u32);
        println!("{:032b}", 10652183u32);
        assert_eq!(
            instr, Instruction::O_Add_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652183u32);
    }
    #[test]
    fn O_Add_u16() {
        let instr = Instruction::decode(279087639u32);
        println!("{:032b}", 279087639u32);
        assert_eq!(
            instr, Instruction::O_Add_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087639u32);
    }
    #[test]
    fn O_Add_u32() {
        let instr = Instruction::decode(547523095u32);
        println!("{:032b}", 547523095u32);
        assert_eq!(
            instr, Instruction::O_Add_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523095u32);
    }
    #[test]
    fn O_Add_u64() {
        let instr = Instruction::decode(815958551u32);
        println!("{:032b}", 815958551u32);
        assert_eq!(
            instr, Instruction::O_Add_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958551u32);
    }
    #[test]
    fn O_Add_U_i8() {
        let instr = Instruction::decode(2158135847u32);
        println!("{:032b}", 2158135847u32);
        assert_eq!(
            instr, Instruction::O_Add_U_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135847u32);
    }
    #[test]
    fn O_Add_U_i16() {
        let instr = Instruction::decode(2426571303u32);
        println!("{:032b}", 2426571303u32);
        assert_eq!(
            instr, Instruction::O_Add_U_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571303u32);
    }
    #[test]
    fn O_Add_U_i32() {
        let instr = Instruction::decode(2695006759u32);
        println!("{:032b}", 2695006759u32);
        assert_eq!(
            instr, Instruction::O_Add_U_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006759u32);
    }
    #[test]
    fn O_Add_U_i64() {
        let instr = Instruction::decode(2963442215u32);
        println!("{:032b}", 2963442215u32);
        assert_eq!(
            instr, Instruction::O_Add_U_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442215u32);
    }
    #[test]
    fn O_Add_S_u8() {
        let instr = Instruction::decode(10652199u32);
        println!("{:032b}", 10652199u32);
        assert_eq!(
            instr, Instruction::O_Add_S_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652199u32);
    }
    #[test]
    fn O_Add_S_u16() {
        let instr = Instruction::decode(279087655u32);
        println!("{:032b}", 279087655u32);
        assert_eq!(
            instr, Instruction::O_Add_S_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087655u32);
    }
    #[test]
    fn O_Add_S_u32() {
        let instr = Instruction::decode(547523111u32);
        println!("{:032b}", 547523111u32);
        assert_eq!(
            instr, Instruction::O_Add_S_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523111u32);
    }
    #[test]
    fn O_Add_S_u64() {
        let instr = Instruction::decode(815958567u32);
        println!("{:032b}", 815958567u32);
        assert_eq!(
            instr, Instruction::O_Add_S_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958567u32);
    }
    #[test]
    fn O_Div_i8() {
        let instr = Instruction::decode(2158135863u32);
        println!("{:032b}", 2158135863u32);
        assert_eq!(
            instr, Instruction::O_Div_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135863u32);
    }
    #[test]
    fn O_Div_i16() {
        let instr = Instruction::decode(2426571319u32);
        println!("{:032b}", 2426571319u32);
        assert_eq!(
            instr, Instruction::O_Div_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571319u32);
    }
    #[test]
    fn O_Div_i32() {
        let instr = Instruction::decode(2695006775u32);
        println!("{:032b}", 2695006775u32);
        assert_eq!(
            instr, Instruction::O_Div_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006775u32);
    }
    #[test]
    fn O_Div_i64() {
        let instr = Instruction::decode(2963442231u32);
        println!("{:032b}", 2963442231u32);
        assert_eq!(
            instr, Instruction::O_Div_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442231u32);
    }
    #[test]
    fn O_Div_u8() {
        let instr = Instruction::decode(10652215u32);
        println!("{:032b}", 10652215u32);
        assert_eq!(
            instr, Instruction::O_Div_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652215u32);
    }
    #[test]
    fn O_Div_u16() {
        let instr = Instruction::decode(279087671u32);
        println!("{:032b}", 279087671u32);
        assert_eq!(
            instr, Instruction::O_Div_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087671u32);
    }
    #[test]
    fn O_Div_u32() {
        let instr = Instruction::decode(547523127u32);
        println!("{:032b}", 547523127u32);
        assert_eq!(
            instr, Instruction::O_Div_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523127u32);
    }
    #[test]
    fn O_Div_u64() {
        let instr = Instruction::decode(815958583u32);
        println!("{:032b}", 815958583u32);
        assert_eq!(
            instr, Instruction::O_Div_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958583u32);
    }
    #[test]
    fn O_Div_E_i8() {
        let instr = Instruction::decode(2158135879u32);
        println!("{:032b}", 2158135879u32);
        assert_eq!(
            instr, Instruction::O_Div_E_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135879u32);
    }
    #[test]
    fn O_Div_E_i16() {
        let instr = Instruction::decode(2426571335u32);
        println!("{:032b}", 2426571335u32);
        assert_eq!(
            instr, Instruction::O_Div_E_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571335u32);
    }
    #[test]
    fn O_Div_E_i32() {
        let instr = Instruction::decode(2695006791u32);
        println!("{:032b}", 2695006791u32);
        assert_eq!(
            instr, Instruction::O_Div_E_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006791u32);
    }
    #[test]
    fn O_Div_E_i64() {
        let instr = Instruction::decode(2963442247u32);
        println!("{:032b}", 2963442247u32);
        assert_eq!(
            instr, Instruction::O_Div_E_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442247u32);
    }
    #[test]
    fn O_Div_E_u8() {
        let instr = Instruction::decode(10652231u32);
        println!("{:032b}", 10652231u32);
        assert_eq!(
            instr, Instruction::O_Div_E_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652231u32);
    }
    #[test]
    fn O_Div_E_u16() {
        let instr = Instruction::decode(279087687u32);
        println!("{:032b}", 279087687u32);
        assert_eq!(
            instr, Instruction::O_Div_E_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087687u32);
    }
    #[test]
    fn O_Div_E_u32() {
        let instr = Instruction::decode(547523143u32);
        println!("{:032b}", 547523143u32);
        assert_eq!(
            instr, Instruction::O_Div_E_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523143u32);
    }
    #[test]
    fn O_Div_E_u64() {
        let instr = Instruction::decode(815958599u32);
        println!("{:032b}", 815958599u32);
        assert_eq!(
            instr, Instruction::O_Div_E_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958599u32);
    }
    #[test]
    fn O_Mul_i8() {
        let instr = Instruction::decode(2158135927u32);
        println!("{:032b}", 2158135927u32);
        assert_eq!(
            instr, Instruction::O_Mul_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135927u32);
    }
    #[test]
    fn O_Mul_i16() {
        let instr = Instruction::decode(2426571383u32);
        println!("{:032b}", 2426571383u32);
        assert_eq!(
            instr, Instruction::O_Mul_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571383u32);
    }
    #[test]
    fn O_Mul_i32() {
        let instr = Instruction::decode(2695006839u32);
        println!("{:032b}", 2695006839u32);
        assert_eq!(
            instr, Instruction::O_Mul_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006839u32);
    }
    #[test]
    fn O_Mul_i64() {
        let instr = Instruction::decode(2963442295u32);
        println!("{:032b}", 2963442295u32);
        assert_eq!(
            instr, Instruction::O_Mul_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442295u32);
    }
    #[test]
    fn O_Mul_u8() {
        let instr = Instruction::decode(10652279u32);
        println!("{:032b}", 10652279u32);
        assert_eq!(
            instr, Instruction::O_Mul_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652279u32);
    }
    #[test]
    fn O_Mul_u16() {
        let instr = Instruction::decode(279087735u32);
        println!("{:032b}", 279087735u32);
        assert_eq!(
            instr, Instruction::O_Mul_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087735u32);
    }
    #[test]
    fn O_Mul_u32() {
        let instr = Instruction::decode(547523191u32);
        println!("{:032b}", 547523191u32);
        assert_eq!(
            instr, Instruction::O_Mul_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523191u32);
    }
    #[test]
    fn O_Mul_u64() {
        let instr = Instruction::decode(815958647u32);
        println!("{:032b}", 815958647u32);
        assert_eq!(
            instr, Instruction::O_Mul_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958647u32);
    }
    #[test]
    fn O_Neg_i8() {
        let instr = Instruction::decode(2147650183u32);
        println!("{:032b}", 2147650183u32);
        assert_eq!(
            instr, Instruction::O_Neg_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650183u32);
    }
    #[test]
    fn O_Neg_i16() {
        let instr = Instruction::decode(2416085639u32);
        println!("{:032b}", 2416085639u32);
        assert_eq!(
            instr, Instruction::O_Neg_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085639u32);
    }
    #[test]
    fn O_Neg_i32() {
        let instr = Instruction::decode(2684521095u32);
        println!("{:032b}", 2684521095u32);
        assert_eq!(
            instr, Instruction::O_Neg_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684521095u32);
    }
    #[test]
    fn O_Neg_i64() {
        let instr = Instruction::decode(2952956551u32);
        println!("{:032b}", 2952956551u32);
        assert_eq!(
            instr, Instruction::O_Neg_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956551u32);
    }
    #[test]
    fn O_Pow_i8() {
        let instr = Instruction::decode(2158135959u32);
        println!("{:032b}", 2158135959u32);
        assert_eq!(
            instr, Instruction::O_Pow_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135959u32);
    }
    #[test]
    fn O_Pow_i16() {
        let instr = Instruction::decode(2426571415u32);
        println!("{:032b}", 2426571415u32);
        assert_eq!(
            instr, Instruction::O_Pow_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571415u32);
    }
    #[test]
    fn O_Pow_i32() {
        let instr = Instruction::decode(2695006871u32);
        println!("{:032b}", 2695006871u32);
        assert_eq!(
            instr, Instruction::O_Pow_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006871u32);
    }
    #[test]
    fn O_Pow_i64() {
        let instr = Instruction::decode(2963442327u32);
        println!("{:032b}", 2963442327u32);
        assert_eq!(
            instr, Instruction::O_Pow_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442327u32);
    }
    #[test]
    fn O_Pow_u8() {
        let instr = Instruction::decode(10652311u32);
        println!("{:032b}", 10652311u32);
        assert_eq!(
            instr, Instruction::O_Pow_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652311u32);
    }
    #[test]
    fn O_Pow_u16() {
        let instr = Instruction::decode(279087767u32);
        println!("{:032b}", 279087767u32);
        assert_eq!(
            instr, Instruction::O_Pow_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087767u32);
    }
    #[test]
    fn O_Pow_u32() {
        let instr = Instruction::decode(547523223u32);
        println!("{:032b}", 547523223u32);
        assert_eq!(
            instr, Instruction::O_Pow_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523223u32);
    }
    #[test]
    fn O_Pow_u64() {
        let instr = Instruction::decode(815958679u32);
        println!("{:032b}", 815958679u32);
        assert_eq!(
            instr, Instruction::O_Pow_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958679u32);
    }
    #[test]
    fn O_Rem_i8() {
        let instr = Instruction::decode(2158135975u32);
        println!("{:032b}", 2158135975u32);
        assert_eq!(
            instr, Instruction::O_Rem_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135975u32);
    }
    #[test]
    fn O_Rem_i16() {
        let instr = Instruction::decode(2426571431u32);
        println!("{:032b}", 2426571431u32);
        assert_eq!(
            instr, Instruction::O_Rem_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571431u32);
    }
    #[test]
    fn O_Rem_i32() {
        let instr = Instruction::decode(2695006887u32);
        println!("{:032b}", 2695006887u32);
        assert_eq!(
            instr, Instruction::O_Rem_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006887u32);
    }
    #[test]
    fn O_Rem_i64() {
        let instr = Instruction::decode(2963442343u32);
        println!("{:032b}", 2963442343u32);
        assert_eq!(
            instr, Instruction::O_Rem_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442343u32);
    }
    #[test]
    fn O_Rem_u8() {
        let instr = Instruction::decode(10652327u32);
        println!("{:032b}", 10652327u32);
        assert_eq!(
            instr, Instruction::O_Rem_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652327u32);
    }
    #[test]
    fn O_Rem_u16() {
        let instr = Instruction::decode(279087783u32);
        println!("{:032b}", 279087783u32);
        assert_eq!(
            instr, Instruction::O_Rem_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087783u32);
    }
    #[test]
    fn O_Rem_u32() {
        let instr = Instruction::decode(547523239u32);
        println!("{:032b}", 547523239u32);
        assert_eq!(
            instr, Instruction::O_Rem_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523239u32);
    }
    #[test]
    fn O_Rem_u64() {
        let instr = Instruction::decode(815958695u32);
        println!("{:032b}", 815958695u32);
        assert_eq!(
            instr, Instruction::O_Rem_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958695u32);
    }
    #[test]
    fn O_Rem_E_i8() {
        let instr = Instruction::decode(2158135991u32);
        println!("{:032b}", 2158135991u32);
        assert_eq!(
            instr, Instruction::O_Rem_E_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135991u32);
    }
    #[test]
    fn O_Rem_E_i16() {
        let instr = Instruction::decode(2426571447u32);
        println!("{:032b}", 2426571447u32);
        assert_eq!(
            instr, Instruction::O_Rem_E_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571447u32);
    }
    #[test]
    fn O_Rem_E_i32() {
        let instr = Instruction::decode(2695006903u32);
        println!("{:032b}", 2695006903u32);
        assert_eq!(
            instr, Instruction::O_Rem_E_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006903u32);
    }
    #[test]
    fn O_Rem_E_i64() {
        let instr = Instruction::decode(2963442359u32);
        println!("{:032b}", 2963442359u32);
        assert_eq!(
            instr, Instruction::O_Rem_E_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442359u32);
    }
    #[test]
    fn O_Rem_E_u8() {
        let instr = Instruction::decode(10652343u32);
        println!("{:032b}", 10652343u32);
        assert_eq!(
            instr, Instruction::O_Rem_E_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652343u32);
    }
    #[test]
    fn O_Rem_E_u16() {
        let instr = Instruction::decode(279087799u32);
        println!("{:032b}", 279087799u32);
        assert_eq!(
            instr, Instruction::O_Rem_E_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087799u32);
    }
    #[test]
    fn O_Rem_E_u32() {
        let instr = Instruction::decode(547523255u32);
        println!("{:032b}", 547523255u32);
        assert_eq!(
            instr, Instruction::O_Rem_E_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523255u32);
    }
    #[test]
    fn O_Rem_E_u64() {
        let instr = Instruction::decode(815958711u32);
        println!("{:032b}", 815958711u32);
        assert_eq!(
            instr, Instruction::O_Rem_E_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958711u32);
    }
    #[test]
    fn O_Shl_i8() {
        let instr = Instruction::decode(2158136007u32);
        println!("{:032b}", 2158136007u32);
        assert_eq!(
            instr, Instruction::O_Shl_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158136007u32);
    }
    #[test]
    fn O_Shl_i16() {
        let instr = Instruction::decode(2426571463u32);
        println!("{:032b}", 2426571463u32);
        assert_eq!(
            instr, Instruction::O_Shl_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571463u32);
    }
    #[test]
    fn O_Shl_i32() {
        let instr = Instruction::decode(2695006919u32);
        println!("{:032b}", 2695006919u32);
        assert_eq!(
            instr, Instruction::O_Shl_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006919u32);
    }
    #[test]
    fn O_Shl_i64() {
        let instr = Instruction::decode(2963442375u32);
        println!("{:032b}", 2963442375u32);
        assert_eq!(
            instr, Instruction::O_Shl_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442375u32);
    }
    #[test]
    fn O_Shl_u8() {
        let instr = Instruction::decode(10652359u32);
        println!("{:032b}", 10652359u32);
        assert_eq!(
            instr, Instruction::O_Shl_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652359u32);
    }
    #[test]
    fn O_Shl_u16() {
        let instr = Instruction::decode(279087815u32);
        println!("{:032b}", 279087815u32);
        assert_eq!(
            instr, Instruction::O_Shl_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087815u32);
    }
    #[test]
    fn O_Shl_u32() {
        let instr = Instruction::decode(547523271u32);
        println!("{:032b}", 547523271u32);
        assert_eq!(
            instr, Instruction::O_Shl_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523271u32);
    }
    #[test]
    fn O_Shl_u64() {
        let instr = Instruction::decode(815958727u32);
        println!("{:032b}", 815958727u32);
        assert_eq!(
            instr, Instruction::O_Shl_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958727u32);
    }
    #[test]
    fn O_Shr_i8() {
        let instr = Instruction::decode(2158136023u32);
        println!("{:032b}", 2158136023u32);
        assert_eq!(
            instr, Instruction::O_Shr_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158136023u32);
    }
    #[test]
    fn O_Shr_i16() {
        let instr = Instruction::decode(2426571479u32);
        println!("{:032b}", 2426571479u32);
        assert_eq!(
            instr, Instruction::O_Shr_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571479u32);
    }
    #[test]
    fn O_Shr_i32() {
        let instr = Instruction::decode(2695006935u32);
        println!("{:032b}", 2695006935u32);
        assert_eq!(
            instr, Instruction::O_Shr_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006935u32);
    }
    #[test]
    fn O_Shr_i64() {
        let instr = Instruction::decode(2963442391u32);
        println!("{:032b}", 2963442391u32);
        assert_eq!(
            instr, Instruction::O_Shr_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442391u32);
    }
    #[test]
    fn O_Shr_u8() {
        let instr = Instruction::decode(10652375u32);
        println!("{:032b}", 10652375u32);
        assert_eq!(
            instr, Instruction::O_Shr_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652375u32);
    }
    #[test]
    fn O_Shr_u16() {
        let instr = Instruction::decode(279087831u32);
        println!("{:032b}", 279087831u32);
        assert_eq!(
            instr, Instruction::O_Shr_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087831u32);
    }
    #[test]
    fn O_Shr_u32() {
        let instr = Instruction::decode(547523287u32);
        println!("{:032b}", 547523287u32);
        assert_eq!(
            instr, Instruction::O_Shr_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523287u32);
    }
    #[test]
    fn O_Shr_u64() {
        let instr = Instruction::decode(815958743u32);
        println!("{:032b}", 815958743u32);
        assert_eq!(
            instr, Instruction::O_Shr_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958743u32);
    }
    #[test]
    fn O_Sub_i8() {
        let instr = Instruction::decode(2158136039u32);
        println!("{:032b}", 2158136039u32);
        assert_eq!(
            instr, Instruction::O_Sub_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158136039u32);
    }
    #[test]
    fn O_Sub_i16() {
        let instr = Instruction::decode(2426571495u32);
        println!("{:032b}", 2426571495u32);
        assert_eq!(
            instr, Instruction::O_Sub_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571495u32);
    }
    #[test]
    fn O_Sub_i32() {
        let instr = Instruction::decode(2695006951u32);
        println!("{:032b}", 2695006951u32);
        assert_eq!(
            instr, Instruction::O_Sub_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006951u32);
    }
    #[test]
    fn O_Sub_i64() {
        let instr = Instruction::decode(2963442407u32);
        println!("{:032b}", 2963442407u32);
        assert_eq!(
            instr, Instruction::O_Sub_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442407u32);
    }
    #[test]
    fn O_Sub_u8() {
        let instr = Instruction::decode(10652391u32);
        println!("{:032b}", 10652391u32);
        assert_eq!(
            instr, Instruction::O_Sub_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652391u32);
    }
    #[test]
    fn O_Sub_u16() {
        let instr = Instruction::decode(279087847u32);
        println!("{:032b}", 279087847u32);
        assert_eq!(
            instr, Instruction::O_Sub_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087847u32);
    }
    #[test]
    fn O_Sub_u32() {
        let instr = Instruction::decode(547523303u32);
        println!("{:032b}", 547523303u32);
        assert_eq!(
            instr, Instruction::O_Sub_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523303u32);
    }
    #[test]
    fn O_Sub_u64() {
        let instr = Instruction::decode(815958759u32);
        println!("{:032b}", 815958759u32);
        assert_eq!(
            instr, Instruction::O_Sub_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958759u32);
    }
    #[test]
    fn O_Sub_U_i8() {
        let instr = Instruction::decode(2158136055u32);
        println!("{:032b}", 2158136055u32);
        assert_eq!(
            instr, Instruction::O_Sub_U_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158136055u32);
    }
    #[test]
    fn O_Sub_U_i16() {
        let instr = Instruction::decode(2426571511u32);
        println!("{:032b}", 2426571511u32);
        assert_eq!(
            instr, Instruction::O_Sub_U_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571511u32);
    }
    #[test]
    fn O_Sub_U_i32() {
        let instr = Instruction::decode(2695006967u32);
        println!("{:032b}", 2695006967u32);
        assert_eq!(
            instr, Instruction::O_Sub_U_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006967u32);
    }
    #[test]
    fn O_Sub_U_i64() {
        let instr = Instruction::decode(2963442423u32);
        println!("{:032b}", 2963442423u32);
        assert_eq!(
            instr, Instruction::O_Sub_U_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442423u32);
    }
    #[test]
    fn S_Abs_i8() {
        let instr = Instruction::decode(2147650056u32);
        println!("{:032b}", 2147650056u32);
        assert_eq!(
            instr, Instruction::S_Abs_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650056u32);
    }
    #[test]
    fn S_Abs_i16() {
        let instr = Instruction::decode(2416085512u32);
        println!("{:032b}", 2416085512u32);
        assert_eq!(
            instr, Instruction::S_Abs_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085512u32);
    }
    #[test]
    fn S_Abs_i32() {
        let instr = Instruction::decode(2684520968u32);
        println!("{:032b}", 2684520968u32);
        assert_eq!(
            instr, Instruction::S_Abs_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684520968u32);
    }
    #[test]
    fn S_Abs_i64() {
        let instr = Instruction::decode(2952956424u32);
        println!("{:032b}", 2952956424u32);
        assert_eq!(
            instr, Instruction::S_Abs_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956424u32);
    }
    #[test]
    fn S_Add_i8() {
        let instr = Instruction::decode(2158135832u32);
        println!("{:032b}", 2158135832u32);
        assert_eq!(
            instr, Instruction::S_Add_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135832u32);
    }
    #[test]
    fn S_Add_i16() {
        let instr = Instruction::decode(2426571288u32);
        println!("{:032b}", 2426571288u32);
        assert_eq!(
            instr, Instruction::S_Add_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571288u32);
    }
    #[test]
    fn S_Add_i32() {
        let instr = Instruction::decode(2695006744u32);
        println!("{:032b}", 2695006744u32);
        assert_eq!(
            instr, Instruction::S_Add_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006744u32);
    }
    #[test]
    fn S_Add_i64() {
        let instr = Instruction::decode(2963442200u32);
        println!("{:032b}", 2963442200u32);
        assert_eq!(
            instr, Instruction::S_Add_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442200u32);
    }
    #[test]
    fn S_Add_u8() {
        let instr = Instruction::decode(10652184u32);
        println!("{:032b}", 10652184u32);
        assert_eq!(
            instr, Instruction::S_Add_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652184u32);
    }
    #[test]
    fn S_Add_u16() {
        let instr = Instruction::decode(279087640u32);
        println!("{:032b}", 279087640u32);
        assert_eq!(
            instr, Instruction::S_Add_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087640u32);
    }
    #[test]
    fn S_Add_u32() {
        let instr = Instruction::decode(547523096u32);
        println!("{:032b}", 547523096u32);
        assert_eq!(
            instr, Instruction::S_Add_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523096u32);
    }
    #[test]
    fn S_Add_u64() {
        let instr = Instruction::decode(815958552u32);
        println!("{:032b}", 815958552u32);
        assert_eq!(
            instr, Instruction::S_Add_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958552u32);
    }
    #[test]
    fn S_Add_U_i8() {
        let instr = Instruction::decode(2158135848u32);
        println!("{:032b}", 2158135848u32);
        assert_eq!(
            instr, Instruction::S_Add_U_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135848u32);
    }
    #[test]
    fn S_Add_U_i16() {
        let instr = Instruction::decode(2426571304u32);
        println!("{:032b}", 2426571304u32);
        assert_eq!(
            instr, Instruction::S_Add_U_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571304u32);
    }
    #[test]
    fn S_Add_U_i32() {
        let instr = Instruction::decode(2695006760u32);
        println!("{:032b}", 2695006760u32);
        assert_eq!(
            instr, Instruction::S_Add_U_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006760u32);
    }
    #[test]
    fn S_Add_U_i64() {
        let instr = Instruction::decode(2963442216u32);
        println!("{:032b}", 2963442216u32);
        assert_eq!(
            instr, Instruction::S_Add_U_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442216u32);
    }
    #[test]
    fn S_Add_S_u8() {
        let instr = Instruction::decode(10652200u32);
        println!("{:032b}", 10652200u32);
        assert_eq!(
            instr, Instruction::S_Add_S_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652200u32);
    }
    #[test]
    fn S_Add_S_u16() {
        let instr = Instruction::decode(279087656u32);
        println!("{:032b}", 279087656u32);
        assert_eq!(
            instr, Instruction::S_Add_S_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087656u32);
    }
    #[test]
    fn S_Add_S_u32() {
        let instr = Instruction::decode(547523112u32);
        println!("{:032b}", 547523112u32);
        assert_eq!(
            instr, Instruction::S_Add_S_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523112u32);
    }
    #[test]
    fn S_Add_S_u64() {
        let instr = Instruction::decode(815958568u32);
        println!("{:032b}", 815958568u32);
        assert_eq!(
            instr, Instruction::S_Add_S_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958568u32);
    }
    #[test]
    fn S_Div_i8() {
        let instr = Instruction::decode(2158135864u32);
        println!("{:032b}", 2158135864u32);
        assert_eq!(
            instr, Instruction::S_Div_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135864u32);
    }
    #[test]
    fn S_Div_i16() {
        let instr = Instruction::decode(2426571320u32);
        println!("{:032b}", 2426571320u32);
        assert_eq!(
            instr, Instruction::S_Div_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571320u32);
    }
    #[test]
    fn S_Div_i32() {
        let instr = Instruction::decode(2695006776u32);
        println!("{:032b}", 2695006776u32);
        assert_eq!(
            instr, Instruction::S_Div_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006776u32);
    }
    #[test]
    fn S_Div_i64() {
        let instr = Instruction::decode(2963442232u32);
        println!("{:032b}", 2963442232u32);
        assert_eq!(
            instr, Instruction::S_Div_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442232u32);
    }
    #[test]
    fn S_Div_u8() {
        let instr = Instruction::decode(10652216u32);
        println!("{:032b}", 10652216u32);
        assert_eq!(
            instr, Instruction::S_Div_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652216u32);
    }
    #[test]
    fn S_Div_u16() {
        let instr = Instruction::decode(279087672u32);
        println!("{:032b}", 279087672u32);
        assert_eq!(
            instr, Instruction::S_Div_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087672u32);
    }
    #[test]
    fn S_Div_u32() {
        let instr = Instruction::decode(547523128u32);
        println!("{:032b}", 547523128u32);
        assert_eq!(
            instr, Instruction::S_Div_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523128u32);
    }
    #[test]
    fn S_Div_u64() {
        let instr = Instruction::decode(815958584u32);
        println!("{:032b}", 815958584u32);
        assert_eq!(
            instr, Instruction::S_Div_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958584u32);
    }
    #[test]
    fn S_Mul_i8() {
        let instr = Instruction::decode(2158135928u32);
        println!("{:032b}", 2158135928u32);
        assert_eq!(
            instr, Instruction::S_Mul_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135928u32);
    }
    #[test]
    fn S_Mul_i16() {
        let instr = Instruction::decode(2426571384u32);
        println!("{:032b}", 2426571384u32);
        assert_eq!(
            instr, Instruction::S_Mul_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571384u32);
    }
    #[test]
    fn S_Mul_i32() {
        let instr = Instruction::decode(2695006840u32);
        println!("{:032b}", 2695006840u32);
        assert_eq!(
            instr, Instruction::S_Mul_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006840u32);
    }
    #[test]
    fn S_Mul_i64() {
        let instr = Instruction::decode(2963442296u32);
        println!("{:032b}", 2963442296u32);
        assert_eq!(
            instr, Instruction::S_Mul_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442296u32);
    }
    #[test]
    fn S_Mul_u8() {
        let instr = Instruction::decode(10652280u32);
        println!("{:032b}", 10652280u32);
        assert_eq!(
            instr, Instruction::S_Mul_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652280u32);
    }
    #[test]
    fn S_Mul_u16() {
        let instr = Instruction::decode(279087736u32);
        println!("{:032b}", 279087736u32);
        assert_eq!(
            instr, Instruction::S_Mul_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087736u32);
    }
    #[test]
    fn S_Mul_u32() {
        let instr = Instruction::decode(547523192u32);
        println!("{:032b}", 547523192u32);
        assert_eq!(
            instr, Instruction::S_Mul_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523192u32);
    }
    #[test]
    fn S_Mul_u64() {
        let instr = Instruction::decode(815958648u32);
        println!("{:032b}", 815958648u32);
        assert_eq!(
            instr, Instruction::S_Mul_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958648u32);
    }
    #[test]
    fn S_Neg_i8() {
        let instr = Instruction::decode(2147650184u32);
        println!("{:032b}", 2147650184u32);
        assert_eq!(
            instr, Instruction::S_Neg_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2147650184u32);
    }
    #[test]
    fn S_Neg_i16() {
        let instr = Instruction::decode(2416085640u32);
        println!("{:032b}", 2416085640u32);
        assert_eq!(
            instr, Instruction::S_Neg_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2416085640u32);
    }
    #[test]
    fn S_Neg_i32() {
        let instr = Instruction::decode(2684521096u32);
        println!("{:032b}", 2684521096u32);
        assert_eq!(
            instr, Instruction::S_Neg_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2684521096u32);
    }
    #[test]
    fn S_Neg_i64() {
        let instr = Instruction::decode(2952956552u32);
        println!("{:032b}", 2952956552u32);
        assert_eq!(
            instr, Instruction::S_Neg_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2952956552u32);
    }
    #[test]
    fn S_Pow_i8() {
        let instr = Instruction::decode(2158135960u32);
        println!("{:032b}", 2158135960u32);
        assert_eq!(
            instr, Instruction::S_Pow_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135960u32);
    }
    #[test]
    fn S_Pow_i16() {
        let instr = Instruction::decode(2426571416u32);
        println!("{:032b}", 2426571416u32);
        assert_eq!(
            instr, Instruction::S_Pow_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571416u32);
    }
    #[test]
    fn S_Pow_i32() {
        let instr = Instruction::decode(2695006872u32);
        println!("{:032b}", 2695006872u32);
        assert_eq!(
            instr, Instruction::S_Pow_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006872u32);
    }
    #[test]
    fn S_Pow_i64() {
        let instr = Instruction::decode(2963442328u32);
        println!("{:032b}", 2963442328u32);
        assert_eq!(
            instr, Instruction::S_Pow_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442328u32);
    }
    #[test]
    fn S_Pow_u8() {
        let instr = Instruction::decode(10652312u32);
        println!("{:032b}", 10652312u32);
        assert_eq!(
            instr, Instruction::S_Pow_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652312u32);
    }
    #[test]
    fn S_Pow_u16() {
        let instr = Instruction::decode(279087768u32);
        println!("{:032b}", 279087768u32);
        assert_eq!(
            instr, Instruction::S_Pow_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087768u32);
    }
    #[test]
    fn S_Pow_u32() {
        let instr = Instruction::decode(547523224u32);
        println!("{:032b}", 547523224u32);
        assert_eq!(
            instr, Instruction::S_Pow_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523224u32);
    }
    #[test]
    fn S_Pow_u64() {
        let instr = Instruction::decode(815958680u32);
        println!("{:032b}", 815958680u32);
        assert_eq!(
            instr, Instruction::S_Pow_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958680u32);
    }
    #[test]
    fn S_Sub_i8() {
        let instr = Instruction::decode(2158136040u32);
        println!("{:032b}", 2158136040u32);
        assert_eq!(
            instr, Instruction::S_Sub_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158136040u32);
    }
    #[test]
    fn S_Sub_i16() {
        let instr = Instruction::decode(2426571496u32);
        println!("{:032b}", 2426571496u32);
        assert_eq!(
            instr, Instruction::S_Sub_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571496u32);
    }
    #[test]
    fn S_Sub_i32() {
        let instr = Instruction::decode(2695006952u32);
        println!("{:032b}", 2695006952u32);
        assert_eq!(
            instr, Instruction::S_Sub_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006952u32);
    }
    #[test]
    fn S_Sub_i64() {
        let instr = Instruction::decode(2963442408u32);
        println!("{:032b}", 2963442408u32);
        assert_eq!(
            instr, Instruction::S_Sub_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442408u32);
    }
    #[test]
    fn S_Sub_u8() {
        let instr = Instruction::decode(10652392u32);
        println!("{:032b}", 10652392u32);
        assert_eq!(
            instr, Instruction::S_Sub_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652392u32);
    }
    #[test]
    fn S_Sub_u16() {
        let instr = Instruction::decode(279087848u32);
        println!("{:032b}", 279087848u32);
        assert_eq!(
            instr, Instruction::S_Sub_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087848u32);
    }
    #[test]
    fn S_Sub_u32() {
        let instr = Instruction::decode(547523304u32);
        println!("{:032b}", 547523304u32);
        assert_eq!(
            instr, Instruction::S_Sub_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523304u32);
    }
    #[test]
    fn S_Sub_u64() {
        let instr = Instruction::decode(815958760u32);
        println!("{:032b}", 815958760u32);
        assert_eq!(
            instr, Instruction::S_Sub_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958760u32);
    }
    #[test]
    fn S_Sub_U_i8() {
        let instr = Instruction::decode(2158136056u32);
        println!("{:032b}", 2158136056u32);
        assert_eq!(
            instr, Instruction::S_Sub_U_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158136056u32);
    }
    #[test]
    fn S_Sub_U_i16() {
        let instr = Instruction::decode(2426571512u32);
        println!("{:032b}", 2426571512u32);
        assert_eq!(
            instr, Instruction::S_Sub_U_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571512u32);
    }
    #[test]
    fn S_Sub_U_i32() {
        let instr = Instruction::decode(2695006968u32);
        println!("{:032b}", 2695006968u32);
        assert_eq!(
            instr, Instruction::S_Sub_U_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006968u32);
    }
    #[test]
    fn S_Sub_U_i64() {
        let instr = Instruction::decode(2963442424u32);
        println!("{:032b}", 2963442424u32);
        assert_eq!(
            instr, Instruction::S_Sub_U_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442424u32);
    }
    #[test]
    fn Abs_f32() {
        let instr = Instruction::decode(3758262793u32);
        println!("{:032b}", 3758262793u32);
        assert_eq!(
            instr, Instruction::Abs_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3758262793u32);
    }
    #[test]
    fn Abs_f64() {
        let instr = Instruction::decode(4026698249u32);
        println!("{:032b}", 4026698249u32);
        assert_eq!(
            instr, Instruction::Abs_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4026698249u32);
    }
    #[test]
    fn Add_f32() {
        let instr = Instruction::decode(3768748569u32);
        println!("{:032b}", 3768748569u32);
        assert_eq!(
            instr, Instruction::Add_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748569u32);
    }
    #[test]
    fn Add_f64() {
        let instr = Instruction::decode(4037184025u32);
        println!("{:032b}", 4037184025u32);
        assert_eq!(
            instr, Instruction::Add_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184025u32);
    }
    #[test]
    fn Div_f32() {
        let instr = Instruction::decode(3768748601u32);
        println!("{:032b}", 3768748601u32);
        assert_eq!(
            instr, Instruction::Div_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748601u32);
    }
    #[test]
    fn Div_f64() {
        let instr = Instruction::decode(4037184057u32);
        println!("{:032b}", 4037184057u32);
        assert_eq!(
            instr, Instruction::Div_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184057u32);
    }
    #[test]
    fn Div_E_f32() {
        let instr = Instruction::decode(3768748617u32);
        println!("{:032b}", 3768748617u32);
        assert_eq!(
            instr, Instruction::Div_E_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748617u32);
    }
    #[test]
    fn Div_E_f64() {
        let instr = Instruction::decode(4037184073u32);
        println!("{:032b}", 4037184073u32);
        assert_eq!(
            instr, Instruction::Div_E_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184073u32);
    }
    #[test]
    fn Log_f32() {
        let instr = Instruction::decode(3768748633u32);
        println!("{:032b}", 3768748633u32);
        assert_eq!(
            instr, Instruction::Log_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748633u32);
    }
    #[test]
    fn Log_f64() {
        let instr = Instruction::decode(4037184089u32);
        println!("{:032b}", 4037184089u32);
        assert_eq!(
            instr, Instruction::Log_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184089u32);
    }
    #[test]
    fn Sqrt_f32() {
        let instr = Instruction::decode(3758262889u32);
        println!("{:032b}", 3758262889u32);
        assert_eq!(
            instr, Instruction::Sqrt_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3758262889u32);
    }
    #[test]
    fn Sqrt_f64() {
        let instr = Instruction::decode(4026698345u32);
        println!("{:032b}", 4026698345u32);
        assert_eq!(
            instr, Instruction::Sqrt_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4026698345u32);
    }
    #[test]
    fn Mul_f32() {
        let instr = Instruction::decode(3768748665u32);
        println!("{:032b}", 3768748665u32);
        assert_eq!(
            instr, Instruction::Mul_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748665u32);
    }
    #[test]
    fn Mul_f64() {
        let instr = Instruction::decode(4037184121u32);
        println!("{:032b}", 4037184121u32);
        assert_eq!(
            instr, Instruction::Mul_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184121u32);
    }
    #[test]
    fn Neg_f32() {
        let instr = Instruction::decode(3758262921u32);
        println!("{:032b}", 3758262921u32);
        assert_eq!(
            instr, Instruction::Neg_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3758262921u32);
    }
    #[test]
    fn Neg_f64() {
        let instr = Instruction::decode(4026698377u32);
        println!("{:032b}", 4026698377u32);
        assert_eq!(
            instr, Instruction::Neg_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4026698377u32);
    }
    #[test]
    fn Pow_f32() {
        let instr = Instruction::decode(3768748697u32);
        println!("{:032b}", 3768748697u32);
        assert_eq!(
            instr, Instruction::Pow_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748697u32);
    }
    #[test]
    fn Pow_f64() {
        let instr = Instruction::decode(4037184153u32);
        println!("{:032b}", 4037184153u32);
        assert_eq!(
            instr, Instruction::Pow_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184153u32);
    }
    #[test]
    fn Rem_f32() {
        let instr = Instruction::decode(3768748713u32);
        println!("{:032b}", 3768748713u32);
        assert_eq!(
            instr, Instruction::Rem_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748713u32);
    }
    #[test]
    fn Rem_f64() {
        let instr = Instruction::decode(4037184169u32);
        println!("{:032b}", 4037184169u32);
        assert_eq!(
            instr, Instruction::Rem_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184169u32);
    }
    #[test]
    fn Rem_E_f32() {
        let instr = Instruction::decode(3768748729u32);
        println!("{:032b}", 3768748729u32);
        assert_eq!(
            instr, Instruction::Rem_E_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748729u32);
    }
    #[test]
    fn Rem_E_f64() {
        let instr = Instruction::decode(4037184185u32);
        println!("{:032b}", 4037184185u32);
        assert_eq!(
            instr, Instruction::Rem_E_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184185u32);
    }
    #[test]
    fn Cbrt_f32() {
        let instr = Instruction::decode(3758263001u32);
        println!("{:032b}", 3758263001u32);
        assert_eq!(
            instr, Instruction::Cbrt_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3758263001u32);
    }
    #[test]
    fn Cbrt_f64() {
        let instr = Instruction::decode(4026698457u32);
        println!("{:032b}", 4026698457u32);
        assert_eq!(
            instr, Instruction::Cbrt_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4026698457u32);
    }
    #[test]
    fn Sub_f32() {
        let instr = Instruction::decode(3768748777u32);
        println!("{:032b}", 3768748777u32);
        assert_eq!(
            instr, Instruction::Sub_f32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 3768748777u32);
    }
    #[test]
    fn Sub_f64() {
        let instr = Instruction::decode(4037184233u32);
        println!("{:032b}", 4037184233u32);
        assert_eq!(
            instr, Instruction::Sub_f64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 4037184233u32);
    }
    #[test]
    fn Fetch_Add_i8() {
        let instr = Instruction::decode(2158135818u32);
        println!("{:032b}", 2158135818u32);
        assert_eq!(
            instr, Instruction::Fetch_Add_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135818u32);
    }
    #[test]
    fn Fetch_Add_i16() {
        let instr = Instruction::decode(2426571274u32);
        println!("{:032b}", 2426571274u32);
        assert_eq!(
            instr, Instruction::Fetch_Add_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571274u32);
    }
    #[test]
    fn Fetch_Add_i32() {
        let instr = Instruction::decode(2695006730u32);
        println!("{:032b}", 2695006730u32);
        assert_eq!(
            instr, Instruction::Fetch_Add_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006730u32);
    }
    #[test]
    fn Fetch_Add_i64() {
        let instr = Instruction::decode(2963442186u32);
        println!("{:032b}", 2963442186u32);
        assert_eq!(
            instr, Instruction::Fetch_Add_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442186u32);
    }
    #[test]
    fn Fetch_Add_u8() {
        let instr = Instruction::decode(10652170u32);
        println!("{:032b}", 10652170u32);
        assert_eq!(
            instr, Instruction::Fetch_Add_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652170u32);
    }
    #[test]
    fn Fetch_Add_u16() {
        let instr = Instruction::decode(279087626u32);
        println!("{:032b}", 279087626u32);
        assert_eq!(
            instr, Instruction::Fetch_Add_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087626u32);
    }
    #[test]
    fn Fetch_Add_u32() {
        let instr = Instruction::decode(547523082u32);
        println!("{:032b}", 547523082u32);
        assert_eq!(
            instr, Instruction::Fetch_Add_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523082u32);
    }
    #[test]
    fn Fetch_Add_u64() {
        let instr = Instruction::decode(815958538u32);
        println!("{:032b}", 815958538u32);
        assert_eq!(
            instr, Instruction::Fetch_Add_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958538u32);
    }
    #[test]
    fn Fetch_Sub_i8() {
        let instr = Instruction::decode(2158135834u32);
        println!("{:032b}", 2158135834u32);
        assert_eq!(
            instr, Instruction::Fetch_Sub_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135834u32);
    }
    #[test]
    fn Fetch_Sub_i16() {
        let instr = Instruction::decode(2426571290u32);
        println!("{:032b}", 2426571290u32);
        assert_eq!(
            instr, Instruction::Fetch_Sub_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571290u32);
    }
    #[test]
    fn Fetch_Sub_i32() {
        let instr = Instruction::decode(2695006746u32);
        println!("{:032b}", 2695006746u32);
        assert_eq!(
            instr, Instruction::Fetch_Sub_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006746u32);
    }
    #[test]
    fn Fetch_Sub_i64() {
        let instr = Instruction::decode(2963442202u32);
        println!("{:032b}", 2963442202u32);
        assert_eq!(
            instr, Instruction::Fetch_Sub_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442202u32);
    }
    #[test]
    fn Fetch_Sub_u8() {
        let instr = Instruction::decode(10652186u32);
        println!("{:032b}", 10652186u32);
        assert_eq!(
            instr, Instruction::Fetch_Sub_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652186u32);
    }
    #[test]
    fn Fetch_Sub_u16() {
        let instr = Instruction::decode(279087642u32);
        println!("{:032b}", 279087642u32);
        assert_eq!(
            instr, Instruction::Fetch_Sub_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087642u32);
    }
    #[test]
    fn Fetch_Sub_u32() {
        let instr = Instruction::decode(547523098u32);
        println!("{:032b}", 547523098u32);
        assert_eq!(
            instr, Instruction::Fetch_Sub_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523098u32);
    }
    #[test]
    fn Fetch_Sub_u64() {
        let instr = Instruction::decode(815958554u32);
        println!("{:032b}", 815958554u32);
        assert_eq!(
            instr, Instruction::Fetch_Sub_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958554u32);
    }
    #[test]
    fn Fetch_Min_i8() {
        let instr = Instruction::decode(2158135850u32);
        println!("{:032b}", 2158135850u32);
        assert_eq!(
            instr, Instruction::Fetch_Min_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135850u32);
    }
    #[test]
    fn Fetch_Min_i16() {
        let instr = Instruction::decode(2426571306u32);
        println!("{:032b}", 2426571306u32);
        assert_eq!(
            instr, Instruction::Fetch_Min_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571306u32);
    }
    #[test]
    fn Fetch_Min_i32() {
        let instr = Instruction::decode(2695006762u32);
        println!("{:032b}", 2695006762u32);
        assert_eq!(
            instr, Instruction::Fetch_Min_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006762u32);
    }
    #[test]
    fn Fetch_Min_i64() {
        let instr = Instruction::decode(2963442218u32);
        println!("{:032b}", 2963442218u32);
        assert_eq!(
            instr, Instruction::Fetch_Min_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442218u32);
    }
    #[test]
    fn Fetch_Min_u8() {
        let instr = Instruction::decode(10652202u32);
        println!("{:032b}", 10652202u32);
        assert_eq!(
            instr, Instruction::Fetch_Min_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652202u32);
    }
    #[test]
    fn Fetch_Min_u16() {
        let instr = Instruction::decode(279087658u32);
        println!("{:032b}", 279087658u32);
        assert_eq!(
            instr, Instruction::Fetch_Min_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087658u32);
    }
    #[test]
    fn Fetch_Min_u32() {
        let instr = Instruction::decode(547523114u32);
        println!("{:032b}", 547523114u32);
        assert_eq!(
            instr, Instruction::Fetch_Min_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523114u32);
    }
    #[test]
    fn Fetch_Min_u64() {
        let instr = Instruction::decode(815958570u32);
        println!("{:032b}", 815958570u32);
        assert_eq!(
            instr, Instruction::Fetch_Min_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958570u32);
    }
    #[test]
    fn Fetch_Max_i8() {
        let instr = Instruction::decode(2158135866u32);
        println!("{:032b}", 2158135866u32);
        assert_eq!(
            instr, Instruction::Fetch_Max_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135866u32);
    }
    #[test]
    fn Fetch_Max_i16() {
        let instr = Instruction::decode(2426571322u32);
        println!("{:032b}", 2426571322u32);
        assert_eq!(
            instr, Instruction::Fetch_Max_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571322u32);
    }
    #[test]
    fn Fetch_Max_i32() {
        let instr = Instruction::decode(2695006778u32);
        println!("{:032b}", 2695006778u32);
        assert_eq!(
            instr, Instruction::Fetch_Max_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006778u32);
    }
    #[test]
    fn Fetch_Max_i64() {
        let instr = Instruction::decode(2963442234u32);
        println!("{:032b}", 2963442234u32);
        assert_eq!(
            instr, Instruction::Fetch_Max_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442234u32);
    }
    #[test]
    fn Fetch_Max_u8() {
        let instr = Instruction::decode(10652218u32);
        println!("{:032b}", 10652218u32);
        assert_eq!(
            instr, Instruction::Fetch_Max_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652218u32);
    }
    #[test]
    fn Fetch_Max_u16() {
        let instr = Instruction::decode(279087674u32);
        println!("{:032b}", 279087674u32);
        assert_eq!(
            instr, Instruction::Fetch_Max_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087674u32);
    }
    #[test]
    fn Fetch_Max_u32() {
        let instr = Instruction::decode(547523130u32);
        println!("{:032b}", 547523130u32);
        assert_eq!(
            instr, Instruction::Fetch_Max_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523130u32);
    }
    #[test]
    fn Fetch_Max_u64() {
        let instr = Instruction::decode(815958586u32);
        println!("{:032b}", 815958586u32);
        assert_eq!(
            instr, Instruction::Fetch_Max_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958586u32);
    }
    #[test]
    fn Fetch_And_i8() {
        let instr = Instruction::decode(2158135882u32);
        println!("{:032b}", 2158135882u32);
        assert_eq!(
            instr, Instruction::Fetch_And_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135882u32);
    }
    #[test]
    fn Fetch_And_i16() {
        let instr = Instruction::decode(2426571338u32);
        println!("{:032b}", 2426571338u32);
        assert_eq!(
            instr, Instruction::Fetch_And_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571338u32);
    }
    #[test]
    fn Fetch_And_i32() {
        let instr = Instruction::decode(2695006794u32);
        println!("{:032b}", 2695006794u32);
        assert_eq!(
            instr, Instruction::Fetch_And_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006794u32);
    }
    #[test]
    fn Fetch_And_i64() {
        let instr = Instruction::decode(2963442250u32);
        println!("{:032b}", 2963442250u32);
        assert_eq!(
            instr, Instruction::Fetch_And_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442250u32);
    }
    #[test]
    fn Fetch_And_u8() {
        let instr = Instruction::decode(10652234u32);
        println!("{:032b}", 10652234u32);
        assert_eq!(
            instr, Instruction::Fetch_And_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652234u32);
    }
    #[test]
    fn Fetch_And_u16() {
        let instr = Instruction::decode(279087690u32);
        println!("{:032b}", 279087690u32);
        assert_eq!(
            instr, Instruction::Fetch_And_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087690u32);
    }
    #[test]
    fn Fetch_And_u32() {
        let instr = Instruction::decode(547523146u32);
        println!("{:032b}", 547523146u32);
        assert_eq!(
            instr, Instruction::Fetch_And_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523146u32);
    }
    #[test]
    fn Fetch_And_u64() {
        let instr = Instruction::decode(815958602u32);
        println!("{:032b}", 815958602u32);
        assert_eq!(
            instr, Instruction::Fetch_And_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958602u32);
    }
    #[test]
    fn Fetch_Nand_i8() {
        let instr = Instruction::decode(2158135898u32);
        println!("{:032b}", 2158135898u32);
        assert_eq!(
            instr, Instruction::Fetch_Nand_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135898u32);
    }
    #[test]
    fn Fetch_Nand_i16() {
        let instr = Instruction::decode(2426571354u32);
        println!("{:032b}", 2426571354u32);
        assert_eq!(
            instr, Instruction::Fetch_Nand_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571354u32);
    }
    #[test]
    fn Fetch_Nand_i32() {
        let instr = Instruction::decode(2695006810u32);
        println!("{:032b}", 2695006810u32);
        assert_eq!(
            instr, Instruction::Fetch_Nand_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006810u32);
    }
    #[test]
    fn Fetch_Nand_i64() {
        let instr = Instruction::decode(2963442266u32);
        println!("{:032b}", 2963442266u32);
        assert_eq!(
            instr, Instruction::Fetch_Nand_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442266u32);
    }
    #[test]
    fn Fetch_Nand_u8() {
        let instr = Instruction::decode(10652250u32);
        println!("{:032b}", 10652250u32);
        assert_eq!(
            instr, Instruction::Fetch_Nand_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652250u32);
    }
    #[test]
    fn Fetch_Nand_u16() {
        let instr = Instruction::decode(279087706u32);
        println!("{:032b}", 279087706u32);
        assert_eq!(
            instr, Instruction::Fetch_Nand_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087706u32);
    }
    #[test]
    fn Fetch_Nand_u32() {
        let instr = Instruction::decode(547523162u32);
        println!("{:032b}", 547523162u32);
        assert_eq!(
            instr, Instruction::Fetch_Nand_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523162u32);
    }
    #[test]
    fn Fetch_Nand_u64() {
        let instr = Instruction::decode(815958618u32);
        println!("{:032b}", 815958618u32);
        assert_eq!(
            instr, Instruction::Fetch_Nand_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958618u32);
    }
    #[test]
    fn Fetch_Or_i8() {
        let instr = Instruction::decode(2158135914u32);
        println!("{:032b}", 2158135914u32);
        assert_eq!(
            instr, Instruction::Fetch_Or_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135914u32);
    }
    #[test]
    fn Fetch_Or_i16() {
        let instr = Instruction::decode(2426571370u32);
        println!("{:032b}", 2426571370u32);
        assert_eq!(
            instr, Instruction::Fetch_Or_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571370u32);
    }
    #[test]
    fn Fetch_Or_i32() {
        let instr = Instruction::decode(2695006826u32);
        println!("{:032b}", 2695006826u32);
        assert_eq!(
            instr, Instruction::Fetch_Or_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006826u32);
    }
    #[test]
    fn Fetch_Or_i64() {
        let instr = Instruction::decode(2963442282u32);
        println!("{:032b}", 2963442282u32);
        assert_eq!(
            instr, Instruction::Fetch_Or_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442282u32);
    }
    #[test]
    fn Fetch_Or_u8() {
        let instr = Instruction::decode(10652266u32);
        println!("{:032b}", 10652266u32);
        assert_eq!(
            instr, Instruction::Fetch_Or_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652266u32);
    }
    #[test]
    fn Fetch_Or_u16() {
        let instr = Instruction::decode(279087722u32);
        println!("{:032b}", 279087722u32);
        assert_eq!(
            instr, Instruction::Fetch_Or_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087722u32);
    }
    #[test]
    fn Fetch_Or_u32() {
        let instr = Instruction::decode(547523178u32);
        println!("{:032b}", 547523178u32);
        assert_eq!(
            instr, Instruction::Fetch_Or_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523178u32);
    }
    #[test]
    fn Fetch_Or_u64() {
        let instr = Instruction::decode(815958634u32);
        println!("{:032b}", 815958634u32);
        assert_eq!(
            instr, Instruction::Fetch_Or_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958634u32);
    }
    #[test]
    fn Fetch_Xor_i8() {
        let instr = Instruction::decode(2158135930u32);
        println!("{:032b}", 2158135930u32);
        assert_eq!(
            instr, Instruction::Fetch_Xor_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135930u32);
    }
    #[test]
    fn Fetch_Xor_i16() {
        let instr = Instruction::decode(2426571386u32);
        println!("{:032b}", 2426571386u32);
        assert_eq!(
            instr, Instruction::Fetch_Xor_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571386u32);
    }
    #[test]
    fn Fetch_Xor_i32() {
        let instr = Instruction::decode(2695006842u32);
        println!("{:032b}", 2695006842u32);
        assert_eq!(
            instr, Instruction::Fetch_Xor_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006842u32);
    }
    #[test]
    fn Fetch_Xor_i64() {
        let instr = Instruction::decode(2963442298u32);
        println!("{:032b}", 2963442298u32);
        assert_eq!(
            instr, Instruction::Fetch_Xor_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442298u32);
    }
    #[test]
    fn Fetch_Xor_u8() {
        let instr = Instruction::decode(10652282u32);
        println!("{:032b}", 10652282u32);
        assert_eq!(
            instr, Instruction::Fetch_Xor_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652282u32);
    }
    #[test]
    fn Fetch_Xor_u16() {
        let instr = Instruction::decode(279087738u32);
        println!("{:032b}", 279087738u32);
        assert_eq!(
            instr, Instruction::Fetch_Xor_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087738u32);
    }
    #[test]
    fn Fetch_Xor_u32() {
        let instr = Instruction::decode(547523194u32);
        println!("{:032b}", 547523194u32);
        assert_eq!(
            instr, Instruction::Fetch_Xor_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523194u32);
    }
    #[test]
    fn Fetch_Xor_u64() {
        let instr = Instruction::decode(815958650u32);
        println!("{:032b}", 815958650u32);
        assert_eq!(
            instr, Instruction::Fetch_Xor_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958650u32);
    }
    #[test]
    fn Cmp_Exchg_i8() {
        let instr = Instruction::decode(2158135946u32);
        println!("{:032b}", 2158135946u32);
        assert_eq!(
            instr, Instruction::Cmp_Exchg_i8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2158135946u32);
    }
    #[test]
    fn Cmp_Exchg_i16() {
        let instr = Instruction::decode(2426571402u32);
        println!("{:032b}", 2426571402u32);
        assert_eq!(
            instr, Instruction::Cmp_Exchg_i16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2426571402u32);
    }
    #[test]
    fn Cmp_Exchg_i32() {
        let instr = Instruction::decode(2695006858u32);
        println!("{:032b}", 2695006858u32);
        assert_eq!(
            instr, Instruction::Cmp_Exchg_i32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2695006858u32);
    }
    #[test]
    fn Cmp_Exchg_i64() {
        let instr = Instruction::decode(2963442314u32);
        println!("{:032b}", 2963442314u32);
        assert_eq!(
            instr, Instruction::Cmp_Exchg_i64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 2963442314u32);
    }
    #[test]
    fn Cmp_Exchg_u8() {
        let instr = Instruction::decode(10652298u32);
        println!("{:032b}", 10652298u32);
        assert_eq!(
            instr, Instruction::Cmp_Exchg_u8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 10652298u32);
    }
    #[test]
    fn Cmp_Exchg_u16() {
        let instr = Instruction::decode(279087754u32);
        println!("{:032b}", 279087754u32);
        assert_eq!(
            instr, Instruction::Cmp_Exchg_u16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 279087754u32);
    }
    #[test]
    fn Cmp_Exchg_u32() {
        let instr = Instruction::decode(547523210u32);
        println!("{:032b}", 547523210u32);
        assert_eq!(
            instr, Instruction::Cmp_Exchg_u32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 547523210u32);
    }
    #[test]
    fn Cmp_Exchg_u64() {
        let instr = Instruction::decode(815958666u32);
        println!("{:032b}", 815958666u32);
        assert_eq!(
            instr, Instruction::Cmp_Exchg_u64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), rs2 : Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 815958666u32);
    }
    #[test]
    fn Atomic_Ld_8() {
        let instr = Instruction::decode(166554u32);
        println!("{:032b}", 166554u32);
        assert_eq!(
            instr, Instruction::Atomic_Ld_8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166554u32);
    }
    #[test]
    fn Atomic_Ld_16() {
        let instr = Instruction::decode(268602010u32);
        println!("{:032b}", 268602010u32);
        assert_eq!(
            instr, Instruction::Atomic_Ld_16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268602010u32);
    }
    #[test]
    fn Atomic_Ld_32() {
        let instr = Instruction::decode(537037466u32);
        println!("{:032b}", 537037466u32);
        assert_eq!(
            instr, Instruction::Atomic_Ld_32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037466u32);
    }
    #[test]
    fn Atomic_Ld_64() {
        let instr = Instruction::decode(805472922u32);
        println!("{:032b}", 805472922u32);
        assert_eq!(
            instr, Instruction::Atomic_Ld_64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805472922u32);
    }
    #[test]
    fn Atomic_St_8() {
        let instr = Instruction::decode(166570u32);
        println!("{:032b}", 166570u32);
        assert_eq!(
            instr, Instruction::Atomic_St_8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166570u32);
    }
    #[test]
    fn Atomic_St_16() {
        let instr = Instruction::decode(268602026u32);
        println!("{:032b}", 268602026u32);
        assert_eq!(
            instr, Instruction::Atomic_St_16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268602026u32);
    }
    #[test]
    fn Atomic_St_32() {
        let instr = Instruction::decode(537037482u32);
        println!("{:032b}", 537037482u32);
        assert_eq!(
            instr, Instruction::Atomic_St_32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037482u32);
    }
    #[test]
    fn Atomic_St_64() {
        let instr = Instruction::decode(805472938u32);
        println!("{:032b}", 805472938u32);
        assert_eq!(
            instr, Instruction::Atomic_St_64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805472938u32);
    }
    #[test]
    fn Spawn() {
        let instr = Instruction::decode(330426u32);
        println!("{:032b}", 330426u32);
        assert_eq!(
            instr, Instruction::Spawn { rd : Register::from_index(10), imm : 10, }
        );
        assert_eq!(instr.encode(), 330426u32);
    }
    #[test]
    fn Wait() {
        let instr = Instruction::decode(166602u32);
        println!("{:032b}", 166602u32);
        assert_eq!(
            instr, Instruction::Wait { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166602u32);
    }
    #[test]
    fn Notify() {
        let instr = Instruction::decode(166618u32);
        println!("{:032b}", 166618u32);
        assert_eq!(
            instr, Instruction::Notify { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166618u32);
    }
    #[test]
    fn Swap_8() {
        let instr = Instruction::decode(166634u32);
        println!("{:032b}", 166634u32);
        assert_eq!(
            instr, Instruction::Swap_8 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 166634u32);
    }
    #[test]
    fn Swap_16() {
        let instr = Instruction::decode(268602090u32);
        println!("{:032b}", 268602090u32);
        assert_eq!(
            instr, Instruction::Swap_16 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 268602090u32);
    }
    #[test]
    fn Swap_32() {
        let instr = Instruction::decode(537037546u32);
        println!("{:032b}", 537037546u32);
        assert_eq!(
            instr, Instruction::Swap_32 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 537037546u32);
    }
    #[test]
    fn Swap_64() {
        let instr = Instruction::decode(805473002u32);
        println!("{:032b}", 805473002u32);
        assert_eq!(
            instr, Instruction::Swap_64 { rd : Register::from_index(10), rs1 :
            Register::from_index(10), }
        );
        assert_eq!(instr.encode(), 805473002u32);
    }
}
