use std::{collections::HashMap, fmt, iter::Peekable, mem, ops::Range};

use logos::{Lexer, Logos, SpannedIter};

use crate::program::Program;

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

#[derive(Logos)]
#[derive(Debug, Clone, Copy)]
#[logos(skip r"[ \t\f]+")]
#[rustfmt::skip]
enum Token_Lexer {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")] Identifier,
    #[regex(r"[\n\r]+")]                Newline,

    #[regex(r"[0-9]+", parse_dec)]
    #[regex(r"0x[0-9a-fA-F]+", parse_hex)]
    #[regex(r"0b[01]+", parse_bin)]
    #[regex(r"0o[0-7]+", parse_oct)]
    Number(i128),

    #[token(".i8")]  Type_i8,  #[token(".u8")]  Type_u8,
    #[token(".i16")] Type_i16, #[token(".u16")] Type_u16,
    #[token(".i32")] Type_i32, #[token(".u32")] Type_u32,
    #[token(".i64")] Type_i64, #[token(".u64")] Type_u64,

    #[token("[")] Open_Bracket,
    #[token("]")] Close_Bracket,
    #[token(":")] Colon,
    #[token(",")] Comma,
    #[token("&")] Ampersand,
}

fn parse_dec(lex: &mut Lexer<Token_Lexer>) -> Option<i128> {
    i128::from_str_radix(lex.slice(), 10).ok()
}

fn parse_hex(lex: &mut Lexer<Token_Lexer>) -> Option<i128> {
    let slice = lex.slice();
    i128::from_str_radix(&slice[2..], 16).ok()
}

fn parse_bin(lex: &mut Lexer<Token_Lexer>) -> Option<i128> {
    let slice = lex.slice();
    i128::from_str_radix(&slice[2..], 2).ok()
}

fn parse_oct(lex: &mut Lexer<Token_Lexer>) -> Option<i128> {
    let slice = lex.slice();
    i128::from_str_radix(&slice[2..], 8).ok()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Identifier,
    Type,
    Newline,
    Number,
    Open_Bracket,
    Close_Bracket,
    Colon,
    Comma,
    Ampersand,
}

#[derive(Clone, Copy)]
union Value {
    number: i128,
    none: (),
}

impl Value {
    pub fn none() -> Self {
        Self { none: () }
    }

    pub fn number(number: i128) -> Self {
        Self { number }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Value")
    }
}

#[derive(Debug, Clone, Copy)]
struct Entry {
    token: Token,
    value: Value,
    start: usize,
    end: usize,
}

impl TryFrom<(Result<Token_Lexer, ()>, Range<usize>)> for Entry {
    type Error = ();

    fn try_from(
        (token_lexer, range): (Result<Token_Lexer, ()>, Range<usize>),
    ) -> Result<Self, Self::Error> {
        let (token, value) = match token_lexer? {
            Token_Lexer::Identifier => (Token::Identifier, Value::none()),
            Token_Lexer::Type_i8 => (Token::Type, Value::number(8)),
            Token_Lexer::Type_u8 => (Token::Type, Value::number(8)),
            Token_Lexer::Type_i16 => (Token::Type, Value::number(16)),
            Token_Lexer::Type_u16 => (Token::Type, Value::number(16)),
            Token_Lexer::Type_i32 => (Token::Type, Value::number(32)),
            Token_Lexer::Type_u32 => (Token::Type, Value::number(32)),
            Token_Lexer::Type_i64 => (Token::Type, Value::number(64)),
            Token_Lexer::Type_u64 => (Token::Type, Value::number(64)),
            Token_Lexer::Newline => (Token::Newline, Value::none()),
            Token_Lexer::Number(n) => (Token::Number, Value::number(n)),
            Token_Lexer::Open_Bracket => (Token::Open_Bracket, Value::none()),
            Token_Lexer::Close_Bracket => (Token::Close_Bracket, Value::none()),
            Token_Lexer::Colon => (Token::Colon, Value::none()),
            Token_Lexer::Comma => (Token::Comma, Value::none()),
            Token_Lexer::Ampersand => (Token::Ampersand, Value::none()),
        };

        let start = range.start;
        let end = range.end;

        Ok(Self {
            token,
            value,
            start,
            end,
        })
    }
}

