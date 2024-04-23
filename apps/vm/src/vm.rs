use byte_code::{Bit_Width, Instruction, Memory_Address, Register};

use crate::{
    convert::{Destination, Source},
    memory::Memory,
    program::Program,
};

pub const REGISTER_COUNT: usize = 64;

pub struct Virtual_Machine<'program> {
    registers:   Box<[u64; REGISTER_COUNT]>,
    environment: Box<[fn(&mut Virtual_Machine)]>,
    memory:      Memory,
    program:     Option<&'program Program>,
}

pub enum Instruction_Pointer {
    Goto(u64),
    Next,
    Halt,
}

impl<'program> Virtual_Machine<'program> {
    pub fn new(memory_size: usize) -> Self {
        let mut vm = Self {
            registers:   Box::new([0; REGISTER_COUNT]),
            environment: Vec::new().into_boxed_slice(),
            memory:      Memory::new(memory_size),
            program:     None,
        };

        vm.set_register(Register::One, 1);
        vm.set_register(Register::Stack_Pointer, memory_size as u64);
        vm.set_register(Register::Frame_Pointer, memory_size as u64);
        vm
    }

    pub fn set_program(&mut self, program: &'program Program) {
        self.program = Some(program);
        self.set_register(Register::Instruction_Pointer, program.start as u64);

        // TODO: Either verify that the program only uses addresses that are within the bounds of
        // its data, or clear out the memory before writing the program data. Otherwise, the program
        // could read memory that was previously used by another program.
        self.memory
            .write(program.data.as_slice(), Memory_Address(0));
    }

    pub fn step(&mut self) -> bool {
        let mut should_continue = true;

        if let Some(instruction) = self.program.and_then(|program| {
            program
                .code
                .get(self.register(Register::Instruction_Pointer) as usize)
        }) {
            println!("------------------");
            println!(
                "{:X}: {:?}",
                self.register(Register::Instruction_Pointer),
                instruction
            );

            match self.execute(*instruction) {
                Instruction_Pointer::Halt => {
                    should_continue = false;
                },
                Instruction_Pointer::Next => self.increment_register(Register::Instruction_Pointer),
                Instruction_Pointer::Goto(value) => {
                    self.set_register(Register::Instruction_Pointer, value);
                },
            }

            println!("r0: {}", self.register(Register::from_index(5)));
            println!("r1: {}", self.register(Register::from_index(6)));
            println!("r2: {}", self.register(Register::from_index(7)));
            println!("r3: {}", self.register(Register::from_index(8)));
            println!("r5: {:b}", self.register(Register::from_index(10)));
        } else {
            should_continue = false;
        }

        should_continue
    }

    #[inline(always)]
    fn register(&self, register: Register) -> u64 {
        self.registers[register.index()]
    }

    #[inline(always)]
    fn set_register(&mut self, register: Register, value: u64) {
        self.registers[register.index()] = value;
    }

    #[inline(always)]
    fn increment_register(&mut self, register: Register) {
        self.registers[register.index()] += 1;
    }

    #[inline(always)]
    fn increment_register_n(&mut self, register: Register, n: u64) {
        self.registers[register.index()] += n;
    }

    #[inline(always)]
    fn decrement_register_n(&mut self, register: Register, n: u64) {
        self.registers[register.index()] -= n;
    }

    fn load(&mut self, register: Register, address: Memory_Address, bit_width: Bit_Width) {
        let mut bytes = [0; 8];
        let data = &mut bytes[..bit_width.byte_count()];

        self.memory.read(address, data);

        self.set_register(register, u64::from_le_bytes(bytes));
    }

    fn store(&mut self, address: Memory_Address, value: u64, bit_width: Bit_Width) {
        let bytes = value.to_le_bytes();
        let data = &bytes[..bit_width.byte_count()];

        self.memory.write(data, address);
    }

    fn push(&mut self, register: Register) {
        self.push_immediate(self.register(register));
    }

    fn push_immediate(&mut self, value: u64) {
        self.decrement_register_n(Register::Stack_Pointer, 8);
        self.store(
            Memory_Address(self.register(Register::Stack_Pointer)),
            value,
            Bit_Width::Sixty_Four,
        );
    }

    fn pop(&mut self, register: Register) {
        self.load(
            register,
            Memory_Address(self.register(Register::Stack_Pointer)),
            Bit_Width::Sixty_Four,
        );
        self.increment_register_n(Register::Stack_Pointer, 8);
    }

    #[inline(always)]
    fn move_register(&mut self, destination: Register, source: Register) {
        self.set_register(destination, self.register(source));
    }

