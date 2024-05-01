use logos::Logos;

use crate::layout::Field;

#[derive(Logos)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[logos(skip " ")]
enum Token {
    #[regex(r"[0-1]+")]
    Binary_Number,

    #[regex(r"[a-z][a-z0-9]*")]
    Identifier,
}

pub struct Instr_Field {
    pub name:   String,
    pub master: &'static Field,
    pub merged: Vec<&'static Field>,
}

pub struct Instr {
    pub name:   &'static str,
    pub ident:  String,
    pub layout: &'static [&'static Field],
    pub fields: Vec<Instr_Field>,
    pub values: Vec<(u32, &'static Field)>,
}

impl Instr {
    pub fn has_rd_field(&self) -> bool {
        // rd doesn't merge right now, so we could just check the master, but this is more
        // future-proof.
        self.fields
            .iter()
            .any(|f| f.master.name == "rd" || f.merged.iter().any(|f| f.name == "rd"))
    }
}

pub struct Instr_Descriptor {
    pub name:   &'static str,
    pub layout: &'static [&'static Field],
    pub bits:   &'static str,
}

impl Instr_Descriptor {
    pub const fn new(
        name: &'static str,
        layout: &'static [&'static Field],
        bits: &'static str,
    ) -> Self {
        Self { name, layout, bits }
    }

    pub fn parse(&self) -> Instr {
        let mut lexer = Token::lexer(self.bits);
        let mut index = 0;

        let mut instr = Instr {
            name:   self.name,
            ident:  self.generate_ident(),
            layout: self.layout,
            fields: Vec::new(),
            values: Vec::new(),
        };

        macro_rules! next_field {
            () => {{
                if let Some(field) = self.layout.get(index) {
                    index += 1;
                    field
                } else {
                    panic!(
                        "Too many fields for instruction '{}'. Expected {}, got {}",
                        self.name,
                        self.layout.len(),
                        index + 1 // +1 because 0-indexed
                    );
                }
            }};
        }

        while let Some(token) = lexer.next() {
            match token {
                Ok(Token::Binary_Number) => {
                    let field = next_field!();
                    let value = lexer.slice();

                    if value.len() != field.count() as usize {
                        panic!(
                            "Invalid number of bits for field '{}' of instruction '{}': '{}'. Expected {} bits, got {}",
                            field.name,
                            self.name,
                            value,
                            field.count(),
                            value.len()
                        );
                    }

                    let value = u32::from_str_radix(value, 2).unwrap();

                    instr.values.push((value, field));
                },

                Ok(Token::Identifier) => {
                    let field = next_field!();
                    let value = lexer.slice();

                    if let Some(Instr_Field { master, merged, .. }) = instr
                        .fields
                        .iter_mut()
                        .find(|Instr_Field { name, .. }| name == value)
                    {
                        if master.ctor.is_some() {
                            panic!(
                                "Unable to merge fields '{}' of instruction '{}' because of type '{}'",
                                value, self.name, master.ty,
                            );
                        }

                        if master.ty != field.ty {
                            panic!(
                                "Unable to merge fields '{}' of instruction '{}' due to type mismatch '{}' vs '{}' of instruction '{}'",
                                value, self.name, master.ty, field.ty, field.name,
                            );
                        }

                        let total_bits_available = master.ty[1..].parse().unwrap();
                        let bit_width_after_merge =
                            merged.iter().fold(master.count(), |c, f| c + f.count())
                                + field.count();

                        if bit_width_after_merge > total_bits_available {
                            panic!(
                                "Unable to merge fields '{}' of instruction '{}'. Total bit width ({}) exceeds that of type '{}'",
                                value, self.name, bit_width_after_merge, master.ty,
                            );
                        }

                        merged.push(field);
                    } else {
                        instr.fields.push(Instr_Field {
                            name:   value.to_owned(),
                            master: field,
                            merged: vec![],
                        });
                    }
                },

                _ => break,
            }
        }

        if index != self.layout.len() {
            panic!(
                "Too few fields for instruction '{}'. Expected {}, got {}.\nMissing fields: {:?}",
                self.name,
                self.layout.len(),
                index,
                &self.layout[index..]
                    .iter()
                    .map(|f| f.name)
                    .collect::<Vec<_>>()
            );
        }

        instr
    }

    pub fn generate_ident(&self) -> String {
        let mut ident = String::with_capacity(self.name.len());

        let mut seen_an_underscore = true;

        for c in self.name.bytes() {
            if c == b'_' {
                seen_an_underscore = true;
                ident.push(c as char);
            } else if c == b'.' {
                ident.push('_');
            } else {
                if seen_an_underscore {
                    ident.push(c.to_ascii_uppercase() as char);
                    seen_an_underscore = false;
                } else {
                    ident.push(c as char);
                }
            }
        }

        ident
    }
}
