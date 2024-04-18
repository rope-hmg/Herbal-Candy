use std::{collections::HashMap, iter::Peekable};

use logos::{Lexer, Logos};

use crate::program::Program;

static TEST: &str = r#"
[code]
    call fibonacci
    halt

[data]
    count .i32 = 10

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

#[derive(Logos)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[logos(skip r"[ \t\f]+")]
#[rustfmt::skip]
enum Token {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")] Identifier,
    #[regex(r"\.[a-z][a-z0-9]+")]       Type,
    #[regex(r"[\n\r]+")]                Newline,
    #[regex(r"[0-9]+")]                 Decimal_Number,
    #[regex(r"0x[0-9a-fA-F]+")]         Hexadecimal_Number,
    #[regex(r"0b[01]+")]                Binary_Number,
    #[regex(r"0o[0-7]+")]               Octal_Number,

    #[token("[")] Open_Bracket,
    #[token("]")] Close_Bracket,
    #[token(":")] Colon,
    #[token(",")] Comma,
    #[token("=")] Equal,
    #[token("&")] Ampersand,
}

pub struct Assembler {
    lexer: Peekable<Lexer<'static, Token>>,
    labels: HashMap<String, usize>,
    data_bytes: Vec<u8>,
    data_addresses: HashMap<String, usize>,
    line: usize,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            lexer: Token::lexer(TEST).peekable(),
            labels: HashMap::new(),
            data_bytes: Vec::new(),
            data_addresses: HashMap::new(),
            line: 1,
        }
    }

    pub fn assemble(mut self) -> Program {
        use Token::*;

        while let Some(_) = self.lexer.peek() {
            // Segment
            if self.matches(Open_Bracket) {
                self.expect(Identifier);
                self.expect(Close_Bracket);
                self.expect(Newline);
            }
            // Label, Instruction, or Data
            else if self.matches(Identifier) {
                // Label
                if self.matches(Colon) {
                    self.expect(Newline);
                }
                // Data
                else if self.matches(Type) {
                    self.expect(Equal);
                    self.expect(Decimal_Number); // TODO: Support other expressions
                }
                // Instruction
                else {
                    loop {
                        if self.matches(Identifier) {}

                        if self.matches(Ampersand) {
                            self.expect(Identifier);
                        }

                        if !self.matches(Comma) {
                            break;
                        }
                    }

                    self.expect(Newline);
                }
            }
            // Empty Line, Who knows?
            else {
                self.next();
            }
        }

        Program {
            data: self.data_bytes,
            code: vec![],
            start: 0,
        }
    }

    fn matches(&mut self, expected: Token) -> bool {
        let mut matches = false;

        if let Some(&Ok(current)) = self.lexer.peek() {
            if current == expected {
                self.next();

                matches = true;
            }
        }

        matches
    }

    fn expect(&mut self, expected: Token) {
        if !self.matches(expected) {
            // TODO: Better error handling
            panic!(
                "Expecting: {:?}, found: {:?} ({})",
                expected,
                self.lexer.peek(),
                self.line
            );
        }
    }

    fn next(&mut self) {
        let next = self.lexer.next();

        if let Some(Ok(Token::Newline)) = next {
            self.line += 1;
        }
    }
}
