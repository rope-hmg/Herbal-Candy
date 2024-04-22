use logos::{Lexer, Logos, SpannedIter};

use crate::token::{Token, Token_Kind, Token_Value, SIGNED};

#[derive(Logos)]
#[derive(Debug, Clone, Copy)]
#[logos(skip r"[ \t\f]+")]
#[rustfmt::skip]
enum Lexed_Token {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")] Identifier,
    #[regex(r"[\n\r]+")]                Newline,

    #[regex(r"[0-9]+", parse_dec)]
    #[regex(r"0x[0-9a-fA-F]+", parse_hex)]
    #[regex(r"0b[01]+", parse_bin)]
    #[regex(r"0o[0-7]+", parse_oct)]
    Integer(u64),

    #[token("[code]")]        Code,
    #[token("[data]")]        Data,
    #[token("call")]          Call,
    #[token("return")]        Return,
    #[token("move")]          Move,
    #[token("load_32")]       Load_32,
    #[token("sub_i32_s")]     Sub_i32_s,
    #[token("add_i32_s")]     Add_i32_s,
    #[token("jump_not_zero")] Jump_Not_Zero,
    #[token("halt")]          Halt,

    #[token(".i8")]  Type_i8,  #[token(".u8")]  Type_u8,
    #[token(".i16")] Type_i16, #[token(".u16")] Type_u16,
    #[token(".i32")] Type_i32, #[token(".u32")] Type_u32,
    #[token(".i64")] Type_i64, #[token(".u64")] Type_u64,

    #[token(":")] Colon,
    #[token(",")] Comma,
    #[token("&")] Ampersand,
}

fn parse_dec(lex: &mut Lexer<Lexed_Token>) -> Option<u64> {
    u64::from_str_radix(lex.slice(), 10).ok()
}

fn parse_hex(lex: &mut Lexer<Lexed_Token>) -> Option<u64> {
    let slice = lex.slice();
    u64::from_str_radix(&slice[2..], 16).ok()
}

fn parse_bin(lex: &mut Lexer<Lexed_Token>) -> Option<u64> {
    let slice = lex.slice();
    u64::from_str_radix(&slice[2..], 2).ok()
}

fn parse_oct(lex: &mut Lexer<Lexed_Token>) -> Option<u64> {
    let slice = lex.slice();
    u64::from_str_radix(&slice[2..], 8).ok()
}

pub struct Assembly_Lexer<'source> {
    tokens: SpannedIter<'source, Lexed_Token>,
    line: usize,
}

impl<'source> Assembly_Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            tokens: Lexed_Token::lexer(source).spanned(),
            line: 0,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn next(&mut self) -> Option<Token> {
        if let Some((lexed_token, range)) = self.tokens.next() {
            let (kind, value, flags) = match lexed_token.expect("Oops. Unexpected token.") {
                Lexed_Token::Identifier => (Token_Kind::Identifier, Token_Value::none(), 0),
                Lexed_Token::Code => (Token_Kind::Code_Segment, Token_Value::none(), 0),
                Lexed_Token::Data => (Token_Kind::Data_Segment, Token_Value::none(), 0),
                // I don't think this is the right way to do it.
                Lexed_Token::Call => (Token_Kind::Instruction, Token_Value::integer(0x1), 0),
                Lexed_Token::Return => (Token_Kind::Instruction, Token_Value::integer(0x2), 0),
                Lexed_Token::Move => (Token_Kind::Instruction, Token_Value::integer(0x3), 0),
                Lexed_Token::Load_32 => (Token_Kind::Instruction, Token_Value::integer(0x4), 0),
                Lexed_Token::Sub_i32_s => (Token_Kind::Instruction, Token_Value::integer(0x5), 0),
                Lexed_Token::Add_i32_s => (Token_Kind::Instruction, Token_Value::integer(0x6), 0),
                Lexed_Token::Jump_Not_Zero => {
                    (Token_Kind::Instruction, Token_Value::integer(0x7), 0)
                },
                Lexed_Token::Halt => (Token_Kind::Instruction, Token_Value::integer(0x8), 0),
                Lexed_Token::Type_i8 => (Token_Kind::Type, Token_Value::integer(8), SIGNED),
                Lexed_Token::Type_u8 => (Token_Kind::Type, Token_Value::integer(8), 0),
                Lexed_Token::Type_i16 => (Token_Kind::Type, Token_Value::integer(16), SIGNED),
                Lexed_Token::Type_u16 => (Token_Kind::Type, Token_Value::integer(16), 0),
                Lexed_Token::Type_i32 => (Token_Kind::Type, Token_Value::integer(32), SIGNED),
                Lexed_Token::Type_u32 => (Token_Kind::Type, Token_Value::integer(32), 0),
                Lexed_Token::Type_i64 => (Token_Kind::Type, Token_Value::integer(64), SIGNED),
                Lexed_Token::Type_u64 => (Token_Kind::Type, Token_Value::integer(64), 0),
                Lexed_Token::Newline => (Token_Kind::Newline, Token_Value::none(), 0),
                Lexed_Token::Integer(n) => (Token_Kind::Number, Token_Value::integer(n), 0),
                Lexed_Token::Colon => (Token_Kind::Colon, Token_Value::none(), 0),
                Lexed_Token::Comma => (Token_Kind::Comma, Token_Value::none(), 0),
                Lexed_Token::Ampersand => (Token_Kind::Ampersand, Token_Value::none(), 0),
            };

            let start = range.start;
            let end = range.end;

            if kind == Token_Kind::Newline {
                self.line += 1;
            }

            Some(Token::new(kind, value, flags, start, end))
        } else {
            None
        }
    }
}
