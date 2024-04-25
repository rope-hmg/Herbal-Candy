use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token_Kind {
    Identifier,
    Instruction,
    Register,
    Code_Segment,
    Data_Segment,
    Type,
    Newline,
    Number,
    Colon,
    Comma,
    Ampersand,
}

#[derive(Clone, Copy)]
pub union Token_Value {
    pub integer: u64,
    pub none:    (),
}

impl Token_Value {
    pub fn none() -> Self {
        Self { none: () }
    }

    pub fn integer(integer: u64) -> Self {
        Self { integer }
    }
}

impl fmt::Debug for Token_Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Value")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind:  Token_Kind,
    pub value: Token_Value,
    pub flags: u8,
    start:     usize,
    end:       usize,
}

impl Token {
    pub fn new(kind: Token_Kind, value: Token_Value, flags: u8, start: usize, end: usize) -> Self {
        Self {
            kind,
            value,
            flags,
            start,
            end,
        }
    }

    pub fn slice<'source>(&self, source: &'source str) -> &'source str {
        &source[self.start..self.end]
    }
}

pub const SIGNED: u8 = 0b0000_0001;
