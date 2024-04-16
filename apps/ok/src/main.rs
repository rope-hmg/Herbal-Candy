#![allow(non_camel_case_types)]

mod byte_code;
mod memory;
mod program;
mod vm;

use std::slice;

use crate::{
    byte_code::{Micro_Op, Register, Three_Registers},
    memory::Memory_Address,
    program::Program,
    vm::Virtual_Machine,
};

fn main() {
    let mut vm = Virtual_Machine::new(1024 * 1024);

    let program = Program {
        data: vec![10, 0, 0, 0, 20, 0, 0, 0],
        code: vec![
            Micro_Op::Load_32(Register::General_Purpose(1), Memory_Address(0)),
            Micro_Op::Load_32(Register::General_Purpose(2), Memory_Address(4)),
            Micro_Op::Saturating_Add_I32(Three_Registers {
                destination: Register::General_Purpose(0),
                source_1: Register::General_Purpose(1),
                source_2: Register::General_Purpose(2),
            }),
        ],
        start: 0,
    };

    vm.run_program(&program);

    println!(
        "Register 0: {}",
        vm.register_value(Register::General_Purpose(0))
    );
    println!(
        "Register 1: {}",
        vm.register_value(Register::General_Purpose(1))
    );
    println!(
        "Register 2: {}",
        vm.register_value(Register::General_Purpose(2))
    );
}