    fn execute(&mut self, instruction: Instruction) -> Instruction_Pointer {
        macro_rules! jump {
            ($r:expr) => {{
                // s_flag indicates that the call is relative to the current instruction pointer.
                // f_flag means the jump is forward if set, backwards otherwise.
                let relative_address = self.register($r.register_2());
                let absolute_address = if $r.s_flag() {
                    self.register(Register::Instruction_Pointer)
                        .wrapping_add_signed(i64::from_u64(relative_address))
                } else {
                    relative_address
                };

                Instruction_Pointer::Goto(absolute_address)
            }};
        }

        macro_rules! jump_and_link {
            ($r:expr) => {{
                self.set_register(
                    $r.register_0(),
                    self.register(Register::Instruction_Pointer) + 1,
                );

                jump!($r)
            }};
        }

        macro_rules! op1 {
            ($r:expr, ($op:tt), $source:ty) => {{
                let source_1 = <$source>::from_u64(self.register($r.register_1()));

                ($op source_1).store(self, $r.register_0());

                Instruction_Pointer::Next
            }};

            ($r:expr, $op:ident, $source:ty) => {{
                let source_1 = <$source>::from_u64(self.register($r.register_1()));

                source_1.$op().store(self, $r.register_0());

                Instruction_Pointer::Next
            }};
        }

        macro_rules! op2 {
            ($r:expr, ($op:tt), $source:ty) => {{
                let source_1 = <$source>::from_u64(self.register($r.register_1()));
                let source_2 = <$source>::from_u64(self.register($r.register_2()));

                (source_1 $op source_2).store(self, $r.register_0());

                Instruction_Pointer::Next
            }};

            ($r:expr, $op:ident, $source:ty) => {{
                let source_1 = <$source>::from_u64(self.register($r.register_1()));
                let source_2 = <$source>::from_u64(self.register($r.register_2()));

                source_1.$op(source_2).store(self, $r.register_0());

                Instruction_Pointer::Next
            }};

            ($r:expr, $op:ident, $source_1:ty, $source_2:ty) => {{
                let source_1 = <$source_1>::from_u64(self.register($r.register_1()));
                let source_2 = <$source_2>::from_u64(self.register($r.register_2()));

                source_1.$op(source_2).store(self, $r.register_0());

                Instruction_Pointer::Next
            }};
        }

        use Instruction::*;

        match instruction {
            // Control Flow
            // ------------
            Halt => Instruction_Pointer::Halt,

            Call(r) => {
                // Arguments are already pushed onto the stack.

                self.push(Register::Instruction_Pointer); // The "return" address (actually the address of the call instruction)
                self.push(Register::Frame_Pointer); // The current frame pointer

                // Set the frame pointer to the current stack pointer. We need to do this for two
                // reasons:
                // 1. We need a reference point to access function arguments that are passed on the
                //    stack.
                // 2. So that the function being called can use the stack without worrying about
                //    cleaning up after itself.
                self.move_register(Register::Frame_Pointer, Register::Stack_Pointer);

                jump!(r)
            },

            Return => {
                // Reset the stack pointer to the top of the frame.
                self.move_register(Register::Frame_Pointer, Register::Stack_Pointer);

                self.pop(Register::Frame_Pointer); // Restore the previous frame pointer
                self.pop(Register::Instruction_Pointer); // Restore the return address

                Instruction_Pointer::Next
            },

            Call_Environment(i) => {
                self.environment[i.immediate() as usize](self);
                Instruction_Pointer::Next
            },

            Break => {
                todo!()
            },

            Jump_And_Link(r) => jump_and_link!(r),

            Jump_Not_Zero(r) => {
                if self.register(r.register_1()) != 0 {
                    jump_and_link!(r)
                } else {
                    Instruction_Pointer::Next
                }
            },

            Jump_Zero(r) => {
                if self.register(r.register_1()) == 0 {
                    jump_and_link!(r)
                } else {
                    Instruction_Pointer::Next
                }
            },

            // Comparison
            // ----------
            Compare_Equal(r) => op2!(r, (==), u64),
            Compare_Not_Equal(r) => op2!(r, (!=), u64),
            Compare_Less(r) => op2!(r, (<), u64),
            Compare_Less_Equal(r) => op2!(r, (<=), u64),
            Compare_Greater(r) => op2!(r, (>), u64),
            Compare_Greater_Equal(r) => op2!(r, (>=), u64),

            Compare_Equal_F32(r) => op2!(r, (==), f32),
            Compare_Not_Equal_F32(r) => op2!(r, (!=), f32),
            Compare_Less_F32(r) => op2!(r, (<), f32),
            Compare_Less_Equal_F32(r) => op2!(r, (<=), f32),
            Compare_Greater_F32(r) => op2!(r, (>), f32),
            Compare_Greater_Equal_F32(r) => op2!(r, (>=), f32),

            Compare_Equal_F64(r) => op2!(r, (==), f64),
            Compare_Not_Equal_F64(r) => op2!(r, (!=), f64),
            Compare_Less_F64(r) => op2!(r, (<), f64),
            Compare_Less_Equal_F64(r) => op2!(r, (<=), f64),
            Compare_Greater_F64(r) => op2!(r, (>), f64),
            Compare_Greater_Equal_F64(r) => op2!(r, (>=), f64),

            // Bitwise
            // -------
            And_I8(r) => op2!(r, (&), u8),
            And_I16(r) => op2!(r, (&), u16),
            And_I32(r) => op2!(r, (&), u32),
            And_I64(r) => op2!(r, (&), u64),
            And_U8(r) => op2!(r, (&), i8),
            And_U16(r) => op2!(r, (&), i16),
            And_U32(r) => op2!(r, (&), i32),
            And_U64(r) => op2!(r, (&), i64),
            Or_I8(r) => op2!(r, (|), u8),
            Or_I16(r) => op2!(r, (|), u16),
            Or_I32(r) => op2!(r, (|), u32),
            Or_I64(r) => op2!(r, (|), u64),
            Or_U8(r) => op2!(r, (|), i8),
            Or_U16(r) => op2!(r, (|), i16),
            Or_U32(r) => op2!(r, (|), i32),
            Or_U64(r) => op2!(r, (|), i64),
            Xor_I8(r) => op2!(r, (^), u8),
            Xor_I16(r) => op2!(r, (^), u16),
            Xor_I32(r) => op2!(r, (^), u32),
            Xor_I64(r) => op2!(r, (^), u64),
            Xor_U8(r) => op2!(r, (^), i8),
            Xor_U16(r) => op2!(r, (^), i16),
            Xor_U32(r) => op2!(r, (^), i32),
            Xor_U64(r) => op2!(r, (^), i64),
            Not_I8(r) => op1!(r, (!), u8),
            Not_I16(r) => op1!(r, (!), u16),
            Not_I32(r) => op1!(r, (!), u32),
            Not_I64(r) => op1!(r, (!), u64),
            Not_U8(r) => op1!(r, (!), i8),
            Not_U16(r) => op1!(r, (!), i16),
            Not_U32(r) => op1!(r, (!), i32),
            Not_U64(r) => op1!(r, (!), i64),
            Shift_Left_I8(r) => op2!(r, (<<), u8),
            Shift_Left_I16(r) => op2!(r, (<<), u16),
            Shift_Left_I32(r) => op2!(r, (<<), u32),
            Shift_Left_I64(r) => op2!(r, (<<), u64),
            Shift_Left_U8(r) => op2!(r, (<<), i8),
            Shift_Left_U16(r) => op2!(r, (<<), i16),
            Shift_Left_U32(r) => op2!(r, (<<), i32),
            Shift_Left_U64(r) => op2!(r, (<<), i64),
            Shift_Right_I8(r) => op2!(r, (>>), u8),
            Shift_Right_I16(r) => op2!(r, (>>), u16),
            Shift_Right_I32(r) => op2!(r, (>>), u32),
            Shift_Right_I64(r) => op2!(r, (>>), u64),
            Shift_Right_U8(r) => op2!(r, (>>), i8),
            Shift_Right_U16(r) => op2!(r, (>>), i16),
            Shift_Right_U32(r) => op2!(r, (>>), i32),
            Shift_Right_U64(r) => op2!(r, (>>), i64),
            Rotate_Left_I8(r) => op2!(r, rotate_left, u8, u32),
            Rotate_Left_I16(r) => op2!(r, rotate_left, u16, u32),
            Rotate_Left_I32(r) => op2!(r, rotate_left, u32, u32),
            Rotate_Left_I64(r) => op2!(r, rotate_left, u64, u32),
            Rotate_Left_U8(r) => op2!(r, rotate_left, i8, u32),
            Rotate_Left_U16(r) => op2!(r, rotate_left, i16, u32),
            Rotate_Left_U32(r) => op2!(r, rotate_left, i32, u32),
            Rotate_Left_U64(r) => op2!(r, rotate_left, i64, u32),
            Rotate_Right_I8(r) => op2!(r, rotate_right, u8, u32),
            Rotate_Right_I16(r) => op2!(r, rotate_right, u16, u32),
            Rotate_Right_I32(r) => op2!(r, rotate_right, u32, u32),
            Rotate_Right_I64(r) => op2!(r, rotate_right, u64, u32),
            Rotate_Right_U8(r) => op2!(r, rotate_right, i8, u32),
            Rotate_Right_U16(r) => op2!(r, rotate_right, i16, u32),
            Rotate_Right_U32(r) => op2!(r, rotate_right, i32, u32),
            Rotate_Right_U64(r) => op2!(r, rotate_right, i64, u32),
            Count_Ones_I8(r) => op1!(r, count_ones, u8),
            Count_Ones_I16(r) => op1!(r, count_ones, u16),
            Count_Ones_I32(r) => op1!(r, count_ones, u32),
            Count_Ones_I64(r) => op1!(r, count_ones, u64),
            Count_Ones_U8(r) => op1!(r, count_ones, i8),
            Count_Ones_U16(r) => op1!(r, count_ones, i16),
            Count_Ones_U32(r) => op1!(r, count_ones, i32),
            Count_Ones_U64(r) => op1!(r, count_ones, i64),
            Leading_Ones_I8(r) => op1!(r, leading_ones, u8),
            Leading_Ones_I16(r) => op1!(r, leading_ones, u16),
            Leading_Ones_I32(r) => op1!(r, leading_ones, u32),
            Leading_Ones_I64(r) => op1!(r, leading_ones, u64),
            Leading_Ones_U8(r) => op1!(r, leading_ones, i8),
            Leading_Ones_U16(r) => op1!(r, leading_ones, i16),
            Leading_Ones_U32(r) => op1!(r, leading_ones, i32),
            Leading_Ones_U64(r) => op1!(r, leading_ones, i64),
            Trailing_Ones_I8(r) => op1!(r, trailing_ones, u8),
            Trailing_Ones_I16(r) => op1!(r, trailing_ones, u16),
            Trailing_Ones_I32(r) => op1!(r, trailing_ones, u32),
            Trailing_Ones_I64(r) => op1!(r, trailing_ones, u64),
            Trailing_Ones_U8(r) => op1!(r, trailing_ones, i8),
            Trailing_Ones_U16(r) => op1!(r, trailing_ones, i16),
            Trailing_Ones_U32(r) => op1!(r, trailing_ones, i32),
            Trailing_Ones_U64(r) => op1!(r, trailing_ones, i64),
            Count_Zeros_I8(r) => op1!(r, count_zeros, u8),
            Count_Zeros_I16(r) => op1!(r, count_zeros, u16),
            Count_Zeros_I32(r) => op1!(r, count_zeros, u32),
            Count_Zeros_I64(r) => op1!(r, count_zeros, u64),
            Count_Zeros_U8(r) => op1!(r, count_zeros, i8),
            Count_Zeros_U16(r) => op1!(r, count_zeros, i16),
            Count_Zeros_U32(r) => op1!(r, count_zeros, i32),
            Count_Zeros_U64(r) => op1!(r, count_zeros, i64),
            Leading_Zeros_I8(r) => op1!(r, leading_zeros, u8),
            Leading_Zeros_I16(r) => op1!(r, leading_zeros, u16),
            Leading_Zeros_I32(r) => op1!(r, leading_zeros, u32),
            Leading_Zeros_I64(r) => op1!(r, leading_zeros, u64),
            Leading_Zeros_U8(r) => op1!(r, leading_zeros, i8),
            Leading_Zeros_U16(r) => op1!(r, leading_zeros, i16),
            Leading_Zeros_U32(r) => op1!(r, leading_zeros, i32),
            Leading_Zeros_U64(r) => op1!(r, leading_zeros, i64),
            Trailing_Zeros_I8(r) => op1!(r, trailing_zeros, u8),
            Trailing_Zeros_I16(r) => op1!(r, trailing_zeros, u16),
            Trailing_Zeros_I32(r) => op1!(r, trailing_zeros, u32),
            Trailing_Zeros_I64(r) => op1!(r, trailing_zeros, u64),
            Trailing_Zeros_U8(r) => op1!(r, trailing_zeros, i8),
            Trailing_Zeros_U16(r) => op1!(r, trailing_zeros, i16),
            Trailing_Zeros_U32(r) => op1!(r, trailing_zeros, i32),
            Trailing_Zeros_U64(r) => op1!(r, trailing_zeros, i64),
            Reverse_Bytes_I8(r) => op1!(r, swap_bytes, u8),
            Reverse_Bytes_I16(r) => op1!(r, swap_bytes, u16),
            Reverse_Bytes_I32(r) => op1!(r, swap_bytes, u32),
            Reverse_Bytes_I64(r) => op1!(r, swap_bytes, u64),
            Reverse_Bytes_U8(r) => op1!(r, swap_bytes, i8),
            Reverse_Bytes_U16(r) => op1!(r, swap_bytes, i16),
            Reverse_Bytes_U32(r) => op1!(r, swap_bytes, i32),
            Reverse_Bytes_U64(r) => op1!(r, swap_bytes, i64),
            Reverse_Bits_I8(r) => op1!(r, reverse_bits, u8),
            Reverse_Bits_I16(r) => op1!(r, reverse_bits, u16),
            Reverse_Bits_I32(r) => op1!(r, reverse_bits, u32),
            Reverse_Bits_I64(r) => op1!(r, reverse_bits, u64),
            Reverse_Bits_U8(r) => op1!(r, reverse_bits, i8),
            Reverse_Bits_U16(r) => op1!(r, reverse_bits, i16),
            Reverse_Bits_U32(r) => op1!(r, reverse_bits, i32),
            Reverse_Bits_U64(r) => op1!(r, reverse_bits, i64),

            // Memory
            // ------
            Load(r) => {
                self.load(
                    r.register_0(),
                    Memory_Address(self.register(r.register_1())),
                    r.bit_width(),
                );

                Instruction_Pointer::Next
            },

            Load_Immediate(i) => {
                self.set_register(i.register(), i.immediate().into_u64());

                Instruction_Pointer::Next
            },

            Store(r) => {
                self.store(
                    Memory_Address(self.register(r.register_0())),
                    self.register(r.register_1()),
                    r.bit_width(),
                );

                Instruction_Pointer::Next
            },

            Store_Immediate(i) => {
                let address = self.register(i.register());
                let bytes = i.immediate().to_le_bytes();

                self.memory.write(&bytes, Memory_Address(address));

                Instruction_Pointer::Next
            },

            Move(r) => {
                self.set_register(r.register_0(), self.register(r.register_1()));
                Instruction_Pointer::Next
            },

            Push(i) => {
                self.push(i.register());
                Instruction_Pointer::Next
            },

            Push_Immediate(i) => {
                self.push_immediate(i.immediate().into_u64());
                Instruction_Pointer::Next
            },

            Pop(i) => {
                self.pop(i.register());
                Instruction_Pointer::Next
            },

            // Checked Arithmetic
            // ------------------
            Checked_Absolute_I8(r) => op1!(r, checked_abs, i8),
            Checked_Absolute_I16(r) => op1!(r, checked_abs, i16),
            Checked_Absolute_I32(r) => op1!(r, checked_abs, i32),
            Checked_Absolute_I64(r) => op1!(r, checked_abs, i64),
            Checked_Add_I8(r) => op2!(r, checked_add, i8),
            Checked_Add_I16(r) => op2!(r, checked_add, i16),
            Checked_Add_I32(r) => op2!(r, checked_add, i32),
            Checked_Add_I64(r) => op2!(r, checked_add, i64),
            Checked_Add_U8(r) => op2!(r, checked_add, u8),
            Checked_Add_U16(r) => op2!(r, checked_add, u16),
            Checked_Add_U32(r) => op2!(r, checked_add, u32),
            Checked_Add_U64(r) => op2!(r, checked_add, u64),
            Checked_Add_Unsigned_I8(r) => op2!(r, checked_add_unsigned, i8, u8),
            Checked_Add_Unsigned_I16(r) => op2!(r, checked_add_unsigned, i16, u16),
            Checked_Add_Unsigned_I32(r) => op2!(r, checked_add_unsigned, i32, u32),
            Checked_Add_Unsigned_I64(r) => op2!(r, checked_add_unsigned, i64, u64),
            Checked_Add_Signed_U8(r) => op2!(r, checked_add_signed, u8, i8),
            Checked_Add_Signed_U16(r) => op2!(r, checked_add_signed, u16, i16),
            Checked_Add_Signed_U32(r) => op2!(r, checked_add_signed, u32, i32),
            Checked_Add_Signed_U64(r) => op2!(r, checked_add_signed, u64, i64),
            Checked_Divide_I8(r) => op2!(r, checked_div, i8),
            Checked_Divide_I16(r) => op2!(r, checked_div, i16),
            Checked_Divide_I32(r) => op2!(r, checked_div, i32),
            Checked_Divide_I64(r) => op2!(r, checked_div, i64),
            Checked_Divide_U8(r) => op2!(r, checked_div, u8),
            Checked_Divide_U16(r) => op2!(r, checked_div, u16),
            Checked_Divide_U32(r) => op2!(r, checked_div, u32),
            Checked_Divide_U64(r) => op2!(r, checked_div, u64),
            Checked_Divide_Euclidean_I8(r) => op2!(r, checked_div_euclid, i8),
            Checked_Divide_Euclidean_I16(r) => op2!(r, checked_div_euclid, i16),
            Checked_Divide_Euclidean_I32(r) => op2!(r, checked_div_euclid, i32),
            Checked_Divide_Euclidean_I64(r) => op2!(r, checked_div_euclid, i64),
            Checked_Divide_Euclidean_U8(r) => op2!(r, checked_div_euclid, u8),
            Checked_Divide_Euclidean_U16(r) => op2!(r, checked_div_euclid, u16),
            Checked_Divide_Euclidean_U32(r) => op2!(r, checked_div_euclid, u32),
            Checked_Divide_Euclidean_U64(r) => op2!(r, checked_div_euclid, u64),
            Checked_Logarithm_I8(r) => op2!(r, checked_ilog, i8),
            Checked_Logarithm_I16(r) => op2!(r, checked_ilog, i16),
            Checked_Logarithm_I32(r) => op2!(r, checked_ilog, i32),
            Checked_Logarithm_I64(r) => op2!(r, checked_ilog, i64),
            Checked_Logarithm_U8(r) => op2!(r, checked_ilog, u8),
            Checked_Logarithm_U16(r) => op2!(r, checked_ilog, u16),
            Checked_Logarithm_U32(r) => op2!(r, checked_ilog, u32),
            Checked_Logarithm_U64(r) => op2!(r, checked_ilog, u64),
            Checked_Square_Root_I8(_r) => todo!(), // op1!(r, checked_isqrt, i8),
            Checked_Square_Root_I16(_r) => todo!(), // op1!(r, checked_isqrt, i16),
            Checked_Square_Root_I32(_r) => todo!(), // op1!(r, checked_isqrt, i32),
            Checked_Square_Root_I64(_r) => todo!(), // op1!(r, checked_isqrt, i64),
            Checked_Square_Root_U8(_r) => todo!(), // op1!(r, checked_isqrt, u8),
            Checked_Square_Root_U16(_r) => todo!(), // op1!(r, checked_isqrt, u16),
            Checked_Square_Root_U32(_r) => todo!(), // op1!(r, checked_isqrt, u32),
            Checked_Square_Root_U64(_r) => todo!(), // op1!(r, checked_isqrt, u64),
            Checked_Multiply_I8(r) => op2!(r, checked_mul, i8),
            Checked_Multiply_I16(r) => op2!(r, checked_mul, i16),
            Checked_Multiply_I32(r) => op2!(r, checked_mul, i32),
            Checked_Multiply_I64(r) => op2!(r, checked_mul, i64),
            Checked_Multiply_U8(r) => op2!(r, checked_mul, u8),
            Checked_Multiply_U16(r) => op2!(r, checked_mul, u16),
            Checked_Multiply_U32(r) => op2!(r, checked_mul, u32),
            Checked_Multiply_U64(r) => op2!(r, checked_mul, u64),
            Checked_Negate_I8(r) => op1!(r, checked_neg, i8),
            Checked_Negate_I16(r) => op1!(r, checked_neg, i16),
            Checked_Negate_I32(r) => op1!(r, checked_neg, i32),
            Checked_Negate_I64(r) => op1!(r, checked_neg, i64),
            Checked_Power_I8(r) => op2!(r, checked_pow, i8, u32),
            Checked_Power_I16(r) => op2!(r, checked_pow, i16, u32),
            Checked_Power_I32(r) => op2!(r, checked_pow, i32, u32),
            Checked_Power_I64(r) => op2!(r, checked_pow, i64, u32),
            Checked_Power_U8(r) => op2!(r, checked_pow, u8, u32),
            Checked_Power_U16(r) => op2!(r, checked_pow, u16, u32),
            Checked_Power_U32(r) => op2!(r, checked_pow, u32, u32),
            Checked_Power_U64(r) => op2!(r, checked_pow, u64, u32),
            Checked_Remainder_I8(r) => op2!(r, checked_rem, i8),
            Checked_Remainder_I16(r) => op2!(r, checked_rem, i16),
            Checked_Remainder_I32(r) => op2!(r, checked_rem, i32),
            Checked_Remainder_I64(r) => op2!(r, checked_rem, i64),
            Checked_Remainder_U8(r) => op2!(r, checked_rem, u8),
            Checked_Remainder_U16(r) => op2!(r, checked_rem, u16),
            Checked_Remainder_U32(r) => op2!(r, checked_rem, u32),
            Checked_Remainder_U64(r) => op2!(r, checked_rem, u64),
            Checked_Remainder_Euclidean_I8(r) => op2!(r, checked_rem_euclid, i8),
            Checked_Remainder_Euclidean_I16(r) => op2!(r, checked_rem_euclid, i16),
            Checked_Remainder_Euclidean_I32(r) => op2!(r, checked_rem_euclid, i32),
            Checked_Remainder_Euclidean_I64(r) => op2!(r, checked_rem_euclid, i64),
            Checked_Remainder_Euclidean_U8(r) => op2!(r, checked_rem_euclid, u8),
            Checked_Remainder_Euclidean_U16(r) => op2!(r, checked_rem_euclid, u16),
            Checked_Remainder_Euclidean_U32(r) => op2!(r, checked_rem_euclid, u32),
            Checked_Remainder_Euclidean_U64(r) => op2!(r, checked_rem_euclid, u64),
            Checked_Shift_Left_I8(r) => op2!(r, checked_shl, i8, u32),
            Checked_Shift_Left_I16(r) => op2!(r, checked_shl, i16, u32),
            Checked_Shift_Left_I32(r) => op2!(r, checked_shl, i32, u32),
            Checked_Shift_Left_I64(r) => op2!(r, checked_shl, i64, u32),
            Checked_Shift_Left_U8(r) => op2!(r, checked_shl, u8, u32),
            Checked_Shift_Left_U16(r) => op2!(r, checked_shl, u16, u32),
            Checked_Shift_Left_U32(r) => op2!(r, checked_shl, u32, u32),
            Checked_Shift_Left_U64(r) => op2!(r, checked_shl, u64, u32),
            Checked_Shift_Right_I8(r) => op2!(r, checked_shr, i8, u32),
            Checked_Shift_Right_I16(r) => op2!(r, checked_shr, i16, u32),
            Checked_Shift_Right_I32(r) => op2!(r, checked_shr, i32, u32),
            Checked_Shift_Right_I64(r) => op2!(r, checked_shr, i64, u32),
            Checked_Shift_Right_U8(r) => op2!(r, checked_shr, u8, u32),
            Checked_Shift_Right_U16(r) => op2!(r, checked_shr, u16, u32),
            Checked_Shift_Right_U32(r) => op2!(r, checked_shr, u32, u32),
            Checked_Shift_Right_U64(r) => op2!(r, checked_shr, u64, u32),
            Checked_Subtract_I8(r) => op2!(r, checked_sub, i8),
            Checked_Subtract_I16(r) => op2!(r, checked_sub, i16),
            Checked_Subtract_I32(r) => op2!(r, checked_sub, i32),
            Checked_Subtract_I64(r) => op2!(r, checked_sub, i64),
            Checked_Subtract_U8(r) => op2!(r, checked_sub, u8),
            Checked_Subtract_U16(r) => op2!(r, checked_sub, u16),
            Checked_Subtract_U32(r) => op2!(r, checked_sub, u32),
            Checked_Subtract_U64(r) => op2!(r, checked_sub, u64),
            Checked_Subtract_Unsigned_I8(r) => op2!(r, checked_sub_unsigned, i8, u8),
            Checked_Subtract_Unsigned_I16(r) => op2!(r, checked_sub_unsigned, i16, u16),
            Checked_Subtract_Unsigned_I32(r) => op2!(r, checked_sub_unsigned, i32, u32),
            Checked_Subtract_Unsigned_I64(r) => op2!(r, checked_sub_unsigned, i64, u64),

            // Overflowing Arithmetic
            // ----------------------
            Overflowing_Absolute_I8(r) => op1!(r, overflowing_abs, i8),
            Overflowing_Absolute_I16(r) => op1!(r, overflowing_abs, i16),
            Overflowing_Absolute_I32(r) => op1!(r, overflowing_abs, i32),
            Overflowing_Absolute_I64(r) => op1!(r, overflowing_abs, i64),
            Overflowing_Add_I8(r) => op2!(r, overflowing_add, i8),
            Overflowing_Add_I16(r) => op2!(r, overflowing_add, i16),
            Overflowing_Add_I32(r) => op2!(r, overflowing_add, i32),
            Overflowing_Add_I64(r) => op2!(r, overflowing_add, i64),
            Overflowing_Add_U8(r) => op2!(r, overflowing_add, u8),
            Overflowing_Add_U16(r) => op2!(r, overflowing_add, u16),
            Overflowing_Add_U32(r) => op2!(r, overflowing_add, u32),
            Overflowing_Add_U64(r) => op2!(r, overflowing_add, u64),
            Overflowing_Add_Unsigned_I8(r) => op2!(r, overflowing_add_unsigned, i8, u8),
            Overflowing_Add_Unsigned_I16(r) => op2!(r, overflowing_add_unsigned, i16, u16),
            Overflowing_Add_Unsigned_I32(r) => op2!(r, overflowing_add_unsigned, i32, u32),
            Overflowing_Add_Unsigned_I64(r) => op2!(r, overflowing_add_unsigned, i64, u64),
            Overflowing_Add_Signed_U8(r) => op2!(r, overflowing_add_signed, u8, i8),
            Overflowing_Add_Signed_U16(r) => op2!(r, overflowing_add_signed, u16, i16),
            Overflowing_Add_Signed_U32(r) => op2!(r, overflowing_add_signed, u32, i32),
            Overflowing_Add_Signed_U64(r) => op2!(r, overflowing_add_signed, u64, i64),
            Overflowing_Divide_I8(r) => op2!(r, overflowing_div, i8),
            Overflowing_Divide_I16(r) => op2!(r, overflowing_div, i16),
            Overflowing_Divide_I32(r) => op2!(r, overflowing_div, i32),
            Overflowing_Divide_I64(r) => op2!(r, overflowing_div, i64),
            Overflowing_Divide_U8(r) => op2!(r, overflowing_div, u8),
            Overflowing_Divide_U16(r) => op2!(r, overflowing_div, u16),
            Overflowing_Divide_U32(r) => op2!(r, overflowing_div, u32),
            Overflowing_Divide_U64(r) => op2!(r, overflowing_div, u64),
            Overflowing_Divide_Euclidean_I8(r) => op2!(r, overflowing_div_euclid, i8),
            Overflowing_Divide_Euclidean_I16(r) => op2!(r, overflowing_div_euclid, i16),
            Overflowing_Divide_Euclidean_I32(r) => op2!(r, overflowing_div_euclid, i32),
            Overflowing_Divide_Euclidean_I64(r) => op2!(r, overflowing_div_euclid, i64),
            Overflowing_Divide_Euclidean_U8(r) => op2!(r, overflowing_div_euclid, u8),
            Overflowing_Divide_Euclidean_U16(r) => op2!(r, overflowing_div_euclid, u16),
            Overflowing_Divide_Euclidean_U32(r) => op2!(r, overflowing_div_euclid, u32),
            Overflowing_Divide_Euclidean_U64(r) => op2!(r, overflowing_div_euclid, u64),
            Overflowing_Multiply_I8(r) => op2!(r, overflowing_mul, i8),
            Overflowing_Multiply_I16(r) => op2!(r, overflowing_mul, i16),
            Overflowing_Multiply_I32(r) => op2!(r, overflowing_mul, i32),
            Overflowing_Multiply_I64(r) => op2!(r, overflowing_mul, i64),
            Overflowing_Multiply_U8(r) => op2!(r, overflowing_mul, u8),
            Overflowing_Multiply_U16(r) => op2!(r, overflowing_mul, u16),
            Overflowing_Multiply_U32(r) => op2!(r, overflowing_mul, u32),
            Overflowing_Multiply_U64(r) => op2!(r, overflowing_mul, u64),
            Overflowing_Negate_I8(r) => op1!(r, overflowing_neg, i8),
            Overflowing_Negate_I16(r) => op1!(r, overflowing_neg, i16),
            Overflowing_Negate_I32(r) => op1!(r, overflowing_neg, i32),
            Overflowing_Negate_I64(r) => op1!(r, overflowing_neg, i64),
            Overflowing_Power_I8(r) => op2!(r, overflowing_pow, i8, u32),
            Overflowing_Power_I16(r) => op2!(r, overflowing_pow, i16, u32),
            Overflowing_Power_I32(r) => op2!(r, overflowing_pow, i32, u32),
            Overflowing_Power_I64(r) => op2!(r, overflowing_pow, i64, u32),
            Overflowing_Power_U8(r) => op2!(r, overflowing_pow, u8, u32),
            Overflowing_Power_U16(r) => op2!(r, overflowing_pow, u16, u32),
            Overflowing_Power_U32(r) => op2!(r, overflowing_pow, u32, u32),
            Overflowing_Power_U64(r) => op2!(r, overflowing_pow, u64, u32),
            Overflowing_Remainder_I8(r) => op2!(r, overflowing_rem, i8),
            Overflowing_Remainder_I16(r) => op2!(r, overflowing_rem, i16),
            Overflowing_Remainder_I32(r) => op2!(r, overflowing_rem, i32),
            Overflowing_Remainder_I64(r) => op2!(r, overflowing_rem, i64),
            Overflowing_Remainder_U8(r) => op2!(r, overflowing_rem, u8),
            Overflowing_Remainder_U16(r) => op2!(r, overflowing_rem, u16),
            Overflowing_Remainder_U32(r) => op2!(r, overflowing_rem, u32),
            Overflowing_Remainder_U64(r) => op2!(r, overflowing_rem, u64),
            Overflowing_Remainder_Euclidean_I8(r) => op2!(r, overflowing_rem_euclid, i8),
            Overflowing_Remainder_Euclidean_I16(r) => op2!(r, overflowing_rem_euclid, i16),
            Overflowing_Remainder_Euclidean_I32(r) => op2!(r, overflowing_rem_euclid, i32),
            Overflowing_Remainder_Euclidean_I64(r) => op2!(r, overflowing_rem_euclid, i64),
            Overflowing_Remainder_Euclidean_U8(r) => op2!(r, overflowing_rem_euclid, u8),
            Overflowing_Remainder_Euclidean_U16(r) => op2!(r, overflowing_rem_euclid, u16),
            Overflowing_Remainder_Euclidean_U32(r) => op2!(r, overflowing_rem_euclid, u32),
            Overflowing_Remainder_Euclidean_U64(r) => op2!(r, overflowing_rem_euclid, u64),
            Overflowing_Shift_Left_I8(r) => op2!(r, overflowing_shl, i8, u32),
            Overflowing_Shift_Left_I16(r) => op2!(r, overflowing_shl, i16, u32),
            Overflowing_Shift_Left_I32(r) => op2!(r, overflowing_shl, i32, u32),
            Overflowing_Shift_Left_I64(r) => op2!(r, overflowing_shl, i64, u32),
            Overflowing_Shift_Left_U8(r) => op2!(r, overflowing_shl, u8, u32),
            Overflowing_Shift_Left_U16(r) => op2!(r, overflowing_shl, u16, u32),
            Overflowing_Shift_Left_U32(r) => op2!(r, overflowing_shl, u32, u32),
            Overflowing_Shift_Left_U64(r) => op2!(r, overflowing_shl, u64, u32),
            Overflowing_Shift_Right_I8(r) => op2!(r, overflowing_shr, i8, u32),
            Overflowing_Shift_Right_I16(r) => op2!(r, overflowing_shr, i16, u32),
            Overflowing_Shift_Right_I32(r) => op2!(r, overflowing_shr, i32, u32),
            Overflowing_Shift_Right_I64(r) => op2!(r, overflowing_shr, i64, u32),
            Overflowing_Shift_Right_U8(r) => op2!(r, overflowing_shr, u8, u32),
            Overflowing_Shift_Right_U16(r) => op2!(r, overflowing_shr, u16, u32),
            Overflowing_Shift_Right_U32(r) => op2!(r, overflowing_shr, u32, u32),
            Overflowing_Shift_Right_U64(r) => op2!(r, overflowing_shr, u64, u32),
            Overflowing_Subtract_I8(r) => op2!(r, overflowing_sub, i8),
            Overflowing_Subtract_I16(r) => op2!(r, overflowing_sub, i16),
            Overflowing_Subtract_I32(r) => op2!(r, overflowing_sub, i32),
            Overflowing_Subtract_I64(r) => op2!(r, overflowing_sub, i64),
            Overflowing_Subtract_U8(r) => op2!(r, overflowing_sub, u8),
            Overflowing_Subtract_U16(r) => op2!(r, overflowing_sub, u16),
            Overflowing_Subtract_U32(r) => op2!(r, overflowing_sub, u32),
            Overflowing_Subtract_U64(r) => op2!(r, overflowing_sub, u64),
            Overflowing_Subtract_Unsigned_I8(r) => op2!(r, overflowing_sub_unsigned, i8, u8),
            Overflowing_Subtract_Unsigned_I16(r) => op2!(r, overflowing_sub_unsigned, i16, u16),
            Overflowing_Subtract_Unsigned_I32(r) => op2!(r, overflowing_sub_unsigned, i32, u32),
            Overflowing_Subtract_Unsigned_I64(r) => op2!(r, overflowing_sub_unsigned, i64, u64),

            // Saturating Arithmetic
            // ---------------------
            Saturating_Absolute_I8(r) => op1!(r, saturating_abs, i8),
            Saturating_Absolute_I16(r) => op1!(r, saturating_abs, i16),
            Saturating_Absolute_I32(r) => op1!(r, saturating_abs, i32),
            Saturating_Absolute_I64(r) => op1!(r, saturating_abs, i64),
            Saturating_Add_I8(r) => op2!(r, saturating_add, i8),
            Saturating_Add_I16(r) => op2!(r, saturating_add, i16),
            Saturating_Add_I32(r) => op2!(r, saturating_add, i32),
            Saturating_Add_I64(r) => op2!(r, saturating_add, i64),
            Saturating_Add_U8(r) => op2!(r, saturating_add, u8),
            Saturating_Add_U16(r) => op2!(r, saturating_add, u16),
            Saturating_Add_U32(r) => op2!(r, saturating_add, u32),
            Saturating_Add_U64(r) => op2!(r, saturating_add, u64),
            Saturating_Add_Unsigned_I8(r) => op2!(r, saturating_add_unsigned, i8, u8),
            Saturating_Add_Unsigned_I16(r) => op2!(r, saturating_add_unsigned, i16, u16),
            Saturating_Add_Unsigned_I32(r) => op2!(r, saturating_add_unsigned, i32, u32),
            Saturating_Add_Unsigned_I64(r) => op2!(r, saturating_add_unsigned, i64, u64),
            Saturating_Add_Signed_U8(r) => op2!(r, saturating_add_signed, u8, i8),
            Saturating_Add_Signed_U16(r) => op2!(r, saturating_add_signed, u16, i16),
            Saturating_Add_Signed_U32(r) => op2!(r, saturating_add_signed, u32, i32),
            Saturating_Add_Signed_U64(r) => op2!(r, saturating_add_signed, u64, i64),
            Saturating_Divide_I8(r) => op2!(r, saturating_div, i8),
            Saturating_Divide_I16(r) => op2!(r, saturating_div, i16),
            Saturating_Divide_I32(r) => op2!(r, saturating_div, i32),
            Saturating_Divide_I64(r) => op2!(r, saturating_div, i64),
            Saturating_Divide_U8(r) => op2!(r, saturating_div, u8),
            Saturating_Divide_U16(r) => op2!(r, saturating_div, u16),
            Saturating_Divide_U32(r) => op2!(r, saturating_div, u32),
            Saturating_Divide_U64(r) => op2!(r, saturating_div, u64),
            Saturating_Multiply_I8(r) => op2!(r, saturating_mul, i8),
            Saturating_Multiply_I16(r) => op2!(r, saturating_mul, i16),
            Saturating_Multiply_I32(r) => op2!(r, saturating_mul, i32),
            Saturating_Multiply_I64(r) => op2!(r, saturating_mul, i64),
            Saturating_Multiply_U8(r) => op2!(r, saturating_mul, u8),
            Saturating_Multiply_U16(r) => op2!(r, saturating_mul, u16),
            Saturating_Multiply_U32(r) => op2!(r, saturating_mul, u32),
            Saturating_Multiply_U64(r) => op2!(r, saturating_mul, u64),
            Saturating_Negate_I8(r) => op1!(r, saturating_neg, i8),
            Saturating_Negate_I16(r) => op1!(r, saturating_neg, i16),
            Saturating_Negate_I32(r) => op1!(r, saturating_neg, i32),
            Saturating_Negate_I64(r) => op1!(r, saturating_neg, i64),
            Saturating_Power_I8(r) => op2!(r, saturating_pow, i8, u32),
            Saturating_Power_I16(r) => op2!(r, saturating_pow, i16, u32),
            Saturating_Power_I32(r) => op2!(r, saturating_pow, i32, u32),
            Saturating_Power_I64(r) => op2!(r, saturating_pow, i64, u32),
            Saturating_Power_U8(r) => op2!(r, saturating_pow, u8, u32),
            Saturating_Power_U16(r) => op2!(r, saturating_pow, u16, u32),
            Saturating_Power_U32(r) => op2!(r, saturating_pow, u32, u32),
            Saturating_Power_U64(r) => op2!(r, saturating_pow, u64, u32),
            Saturating_Subtract_I8(r) => op2!(r, saturating_sub, i8),
            Saturating_Subtract_I16(r) => op2!(r, saturating_sub, i16),
            Saturating_Subtract_I32(r) => op2!(r, saturating_sub, i32),
            Saturating_Subtract_I64(r) => op2!(r, saturating_sub, i64),
            Saturating_Subtract_U8(r) => op2!(r, saturating_sub, u8),
            Saturating_Subtract_U16(r) => op2!(r, saturating_sub, u16),
            Saturating_Subtract_U32(r) => op2!(r, saturating_sub, u32),
            Saturating_Subtract_U64(r) => op2!(r, saturating_sub, u64),
            Saturating_Subtract_Unsigned_I8(r) => op2!(r, saturating_sub_unsigned, i8, u8),
            Saturating_Subtract_Unsigned_I16(r) => op2!(r, saturating_sub_unsigned, i16, u16),
            Saturating_Subtract_Unsigned_I32(r) => op2!(r, saturating_sub_unsigned, i32, u32),
            Saturating_Subtract_Unsigned_I64(r) => op2!(r, saturating_sub_unsigned, i64, u64),

            // Floating Point Arithmetic
            // -------------------------
            Absolute_F32(r) => op1!(r, abs, f32),
            Absolute_F64(r) => op1!(r, abs, f64),
            Add_F32(r) => op2!(r, (+), f32),
            Add_F64(r) => op2!(r, (+), f64),
            Divide_F32(r) => op2!(r, (/), f32),
            Divide_F64(r) => op2!(r, (/), f64),
            Divide_Euclidean_F32(r) => op2!(r, div_euclid, f32),
            Divide_Euclidean_F64(r) => op2!(r, div_euclid, f64),
            Logarithm_F32(r) => op2!(r, log, f32),
            Logarithm_F64(r) => op2!(r, log, f64),
            Square_Root_F32(r) => op1!(r, sqrt, f32),
            Square_Root_F64(r) => op1!(r, sqrt, f64),
            Multiply_F32(r) => op2!(r, (*), f32),
            Multiply_F64(r) => op2!(r, (*), f64),
            Negate_F32(r) => op1!(r, (-), f32),
            Negate_F64(r) => op1!(r, (-), f64),
            Power_F32(r) => op2!(r, powf, f32),
            Power_F64(r) => op2!(r, powf, f64),
            Remainder_F32(r) => op2!(r, (%), f32),
            Remainder_F64(r) => op2!(r, (%), f64),
            Remainder_Euclidean_F32(r) => op2!(r, rem_euclid, f32),
            Remainder_Euclidean_F64(r) => op2!(r, rem_euclid, f64),
            Cube_Root_F32(r) => op1!(r, cbrt, f32),
            Cube_Root_F64(r) => op1!(r, cbrt, f64),
            Subtract_F32(r) => op2!(r, (-), f32),
            Subtract_F64(r) => op2!(r, (-), f64),

            // Invalid
            // -------
            Invalid(i) => {
                use byte_code::{I_Type, R_Type};

                println!("Invalid Instruction: {:032b}", i);
                println!("As R-Type: {:?}", R_Type(i));
                println!("As I-Type: {:?}", I_Type(i));
                Instruction_Pointer::Halt
            },
        }
    }
}

trait Store {
    fn store(&self, vm: &mut Virtual_Machine, register: Register);
}

impl<T> Store for T
where
    T: Destination,
{
    fn store(&self, vm: &mut Virtual_Machine, register: Register) {
        vm.set_register(register, self.into_u64());
    }
}

impl<T> Store for Option<T>
where
    T: Destination,
{
    fn store(&self, vm: &mut Virtual_Machine, register: Register) {
        if let Some(value) = self {
            vm.set_register(register, value.into_u64());
        } else {
            // Set overflow flag
        }
    }
}

impl<T> Store for (T, bool)
where
    T: Destination,
{
    fn store(&self, vm: &mut Virtual_Machine, register: Register) {
        vm.set_register(register, self.0.into_u64());

        if self.1 {
            // Set overflow flag
        }
    }
}
