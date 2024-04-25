use std::mem;

use byte_code::encoding;

use crate::{
    lexer::Assembly_Lexer,
    object::Object,
    parser::parse_instruction,
    token::{Token, Token_Kind},
};

pub struct Assembler<'source> {
    source:          &'source str,
    lexer:           Assembly_Lexer<'source>,
    pub object:      Object, // TEMP pub
    patch_labels:    Vec<(usize, &'source str)>,
    patch_addresses: Vec<(usize, &'source str)>,
    data_mode:       bool,
    can_continue:    bool,
    this_token:      Token,
    next_token:      Token,
    debug_depth:     usize,
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
            patch_labels: Vec::new(),
            patch_addresses: Vec::new(),
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

        for (instr_index, label) in self.patch_labels.drain(..) {
            if let Some(label_index) = self.object.code_labels.get(label) {
                // All immediate jumps are relative to the current instruction, so we calculate the
                // offset.
                let offset = *label_index as isize - instr_index as isize;
                let instr = &mut self.object.code_instrs[instr_index];

                encoding::re_encode_imm(instr, offset as i16);
            } else {
                panic!("Unknown label: {}", label);
            }
        }

        for (index, label) in self.patch_addresses.drain(..) {
            if let Some(layout) = self.object.data_layout.get(label) {
                // TODO: Come up with some way to ensure the instruction size matches the data size
                let instr = &mut self.object.code_instrs[index];

                encoding::re_encode_imm(instr, layout.address as i16);
            } else {
                panic!("Unknown label: {}", label);
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
                    &value.to_le_bytes()[..type_bit_width / 8],
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

        parse_instruction(self);
    }

    pub fn patch_address(&mut self) -> i16 {
        let label = self.entry().slice(self.source);
        let index = self.object.data_layout.get(label);

        if let Some(layout) = index {
            layout.address as i16
        } else {
            // We couldn't find the label, so we'll patch it later. We record the index of the
            // the instruction in the stream and the label. The current instruction hasn't been
            // pushed yet, the index is the current length.
            self.patch_labels
                .push((self.object.code_instrs.len(), label));

            0
        }
    }

    pub fn patch_label(&mut self) -> i16 {
        let label = self.entry().slice(self.source);

        let label_index = self.object.code_labels.get(label);
        let instr_index = self.object.code_instrs.len();

        if let Some(label_index) = label_index {
            (*label_index as isize - instr_index as isize) as i16
        } else {
            // We couldn't find the label, so we'll patch it later. We record the index of the
            // the instruction in the stream and the label. The current instruction hasn't been
            // pushed yet, the index is the current length.
            self.patch_labels
                .push((self.object.code_instrs.len(), label));

            0
        }
    }

    /// Remember that calling `matches` or `expects` will change the current token.
    pub fn entry(&self) -> Token {
        self.this_token
    }

    pub fn expects(&mut self, expected: Token_Kind) {
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

    pub fn matches(&mut self, expected: Token_Kind) -> bool {
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
