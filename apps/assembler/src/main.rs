#![allow(non_camel_case_types)]

mod assembler;
mod lexer;
mod object;
mod parser;
mod token;

static TEST: &str = r#"
[code]
    call_i :fibonacci
    halt

[data]
    count .i32 10

[code]
fibonacci:
    load_a.32  r0,   &count
    move       r2,   r1
loop:
    s_sub.i32  r0,   r0, one
    s_add.i32  r3,   r1, r2
    move       r1,   r2
    move       r2,   r3
    jnz_i      r5,   :loop
    ret
"#;

fn main() {
    assembler::Assembler::new(TEST).unwrap().assemble();
}
