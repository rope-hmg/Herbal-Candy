#![allow(non_camel_case_types)]

use std::io;

use byte_code::Instruction;

use vm::{Program, Virtual_Machine};

fn main() {
    let mut vm = Virtual_Machine::new(1024 * 1024);

    let program = Program {
        data:  vec![10, 0, 0, 0],
        code:  [
            131264, 0, 6401, 74241, 2688949126, 2718377030, 531969, 598529, 4294711872, 256,
        ]
        .iter()
        .copied()
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
