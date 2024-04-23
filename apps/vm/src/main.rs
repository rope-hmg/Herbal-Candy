#![allow(non_camel_case_types)]

mod convert;
mod memory;
mod program;
mod vm;

use std::io;

use byte_code::Instruction;

use crate::{program::Program, vm::Virtual_Machine};

fn main() {
    let mut vm = Virtual_Machine::new(1024 * 1024);

    // assembler::Assembler::new().assemble();
    // v_type 0b0000000000000000000000_0000_000000
    // r_type 0b0_0_00_000000_000000_000000_0000_000000
    // i_type 0b0000000000000000_000000_0000_000000

    let program = Program {
        data:  vec![],
        code:  [
            0b0000000000000011_000101_0001_000011, // Load_Immediate r5, 3
            0b0_0_00_000101_000000_000000_0001_000000, // Call 2
            0b0000000000000000000000_0000_000000,  // Halt
            0b1111111111111100_001010_0001_000011, // Load_Immediate r10, -4
            0b0000000000001010_000101_0001_000011, // Load_Immediate r5, 10
            0b0_0_00_000000_000001_000111_0100_000011, // Move r7, one
            0b1_0_10_000001_000101_000101_1110_000110, // Saturating_Sub_I32 r5, r5, one
            0b1_0_10_000110_000111_001000_0001_000110, // Saturating_Add_I32 r8, r7, r6
            0b0_0_00_000000_000111_000110_0100_000011, // Move r6, r7
            0b0_0_00_000000_001000_000111_0100_000011, // Move r7, r8
            0b1_0_00_001010_000101_001001_0110_000000, // Jump_Not_Zero r9, r5, r10
            0b0000000000000000000000_0010_000000,  // Return
        ]
        .into_iter()
        .map(Instruction::decode)
        .collect(),
        start: 0,
    };

    vm.set_program(&program);

    let mut line = String::new();
    while vm.step() {
        io::stdin().read_line(&mut line).unwrap();
    }
}
