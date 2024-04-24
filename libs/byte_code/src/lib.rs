#![allow(non_camel_case_types)]

pub mod encoding;

mod instruction;
mod register;

pub use crate::{instruction::Instruction, register::Register};
