#![allow(non_camel_case_types)]

mod convert;
mod memory;
mod program;
mod vm;

use std::io;

use byte_code::{Instruction, Register};

use crate::{program::Program, vm::Virtual_Machine};

fn main() {
    let mut vm = Virtual_Machine::new(1024 * 1024);

    // assembler::Assembler::new().assemble();

    let program = Program {
        data:  vec![],
        code:  vec![
            Instruction::Loadi {
                rd:  Register::General_Purpose(5),
                imm: 3,
            },
            Instruction::Call {
                rs2: Register::General_Purpose(5),
            },
            Instruction::Halt,
            Instruction::Loadi {
                rd:  Register::General_Purpose(10),
                imm: -4,
            },
            Instruction::Loadi {
                rd:  Register::General_Purpose(5),
                imm: 10,
            },
            Instruction::Move {
                rd:  Register::General_Purpose(7),
                rs1: Register::One,
            },
            Instruction::S_Sub_i32 {
                rd:  Register::General_Purpose(5),
                rs1: Register::General_Purpose(5),
                rs2: Register::One,
            },
            Instruction::S_Add_i32 {
                rd:  Register::General_Purpose(8),
                rs1: Register::General_Purpose(7),
                rs2: Register::General_Purpose(6),
            },
            Instruction::Move {
                rd:  Register::General_Purpose(6),
                rs1: Register::General_Purpose(7),
            },
            Instruction::Move {
                rd:  Register::General_Purpose(7),
                rs1: Register::General_Purpose(8),
            },
            Instruction::Jnz_R {
                rd:  Register::General_Purpose(9),
                rs1: Register::General_Purpose(5),
                rs2: Register::General_Purpose(10),
            },
            Instruction::Ret,
        ],
        start: 0,
    };

    vm.set_program(&program);

    let mut line = String::new();
    while vm.step() {
        io::stdin().read_line(&mut line).unwrap();
    }
}
