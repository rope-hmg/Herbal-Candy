use instr_codegen::{make_name_good, Instr, INSTRUCTIONS};
use quote::{format_ident, quote};

macro_rules! write_file {
    ($output:ident, $file_name:expr) => {{
        let syntax_tree = syn::parse2($output).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);

        let path = std::path::Path::new("src").join($file_name);
        std::fs::write(path, formatted).unwrap();
    }};
}

fn main() {
    println!("cargo:rerun-if-changed=../../libs/instr_codegen/src/lib.rs");

    let mut token_definitions = Vec::new();
    let mut token_to_token = Vec::new();
    let mut parsers_fns = Vec::new();
    let mut index_to_parser = Vec::new();

    for (index, (name, instr)) in INSTRUCTIONS.iter().enumerate() {
        let index = index as u64;

        let instr_name = format_ident!("{}", make_name_good(name));
        let parser_ident = format_ident!("parse_{}", instr_name.to_string().to_lowercase());

        let instr = match instr {
            Instr::V_Type { .. } => quote!(Instruction::#instr_name),

            Instr::R_Type { fields, .. } => {
                let mut body = quote!();

                if fields.rd {
                    body = if fields.rs1 || fields.rs2 {
                        quote!(#body rd: parse_register_comma(asm),)
                    } else {
                        quote!(#body rd: parse_register(asm),)
                    };
                }

                if fields.rs1 {
                    body = if fields.rs2 {
                        quote!(#body rs1: parse_register_comma(asm),)
                    } else {
                        quote!(#body rs1: parse_register(asm),)
                    };
                }

                if fields.rs2 {
                    body = quote!(#body rs2: parse_register(asm),);
                }

                quote! {
                    Instruction::#instr_name {
                        #body
                    }
                }
            },

            Instr::I_Type {
                fields, conditions, ..
            } => {
                let mut body = quote!();

                // TODO: Generalise this.
                if fields.rd {
                    body = if fields.imm {
                        quote!(#body rd: parse_register_comma(asm),)
                    } else {
                        quote!(#body rd: parse_register(asm),)
                    };
                }

                if fields.imm {
                    body = if conditions.sign {
                        quote!(#body imm: parse_imm(asm),)
                    } else {
                        quote!(#body imm: parse_imm(asm) as u16,)
                    };
                }

                quote! {
                    Instruction::#instr_name {
                        #body
                    }
                }
            },
        };

        token_definitions.push(quote!(#[token(#name)] #instr_name,));

        token_to_token.push(quote! {
            Lexed_Token::#instr_name => (
                Token_Kind::Instruction,
                Token_Value::integer(#index),
                0,
            ),
        });

        parsers_fns.push(quote! {
            fn #parser_ident<'source>(asm: &mut Assembler<'source>) {
                let instr = #instr;
                asm.expects(Token_Kind::Newline);
                asm.object.code_instrs.push(instr.encode());
            }
        });

        index_to_parser.push(quote! {
            #index => #parser_ident(asm),
        });
    }

    let lexer = quote::quote! {
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

            // TODO: Generate these
            #[token("zero", |_| 0)]
            #[token("one", |_| 1)]
            #[token("ip", |_| 2)]
            #[token("sp", |_| 3)]
            #[token("fp", |_| 4)]
            #[token("link", |_| 5)]
            #[regex("r[0-9]+", parse_register_index)]
            Register(u64),

            #[token("[code]")] Code,
            #[token("[data]")] Data,

            #(#token_definitions)*

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

        fn parse_register_index(lex: &mut Lexer<Lexed_Token>) -> u64 {
            let slice = lex.slice();
            u64::from_str_radix(&slice[1..], 10).unwrap() + 6 // TODO: Use the constant from byte_code::register
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
                        Lexed_Token::Register(index) => (Token_Kind::Register, Token_Value::integer(index), 0),
                        Lexed_Token::Code => (Token_Kind::Code_Segment, Token_Value::none(), 0),
                        Lexed_Token::Data => (Token_Kind::Data_Segment, Token_Value::none(), 0),

                        #(#token_to_token)*

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
    };

    let parser = quote! {
        #![allow(non_snake_case)]

        use crate::assembler::Assembler;
        use crate::token::{Token_Kind};

        use byte_code::{Instruction, Register};

        pub fn parse_instruction<'source>(asm: &mut Assembler<'source>) {
            if asm.matches(Token_Kind::Instruction) {
                let index = unsafe { asm.entry().value.integer };

                match index {
                    #(#index_to_parser)*
                    _ => unreachable!(),
                }
            }
        }

        #[inline(always)]
        fn parse_register<'source>(asm: &mut Assembler<'source>) -> Register {
            asm.expects(Token_Kind::Register);
            let index = unsafe { asm.entry().value.integer };
            Register::from_index(index as u8)
        }

        #[inline(always)]
        fn parse_register_comma<'source>(asm: &mut Assembler<'source>) -> Register {
            asm.expects(Token_Kind::Register);
            let index = unsafe { asm.entry().value.integer };
            asm.expects(Token_Kind::Comma);
            Register::from_index(index as u8)
        }

        #[inline(always)]
        fn parse_imm<'source>(asm: &mut Assembler<'source>) -> i16 {
            if asm.matches(Token_Kind::Colon) {
                asm.expects(Token_Kind::Identifier);
                asm.patch_label()
            } else if asm.matches(Token_Kind::Ampersand) {
                asm.expects(Token_Kind::Identifier);
                asm.patch_address()
            } else {
                asm.expects(Token_Kind::Number);
                unsafe { asm.entry().value.integer as i16 }
            }
        }

        #(#parsers_fns)*
    };

    write_file!(lexer, "lexer.rs");
    write_file!(parser, "parser.rs");
}
