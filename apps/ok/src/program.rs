use crate::byte_code::Instruction;

pub struct Program {
    pub data: Vec<u8>,
    pub code: Vec<Instruction>,
    pub start: usize,
}
