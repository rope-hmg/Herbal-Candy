use crate::register::Register;

// 0b00000000000000000000000000111111
const OP_CODE_SHIFT: u32 = 0;
const OP_CODE_MASK: u32 = 0x3F;

// 0b00000000000000000000001111000000
const FUNCT_SHIFT: u32 = 6;
const FUNCT_MASK: u32 = 0xF;

// 0b00000000000000001111110000000000
// 0b00000000001111110000000000000000
// 0b00001111110000000000000000000000
const RD_SHIFT: u32 = 10;
const RS1_SHIFT: u32 = 16;
const RS2_SHIFT: u32 = 22;
pub const REG_MASK: u32 = 0x3F;

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
const IMM_SHIFT: u32 = 16;
const IMM_MASK: u32 = 0xFFFF;

#[inline(always)]
pub fn decode_op_code(instr: u32) -> u8 {
    ((instr >> OP_CODE_SHIFT) & OP_CODE_MASK) as u8
}

#[inline(always)]
pub fn encode_op_code(op_code: u8) -> u32 {
    ((op_code as u32) & OP_CODE_MASK) << OP_CODE_SHIFT
}

#[inline(always)]
pub fn decode_funct(instr: u32) -> u8 {
    ((instr >> FUNCT_SHIFT) & FUNCT_MASK) as u8
}

#[inline(always)]
pub fn encode_funct(funct: u8) -> u32 {
    ((funct as u32) & FUNCT_MASK) << FUNCT_SHIFT
}

#[inline(always)]
pub fn decode_rd(instr: u32) -> Register {
    Register::from_index(((instr >> RD_SHIFT) & REG_MASK) as u8)
}

#[inline(always)]
pub fn encode_rd(rd: Register) -> u32 {
    ((rd.index() as u32) & REG_MASK) << RD_SHIFT
}

#[inline(always)]
pub fn decode_rs1(instr: u32) -> Register {
    Register::from_index(((instr >> RS1_SHIFT) & REG_MASK) as u8)
}

#[inline(always)]
pub fn encode_rs1(rs1: Register) -> u32 {
    ((rs1.index() as u32) & REG_MASK) << RS1_SHIFT
}

#[inline(always)]
pub fn decode_rs2(instr: u32) -> Register {
    Register::from_index(((instr >> RS2_SHIFT) & REG_MASK) as u8)
}

#[inline(always)]
pub fn encode_rs2(rs2: Register) -> u32 {
    ((rs2.index() as u32) & REG_MASK) << RS2_SHIFT
}

#[inline(always)]
pub fn decode_size(instr: u32) -> u8 {
    unmap_size(((instr >> SIZE_SHIFT) & SIZE_MASK) as u8)
}

#[inline(always)]
pub fn encode_size(size: u8) -> u32 {
    ((map_size(size) as u32) & SIZE_MASK) << SIZE_SHIFT
}

#[inline(always)]
pub fn unmap_size(size: u8) -> u8 {
    0b1000 << size
}

#[inline(always)]
pub fn map_size(size: u8) -> u8 {
    match size {
        0b0000_1000 => 0,
        0b0001_0000 => 1,
        0b0010_0000 => 2,
        0b0100_0000 => 3,
        _ => panic!("Invalid size {}", size),
    }
}

#[inline(always)]
pub fn decode_f(instr: u32) -> u8 {
    ((instr >> F_FLAG_SHIFT) & F_FLAG_MASK) as u8
}

#[inline(always)]
pub fn encode_f(f: u8) -> u32 {
    ((f as u32) & F_FLAG_MASK) << F_FLAG_SHIFT
}

#[inline(always)]
pub fn decode_s(instr: u32) -> u8 {
    ((instr >> S_FLAG_SHIFT) & S_FLAG_MASK) as u8
}

#[inline(always)]
pub fn encode_s(s: u8) -> u32 {
    ((s as u32) & S_FLAG_MASK) << S_FLAG_SHIFT
}

#[inline(always)]
pub fn decode_imm(instr: u32) -> i16 {
    ((instr >> IMM_SHIFT) & IMM_MASK) as i16
}

#[inline(always)]
pub fn encode_imm(imm: i16) -> u32 {
    ((imm as u32) & IMM_MASK) << IMM_SHIFT
}
