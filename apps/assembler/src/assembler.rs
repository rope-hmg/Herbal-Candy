use std::mem;

use crate::{
    lexer::Assembly_Lexer,
    object::Object,
    token::{Token, Token_Kind},
};

pub struct Assembler<'source> {
    source: &'source str,
    lexer: Assembly_Lexer<'source>,
    object: Object,
    data_mode: bool,
    can_continue: bool,
    this_token: Token,
    next_token: Token,
    debug_depth: usize,
}

impl<'source> Assembler<'source> {
    /// Create a new Assembler instance.
    ///
    /// Returns None if the source contains no tokens.
    pub fn new(source: &'source str) -> Option<Self> {
        let mut lexer = Assembly_Lexer::new(source);

        lexer.next().map(|n| Self {
            source,
            lexer,
            object: Object::new(),
            data_mode: false,
            can_continue: true,
            this_token: n,
            next_token: n,
            debug_depth: 0,
        })
    }

    pub fn assemble(mut self) -> Object {
        use Token_Kind::*;

        while self.can_continue {
            self.debug_depth = 0;

            if self.matches(Newline) {
                continue;
            }

            if self.matches(Code_Segment) {
                self.expects(Newline);
                println!("Code Segment");
                self.data_mode = false;
                continue;
            }

            if self.matches(Data_Segment) {
                self.expects(Newline);
                self.data_mode = true;
                continue;
            }

            if self.data_mode {
                self.parse_data_entry();
            } else {
                self.parse_code_entry();
            }
        }

        println!("Object: {}", self.object);

        self.object
    }

    fn parse_data_entry(&mut self) {
        use Token_Kind::*;

        if self.matches(Identifier) {
            let lexeme = self.entry().slice(self.source);

            self.expects(Type);

            let type_bit_width = unsafe { self.entry().value.integer } as usize;

            if self.matches(Number) {
                let value = unsafe { self.entry().value.integer };

                self.object.add_data_entry(
                    lexeme,
                    type_bit_width,
                    &value.to_le_bytes()[..type_bit_width / 4],
                );
            }

            self.expects(Newline);
        }
    }

    fn parse_code_entry(&mut self) {
        use Token_Kind::*;

        if self.matches(Identifier) {
            let lexeme = self.entry().slice(self.source);

            self.expects(Colon);
            self.expects(Newline);

            self.object.add_label(lexeme);
        }

        if self.matches(Instruction) {
            // object.code_bytes.push(0);

            while !self.matches(Newline) {
                if self.matches(Identifier) {
                    // register or label
                } else if self.matches(Ampersand) {
                    self.expects(Identifier);
                    // data reference
                } else {
                    // TODO: Better error handling
                    panic!(
                        "Unexpected token while parsing instruction arguments: {:?}",
                        self.next_token.slice(self.source)
                    );
                }

                if !self.matches(Comma) {
                    break;
                }
            }
        }
    }

    /// Remember that calling `matches` or `expects` will change the current token.
    fn entry(&self) -> Token {
        self.this_token
    }

    fn expects(&mut self, expected: Token_Kind) {
        if !self.matches(expected) {
            // TODO: Better error handling
            panic!(
                "Expecting: {:?}, found: {:?} ({})",
                expected,
                self.next_token,
                self.lexer.line()
            );
        }
    }

    fn matches(&mut self, expected: Token_Kind) -> bool {
        let matches = self.next_token.kind == expected;

        if matches {
            println!(
                "{}: {:?}\t{}",
                self.debug_depth,
                expected,
                self.next_token.slice(self.source)
            );
            self.debug_depth += 1;

            if let Some(next) = self.lexer.next() {
                self.this_token = mem::replace(&mut self.next_token, next);
                self.can_continue = true;
            } else {
                self.can_continue = false;
            }
        }

        matches
    }
}
