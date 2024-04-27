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

macro_rules! quote_if {
    ($cond:expr, $($tokens:tt)*) => {
        if $cond {
            quote!($($tokens)*)
        } else {
            quote!()
        }
    };
}

macro_rules! quote_if_let {
    ($pattern:pat = $ident:expr, $($tokens:tt)*) => {
        if let Some($pattern) = $ident {
            quote!($($tokens)*)
        } else {
            quote!()
        }
    };
}

fn main() {
    println!("cargo:rerun-if-changed=../instr_codegen/src/lib.rs");

    let mut instr_idents = Vec::new();
    let mut instr_definitions = Vec::new();
    let mut instr_decode_match_arms = Vec::new();
    let mut instr_patterns = Vec::new();
    let mut instr_creates = Vec::new();
    let mut encode_exprs = Vec::new();
    let mut decode_test_idents = Vec::new();
    let mut encode_test_idents = Vec::new();
    let mut test_instances = Vec::new();
    let mut test_instrs = Vec::new();

    for (name, descriptor) in INSTRUCTIONS.iter() {
        let instr_ident = format_ident!("{}", make_name_good(name));
        let decode_test_ident = format_ident!("decode_{}", instr_ident.to_string().to_lowercase());
        let encode_test_ident = format_ident!("encode_{}", instr_ident.to_string().to_lowercase());

        let instr_definition = match descriptor {
            Instr::V_Type { .. } => quote!(,),

            Instr::R_Type { fields, .. } => {
                let rd = quote_if!(fields.rd, rd:  Register,);
                let rs1 = quote_if!(fields.rs1, rs1: Register,);
                let rs2 = quote_if!(fields.rs2, rs2: Register,);

                quote!({ #rd #rs1 #rs2 },)
            },

            Instr::I_Type {
                fields, conditions, ..
            } => {
                let mut body = quote_if!(fields.rd, rd: Register,);

                if fields.imm {
                    body = if conditions.sign {
                        quote!(#body imm: i16,)
                    } else {
                        quote!(#body imm: u16,)
                    };
                }

                quote!({ #body },)
            },
        };

        let instr_decode_match_arm = match descriptor {
            Instr::V_Type { op_code, funct } => quote!(#op_code if funct == #funct),

            Instr::R_Type {
                op_code,
                funct,
                fields,
                conditions,
            } => {
                let size_check = quote_if_let!(size = conditions.size, && size == #size);
                let f_check = quote_if_let!(f = conditions.f, && f == #f);
                let s_check = quote_if_let!(s = conditions.s, && s == #s);
                let rd_check = quote_if!(fields.rd, &&!rd.is_readonly());

                quote! {
                    #op_code if funct == #funct
                        #size_check
                        #f_check
                        #s_check
                        #rd_check
                }
            },

            Instr::I_Type {
                op_code,
                funct,
                fields,
                ..
            } => {
                let rd_check = if fields.rd {
                    quote!(&&!rd.is_readonly())
                } else {
                    quote!()
                };

                quote!(#op_code if funct == #funct #rd_check)
            },
        };

        let (instr_pattern, instr_create) = match descriptor {
            Instr::V_Type { .. } => (quote!(), quote!()),

            Instr::R_Type { fields, .. } => {
                let rd = quote_if!(fields.rd, rd,);
                let rs1 = quote_if!(fields.rs1, rs1,);
                let rs2 = quote_if!(fields.rs2, rs2,);

                (quote!({ #rd #rs1 #rs2 }), quote!({ #rd #rs1 #rs2 }))
            },

            Instr::I_Type {
                fields, conditions, ..
            } => {
                let rd = quote_if!(fields.rd, rd,);

                let mut p = quote!();
                let mut b = quote!();
                if fields.imm {
                    p = quote!(imm,);
                    b = quote!(imm: imm as u16,);
                }

                (
                    quote!({ #rd #p }),
                    if conditions.sign {
                        quote!({ #rd #p })
                    } else {
                        quote!({ #rd #b })
                    },
                )
            },
        };

        let encode_expr = match descriptor {
            Instr::V_Type { op_code, funct } => {
                quote!(encode_funct(#funct) | encode_op_code(#op_code))
            },
            Instr::R_Type {
                op_code,
                funct,
                fields,
                conditions,
            } => {
                let rd = quote_if!(fields.rd, encode_rd(*rd) |);
                let rs1 = quote_if!(fields.rs1, encode_rs1(*rs1) |);
                let rs2 = quote_if!(fields.rs2, encode_rs2(*rs2) |);
                let size = quote_if_let!(size = conditions.size, encode_size(#size) |);
                let f = quote_if_let!(f = conditions.f, encode_f(#f) |);
                let s = quote_if_let!(s = conditions.s, encode_s(#s) |);

                quote!(#s #f #size #rs2 #rs1 #rd encode_funct(#funct) | encode_op_code(#op_code))
            },
            Instr::I_Type {
                op_code,
                funct,
                fields,
                conditions,
            } => {
                let rd = quote_if!(fields.rd, encode_rd(*rd) |);

                let mut imm = quote!();
                if fields.imm {
                    if conditions.sign {
                        imm = quote!(encode_imm(*imm) |)
                    } else {
                        imm = quote!(encode_imm(*imm as i16) |)
                    }
                }

                quote!(#imm #rd encode_funct(#funct) | encode_op_code(#op_code))
            },
        };

        // TODO: These should be encoded using the `encode_x` functions, but I'm not sure what the
        // best way to share code between build and runtime is. Might need to lift the encoding out
        // into it's own crate. Although it wouldn't actually affect if the tests pass or not, since
        // they're using the runtime encode and decode functions.
        let (test_instr, test_instance) = match descriptor {
            Instr::V_Type { op_code, funct } => (
                ((*funct as u32) << 6) | *op_code as u32,
                quote!(Instruction::#instr_ident),
            ),

            Instr::R_Type {
                op_code,
                funct,
                fields,
                conditions,
            } => {
                let mut instr = 0u32;
                let mut body = quote!();

                if fields.rd {
                    instr |= (5 + 6) << 10;
                    body = quote!(#body rd: Register::General_Purpose(5),);
                }

                if fields.rs1 {
                    instr |= (8 + 6) << 16;
                    body = quote!(#body rs1: Register::General_Purpose(8),);
                }

                if fields.rs2 {
                    instr |= (30 + 6) << 22;
                    body = quote!(#body rs2: Register::General_Purpose(30),);
                }

                if let Some(size) = conditions.size {
                    instr |= (map_size(size) as u32) << 28;
                }

                if let Some(f) = conditions.f {
                    instr |= (f as u32) << 30;
                }

                if let Some(s) = conditions.s {
                    instr |= (s as u32) << 31;
                }

                (
                    instr | ((*funct as u32) << 6) | *op_code as u32,
                    quote!(Instruction::#instr_ident { #body }),
                )
            },

            Instr::I_Type {
                op_code,
                funct,
                fields,
                conditions,
            } => {
                let mut instr = 0u32;
                let mut body = quote!();

                if fields.rd {
                    instr |= (5 + 6) << 10;
                    body = quote!(#body rd: Register::General_Purpose(5),);
                }

                if fields.imm {
                    if conditions.sign {
                        instr |= 0b1111111111110000 << 16;
                        body = quote!(#body imm: -16,);
                    } else {
                        instr |= 0b000000000010000 << 16;
                        body = quote!(#body imm: 16,);
                    }
                }

                (
                    instr | ((*funct as u32) << 6) | *op_code as u32,
                    quote!(Instruction::#instr_ident { #body }),
                )
            },
        };

        instr_idents.push(instr_ident);
        instr_definitions.push(instr_definition);
        instr_decode_match_arms.push(instr_decode_match_arm);
        instr_patterns.push(instr_pattern);
        instr_creates.push(instr_create);
        encode_exprs.push(encode_expr);
        decode_test_idents.push(decode_test_ident);
        encode_test_idents.push(encode_test_ident);
        test_instrs.push(test_instr);
        test_instances.push(test_instance);
    }

    let output = quote! {
        use crate::register::Register;
        use crate::encoding::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Instruction {
            #(#instr_idents #instr_definitions)*
            Invalid(u32),
        }

        impl Instruction {
            pub fn decode(instr: u32) -> Self {
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
                    #(
                        #instr_decode_match_arms => Self::#instr_idents #instr_creates,
                    )*
                    _ => Self::Invalid(instr),
                }
            }

            pub fn encode(&self) -> u32 {
                match self {
                    #(
                        Self::#instr_idents #instr_patterns => #encode_exprs,
                    )*
                    Self::Invalid(instr) => *instr,
                }
            }
        }

        #[cfg(test)]
        mod tests {
            #![allow(non_snake_case)]
            use super::*;

            #(
                #[test]
                fn #decode_test_idents() {
                    assert_eq!(Instruction::decode(#test_instrs), #test_instances);
                }

                #[test]
                fn #encode_test_idents() {
                    assert_eq!(#test_instances.encode(), #test_instrs);
                }
            )*
        }
    };

    let syntax_tree = syn::parse2(output).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let path = std::path::Path::new("src").join("instruction.rs");
    std::fs::write(path, formatted).unwrap();
}
