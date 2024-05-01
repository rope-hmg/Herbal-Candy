#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field {
    pub mask: u32,
    pub name: &'static str,
    pub ty:   &'static str,
    pub ctor: Option<&'static str>,
}

impl Field {
    pub const fn count(&self) -> u32 {
        self.mask.count_ones()
    }

    pub const fn shift(&self) -> u32 {
        self.mask.trailing_zeros()
    }
}

macro_rules! fields {
    ($(static $ident:ident : $t:ty = $mask:expr $( => $ctor:expr )? ),*,) => {
        $(
            pub static $ident: Field = Field {
                mask: $mask,
                name: stringify!($ident),
                ty:   stringify!($t),
                ctor: {
                    #[allow(unused_mut, unused_assignments)]
                    let mut ctor = None;

                    $(
                        ctor = Some($ctor);
                    )?

                    ctor
                },
            };
        )*

        pub static FIELDS: &[&Field] = &[$(&$ident,)*];
    };
}

#[rustfmt::skip]
fields![
    // I_TYPE
    static OP_CODE: u8       = 0b00000000_00000000_00000000_00001111,
    static FUNCT_4: u8       = 0b00000000_00000000_00000000_11110000,
    static RD:      Register = 0b00000000_00000000_00111111_00000000 => "from_index",
    static IMM_I16: i16      = 0b01111111_11111111_10000000_00000000,
    static SIGN:    u8       = 0b10000000_00000000_00000000_00000000,

    // U_TYPE
    //     OP_CODE             0b00000000_00000000_00000000_00001111
    //     FUNCT_4             0b00000000_00000000_00000000_11110000
    //     RD                  0b00000000_00000000_00111111_00000000
    static IMM_U16: u16      = 0b01111111_11111111_10000000_00000000,
    //     SIGN                0b10000000_00000000_00000000_00000000

    // R_TYPE
    //     OP_CODE             0b00000000_00000000_00000000_00001111
    //     FUNCT_4             0b00000000_00000000_00000000_11110000
    //     RD                  0b00000000_00000000_00111111_00000000
    static RS1:     Register = 0b00000000_00001111_11000000_00000000 => "from_index",
    static RS2:     Register = 0b00000011_11110000_00000000_00000000 => "from_index",
    static FUNCT_2: u8       = 0b00001100_00000000_00000000_00000000,
    static SIZE:    u8       = 0b00110000_00000000_00000000_00000000,
    static FLOAT:   u8       = 0b01000000_00000000_00000000_00000000,
    //     SIGN                0b10000000_00000000_00000000_00000000

    // J_TYPE
    //     OP_CODE             0b00000000_00000000_00000000_00001111
    //     FUNCT_4             0b00000000_00000000_00000000_11110000
    static IMM_24:  i32      = 0b11111111_11111111_11111111_00000000,

    // B_TYPE
    //     OP_CODE             0b00000000_00000000_00000000_00001111
    //     FUNCT_4             0b00000000_00000000_00000000_11110000
    static IMM_L6:  i16      = 0b00000000_00000000_00111111_00000000,
    //     RS1                 0b00000000_00001111_11000000_00000000
    //     RS2                 0b00000011_11110000_00000000_00000000
    static IMM_H6:  i16      = 0b11111100_00000000_00000000_00000000,
];

// NOTE:
// The order of the fields is the order in which they're parsed out of an instruction descriptor.
pub static I_TYPE: &[&Field] = &[&OP_CODE, &FUNCT_4, &RD, &IMM_I16, &SIGN];
pub static U_TYPE: &[&Field] = &[&OP_CODE, &FUNCT_4, &RD, &IMM_U16, &SIGN];
pub static R_TYPE: &[&Field] = &[&OP_CODE, &FUNCT_4, &RD, &RS1, &RS2, &FUNCT_2, &SIZE, &FLOAT, &SIGN];
pub static J_TYPE: &[&Field] = &[&OP_CODE, &FUNCT_4, &IMM_24];
pub static B_TYPE: &[&Field] = &[&OP_CODE, &FUNCT_4, &IMM_L6, &RS1, &RS2, &IMM_H6];
