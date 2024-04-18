#![allow(non_camel_case_types)]

mod assembler;
mod lexer;
mod object;
mod token;

static TEST: &str = r#"
[code]
    call fibonacci
    halt

[data]
    count .i32 10

[code]
fibonacci:
    load_32       r0,   &count
    move          r2,   r1
loop:
    sub_i32_s     r0,   r0, one
    add_i32_s     r3,   r1, r2
    move          r1,   r2
    move          r2,   r3
    jump_not_zero loop, r0
    return
"#;

fn main() {
    assembler::Assembler::new(TEST).unwrap().assemble();
}
