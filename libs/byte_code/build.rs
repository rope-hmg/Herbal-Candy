use instr_codegen::{instr::Instr_Field, layout::FIELDS, INSTRUCTIONS};
use quote::{format_ident, quote};

macro_rules! quote_if {
    ($cond:expr, $($tokens:tt)*) => {
        if $cond {
            quote!($($tokens)*)
        } else {
            quote!()
        }
    };
}

fn main() {
    println!("cargo:rerun-if-changed=../instr_codegen/src/lib.rs");

    let mut field_decodes = quote!();

    for field in FIELDS.iter() {
        let field_ident = format_ident!("{}", field.name.to_lowercase());

        let mask = field.mask;
        let shift = field.shift();

        // TODO: Make this data driven
        field_decodes = quote! {
            #field_decodes
            let #field_ident = (instr & #mask) >> #shift;
        };
    }

    let mut instr_idents = Vec::new();
    let mut instr_definitions = Vec::new();
    let mut instr_decode_match_arms = Vec::new();
    let mut instr_instantiations = Vec::new();
    let mut instr_patterns = Vec::new();
    let mut encode_exprs = Vec::new();
    let mut decode_test_idents = Vec::new();
    let mut encode_test_idents = Vec::new();
    let mut test_instances = Vec::new();
    let mut test_instrs = Vec::new();

    for descriptor in INSTRUCTIONS.iter() {
        let instr = descriptor.parse();

        let instr_ident = format_ident!("{}", instr.ident);
        let decode_test_ident = format_ident!("decode_{}", instr_ident.to_string().to_lowercase());
        let encode_test_ident = format_ident!("encode_{}", instr_ident.to_string().to_lowercase());

        let instr_definition = {
            if instr.fields.is_empty() {
                quote!(,)
            } else {
                let mut body = quote!();

                for Instr_Field { name, master, .. } in instr.fields.iter() {
                    let field_ident = format_ident!("{}", name);
                    let field_type = format_ident!("{}", master.ty);

                    body = quote!(#body #field_ident: #field_type,);
                }

                quote!({ #body },)
            }
        };

        let instr_decode_match_arm = {
            let op_code = instr.values[0].0;
            let funct_4 = instr.values[1].0;

            let mut match_arm = quote!(#op_code if funct_4 == #funct_4);

            for (value, field) in instr.values.iter().skip(2) {
                let ident = format_ident!("{}", field.name.to_lowercase());
                match_arm = quote!(#match_arm && #ident == #value);
            }

            let rd_check = quote_if!(instr.has_rd_field(), &&rd > 7);

            quote!(#match_arm #rd_check)
        };

        let instr_instantiation = {
            let mut body = quote!();

            if instr.fields.is_empty() {
                body
            } else {
                for Instr_Field {
                    name,
                    master,
                    merged,
                } in instr.fields.iter()
                {
                    let name_ident = format_ident!("{}", name);
                    let type_ident = format_ident!("{}", master.ty);

                    if let Some(ctor) = master.ctor {
                        let ctor = format_ident!("{}", ctor);

                        body = quote!(#body #name_ident: #type_ident::#ctor(#name_ident as u8),);
                    } else {
                        let ident = format_ident!("{}", master.name.to_lowercase());

                        if merged.is_empty() {
                            body = quote!(#body #name_ident: #ident as #type_ident,);
                        } else {
                            // The fields are already shifted to the right:
                            //
                            // master:   00000000 00000000 00000000 01111111
                            // merged 0: 00000000 00000000 00000000 00001111
                            // merged N: 00000000 00000000 00000000 00000011
                            //
                            // result:   00000000 00000000 00011111 11111111
                            //                                |master |0  |N
                            //
                            // count the number of ones in the merged masks
                            // shift the field to the left by that count
                            // reduce the shift count by the number of ones in the next field
                            // repeat until all fields are merged

                            let mut shift_count = merged.iter().fold(0, |c, f| c + f.count());
                            let mut merge_expr = quote!((#ident << #shift_count) as #type_ident);

                            for field in merged.iter() {
                                let ident = format_ident!("{}", field.name.to_lowercase());

                                shift_count -= field.count();
                                if shift_count == 0 {
                                    merge_expr = quote!(#merge_expr | #ident as #type_ident);
                                } else {
                                    merge_expr = quote!(#merge_expr | (#ident << #shift_count) as #type_ident);
                                }
                            }

                            body = quote!(#body #name_ident: #merge_expr,);
                        }
                    }
                }

                quote!({ #body })
            }
        };

        let instr_pattern = {
            let mut body = quote!();

            if instr.fields.is_empty() {
                body
            } else {
                for Instr_Field { name, .. } in instr.fields.iter() {
                    let ident = format_ident!("{}", name);
                    body = quote!(#body #ident,);
                }

                quote!({ #body })
            }
        };

        let encode_expr = {
            let mut expr = 0;

            for (value, field) in instr.values.iter() {
                let shift = field.shift();
                let mask = field.mask;

                expr |= (value << shift) & mask;
            }

            let mut expr = quote!(#expr);

            for Instr_Field {
                name,
                master,
                merged,
            } in instr.fields.iter()
            {
                let ident = format_ident!("{}", name);
                let shift = master.shift();
                let mask = master.mask;

                if master.ctor.is_some() {
                    expr = quote!(#expr | (((#ident.index() as u32) << #shift) & #mask));
                } else {
                    if merged.is_empty() {
                        expr = quote!(#expr | (((*#ident as u32) << #shift) & #mask));
                    } else {
                        // We need to unmerge the result and put the bits in the correct fields:
                        //
                        // result:   00000000 00000000 00011111 11111111
                        //                                |master |0  |N
                        //
                        // master:   11111110 00000000 00000000 00000000
                        // merged 0: 00000000 00000000 11110000 00000000
                        // merged N: 00000000 00000000 00000000 00011000
                        //
                        // count the number of ones in the merged masks
                        // reduce the field's shift value by that count
                        // shift the field to the left by the reduced amount
                        // repeat until all fields are unmerged
                        let mut shift_reduce = merged.iter().fold(0, |c, f| c + f.count());
                        let mut shift_amount = shift - shift_reduce;

                        expr = quote!(#expr | (((*#ident as u32) << #shift_amount) & #mask));
                        for field in merged.iter() {
                            shift_reduce -= field.count();
                            shift_amount = field.shift() - shift_reduce;

                            let mask = field.mask;

                            expr = quote!(#expr | (((*#ident as u32) << #shift_amount) & #mask));
                        }
                    }
                }
            }

            expr
        };

        let (test_instr, test_instance) = {
            let mut test_instr = 0u32;

            for (value, field) in instr.values.iter() {
                test_instr |= (value << field.shift()) & field.mask;
            }

            let test_fields = if instr.fields.is_empty() {
                quote!()
            } else {
                let mut test_fields = quote!();

                for Instr_Field {
                    name,
                    master,
                    merged,
                } in instr.fields.iter()
                {
                    let name_ident = format_ident!("{}", name);
                    let type_ident = format_ident!("{}", master.ty);

                    let value = 10u32;

                    if merged.is_empty() {
                        test_instr |= (value << master.shift()) & master.mask;
                    } else {
                        let mut shift_reduce = merged.iter().fold(0, |c, f| c + f.count());
                        let mut shift_amount = master.shift() - shift_reduce;
                        test_instr |= (value << shift_amount) & master.mask;

                        for field in merged.iter() {
                            shift_reduce -= field.count();
                            shift_amount = field.shift() - shift_reduce;
                            test_instr |= (value << shift_amount) & field.mask;
                        }
                    }

                    if let Some(ctor) = master.ctor {
                        let ctor = format_ident!("{}", ctor);
                        test_fields = quote!(#test_fields #name_ident: #type_ident::#ctor(10),);
                    } else {
                        test_fields = quote!(#test_fields #name_ident: 10,);
                    }
                }

                quote!({ #test_fields })
            };

            (test_instr, quote!(Instruction::#instr_ident #test_fields))
        };

        instr_idents.push(instr_ident);
        instr_definitions.push(instr_definition);
        instr_decode_match_arms.push(instr_decode_match_arm);
        instr_instantiations.push(instr_instantiation);
        instr_patterns.push(instr_pattern);
        encode_exprs.push(encode_expr);
        decode_test_idents.push(decode_test_ident);
        encode_test_idents.push(encode_test_ident);
        test_instrs.push(test_instr);
        test_instances.push(test_instance);
    }

    let output = quote! {
        use crate::register::Register;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Instruction {
            #(#instr_idents #instr_definitions)*
            Invalid(u32),
        }

        impl Instruction {
            pub fn decode(instr: u32) -> Self {
                #field_decodes

                match op_code {
                    #(
                        #instr_decode_match_arms => Self::#instr_idents #instr_instantiations,
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
                fn #instr_idents() {
                    let instr = Instruction::decode(#test_instrs);
                    println!("{:032b}", #test_instrs);
                    assert_eq!(instr, #test_instances);
                    assert_eq!(instr.encode(), #test_instrs);
                }
            )*
        }
    };

    let syntax_tree = syn::parse2(output).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let path = std::path::Path::new("src").join("instruction.rs");
    std::fs::write(path, formatted).unwrap();
}
