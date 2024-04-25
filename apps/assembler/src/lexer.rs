use logos::{Lexer, Logos, SpannedIter};
use crate::token::{Token, Token_Kind, Token_Value, SIGNED};
#[derive(Logos)]
#[derive(Debug, Clone, Copy)]
#[logos(skip r"[ \t\f]+")]
#[rustfmt::skip]
enum Lexed_Token {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex(r"[\n\r]+")]
    Newline,
    #[regex(r"[0-9]+", parse_dec)]
    #[regex(r"0x[0-9a-fA-F]+", parse_hex)]
    #[regex(r"0b[01]+", parse_bin)]
    #[regex(r"0o[0-7]+", parse_oct)]
    Integer(u64),
    #[token("zero", |_|0)]
    #[token("one", |_|1)]
    #[token("ip", |_|2)]
    #[token("sp", |_|3)]
    #[token("fp", |_|4)]
    #[token("link", |_|5)]
    #[regex("r[0-9]+", parse_register_index)]
    Register(u64),
    #[token("[code]")]
    Code,
    #[token("[data]")]
    Data,
    #[token("halt")]
    Halt,
    #[token("trap")]
    Trap,
    #[token("call")]
    Call,
    #[token("callr")]
    Callr,
    #[token("calli")]
    Calli,
    #[token("ret")]
    Ret,
    #[token("ecall")]
    Ecall,
    #[token("break")]
    Break,
    #[token("jal")]
    Jal,
    #[token("jalr")]
    Jalr,
    #[token("jali")]
    Jali,
    #[token("jnz")]
    Jnz,
    #[token("jnzr")]
    Jnzr,
    #[token("jnzi")]
    Jnzi,
    #[token("jiz")]
    Jiz,
    #[token("jizr")]
    Jizr,
    #[token("jizi")]
    Jizi,
    #[token("load.8")]
    Load_8,
    #[token("load.16")]
    Load_16,
    #[token("load.32")]
    Load_32,
    #[token("load.64")]
    Load_64,
    #[token("loadi")]
    Loadi,
    #[token("loada.8")]
    Loada_8,
    #[token("loada.16")]
    Loada_16,
    #[token("loada.32")]
    Loada_32,
    #[token("loada.64")]
    Loada_64,
    #[token("store.8")]
    Store_8,
    #[token("store.16")]
    Store_16,
    #[token("store.32")]
    Store_32,
    #[token("store.64")]
    Store_64,
    #[token("storei")]
    Storei,
    #[token("move")]
    Move,
    #[token("push")]
    Push,
    #[token("pushi")]
    Pushi,
    #[token("pop")]
    Pop,
    #[token("ie")]
    Ie,
    #[token("ie.f32")]
    Ie_f32,
    #[token("ie.f64")]
    Ie_f64,
    #[token("ne")]
    Ne,
    #[token("ne.f32")]
    Ne_f32,
    #[token("ne.f64")]
    Ne_f64,
    #[token("lt")]
    Lt,
    #[token("lt.f32")]
    Lt_f32,
    #[token("lt.f64")]
    Lt_f64,
    #[token("le")]
    Le,
    #[token("le.f32")]
    Le_f32,
    #[token("le.f64")]
    Le_f64,
    #[token("gt")]
    Gt,
    #[token("gt.f32")]
    Gt_f32,
    #[token("gt.f64")]
    Gt_f64,
    #[token("ge")]
    Ge,
    #[token("ge.f32")]
    Ge_f32,
    #[token("ge.f64")]
    Ge_f64,
    #[token("and.i8")]
    And_i8,
    #[token("and.i16")]
    And_i16,
    #[token("and.i32")]
    And_i32,
    #[token("and.i64")]
    And_i64,
    #[token("and.u8")]
    And_u8,
    #[token("and.u16")]
    And_u16,
    #[token("and.u32")]
    And_u32,
    #[token("and.u64")]
    And_u64,
    #[token("or.i8")]
    Or_i8,
    #[token("or.i16")]
    Or_i16,
    #[token("or.i32")]
    Or_i32,
    #[token("or.i64")]
    Or_i64,
    #[token("or.u8")]
    Or_u8,
    #[token("or.u16")]
    Or_u16,
    #[token("or.u32")]
    Or_u32,
    #[token("or.u64")]
    Or_u64,
    #[token("xor.i8")]
    Xor_i8,
    #[token("xor.i16")]
    Xor_i16,
    #[token("xor.i32")]
    Xor_i32,
    #[token("xor.i64")]
    Xor_i64,
    #[token("xor.u8")]
    Xor_u8,
    #[token("xor.u16")]
    Xor_u16,
    #[token("xor.u32")]
    Xor_u32,
    #[token("xor.u64")]
    Xor_u64,
    #[token("not.i8")]
    Not_i8,
    #[token("not.i16")]
    Not_i16,
    #[token("not.i32")]
    Not_i32,
    #[token("not.i64")]
    Not_i64,
    #[token("not.u8")]
    Not_u8,
    #[token("not.u16")]
    Not_u16,
    #[token("not.u32")]
    Not_u32,
    #[token("not.u64")]
    Not_u64,
    #[token("shl.i8")]
    Shl_i8,
    #[token("shl.i16")]
    Shl_i16,
    #[token("shl.i32")]
    Shl_i32,
    #[token("shl.i64")]
    Shl_i64,
    #[token("shl.u8")]
    Shl_u8,
    #[token("shl.u16")]
    Shl_u16,
    #[token("shl.u32")]
    Shl_u32,
    #[token("shl.u64")]
    Shl_u64,
    #[token("shr.i8")]
    Shr_i8,
    #[token("shr.i16")]
    Shr_i16,
    #[token("shr.i32")]
    Shr_i32,
    #[token("shr.i64")]
    Shr_i64,
    #[token("shr.u8")]
    Shr_u8,
    #[token("shr.u16")]
    Shr_u16,
    #[token("shr.u32")]
    Shr_u32,
    #[token("shr.u64")]
    Shr_u64,
    #[token("rotl.i8")]
    Rotl_i8,
    #[token("rotl.i16")]
    Rotl_i16,
    #[token("rotl.i32")]
    Rotl_i32,
    #[token("rotl.i64")]
    Rotl_i64,
    #[token("rotl.u8")]
    Rotl_u8,
    #[token("rotl.u16")]
    Rotl_u16,
    #[token("rotl.u32")]
    Rotl_u32,
    #[token("rotl.u64")]
    Rotl_u64,
    #[token("rotr.i8")]
    Rotr_i8,
    #[token("rotr.i16")]
    Rotr_i16,
    #[token("rotr.i32")]
    Rotr_i32,
    #[token("rotr.i64")]
    Rotr_i64,
    #[token("rotr.u8")]
    Rotr_u8,
    #[token("rotr.u16")]
    Rotr_u16,
    #[token("rotr.u32")]
    Rotr_u32,
    #[token("rotr.u64")]
    Rotr_u64,
    #[token("count_ones.i8")]
    Count_Ones_i8,
    #[token("count_ones.i16")]
    Count_Ones_i16,
    #[token("count_ones.i32")]
    Count_Ones_i32,
    #[token("count_ones.i64")]
    Count_Ones_i64,
    #[token("count_ones.u8")]
    Count_Ones_u8,
    #[token("count_ones.u16")]
    Count_Ones_u16,
    #[token("count_ones.u32")]
    Count_Ones_u32,
    #[token("count_ones.u64")]
    Count_Ones_u64,
    #[token("leading_ones.i8")]
    Leading_Ones_i8,
    #[token("leading_ones.i16")]
    Leading_Ones_i16,
    #[token("leading_ones.i32")]
    Leading_Ones_i32,
    #[token("leading_ones.i64")]
    Leading_Ones_i64,
    #[token("leading_ones.u8")]
    Leading_Ones_u8,
    #[token("leading_ones.u16")]
    Leading_Ones_u16,
    #[token("leading_ones.u32")]
    Leading_Ones_u32,
    #[token("leading_ones.u64")]
    Leading_Ones_u64,
    #[token("trailing_ones.i8")]
    Trailing_Ones_i8,
    #[token("trailing_ones.i16")]
    Trailing_Ones_i16,
    #[token("trailing_ones.i32")]
    Trailing_Ones_i32,
    #[token("trailing_ones.i64")]
    Trailing_Ones_i64,
    #[token("trailing_ones.u8")]
    Trailing_Ones_u8,
    #[token("trailing_ones.u16")]
    Trailing_Ones_u16,
    #[token("trailing_ones.u32")]
    Trailing_Ones_u32,
    #[token("trailing_ones.u64")]
    Trailing_Ones_u64,
    #[token("count_zeros.i8")]
    Count_Zeros_i8,
    #[token("count_zeros.i16")]
    Count_Zeros_i16,
    #[token("count_zeros.i32")]
    Count_Zeros_i32,
    #[token("count_zeros.i64")]
    Count_Zeros_i64,
    #[token("count_zeros.u8")]
    Count_Zeros_u8,
    #[token("count_zeros.u16")]
    Count_Zeros_u16,
    #[token("count_zeros.u32")]
    Count_Zeros_u32,
    #[token("count_zeros.u64")]
    Count_Zeros_u64,
    #[token("leading_zeros.i8")]
    Leading_Zeros_i8,
    #[token("leading_zeros.i16")]
    Leading_Zeros_i16,
    #[token("leading_zeros.i32")]
    Leading_Zeros_i32,
    #[token("leading_zeros.i64")]
    Leading_Zeros_i64,
    #[token("leading_zeros.u8")]
    Leading_Zeros_u8,
    #[token("leading_zeros.u16")]
    Leading_Zeros_u16,
    #[token("leading_zeros.u32")]
    Leading_Zeros_u32,
    #[token("leading_zeros.u64")]
    Leading_Zeros_u64,
    #[token("trailing_zeros.i8")]
    Trailing_Zeros_i8,
    #[token("trailing_zeros.i16")]
    Trailing_Zeros_i16,
    #[token("trailing_zeros.i32")]
    Trailing_Zeros_i32,
    #[token("trailing_zeros.i64")]
    Trailing_Zeros_i64,
    #[token("trailing_zeros.u8")]
    Trailing_Zeros_u8,
    #[token("trailing_zeros.u16")]
    Trailing_Zeros_u16,
    #[token("trailing_zeros.u32")]
    Trailing_Zeros_u32,
    #[token("trailing_zeros.u64")]
    Trailing_Zeros_u64,
    #[token("reverse_bytes.i8")]
    Reverse_Bytes_i8,
    #[token("reverse_bytes.i16")]
    Reverse_Bytes_i16,
    #[token("reverse_bytes.i32")]
    Reverse_Bytes_i32,
    #[token("reverse_bytes.i64")]
    Reverse_Bytes_i64,
    #[token("reverse_bytes.u8")]
    Reverse_Bytes_u8,
    #[token("reverse_bytes.u16")]
    Reverse_Bytes_u16,
    #[token("reverse_bytes.u32")]
    Reverse_Bytes_u32,
    #[token("reverse_bytes.u64")]
    Reverse_Bytes_u64,
    #[token("reverse_bits.i8")]
    Reverse_Bits_i8,
    #[token("reverse_bits.i16")]
    Reverse_Bits_i16,
    #[token("reverse_bits.i32")]
    Reverse_Bits_i32,
    #[token("reverse_bits.i64")]
    Reverse_Bits_i64,
    #[token("reverse_bits.u8")]
    Reverse_Bits_u8,
    #[token("reverse_bits.u16")]
    Reverse_Bits_u16,
    #[token("reverse_bits.u32")]
    Reverse_Bits_u32,
    #[token("reverse_bits.u64")]
    Reverse_Bits_u64,
    #[token("c_abs.i8")]
    C_Abs_i8,
    #[token("c_abs.i16")]
    C_Abs_i16,
    #[token("c_abs.i32")]
    C_Abs_i32,
    #[token("c_abs.i64")]
    C_Abs_i64,
    #[token("c_add.i8")]
    C_Add_i8,
    #[token("c_add.i16")]
    C_Add_i16,
    #[token("c_add.i32")]
    C_Add_i32,
    #[token("c_add.i64")]
    C_Add_i64,
    #[token("c_add.u8")]
    C_Add_u8,
    #[token("c_add.u16")]
    C_Add_u16,
    #[token("c_add.u32")]
    C_Add_u32,
    #[token("c_add.u64")]
    C_Add_u64,
    #[token("c_add_u.i8")]
    C_Add_U_i8,
    #[token("c_add_u.i16")]
    C_Add_U_i16,
    #[token("c_add_u.i32")]
    C_Add_U_i32,
    #[token("c_add_u.i64")]
    C_Add_U_i64,
    #[token("c_add_s.u8")]
    C_Add_S_u8,
    #[token("c_add_s.u16")]
    C_Add_S_u16,
    #[token("c_add_s.u32")]
    C_Add_S_u32,
    #[token("c_add_s.u64")]
    C_Add_S_u64,
    #[token("c_div.i8")]
    C_Div_i8,
    #[token("c_div.i16")]
    C_Div_i16,
    #[token("c_div.i32")]
    C_Div_i32,
    #[token("c_div.i64")]
    C_Div_i64,
    #[token("c_div.u8")]
    C_Div_u8,
    #[token("c_div.u16")]
    C_Div_u16,
    #[token("c_div.u32")]
    C_Div_u32,
    #[token("c_div.u64")]
    C_Div_u64,
    #[token("c_div_e.i8")]
    C_Div_E_i8,
    #[token("c_div_e.i16")]
    C_Div_E_i16,
    #[token("c_div_e.i32")]
    C_Div_E_i32,
    #[token("c_div_e.i64")]
    C_Div_E_i64,
    #[token("c_div_e.u8")]
    C_Div_E_u8,
    #[token("c_div_e.u16")]
    C_Div_E_u16,
    #[token("c_div_e.u32")]
    C_Div_E_u32,
    #[token("c_div_e.u64")]
    C_Div_E_u64,
    #[token("c_log.i8")]
    C_Log_i8,
    #[token("c_log.i16")]
    C_Log_i16,
    #[token("c_log.i32")]
    C_Log_i32,
    #[token("c_log.i64")]
    C_Log_i64,
    #[token("c_log.u8")]
    C_Log_u8,
    #[token("c_log.u16")]
    C_Log_u16,
    #[token("c_log.u32")]
    C_Log_u32,
    #[token("c_log.u64")]
    C_Log_u64,
    #[token("c_sqrt.i8")]
    C_Sqrt_i8,
    #[token("c_sqrt.i16")]
    C_Sqrt_i16,
    #[token("c_sqrt.i32")]
    C_Sqrt_i32,
    #[token("c_sqrt.i64")]
    C_Sqrt_i64,
    #[token("c_sqrt.u8")]
    C_Sqrt_u8,
    #[token("c_sqrt.u16")]
    C_Sqrt_u16,
    #[token("c_sqrt.u32")]
    C_Sqrt_u32,
    #[token("c_sqrt.u64")]
    C_Sqrt_u64,
    #[token("c_mul.i8")]
    C_Mul_i8,
    #[token("c_mul.i16")]
    C_Mul_i16,
    #[token("c_mul.i32")]
    C_Mul_i32,
    #[token("c_mul.i64")]
    C_Mul_i64,
    #[token("c_mul.u8")]
    C_Mul_u8,
    #[token("c_mul.u16")]
    C_Mul_u16,
    #[token("c_mul.u32")]
    C_Mul_u32,
    #[token("c_mul.u64")]
    C_Mul_u64,
    #[token("c_neg.i8")]
    C_Neg_i8,
    #[token("c_neg.i16")]
    C_Neg_i16,
    #[token("c_neg.i32")]
    C_Neg_i32,
    #[token("c_neg.i64")]
    C_Neg_i64,
    #[token("c_pow.i8")]
    C_Pow_i8,
    #[token("c_pow.i16")]
    C_Pow_i16,
    #[token("c_pow.i32")]
    C_Pow_i32,
    #[token("c_pow.i64")]
    C_Pow_i64,
    #[token("c_pow.u8")]
    C_Pow_u8,
    #[token("c_pow.u16")]
    C_Pow_u16,
    #[token("c_pow.u32")]
    C_Pow_u32,
    #[token("c_pow.u64")]
    C_Pow_u64,
    #[token("c_rem.i8")]
    C_Rem_i8,
    #[token("c_rem.i16")]
    C_Rem_i16,
    #[token("c_rem.i32")]
    C_Rem_i32,
    #[token("c_rem.i64")]
    C_Rem_i64,
    #[token("c_rem.u8")]
    C_Rem_u8,
    #[token("c_rem.u16")]
    C_Rem_u16,
    #[token("c_rem.u32")]
    C_Rem_u32,
    #[token("c_rem.u64")]
    C_Rem_u64,
    #[token("c_rem_e.i8")]
    C_Rem_E_i8,
    #[token("c_rem_e.i16")]
    C_Rem_E_i16,
    #[token("c_rem_e.i32")]
    C_Rem_E_i32,
    #[token("c_rem_e.i64")]
    C_Rem_E_i64,
    #[token("c_rem_e.u8")]
    C_Rem_E_u8,
    #[token("c_rem_e.u16")]
    C_Rem_E_u16,
    #[token("c_rem_e.u32")]
    C_Rem_E_u32,
    #[token("c_rem_e.u64")]
    C_Rem_E_u64,
    #[token("c_shl.i8")]
    C_Shl_i8,
    #[token("c_shl.i16")]
    C_Shl_i16,
    #[token("c_shl.i32")]
    C_Shl_i32,
    #[token("c_shl.i64")]
    C_Shl_i64,
    #[token("c_shl.u8")]
    C_Shl_u8,
    #[token("c_shl.u16")]
    C_Shl_u16,
    #[token("c_shl.u32")]
    C_Shl_u32,
    #[token("c_shl.u64")]
    C_Shl_u64,
    #[token("c_shr.i8")]
    C_Shr_i8,
    #[token("c_shr.i16")]
    C_Shr_i16,
    #[token("c_shr.i32")]
    C_Shr_i32,
    #[token("c_shr.i64")]
    C_Shr_i64,
    #[token("c_shr.u8")]
    C_Shr_u8,
    #[token("c_shr.u16")]
    C_Shr_u16,
    #[token("c_shr.u32")]
    C_Shr_u32,
    #[token("c_shr.u64")]
    C_Shr_u64,
    #[token("c_sub.i8")]
    C_Sub_i8,
    #[token("c_sub.i16")]
    C_Sub_i16,
    #[token("c_sub.i32")]
    C_Sub_i32,
    #[token("c_sub.i64")]
    C_Sub_i64,
    #[token("c_sub.u8")]
    C_Sub_u8,
    #[token("c_sub.u16")]
    C_Sub_u16,
    #[token("c_sub.u32")]
    C_Sub_u32,
    #[token("c_sub.u64")]
    C_Sub_u64,
    #[token("c_sub_u.i8")]
    C_Sub_U_i8,
    #[token("c_sub_u.i16")]
    C_Sub_U_i16,
    #[token("c_sub_u.i32")]
    C_Sub_U_i32,
    #[token("c_sub_u.i64")]
    C_Sub_U_i64,
    #[token("o_abs.i8")]
    O_Abs_i8,
    #[token("o_abs.i16")]
    O_Abs_i16,
    #[token("o_abs.i32")]
    O_Abs_i32,
    #[token("o_abs.i64")]
    O_Abs_i64,
    #[token("o_add.i8")]
    O_Add_i8,
    #[token("o_add.i16")]
    O_Add_i16,
    #[token("o_add.i32")]
    O_Add_i32,
    #[token("o_add.i64")]
    O_Add_i64,
    #[token("o_add.u8")]
    O_Add_u8,
    #[token("o_add.u16")]
    O_Add_u16,
    #[token("o_add.u32")]
    O_Add_u32,
    #[token("o_add.u64")]
    O_Add_u64,
    #[token("o_add_u.i8")]
    O_Add_U_i8,
    #[token("o_add_u.i16")]
    O_Add_U_i16,
    #[token("o_add_u.i32")]
    O_Add_U_i32,
    #[token("o_add_u.i64")]
    O_Add_U_i64,
    #[token("o_add_s.u8")]
    O_Add_S_u8,
    #[token("o_add_s.u16")]
    O_Add_S_u16,
    #[token("o_add_s.u32")]
    O_Add_S_u32,
    #[token("o_add_s.u64")]
    O_Add_S_u64,
    #[token("o_div.i8")]
    O_Div_i8,
    #[token("o_div.i16")]
    O_Div_i16,
    #[token("o_div.i32")]
    O_Div_i32,
    #[token("o_div.i64")]
    O_Div_i64,
    #[token("o_div.u8")]
    O_Div_u8,
    #[token("o_div.u16")]
    O_Div_u16,
    #[token("o_div.u32")]
    O_Div_u32,
    #[token("o_div.u64")]
    O_Div_u64,
    #[token("o_div_e.i8")]
    O_Div_E_i8,
    #[token("o_div_e.i16")]
    O_Div_E_i16,
    #[token("o_div_e.i32")]
    O_Div_E_i32,
    #[token("o_div_e.i64")]
    O_Div_E_i64,
    #[token("o_div_e.u8")]
    O_Div_E_u8,
    #[token("o_div_e.u16")]
    O_Div_E_u16,
    #[token("o_div_e.u32")]
    O_Div_E_u32,
    #[token("o_div_e.u64")]
    O_Div_E_u64,
    #[token("o_mul.i8")]
    O_Mul_i8,
    #[token("o_mul.i16")]
    O_Mul_i16,
    #[token("o_mul.i32")]
    O_Mul_i32,
    #[token("o_mul.i64")]
    O_Mul_i64,
    #[token("o_mul.u8")]
    O_Mul_u8,
    #[token("o_mul.u16")]
    O_Mul_u16,
    #[token("o_mul.u32")]
    O_Mul_u32,
    #[token("o_mul.u64")]
    O_Mul_u64,
    #[token("o_neg.i8")]
    O_Neg_i8,
    #[token("o_neg.i16")]
    O_Neg_i16,
    #[token("o_neg.i32")]
    O_Neg_i32,
    #[token("o_neg.i64")]
    O_Neg_i64,
    #[token("o_pow.i8")]
    O_Pow_i8,
    #[token("o_pow.i16")]
    O_Pow_i16,
    #[token("o_pow.i32")]
    O_Pow_i32,
    #[token("o_pow.i64")]
    O_Pow_i64,
    #[token("o_pow.u8")]
    O_Pow_u8,
    #[token("o_pow.u16")]
    O_Pow_u16,
    #[token("o_pow.u32")]
    O_Pow_u32,
    #[token("o_pow.u64")]
    O_Pow_u64,
    #[token("o_rem.i8")]
    O_Rem_i8,
    #[token("o_rem.i16")]
    O_Rem_i16,
    #[token("o_rem.i32")]
    O_Rem_i32,
    #[token("o_rem.i64")]
    O_Rem_i64,
    #[token("o_rem.u8")]
    O_Rem_u8,
    #[token("o_rem.u16")]
    O_Rem_u16,
    #[token("o_rem.u32")]
    O_Rem_u32,
    #[token("o_rem.u64")]
    O_Rem_u64,
    #[token("o_rem_e.i8")]
    O_Rem_E_i8,
    #[token("o_rem_e.i16")]
    O_Rem_E_i16,
    #[token("o_rem_e.i32")]
    O_Rem_E_i32,
    #[token("o_rem_e.i64")]
    O_Rem_E_i64,
    #[token("o_rem_e.u8")]
    O_Rem_E_u8,
    #[token("o_rem_e.u16")]
    O_Rem_E_u16,
    #[token("o_rem_e.u32")]
    O_Rem_E_u32,
    #[token("o_rem_e.u64")]
    O_Rem_E_u64,
    #[token("o_shl.i8")]
    O_Shl_i8,
    #[token("o_shl.i16")]
    O_Shl_i16,
    #[token("o_shl.i32")]
    O_Shl_i32,
    #[token("o_shl.i64")]
    O_Shl_i64,
    #[token("o_shl.u8")]
    O_Shl_u8,
    #[token("o_shl.u16")]
    O_Shl_u16,
    #[token("o_shl.u32")]
    O_Shl_u32,
    #[token("o_shl.u64")]
    O_Shl_u64,
    #[token("o_shr.i8")]
    O_Shr_i8,
    #[token("o_shr.i16")]
    O_Shr_i16,
    #[token("o_shr.i32")]
    O_Shr_i32,
    #[token("o_shr.i64")]
    O_Shr_i64,
    #[token("o_shr.u8")]
    O_Shr_u8,
    #[token("o_shr.u16")]
    O_Shr_u16,
    #[token("o_shr.u32")]
    O_Shr_u32,
    #[token("o_shr.u64")]
    O_Shr_u64,
    #[token("o_sub.i8")]
    O_Sub_i8,
    #[token("o_sub.i16")]
    O_Sub_i16,
    #[token("o_sub.i32")]
    O_Sub_i32,
    #[token("o_sub.i64")]
    O_Sub_i64,
    #[token("o_sub.u8")]
    O_Sub_u8,
    #[token("o_sub.u16")]
    O_Sub_u16,
    #[token("o_sub.u32")]
    O_Sub_u32,
    #[token("o_sub.u64")]
    O_Sub_u64,
    #[token("o_sub_u.i8")]
    O_Sub_U_i8,
    #[token("o_sub_u.i16")]
    O_Sub_U_i16,
    #[token("o_sub_u.i32")]
    O_Sub_U_i32,
    #[token("o_sub_u.i64")]
    O_Sub_U_i64,
    #[token("s_abs.i8")]
    S_Abs_i8,
    #[token("s_abs.i16")]
    S_Abs_i16,
    #[token("s_abs.i32")]
    S_Abs_i32,
    #[token("s_abs.i64")]
    S_Abs_i64,
    #[token("s_add.i8")]
    S_Add_i8,
    #[token("s_add.i16")]
    S_Add_i16,
    #[token("s_add.i32")]
    S_Add_i32,
    #[token("s_add.i64")]
    S_Add_i64,
    #[token("s_add.u8")]
    S_Add_u8,
    #[token("s_add.u16")]
    S_Add_u16,
    #[token("s_add.u32")]
    S_Add_u32,
    #[token("s_add.u64")]
    S_Add_u64,
    #[token("s_add_u.i8")]
    S_Add_U_i8,
    #[token("s_add_u.i16")]
    S_Add_U_i16,
    #[token("s_add_u.i32")]
    S_Add_U_i32,
    #[token("s_add_u.i64")]
    S_Add_U_i64,
    #[token("s_add_s.u8")]
    S_Add_S_u8,
    #[token("s_add_s.u16")]
    S_Add_S_u16,
    #[token("s_add_s.u32")]
    S_Add_S_u32,
    #[token("s_add_s.u64")]
    S_Add_S_u64,
    #[token("s_div.i8")]
    S_Div_i8,
    #[token("s_div.i16")]
    S_Div_i16,
    #[token("s_div.i32")]
    S_Div_i32,
    #[token("s_div.i64")]
    S_Div_i64,
    #[token("s_div.u8")]
    S_Div_u8,
    #[token("s_div.u16")]
    S_Div_u16,
    #[token("s_div.u32")]
    S_Div_u32,
    #[token("s_div.u64")]
    S_Div_u64,
    #[token("s_mul.i8")]
    S_Mul_i8,
    #[token("s_mul.i16")]
    S_Mul_i16,
    #[token("s_mul.i32")]
    S_Mul_i32,
    #[token("s_mul.i64")]
    S_Mul_i64,
    #[token("s_mul.u8")]
    S_Mul_u8,
    #[token("s_mul.u16")]
    S_Mul_u16,
    #[token("s_mul.u32")]
    S_Mul_u32,
    #[token("s_mul.u64")]
    S_Mul_u64,
    #[token("s_neg.i8")]
    S_Neg_i8,
    #[token("s_neg.i16")]
    S_Neg_i16,
    #[token("s_neg.i32")]
    S_Neg_i32,
    #[token("s_neg.i64")]
    S_Neg_i64,
    #[token("s_pow.i8")]
    S_Pow_i8,
    #[token("s_pow.i16")]
    S_Pow_i16,
    #[token("s_pow.i32")]
    S_Pow_i32,
    #[token("s_pow.i64")]
    S_Pow_i64,
    #[token("s_pow.u8")]
    S_Pow_u8,
    #[token("s_pow.u16")]
    S_Pow_u16,
    #[token("s_pow.u32")]
    S_Pow_u32,
    #[token("s_pow.u64")]
    S_Pow_u64,
    #[token("s_sub.i8")]
    S_Sub_i8,
    #[token("s_sub.i16")]
    S_Sub_i16,
    #[token("s_sub.i32")]
    S_Sub_i32,
    #[token("s_sub.i64")]
    S_Sub_i64,
    #[token("s_sub.u8")]
    S_Sub_u8,
    #[token("s_sub.u16")]
    S_Sub_u16,
    #[token("s_sub.u32")]
    S_Sub_u32,
    #[token("s_sub.u64")]
    S_Sub_u64,
    #[token("s_sub_u.i8")]
    S_Sub_U_i8,
    #[token("s_sub_u.i16")]
    S_Sub_U_i16,
    #[token("s_sub_u.i32")]
    S_Sub_U_i32,
    #[token("s_sub_u.i64")]
    S_Sub_U_i64,
    #[token("abs.f32")]
    Abs_f32,
    #[token("abs.f64")]
    Abs_f64,
    #[token("add.f32")]
    Add_f32,
    #[token("add.f64")]
    Add_f64,
    #[token("div.f32")]
    Div_f32,
    #[token("div.f64")]
    Div_f64,
    #[token("div_e.f32")]
    Div_E_f32,
    #[token("div_e.f64")]
    Div_E_f64,
    #[token("log.f32")]
    Log_f32,
    #[token("log.f64")]
    Log_f64,
    #[token("sqrt.f32")]
    Sqrt_f32,
    #[token("sqrt.f64")]
    Sqrt_f64,
    #[token("mul.f32")]
    Mul_f32,
    #[token("mul.f64")]
    Mul_f64,
    #[token("neg.f32")]
    Neg_f32,
    #[token("neg.f64")]
    Neg_f64,
    #[token("pow.f32")]
    Pow_f32,
    #[token("pow.f64")]
    Pow_f64,
    #[token("rem.f32")]
    Rem_f32,
    #[token("rem.f64")]
    Rem_f64,
    #[token("rem_e.f32")]
    Rem_E_f32,
    #[token("rem_e.f64")]
    Rem_E_f64,
    #[token("cbrt.f32")]
    Cbrt_f32,
    #[token("cbrt.f64")]
    Cbrt_f64,
    #[token("sub.f32")]
    Sub_f32,
    #[token("sub.f64")]
    Sub_f64,
    #[token(".i8")]
    Type_i8,
    #[token(".u8")]
    Type_u8,
    #[token(".i16")]
    Type_i16,
    #[token(".u16")]
    Type_u16,
    #[token(".i32")]
    Type_i32,
    #[token(".u32")]
    Type_u32,
    #[token(".i64")]
    Type_i64,
    #[token(".u64")]
    Type_u64,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("&")]
    Ampersand,
}
fn parse_dec(lex: &mut Lexer<Lexed_Token>) -> Option<u64> {
    u64::from_str_radix(lex.slice(), 10).ok()
}
fn parse_hex(lex: &mut Lexer<Lexed_Token>) -> Option<u64> {
    let slice = lex.slice();
    u64::from_str_radix(&slice[2..], 16).ok()
}
fn parse_bin(lex: &mut Lexer<Lexed_Token>) -> Option<u64> {
    let slice = lex.slice();
    u64::from_str_radix(&slice[2..], 2).ok()
}
fn parse_oct(lex: &mut Lexer<Lexed_Token>) -> Option<u64> {
    let slice = lex.slice();
    u64::from_str_radix(&slice[2..], 8).ok()
}
fn parse_register_index(lex: &mut Lexer<Lexed_Token>) -> u64 {
    let slice = lex.slice();
    u64::from_str_radix(&slice[1..], 10).unwrap() + 6
}
pub struct Assembly_Lexer<'source> {
    tokens: SpannedIter<'source, Lexed_Token>,
    line: usize,
}
impl<'source> Assembly_Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            tokens: Lexed_Token::lexer(source).spanned(),
            line: 0,
        }
    }
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn next(&mut self) -> Option<Token> {
        if let Some((lexed_token, range)) = self.tokens.next() {
            let (kind, value, flags) = match lexed_token
                .expect("Oops. Unexpected token.")
            {
                Lexed_Token::Identifier => {
                    (Token_Kind::Identifier, Token_Value::none(), 0)
                }
                Lexed_Token::Register(index) => {
                    (Token_Kind::Register, Token_Value::integer(index), 0)
                }
                Lexed_Token::Code => (Token_Kind::Code_Segment, Token_Value::none(), 0),
                Lexed_Token::Data => (Token_Kind::Data_Segment, Token_Value::none(), 0),
                Lexed_Token::Halt => {
                    (Token_Kind::Instruction, Token_Value::integer(0u64), 0)
                }
                Lexed_Token::Trap => {
                    (Token_Kind::Instruction, Token_Value::integer(1u64), 0)
                }
                Lexed_Token::Call => {
                    (Token_Kind::Instruction, Token_Value::integer(2u64), 0)
                }
                Lexed_Token::Callr => {
                    (Token_Kind::Instruction, Token_Value::integer(3u64), 0)
                }
                Lexed_Token::Calli => {
                    (Token_Kind::Instruction, Token_Value::integer(4u64), 0)
                }
                Lexed_Token::Ret => {
                    (Token_Kind::Instruction, Token_Value::integer(5u64), 0)
                }
                Lexed_Token::Ecall => {
                    (Token_Kind::Instruction, Token_Value::integer(6u64), 0)
                }
                Lexed_Token::Break => {
                    (Token_Kind::Instruction, Token_Value::integer(7u64), 0)
                }
                Lexed_Token::Jal => {
                    (Token_Kind::Instruction, Token_Value::integer(8u64), 0)
                }
                Lexed_Token::Jalr => {
                    (Token_Kind::Instruction, Token_Value::integer(9u64), 0)
                }
                Lexed_Token::Jali => {
                    (Token_Kind::Instruction, Token_Value::integer(10u64), 0)
                }
                Lexed_Token::Jnz => {
                    (Token_Kind::Instruction, Token_Value::integer(11u64), 0)
                }
                Lexed_Token::Jnzr => {
                    (Token_Kind::Instruction, Token_Value::integer(12u64), 0)
                }
                Lexed_Token::Jnzi => {
                    (Token_Kind::Instruction, Token_Value::integer(13u64), 0)
                }
                Lexed_Token::Jiz => {
                    (Token_Kind::Instruction, Token_Value::integer(14u64), 0)
                }
                Lexed_Token::Jizr => {
                    (Token_Kind::Instruction, Token_Value::integer(15u64), 0)
                }
                Lexed_Token::Jizi => {
                    (Token_Kind::Instruction, Token_Value::integer(16u64), 0)
                }
                Lexed_Token::Load_8 => {
                    (Token_Kind::Instruction, Token_Value::integer(17u64), 0)
                }
                Lexed_Token::Load_16 => {
                    (Token_Kind::Instruction, Token_Value::integer(18u64), 0)
                }
                Lexed_Token::Load_32 => {
                    (Token_Kind::Instruction, Token_Value::integer(19u64), 0)
                }
                Lexed_Token::Load_64 => {
                    (Token_Kind::Instruction, Token_Value::integer(20u64), 0)
                }
                Lexed_Token::Loadi => {
                    (Token_Kind::Instruction, Token_Value::integer(21u64), 0)
                }
                Lexed_Token::Loada_8 => {
                    (Token_Kind::Instruction, Token_Value::integer(22u64), 0)
                }
                Lexed_Token::Loada_16 => {
                    (Token_Kind::Instruction, Token_Value::integer(23u64), 0)
                }
                Lexed_Token::Loada_32 => {
                    (Token_Kind::Instruction, Token_Value::integer(24u64), 0)
                }
                Lexed_Token::Loada_64 => {
                    (Token_Kind::Instruction, Token_Value::integer(25u64), 0)
                }
                Lexed_Token::Store_8 => {
                    (Token_Kind::Instruction, Token_Value::integer(26u64), 0)
                }
                Lexed_Token::Store_16 => {
                    (Token_Kind::Instruction, Token_Value::integer(27u64), 0)
                }
                Lexed_Token::Store_32 => {
                    (Token_Kind::Instruction, Token_Value::integer(28u64), 0)
                }
                Lexed_Token::Store_64 => {
                    (Token_Kind::Instruction, Token_Value::integer(29u64), 0)
                }
                Lexed_Token::Storei => {
                    (Token_Kind::Instruction, Token_Value::integer(30u64), 0)
                }
                Lexed_Token::Move => {
                    (Token_Kind::Instruction, Token_Value::integer(31u64), 0)
                }
                Lexed_Token::Push => {
                    (Token_Kind::Instruction, Token_Value::integer(32u64), 0)
                }
                Lexed_Token::Pushi => {
                    (Token_Kind::Instruction, Token_Value::integer(33u64), 0)
                }
                Lexed_Token::Pop => {
                    (Token_Kind::Instruction, Token_Value::integer(34u64), 0)
                }
                Lexed_Token::Ie => {
                    (Token_Kind::Instruction, Token_Value::integer(35u64), 0)
                }
                Lexed_Token::Ie_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(36u64), 0)
                }
                Lexed_Token::Ie_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(37u64), 0)
                }
                Lexed_Token::Ne => {
                    (Token_Kind::Instruction, Token_Value::integer(38u64), 0)
                }
                Lexed_Token::Ne_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(39u64), 0)
                }
                Lexed_Token::Ne_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(40u64), 0)
                }
                Lexed_Token::Lt => {
                    (Token_Kind::Instruction, Token_Value::integer(41u64), 0)
                }
                Lexed_Token::Lt_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(42u64), 0)
                }
                Lexed_Token::Lt_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(43u64), 0)
                }
                Lexed_Token::Le => {
                    (Token_Kind::Instruction, Token_Value::integer(44u64), 0)
                }
                Lexed_Token::Le_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(45u64), 0)
                }
                Lexed_Token::Le_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(46u64), 0)
                }
                Lexed_Token::Gt => {
                    (Token_Kind::Instruction, Token_Value::integer(47u64), 0)
                }
                Lexed_Token::Gt_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(48u64), 0)
                }
                Lexed_Token::Gt_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(49u64), 0)
                }
                Lexed_Token::Ge => {
                    (Token_Kind::Instruction, Token_Value::integer(50u64), 0)
                }
                Lexed_Token::Ge_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(51u64), 0)
                }
                Lexed_Token::Ge_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(52u64), 0)
                }
                Lexed_Token::And_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(53u64), 0)
                }
                Lexed_Token::And_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(54u64), 0)
                }
                Lexed_Token::And_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(55u64), 0)
                }
                Lexed_Token::And_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(56u64), 0)
                }
                Lexed_Token::And_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(57u64), 0)
                }
                Lexed_Token::And_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(58u64), 0)
                }
                Lexed_Token::And_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(59u64), 0)
                }
                Lexed_Token::And_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(60u64), 0)
                }
                Lexed_Token::Or_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(61u64), 0)
                }
                Lexed_Token::Or_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(62u64), 0)
                }
                Lexed_Token::Or_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(63u64), 0)
                }
                Lexed_Token::Or_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(64u64), 0)
                }
                Lexed_Token::Or_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(65u64), 0)
                }
                Lexed_Token::Or_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(66u64), 0)
                }
                Lexed_Token::Or_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(67u64), 0)
                }
                Lexed_Token::Or_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(68u64), 0)
                }
                Lexed_Token::Xor_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(69u64), 0)
                }
                Lexed_Token::Xor_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(70u64), 0)
                }
                Lexed_Token::Xor_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(71u64), 0)
                }
                Lexed_Token::Xor_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(72u64), 0)
                }
                Lexed_Token::Xor_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(73u64), 0)
                }
                Lexed_Token::Xor_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(74u64), 0)
                }
                Lexed_Token::Xor_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(75u64), 0)
                }
                Lexed_Token::Xor_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(76u64), 0)
                }
                Lexed_Token::Not_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(77u64), 0)
                }
                Lexed_Token::Not_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(78u64), 0)
                }
                Lexed_Token::Not_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(79u64), 0)
                }
                Lexed_Token::Not_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(80u64), 0)
                }
                Lexed_Token::Not_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(81u64), 0)
                }
                Lexed_Token::Not_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(82u64), 0)
                }
                Lexed_Token::Not_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(83u64), 0)
                }
                Lexed_Token::Not_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(84u64), 0)
                }
                Lexed_Token::Shl_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(85u64), 0)
                }
                Lexed_Token::Shl_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(86u64), 0)
                }
                Lexed_Token::Shl_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(87u64), 0)
                }
                Lexed_Token::Shl_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(88u64), 0)
                }
                Lexed_Token::Shl_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(89u64), 0)
                }
                Lexed_Token::Shl_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(90u64), 0)
                }
                Lexed_Token::Shl_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(91u64), 0)
                }
                Lexed_Token::Shl_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(92u64), 0)
                }
                Lexed_Token::Shr_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(93u64), 0)
                }
                Lexed_Token::Shr_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(94u64), 0)
                }
                Lexed_Token::Shr_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(95u64), 0)
                }
                Lexed_Token::Shr_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(96u64), 0)
                }
                Lexed_Token::Shr_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(97u64), 0)
                }
                Lexed_Token::Shr_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(98u64), 0)
                }
                Lexed_Token::Shr_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(99u64), 0)
                }
                Lexed_Token::Shr_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(100u64), 0)
                }
                Lexed_Token::Rotl_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(101u64), 0)
                }
                Lexed_Token::Rotl_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(102u64), 0)
                }
                Lexed_Token::Rotl_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(103u64), 0)
                }
                Lexed_Token::Rotl_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(104u64), 0)
                }
                Lexed_Token::Rotl_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(105u64), 0)
                }
                Lexed_Token::Rotl_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(106u64), 0)
                }
                Lexed_Token::Rotl_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(107u64), 0)
                }
                Lexed_Token::Rotl_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(108u64), 0)
                }
                Lexed_Token::Rotr_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(109u64), 0)
                }
                Lexed_Token::Rotr_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(110u64), 0)
                }
                Lexed_Token::Rotr_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(111u64), 0)
                }
                Lexed_Token::Rotr_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(112u64), 0)
                }
                Lexed_Token::Rotr_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(113u64), 0)
                }
                Lexed_Token::Rotr_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(114u64), 0)
                }
                Lexed_Token::Rotr_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(115u64), 0)
                }
                Lexed_Token::Rotr_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(116u64), 0)
                }
                Lexed_Token::Count_Ones_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(117u64), 0)
                }
                Lexed_Token::Count_Ones_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(118u64), 0)
                }
                Lexed_Token::Count_Ones_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(119u64), 0)
                }
                Lexed_Token::Count_Ones_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(120u64), 0)
                }
                Lexed_Token::Count_Ones_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(121u64), 0)
                }
                Lexed_Token::Count_Ones_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(122u64), 0)
                }
                Lexed_Token::Count_Ones_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(123u64), 0)
                }
                Lexed_Token::Count_Ones_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(124u64), 0)
                }
                Lexed_Token::Leading_Ones_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(125u64), 0)
                }
                Lexed_Token::Leading_Ones_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(126u64), 0)
                }
                Lexed_Token::Leading_Ones_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(127u64), 0)
                }
                Lexed_Token::Leading_Ones_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(128u64), 0)
                }
                Lexed_Token::Leading_Ones_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(129u64), 0)
                }
                Lexed_Token::Leading_Ones_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(130u64), 0)
                }
                Lexed_Token::Leading_Ones_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(131u64), 0)
                }
                Lexed_Token::Leading_Ones_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(132u64), 0)
                }
                Lexed_Token::Trailing_Ones_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(133u64), 0)
                }
                Lexed_Token::Trailing_Ones_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(134u64), 0)
                }
                Lexed_Token::Trailing_Ones_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(135u64), 0)
                }
                Lexed_Token::Trailing_Ones_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(136u64), 0)
                }
                Lexed_Token::Trailing_Ones_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(137u64), 0)
                }
                Lexed_Token::Trailing_Ones_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(138u64), 0)
                }
                Lexed_Token::Trailing_Ones_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(139u64), 0)
                }
                Lexed_Token::Trailing_Ones_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(140u64), 0)
                }
                Lexed_Token::Count_Zeros_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(141u64), 0)
                }
                Lexed_Token::Count_Zeros_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(142u64), 0)
                }
                Lexed_Token::Count_Zeros_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(143u64), 0)
                }
                Lexed_Token::Count_Zeros_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(144u64), 0)
                }
                Lexed_Token::Count_Zeros_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(145u64), 0)
                }
                Lexed_Token::Count_Zeros_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(146u64), 0)
                }
                Lexed_Token::Count_Zeros_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(147u64), 0)
                }
                Lexed_Token::Count_Zeros_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(148u64), 0)
                }
                Lexed_Token::Leading_Zeros_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(149u64), 0)
                }
                Lexed_Token::Leading_Zeros_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(150u64), 0)
                }
                Lexed_Token::Leading_Zeros_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(151u64), 0)
                }
                Lexed_Token::Leading_Zeros_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(152u64), 0)
                }
                Lexed_Token::Leading_Zeros_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(153u64), 0)
                }
                Lexed_Token::Leading_Zeros_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(154u64), 0)
                }
                Lexed_Token::Leading_Zeros_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(155u64), 0)
                }
                Lexed_Token::Leading_Zeros_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(156u64), 0)
                }
                Lexed_Token::Trailing_Zeros_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(157u64), 0)
                }
                Lexed_Token::Trailing_Zeros_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(158u64), 0)
                }
                Lexed_Token::Trailing_Zeros_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(159u64), 0)
                }
                Lexed_Token::Trailing_Zeros_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(160u64), 0)
                }
                Lexed_Token::Trailing_Zeros_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(161u64), 0)
                }
                Lexed_Token::Trailing_Zeros_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(162u64), 0)
                }
                Lexed_Token::Trailing_Zeros_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(163u64), 0)
                }
                Lexed_Token::Trailing_Zeros_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(164u64), 0)
                }
                Lexed_Token::Reverse_Bytes_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(165u64), 0)
                }
                Lexed_Token::Reverse_Bytes_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(166u64), 0)
                }
                Lexed_Token::Reverse_Bytes_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(167u64), 0)
                }
                Lexed_Token::Reverse_Bytes_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(168u64), 0)
                }
                Lexed_Token::Reverse_Bytes_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(169u64), 0)
                }
                Lexed_Token::Reverse_Bytes_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(170u64), 0)
                }
                Lexed_Token::Reverse_Bytes_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(171u64), 0)
                }
                Lexed_Token::Reverse_Bytes_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(172u64), 0)
                }
                Lexed_Token::Reverse_Bits_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(173u64), 0)
                }
                Lexed_Token::Reverse_Bits_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(174u64), 0)
                }
                Lexed_Token::Reverse_Bits_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(175u64), 0)
                }
                Lexed_Token::Reverse_Bits_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(176u64), 0)
                }
                Lexed_Token::Reverse_Bits_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(177u64), 0)
                }
                Lexed_Token::Reverse_Bits_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(178u64), 0)
                }
                Lexed_Token::Reverse_Bits_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(179u64), 0)
                }
                Lexed_Token::Reverse_Bits_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(180u64), 0)
                }
                Lexed_Token::C_Abs_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(181u64), 0)
                }
                Lexed_Token::C_Abs_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(182u64), 0)
                }
                Lexed_Token::C_Abs_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(183u64), 0)
                }
                Lexed_Token::C_Abs_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(184u64), 0)
                }
                Lexed_Token::C_Add_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(185u64), 0)
                }
                Lexed_Token::C_Add_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(186u64), 0)
                }
                Lexed_Token::C_Add_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(187u64), 0)
                }
                Lexed_Token::C_Add_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(188u64), 0)
                }
                Lexed_Token::C_Add_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(189u64), 0)
                }
                Lexed_Token::C_Add_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(190u64), 0)
                }
                Lexed_Token::C_Add_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(191u64), 0)
                }
                Lexed_Token::C_Add_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(192u64), 0)
                }
                Lexed_Token::C_Add_U_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(193u64), 0)
                }
                Lexed_Token::C_Add_U_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(194u64), 0)
                }
                Lexed_Token::C_Add_U_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(195u64), 0)
                }
                Lexed_Token::C_Add_U_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(196u64), 0)
                }
                Lexed_Token::C_Add_S_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(197u64), 0)
                }
                Lexed_Token::C_Add_S_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(198u64), 0)
                }
                Lexed_Token::C_Add_S_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(199u64), 0)
                }
                Lexed_Token::C_Add_S_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(200u64), 0)
                }
                Lexed_Token::C_Div_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(201u64), 0)
                }
                Lexed_Token::C_Div_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(202u64), 0)
                }
                Lexed_Token::C_Div_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(203u64), 0)
                }
                Lexed_Token::C_Div_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(204u64), 0)
                }
                Lexed_Token::C_Div_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(205u64), 0)
                }
                Lexed_Token::C_Div_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(206u64), 0)
                }
                Lexed_Token::C_Div_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(207u64), 0)
                }
                Lexed_Token::C_Div_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(208u64), 0)
                }
                Lexed_Token::C_Div_E_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(209u64), 0)
                }
                Lexed_Token::C_Div_E_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(210u64), 0)
                }
                Lexed_Token::C_Div_E_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(211u64), 0)
                }
                Lexed_Token::C_Div_E_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(212u64), 0)
                }
                Lexed_Token::C_Div_E_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(213u64), 0)
                }
                Lexed_Token::C_Div_E_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(214u64), 0)
                }
                Lexed_Token::C_Div_E_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(215u64), 0)
                }
                Lexed_Token::C_Div_E_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(216u64), 0)
                }
                Lexed_Token::C_Log_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(217u64), 0)
                }
                Lexed_Token::C_Log_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(218u64), 0)
                }
                Lexed_Token::C_Log_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(219u64), 0)
                }
                Lexed_Token::C_Log_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(220u64), 0)
                }
                Lexed_Token::C_Log_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(221u64), 0)
                }
                Lexed_Token::C_Log_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(222u64), 0)
                }
                Lexed_Token::C_Log_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(223u64), 0)
                }
                Lexed_Token::C_Log_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(224u64), 0)
                }
                Lexed_Token::C_Sqrt_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(225u64), 0)
                }
                Lexed_Token::C_Sqrt_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(226u64), 0)
                }
                Lexed_Token::C_Sqrt_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(227u64), 0)
                }
                Lexed_Token::C_Sqrt_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(228u64), 0)
                }
                Lexed_Token::C_Sqrt_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(229u64), 0)
                }
                Lexed_Token::C_Sqrt_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(230u64), 0)
                }
                Lexed_Token::C_Sqrt_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(231u64), 0)
                }
                Lexed_Token::C_Sqrt_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(232u64), 0)
                }
                Lexed_Token::C_Mul_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(233u64), 0)
                }
                Lexed_Token::C_Mul_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(234u64), 0)
                }
                Lexed_Token::C_Mul_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(235u64), 0)
                }
                Lexed_Token::C_Mul_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(236u64), 0)
                }
                Lexed_Token::C_Mul_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(237u64), 0)
                }
                Lexed_Token::C_Mul_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(238u64), 0)
                }
                Lexed_Token::C_Mul_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(239u64), 0)
                }
                Lexed_Token::C_Mul_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(240u64), 0)
                }
                Lexed_Token::C_Neg_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(241u64), 0)
                }
                Lexed_Token::C_Neg_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(242u64), 0)
                }
                Lexed_Token::C_Neg_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(243u64), 0)
                }
                Lexed_Token::C_Neg_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(244u64), 0)
                }
                Lexed_Token::C_Pow_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(245u64), 0)
                }
                Lexed_Token::C_Pow_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(246u64), 0)
                }
                Lexed_Token::C_Pow_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(247u64), 0)
                }
                Lexed_Token::C_Pow_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(248u64), 0)
                }
                Lexed_Token::C_Pow_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(249u64), 0)
                }
                Lexed_Token::C_Pow_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(250u64), 0)
                }
                Lexed_Token::C_Pow_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(251u64), 0)
                }
                Lexed_Token::C_Pow_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(252u64), 0)
                }
                Lexed_Token::C_Rem_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(253u64), 0)
                }
                Lexed_Token::C_Rem_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(254u64), 0)
                }
                Lexed_Token::C_Rem_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(255u64), 0)
                }
                Lexed_Token::C_Rem_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(256u64), 0)
                }
                Lexed_Token::C_Rem_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(257u64), 0)
                }
                Lexed_Token::C_Rem_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(258u64), 0)
                }
                Lexed_Token::C_Rem_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(259u64), 0)
                }
                Lexed_Token::C_Rem_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(260u64), 0)
                }
                Lexed_Token::C_Rem_E_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(261u64), 0)
                }
                Lexed_Token::C_Rem_E_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(262u64), 0)
                }
                Lexed_Token::C_Rem_E_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(263u64), 0)
                }
                Lexed_Token::C_Rem_E_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(264u64), 0)
                }
                Lexed_Token::C_Rem_E_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(265u64), 0)
                }
                Lexed_Token::C_Rem_E_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(266u64), 0)
                }
                Lexed_Token::C_Rem_E_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(267u64), 0)
                }
                Lexed_Token::C_Rem_E_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(268u64), 0)
                }
                Lexed_Token::C_Shl_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(269u64), 0)
                }
                Lexed_Token::C_Shl_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(270u64), 0)
                }
                Lexed_Token::C_Shl_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(271u64), 0)
                }
                Lexed_Token::C_Shl_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(272u64), 0)
                }
                Lexed_Token::C_Shl_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(273u64), 0)
                }
                Lexed_Token::C_Shl_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(274u64), 0)
                }
                Lexed_Token::C_Shl_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(275u64), 0)
                }
                Lexed_Token::C_Shl_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(276u64), 0)
                }
                Lexed_Token::C_Shr_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(277u64), 0)
                }
                Lexed_Token::C_Shr_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(278u64), 0)
                }
                Lexed_Token::C_Shr_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(279u64), 0)
                }
                Lexed_Token::C_Shr_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(280u64), 0)
                }
                Lexed_Token::C_Shr_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(281u64), 0)
                }
                Lexed_Token::C_Shr_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(282u64), 0)
                }
                Lexed_Token::C_Shr_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(283u64), 0)
                }
                Lexed_Token::C_Shr_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(284u64), 0)
                }
                Lexed_Token::C_Sub_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(285u64), 0)
                }
                Lexed_Token::C_Sub_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(286u64), 0)
                }
                Lexed_Token::C_Sub_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(287u64), 0)
                }
                Lexed_Token::C_Sub_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(288u64), 0)
                }
                Lexed_Token::C_Sub_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(289u64), 0)
                }
                Lexed_Token::C_Sub_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(290u64), 0)
                }
                Lexed_Token::C_Sub_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(291u64), 0)
                }
                Lexed_Token::C_Sub_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(292u64), 0)
                }
                Lexed_Token::C_Sub_U_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(293u64), 0)
                }
                Lexed_Token::C_Sub_U_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(294u64), 0)
                }
                Lexed_Token::C_Sub_U_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(295u64), 0)
                }
                Lexed_Token::C_Sub_U_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(296u64), 0)
                }
                Lexed_Token::O_Abs_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(297u64), 0)
                }
                Lexed_Token::O_Abs_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(298u64), 0)
                }
                Lexed_Token::O_Abs_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(299u64), 0)
                }
                Lexed_Token::O_Abs_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(300u64), 0)
                }
                Lexed_Token::O_Add_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(301u64), 0)
                }
                Lexed_Token::O_Add_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(302u64), 0)
                }
                Lexed_Token::O_Add_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(303u64), 0)
                }
                Lexed_Token::O_Add_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(304u64), 0)
                }
                Lexed_Token::O_Add_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(305u64), 0)
                }
                Lexed_Token::O_Add_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(306u64), 0)
                }
                Lexed_Token::O_Add_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(307u64), 0)
                }
                Lexed_Token::O_Add_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(308u64), 0)
                }
                Lexed_Token::O_Add_U_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(309u64), 0)
                }
                Lexed_Token::O_Add_U_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(310u64), 0)
                }
                Lexed_Token::O_Add_U_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(311u64), 0)
                }
                Lexed_Token::O_Add_U_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(312u64), 0)
                }
                Lexed_Token::O_Add_S_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(313u64), 0)
                }
                Lexed_Token::O_Add_S_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(314u64), 0)
                }
                Lexed_Token::O_Add_S_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(315u64), 0)
                }
                Lexed_Token::O_Add_S_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(316u64), 0)
                }
                Lexed_Token::O_Div_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(317u64), 0)
                }
                Lexed_Token::O_Div_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(318u64), 0)
                }
                Lexed_Token::O_Div_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(319u64), 0)
                }
                Lexed_Token::O_Div_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(320u64), 0)
                }
                Lexed_Token::O_Div_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(321u64), 0)
                }
                Lexed_Token::O_Div_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(322u64), 0)
                }
                Lexed_Token::O_Div_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(323u64), 0)
                }
                Lexed_Token::O_Div_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(324u64), 0)
                }
                Lexed_Token::O_Div_E_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(325u64), 0)
                }
                Lexed_Token::O_Div_E_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(326u64), 0)
                }
                Lexed_Token::O_Div_E_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(327u64), 0)
                }
                Lexed_Token::O_Div_E_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(328u64), 0)
                }
                Lexed_Token::O_Div_E_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(329u64), 0)
                }
                Lexed_Token::O_Div_E_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(330u64), 0)
                }
                Lexed_Token::O_Div_E_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(331u64), 0)
                }
                Lexed_Token::O_Div_E_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(332u64), 0)
                }
                Lexed_Token::O_Mul_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(333u64), 0)
                }
                Lexed_Token::O_Mul_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(334u64), 0)
                }
                Lexed_Token::O_Mul_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(335u64), 0)
                }
                Lexed_Token::O_Mul_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(336u64), 0)
                }
                Lexed_Token::O_Mul_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(337u64), 0)
                }
                Lexed_Token::O_Mul_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(338u64), 0)
                }
                Lexed_Token::O_Mul_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(339u64), 0)
                }
                Lexed_Token::O_Mul_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(340u64), 0)
                }
                Lexed_Token::O_Neg_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(341u64), 0)
                }
                Lexed_Token::O_Neg_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(342u64), 0)
                }
                Lexed_Token::O_Neg_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(343u64), 0)
                }
                Lexed_Token::O_Neg_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(344u64), 0)
                }
                Lexed_Token::O_Pow_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(345u64), 0)
                }
                Lexed_Token::O_Pow_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(346u64), 0)
                }
                Lexed_Token::O_Pow_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(347u64), 0)
                }
                Lexed_Token::O_Pow_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(348u64), 0)
                }
                Lexed_Token::O_Pow_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(349u64), 0)
                }
                Lexed_Token::O_Pow_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(350u64), 0)
                }
                Lexed_Token::O_Pow_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(351u64), 0)
                }
                Lexed_Token::O_Pow_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(352u64), 0)
                }
                Lexed_Token::O_Rem_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(353u64), 0)
                }
                Lexed_Token::O_Rem_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(354u64), 0)
                }
                Lexed_Token::O_Rem_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(355u64), 0)
                }
                Lexed_Token::O_Rem_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(356u64), 0)
                }
                Lexed_Token::O_Rem_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(357u64), 0)
                }
                Lexed_Token::O_Rem_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(358u64), 0)
                }
                Lexed_Token::O_Rem_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(359u64), 0)
                }
                Lexed_Token::O_Rem_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(360u64), 0)
                }
                Lexed_Token::O_Rem_E_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(361u64), 0)
                }
                Lexed_Token::O_Rem_E_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(362u64), 0)
                }
                Lexed_Token::O_Rem_E_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(363u64), 0)
                }
                Lexed_Token::O_Rem_E_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(364u64), 0)
                }
                Lexed_Token::O_Rem_E_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(365u64), 0)
                }
                Lexed_Token::O_Rem_E_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(366u64), 0)
                }
                Lexed_Token::O_Rem_E_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(367u64), 0)
                }
                Lexed_Token::O_Rem_E_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(368u64), 0)
                }
                Lexed_Token::O_Shl_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(369u64), 0)
                }
                Lexed_Token::O_Shl_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(370u64), 0)
                }
                Lexed_Token::O_Shl_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(371u64), 0)
                }
                Lexed_Token::O_Shl_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(372u64), 0)
                }
                Lexed_Token::O_Shl_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(373u64), 0)
                }
                Lexed_Token::O_Shl_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(374u64), 0)
                }
                Lexed_Token::O_Shl_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(375u64), 0)
                }
                Lexed_Token::O_Shl_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(376u64), 0)
                }
                Lexed_Token::O_Shr_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(377u64), 0)
                }
                Lexed_Token::O_Shr_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(378u64), 0)
                }
                Lexed_Token::O_Shr_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(379u64), 0)
                }
                Lexed_Token::O_Shr_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(380u64), 0)
                }
                Lexed_Token::O_Shr_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(381u64), 0)
                }
                Lexed_Token::O_Shr_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(382u64), 0)
                }
                Lexed_Token::O_Shr_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(383u64), 0)
                }
                Lexed_Token::O_Shr_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(384u64), 0)
                }
                Lexed_Token::O_Sub_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(385u64), 0)
                }
                Lexed_Token::O_Sub_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(386u64), 0)
                }
                Lexed_Token::O_Sub_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(387u64), 0)
                }
                Lexed_Token::O_Sub_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(388u64), 0)
                }
                Lexed_Token::O_Sub_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(389u64), 0)
                }
                Lexed_Token::O_Sub_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(390u64), 0)
                }
                Lexed_Token::O_Sub_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(391u64), 0)
                }
                Lexed_Token::O_Sub_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(392u64), 0)
                }
                Lexed_Token::O_Sub_U_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(393u64), 0)
                }
                Lexed_Token::O_Sub_U_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(394u64), 0)
                }
                Lexed_Token::O_Sub_U_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(395u64), 0)
                }
                Lexed_Token::O_Sub_U_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(396u64), 0)
                }
                Lexed_Token::S_Abs_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(397u64), 0)
                }
                Lexed_Token::S_Abs_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(398u64), 0)
                }
                Lexed_Token::S_Abs_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(399u64), 0)
                }
                Lexed_Token::S_Abs_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(400u64), 0)
                }
                Lexed_Token::S_Add_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(401u64), 0)
                }
                Lexed_Token::S_Add_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(402u64), 0)
                }
                Lexed_Token::S_Add_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(403u64), 0)
                }
                Lexed_Token::S_Add_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(404u64), 0)
                }
                Lexed_Token::S_Add_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(405u64), 0)
                }
                Lexed_Token::S_Add_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(406u64), 0)
                }
                Lexed_Token::S_Add_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(407u64), 0)
                }
                Lexed_Token::S_Add_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(408u64), 0)
                }
                Lexed_Token::S_Add_U_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(409u64), 0)
                }
                Lexed_Token::S_Add_U_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(410u64), 0)
                }
                Lexed_Token::S_Add_U_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(411u64), 0)
                }
                Lexed_Token::S_Add_U_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(412u64), 0)
                }
                Lexed_Token::S_Add_S_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(413u64), 0)
                }
                Lexed_Token::S_Add_S_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(414u64), 0)
                }
                Lexed_Token::S_Add_S_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(415u64), 0)
                }
                Lexed_Token::S_Add_S_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(416u64), 0)
                }
                Lexed_Token::S_Div_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(417u64), 0)
                }
                Lexed_Token::S_Div_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(418u64), 0)
                }
                Lexed_Token::S_Div_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(419u64), 0)
                }
                Lexed_Token::S_Div_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(420u64), 0)
                }
                Lexed_Token::S_Div_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(421u64), 0)
                }
                Lexed_Token::S_Div_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(422u64), 0)
                }
                Lexed_Token::S_Div_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(423u64), 0)
                }
                Lexed_Token::S_Div_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(424u64), 0)
                }
                Lexed_Token::S_Mul_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(425u64), 0)
                }
                Lexed_Token::S_Mul_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(426u64), 0)
                }
                Lexed_Token::S_Mul_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(427u64), 0)
                }
                Lexed_Token::S_Mul_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(428u64), 0)
                }
                Lexed_Token::S_Mul_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(429u64), 0)
                }
                Lexed_Token::S_Mul_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(430u64), 0)
                }
                Lexed_Token::S_Mul_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(431u64), 0)
                }
                Lexed_Token::S_Mul_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(432u64), 0)
                }
                Lexed_Token::S_Neg_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(433u64), 0)
                }
                Lexed_Token::S_Neg_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(434u64), 0)
                }
                Lexed_Token::S_Neg_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(435u64), 0)
                }
                Lexed_Token::S_Neg_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(436u64), 0)
                }
                Lexed_Token::S_Pow_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(437u64), 0)
                }
                Lexed_Token::S_Pow_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(438u64), 0)
                }
                Lexed_Token::S_Pow_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(439u64), 0)
                }
                Lexed_Token::S_Pow_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(440u64), 0)
                }
                Lexed_Token::S_Pow_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(441u64), 0)
                }
                Lexed_Token::S_Pow_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(442u64), 0)
                }
                Lexed_Token::S_Pow_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(443u64), 0)
                }
                Lexed_Token::S_Pow_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(444u64), 0)
                }
                Lexed_Token::S_Sub_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(445u64), 0)
                }
                Lexed_Token::S_Sub_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(446u64), 0)
                }
                Lexed_Token::S_Sub_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(447u64), 0)
                }
                Lexed_Token::S_Sub_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(448u64), 0)
                }
                Lexed_Token::S_Sub_u8 => {
                    (Token_Kind::Instruction, Token_Value::integer(449u64), 0)
                }
                Lexed_Token::S_Sub_u16 => {
                    (Token_Kind::Instruction, Token_Value::integer(450u64), 0)
                }
                Lexed_Token::S_Sub_u32 => {
                    (Token_Kind::Instruction, Token_Value::integer(451u64), 0)
                }
                Lexed_Token::S_Sub_u64 => {
                    (Token_Kind::Instruction, Token_Value::integer(452u64), 0)
                }
                Lexed_Token::S_Sub_U_i8 => {
                    (Token_Kind::Instruction, Token_Value::integer(453u64), 0)
                }
                Lexed_Token::S_Sub_U_i16 => {
                    (Token_Kind::Instruction, Token_Value::integer(454u64), 0)
                }
                Lexed_Token::S_Sub_U_i32 => {
                    (Token_Kind::Instruction, Token_Value::integer(455u64), 0)
                }
                Lexed_Token::S_Sub_U_i64 => {
                    (Token_Kind::Instruction, Token_Value::integer(456u64), 0)
                }
                Lexed_Token::Abs_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(457u64), 0)
                }
                Lexed_Token::Abs_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(458u64), 0)
                }
                Lexed_Token::Add_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(459u64), 0)
                }
                Lexed_Token::Add_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(460u64), 0)
                }
                Lexed_Token::Div_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(461u64), 0)
                }
                Lexed_Token::Div_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(462u64), 0)
                }
                Lexed_Token::Div_E_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(463u64), 0)
                }
                Lexed_Token::Div_E_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(464u64), 0)
                }
                Lexed_Token::Log_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(465u64), 0)
                }
                Lexed_Token::Log_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(466u64), 0)
                }
                Lexed_Token::Sqrt_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(467u64), 0)
                }
                Lexed_Token::Sqrt_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(468u64), 0)
                }
                Lexed_Token::Mul_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(469u64), 0)
                }
                Lexed_Token::Mul_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(470u64), 0)
                }
                Lexed_Token::Neg_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(471u64), 0)
                }
                Lexed_Token::Neg_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(472u64), 0)
                }
                Lexed_Token::Pow_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(473u64), 0)
                }
                Lexed_Token::Pow_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(474u64), 0)
                }
                Lexed_Token::Rem_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(475u64), 0)
                }
                Lexed_Token::Rem_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(476u64), 0)
                }
                Lexed_Token::Rem_E_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(477u64), 0)
                }
                Lexed_Token::Rem_E_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(478u64), 0)
                }
                Lexed_Token::Cbrt_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(479u64), 0)
                }
                Lexed_Token::Cbrt_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(480u64), 0)
                }
                Lexed_Token::Sub_f32 => {
                    (Token_Kind::Instruction, Token_Value::integer(481u64), 0)
                }
                Lexed_Token::Sub_f64 => {
                    (Token_Kind::Instruction, Token_Value::integer(482u64), 0)
                }
                Lexed_Token::Type_i8 => {
                    (Token_Kind::Type, Token_Value::integer(8), SIGNED)
                }
                Lexed_Token::Type_u8 => (Token_Kind::Type, Token_Value::integer(8), 0),
                Lexed_Token::Type_i16 => {
                    (Token_Kind::Type, Token_Value::integer(16), SIGNED)
                }
                Lexed_Token::Type_u16 => (Token_Kind::Type, Token_Value::integer(16), 0),
                Lexed_Token::Type_i32 => {
                    (Token_Kind::Type, Token_Value::integer(32), SIGNED)
                }
                Lexed_Token::Type_u32 => (Token_Kind::Type, Token_Value::integer(32), 0),
                Lexed_Token::Type_i64 => {
                    (Token_Kind::Type, Token_Value::integer(64), SIGNED)
                }
                Lexed_Token::Type_u64 => (Token_Kind::Type, Token_Value::integer(64), 0),
                Lexed_Token::Newline => (Token_Kind::Newline, Token_Value::none(), 0),
                Lexed_Token::Integer(n) => {
                    (Token_Kind::Number, Token_Value::integer(n), 0)
                }
                Lexed_Token::Colon => (Token_Kind::Colon, Token_Value::none(), 0),
                Lexed_Token::Comma => (Token_Kind::Comma, Token_Value::none(), 0),
                Lexed_Token::Ampersand => (Token_Kind::Ampersand, Token_Value::none(), 0),
            };
            let start = range.start;
            let end = range.end;
            if kind == Token_Kind::Newline {
                self.line += 1;
            }
            Some(Token::new(kind, value, flags, start, end))
        } else {
            None
        }
    }
}
