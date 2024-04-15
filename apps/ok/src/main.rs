#![allow(non_camel_case_types)]

mod byte_code;
mod memory;
mod vm;

use std::slice;

use crate::{
    byte_code::{Micro_Op, Register},
    memory::Memory_Address,
    vm::Virtual_Machine,
};

fn main() {
    let mut vm = Virtual_Machine::new(1024 * 1024);

    let ten = 10i32.to_le_bytes();
    let twenty = 20i32.to_le_bytes();

    unsafe {
        slice::from_raw_parts_mut(vm.memory.slot_mut(Memory_Address(0)), 4).copy_from_slice(&ten);
        slice::from_raw_parts_mut(vm.memory.slot_mut(Memory_Address(4)), 4)
            .copy_from_slice(&twenty);
    }

    let instructions = &[
        Micro_Op::Load_32(Register::General_Purpose(1), Memory_Address(0)),
        Micro_Op::Load_32(Register::General_Purpose(2), Memory_Address(4)),
        Micro_Op::Saturating_Add_I32(
            Register::General_Purpose(0),
            Register::General_Purpose(1),
            Register::General_Purpose(2),
        ),
    ];

    vm.run_program(instructions, 0);

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
