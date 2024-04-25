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
    ld_a.32   r0,   &count
    mov       r2,   one
loop:
    s_sub.i32 r0,   r0, one
    s_add.i32 r3,   r1, r2
    mov       r1,   r2
    mov       r2,   r3
    jnz_i     r0,   :loop
    ret
"#;

fn main() {
    assembler::Assembler::new(TEST).unwrap().assemble();
}