pub struct Asm_Lexer<'source> {
    tokens: Peekable<SpannedIter<'source, Token_Lexer>>,
    line: usize,
}

impl<'source> Asm_Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            tokens: Token_Lexer::lexer(source).spanned().peekable(),
            line: 0,
        }
    }

    fn next(&mut self) -> Option<Entry> {
        let next = self.tokens.next().map(|result| result.try_into().unwrap());

        if let Some(Entry {
            token: Token::Newline,
            ..
        }) = next
        {
            self.line += 1;
        }

        next
    }
}

#[derive(Debug, Clone, Copy)]
struct Data_Layout {
    address: usize,
    size: usize,
}

pub struct Assembler {
    source: &'static str,
    lexer: Asm_Lexer<'static>,
    labels: HashMap<String, usize>,
    data_bytes: Vec<u8>,
    data_layouts: HashMap<String, Data_Layout>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            source: TEST,
            lexer: Asm_Lexer::new(TEST),
            labels: HashMap::new(),
            data_bytes: Vec::new(),
            data_layouts: HashMap::new(),
        }
    }

    pub fn assemble(mut self) -> Program {
        let mut under_scrutiny = self.lexer.next();

        'no_more_tokens: while let Some(mut entry_under_scrutiny) = under_scrutiny {
            let mut entry_just_matched = entry_under_scrutiny;

            /// This macro only exists to remind you that `entry` is mutated after every `matches!` call.
            #[rustfmt::skip]
            macro_rules! entry { () => { entry_just_matched }; }

            let mut depth = 0;
            macro_rules! matches {
                ($expected:ident) => {{
                    let mut matches = false;

                    if entry_under_scrutiny.token == Token::$expected {
                        println!(
                            "{depth}: {:?}\t{}",
                            Token::$expected,
                            &self.source[entry_under_scrutiny.start..entry_under_scrutiny.end]
                        );
                        {
                            let new_depth = depth + 1;
                            let _ = mem::replace(&mut depth, new_depth);
                        }

                        entry_just_matched = entry_under_scrutiny;

                        under_scrutiny = self.lexer.next();
                        let _ = mem::replace(
                            &mut entry_under_scrutiny,
                            if let Some(n) = under_scrutiny {
                                n
                            } else {
                                break 'no_more_tokens;
                            },
                        );

                        matches = true;
                    }

                    matches
                }};
            }

            macro_rules! expect {
                ($expected:ident) => {
                    if !matches!($expected) {
                        // TODO: Better error handling
                        panic!(
                            "Expecting: {:?}, found: {:?} ({})",
                            Token::$expected,
                            entry!(),
                            self.lexer.line
                        );
                    }
                };
            }

            // Segment
            if matches!(Open_Bracket) {
                expect!(Identifier);
                expect!(Close_Bracket);
            }
            // Label, Instruction, or Data
            else if matches!(Identifier) {
                let Entry {
                    start: lexeme_start,
                    end: lexeme_end,
                    ..
                } = entry!();

                // Label
                if matches!(Colon) {
                }
                // Data
                else if matches!(Type) {
                    let type_bit_width = unsafe { entry!().value.number };

                    if matches!(Number) {
                        let lexeme = &self.source[lexeme_start..lexeme_end];
                        let address = self.data_bytes.len();
                        let size = type_bit_width as usize;
                        self.data_layouts
                            .insert(lexeme.to_string(), Data_Layout { address, size });

                        let Entry { value, .. } = entry!();
                        self.data_bytes
                            .extend_from_slice(&unsafe { value.number }.to_le_bytes()[..size / 4]);
                    }
                }
                // Instruction
                else {
                    while !matches!(Newline) {
                        if matches!(Identifier) {
                        } else if matches!(Ampersand) {
                            expect!(Identifier);
                        }

                        if !matches!(Comma) {
                            break;
                        }
                    }
                }
            } else {
                expect!(Newline);
            }
        }

        println!("---------------------------");
        println!("{:?}", self.data_bytes);
        println!("{:?}", self.data_layouts);

        Program {
            data: self.data_bytes,
            code: vec![],
            start: 0,
        }
    }
}
