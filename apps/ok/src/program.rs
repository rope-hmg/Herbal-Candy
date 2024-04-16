use crate::byte_code::Micro_Op;

pub struct Program {
    pub data: Vec<u8>,
    pub code: Vec<Micro_Op>,
    pub start: usize,
}
