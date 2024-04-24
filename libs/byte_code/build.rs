use instr_codegen::{make_name_good, Instr, INSTRUCTIONS};
use quote::{format_ident, quote};

// TODO: Figure out how to import `byte_code::encoding::map_size`.
#[inline(always)]
pub fn map_size(size: u8) -> u8 {
    match size {
        0b0000_1000 => 0,
        0b0001_0000 => 1,
        0b0010_0000 => 2,
        0b0100_0000 => 3,
        _ => panic!("Invalid size {}", size),
    }
}

fn main() {
    let mut enum_variants = Vec::new();
    let mut decode_match_arms = Vec::new();
    let mut encode_match_arms = Vec::new();
    let mut test_functs = Vec::new();

    for (name, instr) in INSTRUCTIONS.iter() {
        let instr_ident = format_ident!("{}", make_name_good(name));
        let decode_test_ident = format_ident!("decode_{}", instr_ident);
        let encode_test_ident = format_ident!("encode_{}", instr_ident);

        let (enum_definition, decode_match_arm, encode_match_arm, bytes, instr) = match instr {
            Instr::V_Type { op_code, funct } => {
                let encoded = (*funct as u32) << 6 | *op_code as u32;

                (
                    quote!(#instr_ident,),
                    quote!(#op_code if funct == #funct => Instruction::#instr_ident,),
                    quote!(Instruction::#instr_ident => #encoded,),
                    encoded,
                    quote!(Instruction::#instr_ident),
                )
            },

            Instr::R_Type {
                op_code,
                funct,
                fields,
                conditions,
            } => {
                let (rd_check, rd_decl, rd_decode, rd_encode, rd_test, rd_bytes) = if fields.rd {
                    (
                        quote!(&&!rd.is_readonly()),
                        quote!(rd: Register,),
                        quote!(rd,),
                        quote!(encode_rd(*rd)),
                        quote!(rd: Register::General_Purpose(5),),
                        0b000101,
                    )
                } else {
                    (quote!(), quote!(), quote!(), quote!(0), quote!(), 0)
                };

                let (rs1_decl, rs1_decode, rs1_encode, rs1_test, rs1_bytes) = if fields.rs1 {
                    (
                        quote!(rs1: Register,),
                        quote!(rs1,),
                        quote!(encode_rs1(*rs1)),
                        quote!(rs1: Register::General_Purpose(8),),
                        0b001000,
                    )
                } else {
                    (quote!(), quote!(), quote!(0), quote!(), 0)
                };

                let (rs2_decl, rs2_decode, rs2_encode, rs2_test, rs2_bytes) = if fields.rs2 {
                    (
                        quote!(rs2: Register,),
                        quote!(rs2,),
                        quote!(encode_rs2(*rs2)),
                        quote!(rs2: Register::General_Purpose(6),),
                        0b000110,
                    )
                } else {
                    (quote!(), quote!(), quote!(0), quote!(), 0)
                };

                let (size_match_condition, (size_byte, size_value, size_encode)) =
                    if let Some(size) = conditions.size {
                        (
                            quote!(&& size == #size),
                            (map_size(size), size, quote!(encode_size(#size))),
                        )
                    } else {
                        (
                            quote!(),
                            if fields.size {
                                (0b11, 64, quote!(encode_size(*size)))
                            } else {
                                (0, 0, quote!(0))
                            },
                        )
                    };

                let (size_decl, size_decode, size_test) = if fields.size {
                    (quote!(size: u8,), quote!(size,), quote!(size: #size_value,))
                } else {
                    (quote!(), quote!(), quote!())
                };

                let (f_match_condition, f_byte, f_value, f_encode) = if let Some(f) = conditions.f {
                    (quote!(&& f == #f), f, f == 1, quote!(encode_f(#f)))
                } else {
                    (
                        quote!(),
                        0,
                        false,
                        if fields.f {
                            quote!(encode_f(*f))
                        } else {
                            quote!(0)
                        },
                    )
                };

                let (f_decl, f_decode, f_test) = if fields.f {
                    (quote!(f: bool,), quote!(f,), quote!(f: #f_value,))
                } else {
                    (quote!(), quote!(), quote!())
                };

                let (s_match_condition, s_byte, s_value, s_encode) = if let Some(s) = conditions.s {
                    (quote!(&& s == #s), s, s == 1, quote!(encode_s(#s)))
                } else {
                    (
                        quote!(),
                        0,
                        false,
                        if fields.s {
                            quote!(encode_s(*s))
                        } else {
                            quote!(0)
                        },
                    )
                };

                let (s_decl, s_decode, s_test) = if fields.s {
                    (quote!(s: bool,), quote!(s,), quote!(s: #s_value,))
                } else {
                    (quote!(), quote!(), quote!())
                };

                let instr_match = quote! {
                    Instruction::#instr_ident {
                        #rd_decode
                        #rs1_decode
                        #rs2_decode
                        #size_decode
                        #f_decode
                        #s_decode
                    }
                };

                (
                    quote! {
                        #instr_ident {
                            #rd_decl
                            #rs1_decl
                            #rs2_decl
                            #size_decl
                            #f_decl
                            #s_decl
                        },
                    },
                    quote! {
                        #op_code if funct == #funct
                            #size_match_condition
                            #f_match_condition
                            #s_match_condition
                            #rd_check
                            => #instr_match,
                    },
                    quote!(#instr_match => #s_encode | #f_encode | #size_encode | #rs2_encode | #rs1_encode | #rd_encode | encode_funct(#funct) | encode_op_code(#op_code),),
                    (s_byte as u32) << 31
                        | (f_byte as u32) << 30
                        | (size_byte as u32) << 28
                        | (rs2_bytes as u32) << 22
                        | (rs1_bytes as u32) << 16
                        | (rd_bytes as u32) << 10
                        | (*funct as u32) << 6
                        | *op_code as u32,
                    quote!(Instruction::#instr_ident {
                        #rd_test
                        #rs1_test
                        #rs2_test
                        #size_test
                        #f_test
                        #s_test
                    }),
                )
            },

            Instr::I_Type {
                op_code,
                funct,
                fields,
            } => {
                let (rd_check, rd_decl, rd_decode, rd_encode, rd_test, rd_bytes) = if fields.rd {
                    (
                        quote!(&&!rd.is_readonly()),
                        quote!(rd: Register,),
                        quote!(rd,),
                        quote!(encode_rd(*rd)),
                        quote!(rd: Register::General_Purpose(5),),
                        0b000101,
                    )
                } else {
                    (quote!(), quote!(), quote!(), quote!(0), quote!(), 0)
                };

                let (imm_decl, imm_decode, imm_encode, imm_test, imm_bytes) = if fields.imm {
                    (
                        quote!(imm: i16,),
                        quote!(imm,),
                        quote!(encode_imm(*imm)),
                        quote!(imm: -16,),
                        0b1111111111110000,
                    )
                } else {
                    (quote!(), quote!(), quote!(0), quote!(), 0)
                };

                let instr_match = quote! {
                    Instruction::#instr_ident {
                        #rd_decode
                        #imm_decode
                    }
                };

                (
                    quote! {
                        #instr_ident {
                            #rd_decl
                            #imm_decl
                        },
                    },
                    quote! {
                        #op_code if funct == #funct
                            #rd_check
                            => #instr_match,
                    },
                    quote!(#instr_match => #imm_encode | #rd_encode | encode_funct(#funct) | encode_op_code(#op_code),),
                    (imm_bytes as u32) << 16
                        | (rd_bytes as u32) << 10
                        | (*funct as u32) << 6
                        | *op_code as u32,
                    quote!(Instruction::#instr_ident {
                        #rd_test
                        #imm_test
                    }),
                )
            },
        };

        enum_variants.push(enum_definition);
        decode_match_arms.push(decode_match_arm);
        encode_match_arms.push(encode_match_arm);
        test_functs.push(quote! {
            #[test]
            fn #decode_test_ident() {
                assert_eq!(Instruction::decode(#bytes), #instr);
            }

            #[test]
            fn #encode_test_ident() {
                assert_eq!(#instr.encode(), #bytes);
            }
        });
    }

    let output = quote! {
        use crate::register::Register;
        use crate::encoding::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Instruction {
            #(#enum_variants)*
            Invalid(u32),
        }

        impl Instruction {
            pub fn decode(instr: u32) -> Instruction {
                let op_code   = decode_op_code(instr);
                let funct     = decode_funct  (instr);
                let rd        = decode_rd     (instr);
                let rs1       = decode_rs1    (instr);
                let rs2       = decode_rs2    (instr);
                let size      = decode_size   (instr);
                let f         = decode_f      (instr);
                let s         = decode_s      (instr);
                let imm       = decode_imm    (instr);

                match op_code {
                    #(#decode_match_arms)*
                    _ => Instruction::Invalid(instr),
                }
            }

            pub fn encode(&self) -> u32 {
                match self {
                    #(#encode_match_arms)*
                    Instruction::Invalid(instr) => *instr,
                }
            }
        }

        #[cfg(test)]
        mod tests {
            #![allow(non_snake_case)]
            use super::*;

            #(#test_functs)*
        }
    };

    let syntax_tree = syn::parse2(output).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let path = std::path::Path::new("src").join("instruction.rs");
    std::fs::write(path, formatted).unwrap();
}
