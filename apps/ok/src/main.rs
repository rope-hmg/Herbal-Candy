#![allow(non_camel_case_types)]

mod byte_code;
mod memory;
mod program;
mod vm;

use std::slice;

use crate::{
    byte_code::{Instruction, Register, Three_Registers},
    memory::Memory_Address,
    program::Program,
    vm::Virtual_Machine,
};

fn main() {
    let mut vm = Virtual_Machine::new(1024 * 1024);

    let program = Program {
        data: vec![10, 0, 0, 0],
        code: vec![
            Instruction::Call(2),
            Instruction::Halt,
            Instruction::Load_32(Register::General_Purpose(0), Memory_Address(0)),
            Instruction::Move(Register::General_Purpose(2), Register::One),
            Instruction::Saturating_Sub_I32(Three_Registers {
                destination: Register::General_Purpose(0),
                source_1: Register::General_Purpose(0),
                source_2: Register::One,
            }),
            Instruction::Saturating_Add_I32(Three_Registers {
                destination: Register::General_Purpose(3),
                source_1: Register::General_Purpose(1),
                source_2: Register::General_Purpose(2),
            }),
            Instruction::Move(Register::General_Purpose(1), Register::General_Purpose(2)),
            Instruction::Move(Register::General_Purpose(2), Register::General_Purpose(3)),
            Instruction::Jump_Not_Zero(4, Register::General_Purpose(0)),
            Instruction::Return,
        ],
        start: 0,
    };

    vm.run_program(&program);
}
