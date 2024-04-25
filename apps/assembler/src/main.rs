#![allow(non_camel_case_types)]

use std::env::args;

mod assembler;
mod lexer;
mod object;
mod parser;
mod token;

fn main() {
    args().skip(1).for_each(|arg| {
        let source = std::fs::read_to_string(arg).unwrap();
        let object = assembler::Assembler::new(&source).unwrap().assemble();
    });
}
