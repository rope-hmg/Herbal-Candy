use byte_code::{Instruction, Register};

use crate::{
    convert::{Destination, Source},
    memory::{Memory, Memory_Address},
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

    /// Prepares the stack and the frame for a function call.
    fn call(&mut self) {
        // Arguments are already pushed onto the stack.

        self.push(Register::Instruction_Pointer); // The "return" address (actually the address of the call instruction)
        self.push(Register::Frame_Pointer); // The current frame pointer

        // Set the frame pointer to the current stack pointer. We need to do this for two
        // reasons:
        // 1. We need a reference point to access function arguments that are passed on the stack.
        // 2. So that the function being called can use the stack without worrying about cleaning up
        //    after itself.
        self.move_register(Register::Frame_Pointer, Register::Stack_Pointer);
    }

    /// Stores the address of the next instruction in the specified register.
    fn link(&mut self, rd: Register) {
        self.set_register(rd, self.register(Register::Instruction_Pointer) + 1);
    }

    fn load(&mut self, register: Register, address: Memory_Address, bit_width: u8) {
        let mut bytes = [0; 8];
        let data = &mut bytes[..(bit_width / 8) as usize];

        self.memory.read(address, data);

        self.set_register(register, u64::from_le_bytes(bytes));
    }

    fn store(&mut self, address: Memory_Address, value: u64, bit_width: u8) {
        let bytes = value.to_le_bytes();
        let data = &bytes[..(bit_width / 8) as usize];

        self.memory.write(data, address);
    }

    #[inline(always)]
    fn push(&mut self, register: Register) {
        self.push_immediate(self.register(register));
    }

    #[inline]
    fn push_immediate(&mut self, value: u64) {
        self.decrement_register_n(Register::Stack_Pointer, 8);
        self.store(
            Memory_Address(self.register(Register::Stack_Pointer)),
            value,
            64,
        );
    }

    fn pop(&mut self, register: Register) {
        self.load(
            register,
            Memory_Address(self.register(Register::Stack_Pointer)),
            64,
        );
        self.increment_register_n(Register::Stack_Pointer, 8);
    }

    #[inline(always)]
    fn move_register(&mut self, destination: Register, source: Register) {
        self.set_register(destination, self.register(source));
    }

    fn execute(&mut self, instruction: Instruction) -> Instruction_Pointer {
        macro_rules! jump_relative {
            ($rs2:expr) => {{
                let relative_address = self.register($rs2);
                let absolute_address = self
                    .register(Register::Instruction_Pointer)
                    .wrapping_add_signed(i64::from_u64(relative_address));

                Instruction_Pointer::Goto(absolute_address)
            }};
        }

        macro_rules! op1 {
            ($rd:expr, $rs1:expr, ($op:tt), $source:ty) => {{
                let source_1 = <$source>::from_u64(self.register($rs1));

                ($op source_1).store(self, $rd);

                Instruction_Pointer::Next
            }};

            ($rd:expr, $rs1:expr, $op:ident, $source:ty) => {{
                let source_1 = <$source>::from_u64(self.register($rs1));

                source_1.$op().store(self, $rd);

                Instruction_Pointer::Next
            }};
        }

        macro_rules! op2 {
            ($rd:expr, $rs1:expr, $rs2:expr, ($op:tt), $source:ty) => {{
                let source_1 = <$source>::from_u64(self.register($rs1));
                let source_2 = <$source>::from_u64(self.register($rs2));

                (source_1 $op source_2).store(self, $rd);

                Instruction_Pointer::Next
            }};

            ($rd:expr, $rs1:expr, $rs2:expr, $op:ident, $source:ty) => {{
                let source_1 = <$source>::from_u64(self.register($rs1));
                let source_2 = <$source>::from_u64(self.register($rs2));

                source_1.$op(source_2).store(self, $rd);

                Instruction_Pointer::Next
            }};

            ($rd:expr, $rs1:expr, $rs2:expr, $op:ident, $source_1:ty, $source_2:ty) => {{
                let source_1 = <$source_1>::from_u64(self.register($rs1));
                let source_2 = <$source_2>::from_u64(self.register($rs2));

                source_1.$op(source_2).store(self, $rd);

                Instruction_Pointer::Next
            }};
        }

        use Instruction::*;

        match instruction {
            // Control Flow
            // ------------
            Halt => Instruction_Pointer::Halt,

            Trap => todo!(),

            Call { rs2 } => {
                self.call();
                Instruction_Pointer::Goto(self.register(rs2))
            },

            Call_R { rs2 } => {
                self.call();
                jump_relative!(rs2)
            },

            Ret => {
                // Reset the stack pointer to the top of the frame.
                self.move_register(Register::Frame_Pointer, Register::Stack_Pointer);

                self.pop(Register::Frame_Pointer); // Restore the previous frame pointer
                self.pop(Register::Instruction_Pointer); // Restore the return address

                Instruction_Pointer::Next
            },

            Ecall { imm } => {
                self.environment[imm as usize](self);
                Instruction_Pointer::Next
            },

            Break => {
                todo!()
            },

            Jal { rd, rs2 } => {
                self.link(rd);
                Instruction_Pointer::Goto(self.register(rs2))
            },

            Jal_R { rd, rs2 } => {
                self.link(rd);
                jump_relative!(rs2)
            },

            Jnz { rd, rs1, rs2 } => {
                if self.register(rs1) != 0 {
                    self.link(rd);
                    Instruction_Pointer::Goto(self.register(rs2))
                } else {
                    Instruction_Pointer::Next
                }
            },

            Jnz_R { rd, rs1, rs2 } => {
                if self.register(rs1) != 0 {
                    self.link(rd);
                    jump_relative!(rs2)
                } else {
                    Instruction_Pointer::Next
                }
            },

            Jiz { rd, rs1, rs2 } => {
                if self.register(rs1) == 0 {
                    self.link(rd);
                    Instruction_Pointer::Goto(self.register(rs2))
                } else {
                    Instruction_Pointer::Next
                }
            },

            Jiz_R { rd, rs1, rs2 } => {
                if self.register(rs1) == 0 {
                    self.link(rd);
                    jump_relative!(rs2)
                } else {
                    Instruction_Pointer::Next
                }
            },

            // Memory
            // ------
            Load { rd, rs1, size } => {
                self.load(rd, Memory_Address(self.register(rs1)), size);

                Instruction_Pointer::Next
            },

            Loadi { rd, imm } => {
                self.set_register(rd, imm.into_u64());

                Instruction_Pointer::Next
            },

            Store { rd, rs1, size } => {
                self.store(Memory_Address(self.register(rd)), self.register(rs1), size);

                Instruction_Pointer::Next
            },

            Storei { rd, imm } => {
                let address = self.register(rd);
                let bytes = imm.to_le_bytes();

                self.memory.write(&bytes, Memory_Address(address));

                Instruction_Pointer::Next
            },

            Move { rd, rs1 } => {
                self.set_register(rd, self.register(rs1));
                Instruction_Pointer::Next
            },

            Push { rd } => {
                self.push(rd);
                Instruction_Pointer::Next
            },

            Pushi { imm } => {
                self.push_immediate(imm.into_u64());
                Instruction_Pointer::Next
            },

            Pop { rd } => {
                self.pop(rd);
                Instruction_Pointer::Next
            },

            // Comparison
            // ----------
            Ie { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (==), u64),
            Ne { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (!=), u64),
            Lt { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (<), u64),
            Le { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (<=), u64),
            Gt { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (>), u64),
            Ge { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (>=), u64),

            Ie_f32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (==), f32),
            Ne_f32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (!=), f32),
            Lt_f32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (<), f32),
            Le_f32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (<=), f32),
            Gt_f32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (>), f32),
            Ge_f32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (>=), f32),

            Ie_f64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (==), f64),
            Ne_f64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (!=), f64),
            Lt_f64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (<), f64),
            Le_f64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (<=), f64),
            Gt_f64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (>), f64),
            Ge_f64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2 , (>=), f64),

            // Bitwise
            // -------
            And_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (&), u8),
            And_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (&), u16),
            And_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (&), u32),
            And_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (&), u64),
            And_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (&), i8),
            And_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (&), i16),
            And_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (&), i32),
            And_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (&), i64),
            Or_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (|), u8),
            Or_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (|), u16),
            Or_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (|), u32),
            Or_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (|), u64),
            Or_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (|), i8),
            Or_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (|), i16),
            Or_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (|), i32),
            Or_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (|), i64),
            Xor_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (^), u8),
            Xor_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (^), u16),
            Xor_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (^), u32),
            Xor_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (^), u64),
            Xor_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (^), i8),
            Xor_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (^), i16),
            Xor_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (^), i32),
            Xor_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (^), i64),
            Not_u8 { rd, rs1 } => op1!(rd, rs1, (!), u8),
            Not_u16 { rd, rs1 } => op1!(rd, rs1, (!), u16),
            Not_u32 { rd, rs1 } => op1!(rd, rs1, (!), u32),
            Not_u64 { rd, rs1 } => op1!(rd, rs1, (!), u64),
            Not_i8 { rd, rs1 } => op1!(rd, rs1, (!), i8),
            Not_i16 { rd, rs1 } => op1!(rd, rs1, (!), i16),
            Not_i32 { rd, rs1 } => op1!(rd, rs1, (!), i32),
            Not_i64 { rd, rs1 } => op1!(rd, rs1, (!), i64),
            Shl_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (<<), u8),
            Shl_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (<<), u16),
            Shl_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (<<), u32),
            Shl_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (<<), u64),
            Shl_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (<<), i8),
            Shl_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (<<), i16),
            Shl_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (<<), i32),
            Shl_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (<<), i64),
            Shr_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (>>), u8),
            Shr_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (>>), u16),
            Shr_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (>>), u32),
            Shr_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (>>), u64),
            Shr_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (>>), i8),
            Shr_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (>>), i16),
            Shr_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (>>), i32),
            Shr_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, (>>), i64),
            Rotl_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_left, u8, u32),
            Rotl_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_left, u16, u32),
            Rotl_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_left, u32, u32),
            Rotl_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_left, u64, u32),
            Rotl_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_left, i8, u32),
            Rotl_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_left, i16, u32),
            Rotl_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_left, i32, u32),
            Rotl_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_left, i64, u32),
            Rotr_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_right, u8, u32),
            Rotr_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_right, u16, u32),
            Rotr_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_right, u32, u32),
            Rotr_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_right, u64, u32),
            Rotr_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_right, i8, u32),
            Rotr_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_right, i16, u32),
            Rotr_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_right, i32, u32),
            Rotr_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rotate_right, i64, u32),
            Count_Ones_u8 { rd, rs1 } => op1!(rd, rs1, count_ones, u8),
            Count_Ones_u16 { rd, rs1 } => op1!(rd, rs1, count_ones, u16),
            Count_Ones_u32 { rd, rs1 } => op1!(rd, rs1, count_ones, u32),
            Count_Ones_u64 { rd, rs1 } => op1!(rd, rs1, count_ones, u64),
            Count_Ones_i8 { rd, rs1 } => op1!(rd, rs1, count_ones, i8),
            Count_Ones_i16 { rd, rs1 } => op1!(rd, rs1, count_ones, i16),
            Count_Ones_i32 { rd, rs1 } => op1!(rd, rs1, count_ones, i32),
            Count_Ones_i64 { rd, rs1 } => op1!(rd, rs1, count_ones, i64),
            Leading_Ones_u8 { rd, rs1 } => op1!(rd, rs1, leading_ones, u8),
            Leading_Ones_u16 { rd, rs1 } => op1!(rd, rs1, leading_ones, u16),
            Leading_Ones_u32 { rd, rs1 } => op1!(rd, rs1, leading_ones, u32),
            Leading_Ones_u64 { rd, rs1 } => op1!(rd, rs1, leading_ones, u64),
            Leading_Ones_i8 { rd, rs1 } => op1!(rd, rs1, leading_ones, i8),
            Leading_Ones_i16 { rd, rs1 } => op1!(rd, rs1, leading_ones, i16),
            Leading_Ones_i32 { rd, rs1 } => op1!(rd, rs1, leading_ones, i32),
            Leading_Ones_i64 { rd, rs1 } => op1!(rd, rs1, leading_ones, i64),
            Trailing_Ones_u8 { rd, rs1 } => op1!(rd, rs1, trailing_ones, u8),
            Trailing_Ones_u16 { rd, rs1 } => op1!(rd, rs1, trailing_ones, u16),
            Trailing_Ones_u32 { rd, rs1 } => op1!(rd, rs1, trailing_ones, u32),
            Trailing_Ones_u64 { rd, rs1 } => op1!(rd, rs1, trailing_ones, u64),
            Trailing_Ones_i8 { rd, rs1 } => op1!(rd, rs1, trailing_ones, i8),
            Trailing_Ones_i16 { rd, rs1 } => op1!(rd, rs1, trailing_ones, i16),
            Trailing_Ones_i32 { rd, rs1 } => op1!(rd, rs1, trailing_ones, i32),
            Trailing_Ones_i64 { rd, rs1 } => op1!(rd, rs1, trailing_ones, i64),
            Count_Zeros_u8 { rd, rs1 } => op1!(rd, rs1, count_zeros, u8),
            Count_Zeros_u16 { rd, rs1 } => op1!(rd, rs1, count_zeros, u16),
            Count_Zeros_u32 { rd, rs1 } => op1!(rd, rs1, count_zeros, u32),
            Count_Zeros_u64 { rd, rs1 } => op1!(rd, rs1, count_zeros, u64),
            Count_Zeros_i8 { rd, rs1 } => op1!(rd, rs1, count_zeros, i8),
            Count_Zeros_i16 { rd, rs1 } => op1!(rd, rs1, count_zeros, i16),
            Count_Zeros_i32 { rd, rs1 } => op1!(rd, rs1, count_zeros, i32),
            Count_Zeros_i64 { rd, rs1 } => op1!(rd, rs1, count_zeros, i64),
            Leading_Zeros_u8 { rd, rs1 } => op1!(rd, rs1, leading_zeros, u8),
            Leading_Zeros_u16 { rd, rs1 } => op1!(rd, rs1, leading_zeros, u16),
            Leading_Zeros_u32 { rd, rs1 } => op1!(rd, rs1, leading_zeros, u32),
            Leading_Zeros_u64 { rd, rs1 } => op1!(rd, rs1, leading_zeros, u64),
            Leading_Zeros_i8 { rd, rs1 } => op1!(rd, rs1, leading_zeros, i8),
            Leading_Zeros_i16 { rd, rs1 } => op1!(rd, rs1, leading_zeros, i16),
            Leading_Zeros_i32 { rd, rs1 } => op1!(rd, rs1, leading_zeros, i32),
            Leading_Zeros_i64 { rd, rs1 } => op1!(rd, rs1, leading_zeros, i64),
            Trailing_Zeros_u8 { rd, rs1 } => op1!(rd, rs1, trailing_zeros, u8),
            Trailing_Zeros_u16 { rd, rs1 } => op1!(rd, rs1, trailing_zeros, u16),
            Trailing_Zeros_u32 { rd, rs1 } => op1!(rd, rs1, trailing_zeros, u32),
            Trailing_Zeros_u64 { rd, rs1 } => op1!(rd, rs1, trailing_zeros, u64),
            Trailing_Zeros_i8 { rd, rs1 } => op1!(rd, rs1, trailing_zeros, i8),
            Trailing_Zeros_i16 { rd, rs1 } => op1!(rd, rs1, trailing_zeros, i16),
            Trailing_Zeros_i32 { rd, rs1 } => op1!(rd, rs1, trailing_zeros, i32),
            Trailing_Zeros_i64 { rd, rs1 } => op1!(rd, rs1, trailing_zeros, i64),
            Reverse_Bytes_u8 { rd, rs1 } => op1!(rd, rs1, swap_bytes, u8),
            Reverse_Bytes_u16 { rd, rs1 } => op1!(rd, rs1, swap_bytes, u16),
            Reverse_Bytes_u32 { rd, rs1 } => op1!(rd, rs1, swap_bytes, u32),
            Reverse_Bytes_u64 { rd, rs1 } => op1!(rd, rs1, swap_bytes, u64),
            Reverse_Bytes_i8 { rd, rs1 } => op1!(rd, rs1, swap_bytes, i8),
            Reverse_Bytes_i16 { rd, rs1 } => op1!(rd, rs1, swap_bytes, i16),
            Reverse_Bytes_i32 { rd, rs1 } => op1!(rd, rs1, swap_bytes, i32),
            Reverse_Bytes_i64 { rd, rs1 } => op1!(rd, rs1, swap_bytes, i64),
            Reverse_Bits_u8 { rd, rs1 } => op1!(rd, rs1, reverse_bits, u8),
            Reverse_Bits_u16 { rd, rs1 } => op1!(rd, rs1, reverse_bits, u16),
            Reverse_Bits_u32 { rd, rs1 } => op1!(rd, rs1, reverse_bits, u32),
            Reverse_Bits_u64 { rd, rs1 } => op1!(rd, rs1, reverse_bits, u64),
            Reverse_Bits_i8 { rd, rs1 } => op1!(rd, rs1, reverse_bits, i8),
            Reverse_Bits_i16 { rd, rs1 } => op1!(rd, rs1, reverse_bits, i16),
            Reverse_Bits_i32 { rd, rs1 } => op1!(rd, rs1, reverse_bits, i32),
            Reverse_Bits_i64 { rd, rs1 } => op1!(rd, rs1, reverse_bits, i64),

            // Checked Arithmetic
            // ------------------
            C_Abs_i8 { rd, rs1 } => op1!(rd, rs1, checked_abs, i8),
            C_Abs_i16 { rd, rs1 } => op1!(rd, rs1, checked_abs, i16),
            C_Abs_i32 { rd, rs1 } => op1!(rd, rs1, checked_abs, i32),
            C_Abs_i64 { rd, rs1 } => op1!(rd, rs1, checked_abs, i64),
            C_Add_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add, i8),
            C_Add_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add, i16),
            C_Add_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add, i32),
            C_Add_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add, i64),
            C_Add_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add, u8),
            C_Add_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add, u16),
            C_Add_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add, u32),
            C_Add_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add, u64),
            C_Add_U_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add_unsigned, i8, u8),
            C_Add_U_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add_unsigned, i16, u16),
            C_Add_U_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add_unsigned, i32, u32),
            C_Add_U_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add_unsigned, i64, u64),
            C_Add_S_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add_signed, u8, i8),
            C_Add_S_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add_signed, u16, i16),
            C_Add_S_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add_signed, u32, i32),
            C_Add_S_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_add_signed, u64, i64),
            C_Div_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div, i8),
            C_Div_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div, i16),
            C_Div_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div, i32),
            C_Div_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div, i64),
            C_Div_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div, u8),
            C_Div_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div, u16),
            C_Div_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div, u32),
            C_Div_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div, u64),
            C_Div_E_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div_euclid, i8),
            C_Div_E_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div_euclid, i16),
            C_Div_E_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div_euclid, i32),
            C_Div_E_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div_euclid, i64),
            C_Div_E_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div_euclid, u8),
            C_Div_E_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div_euclid, u16),
            C_Div_E_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div_euclid, u32),
            C_Div_E_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_div_euclid, u64),
            C_Log_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_ilog, i8),
            C_Log_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_ilog, i16),
            C_Log_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_ilog, i32),
            C_Log_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_ilog, i64),
            C_Log_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_ilog, u8),
            C_Log_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_ilog, u16),
            C_Log_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_ilog, u32),
            C_Log_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_ilog, u64),
            C_Sqrt_i8 { .. } => todo!(),  // op1!(r, checked_isqrt, i8),
            C_Sqrt_i16 { .. } => todo!(), // op1!(r, checked_isqrt, i16),
            C_Sqrt_i32 { .. } => todo!(), // op1!(r, checked_isqrt, i32),
            C_Sqrt_i64 { .. } => todo!(), // op1!(r, checked_isqrt, i64),
            C_Sqrt_u8 { .. } => todo!(),  // op1!(r, checked_isqrt, u8),
            C_Sqrt_u16 { .. } => todo!(), // op1!(r, checked_isqrt, u16),
            C_Sqrt_u32 { .. } => todo!(), // op1!(r, checked_isqrt, u32),
            C_Sqrt_u64 { .. } => todo!(), // op1!(r, checked_isqrt, u64),
            C_Mul_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_mul, i8),
            C_Mul_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_mul, i16),
            C_Mul_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_mul, i32),
            C_Mul_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_mul, i64),
            C_Mul_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_mul, u8),
            C_Mul_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_mul, u16),
            C_Mul_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_mul, u32),
            C_Mul_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_mul, u64),
            C_Neg_i8 { rd, rs1 } => op1!(rd, rs1, checked_neg, i8),
            C_Neg_i16 { rd, rs1 } => op1!(rd, rs1, checked_neg, i16),
            C_Neg_i32 { rd, rs1 } => op1!(rd, rs1, checked_neg, i32),
            C_Neg_i64 { rd, rs1 } => op1!(rd, rs1, checked_neg, i64),
            C_Pow_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_pow, i8, u32),
            C_Pow_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_pow, i16, u32),
            C_Pow_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_pow, i32, u32),
            C_Pow_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_pow, i64, u32),
            C_Pow_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_pow, u8, u32),
            C_Pow_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_pow, u16, u32),
            C_Pow_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_pow, u32, u32),
            C_Pow_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_pow, u64, u32),
            C_Rem_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem, i8),
            C_Rem_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem, i16),
            C_Rem_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem, i32),
            C_Rem_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem, i64),
            C_Rem_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem, u8),
            C_Rem_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem, u16),
            C_Rem_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem, u32),
            C_Rem_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem, u64),
            C_Rem_E_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem_euclid, i8),
            C_Rem_E_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem_euclid, i16),
            C_Rem_E_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem_euclid, i32),
            C_Rem_E_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem_euclid, i64),
            C_Rem_E_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem_euclid, u8),
            C_Rem_E_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem_euclid, u16),
            C_Rem_E_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem_euclid, u32),
            C_Rem_E_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_rem_euclid, u64),
            C_Shl_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shl, i8, u32),
            C_Shl_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shl, i16, u32),
            C_Shl_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shl, i32, u32),
            C_Shl_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shl, i64, u32),
            C_Shl_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shl, u8, u32),
            C_Shl_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shl, u16, u32),
            C_Shl_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shl, u32, u32),
            C_Shl_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shl, u64, u32),
            C_Shr_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shr, i8, u32),
            C_Shr_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shr, i16, u32),
            C_Shr_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shr, i32, u32),
            C_Shr_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shr, i64, u32),
            C_Shr_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shr, u8, u32),
            C_Shr_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shr, u16, u32),
            C_Shr_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shr, u32, u32),
            C_Shr_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_shr, u64, u32),
            C_Sub_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_sub, i8),
            C_Sub_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_sub, i16),
            C_Sub_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_sub, i32),
            C_Sub_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_sub, i64),
            C_Sub_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_sub, u8),
            C_Sub_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_sub, u16),
            C_Sub_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_sub, u32),
            C_Sub_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_sub, u64),
            C_Sub_U_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_sub_unsigned, i8, u8),
            C_Sub_U_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_sub_unsigned, i16, u16),
            C_Sub_U_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_sub_unsigned, i32, u32),
            C_Sub_U_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, checked_sub_unsigned, i64, u64),

            // Overflowing Arithmetic
            // ----------------------
            O_Abs_i8 { rd, rs1 } => op1!(rd, rs1, overflowing_abs, i8),
            O_Abs_i16 { rd, rs1 } => op1!(rd, rs1, overflowing_abs, i16),
            O_Abs_i32 { rd, rs1 } => op1!(rd, rs1, overflowing_abs, i32),
            O_Abs_i64 { rd, rs1 } => op1!(rd, rs1, overflowing_abs, i64),
            O_Add_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add, i8),
            O_Add_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add, i16),
            O_Add_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add, i32),
            O_Add_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add, i64),
            O_Add_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add, u8),
            O_Add_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add, u16),
            O_Add_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add, u32),
            O_Add_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add, u64),
            O_Add_U_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add_unsigned, i8, u8),
            O_Add_U_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add_unsigned, i16, u16),
            O_Add_U_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add_unsigned, i32, u32),
            O_Add_U_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add_unsigned, i64, u64),
            O_Add_S_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add_signed, u8, i8),
            O_Add_S_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add_signed, u16, i16),
            O_Add_S_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add_signed, u32, i32),
            O_Add_S_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_add_signed, u64, i64),
            O_Div_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div, i8),
            O_Div_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div, i16),
            O_Div_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div, i32),
            O_Div_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div, i64),
            O_Div_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div, u8),
            O_Div_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div, u16),
            O_Div_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div, u32),
            O_Div_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div, u64),
            O_Div_E_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div_euclid, i8),
            O_Div_E_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div_euclid, i16),
            O_Div_E_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div_euclid, i32),
            O_Div_E_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div_euclid, i64),
            O_Div_E_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div_euclid, u8),
            O_Div_E_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div_euclid, u16),
            O_Div_E_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div_euclid, u32),
            O_Div_E_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_div_euclid, u64),
            O_Mul_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_mul, i8),
            O_Mul_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_mul, i16),
            O_Mul_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_mul, i32),
            O_Mul_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_mul, i64),
            O_Mul_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_mul, u8),
            O_Mul_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_mul, u16),
            O_Mul_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_mul, u32),
            O_Mul_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_mul, u64),
            O_Neg_i8 { rd, rs1 } => op1!(rd, rs1, overflowing_neg, i8),
            O_Neg_i16 { rd, rs1 } => op1!(rd, rs1, overflowing_neg, i16),
            O_Neg_i32 { rd, rs1 } => op1!(rd, rs1, overflowing_neg, i32),
            O_Neg_i64 { rd, rs1 } => op1!(rd, rs1, overflowing_neg, i64),
            O_Pow_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_pow, i8, u32),
            O_Pow_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_pow, i16, u32),
            O_Pow_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_pow, i32, u32),
            O_Pow_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_pow, i64, u32),
            O_Pow_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_pow, u8, u32),
            O_Pow_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_pow, u16, u32),
            O_Pow_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_pow, u32, u32),
            O_Pow_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_pow, u64, u32),
            O_Rem_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem, i8),
            O_Rem_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem, i16),
            O_Rem_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem, i32),
            O_Rem_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem, i64),
            O_Rem_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem, u8),
            O_Rem_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem, u16),
            O_Rem_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem, u32),
            O_Rem_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem, u64),
            O_Rem_E_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem_euclid, i8),
            O_Rem_E_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem_euclid, i16),
            O_Rem_E_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem_euclid, i32),
            O_Rem_E_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem_euclid, i64),
            O_Rem_E_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem_euclid, u8),
            O_Rem_E_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem_euclid, u16),
            O_Rem_E_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem_euclid, u32),
            O_Rem_E_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_rem_euclid, u64),
            O_Shl_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shl, i8, u32),
            O_Shl_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shl, i16, u32),
            O_Shl_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shl, i32, u32),
            O_Shl_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shl, i64, u32),
            O_Shl_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shl, u8, u32),
            O_Shl_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shl, u16, u32),
            O_Shl_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shl, u32, u32),
            O_Shl_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shl, u64, u32),
            O_Shr_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shr, i8, u32),
            O_Shr_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shr, i16, u32),
            O_Shr_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shr, i32, u32),
            O_Shr_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shr, i64, u32),
            O_Shr_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shr, u8, u32),
            O_Shr_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shr, u16, u32),
            O_Shr_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shr, u32, u32),
            O_Shr_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_shr, u64, u32),
            O_Sub_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_sub, i8),
            O_Sub_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_sub, i16),
            O_Sub_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_sub, i32),
            O_Sub_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_sub, i64),
            O_Sub_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_sub, u8),
            O_Sub_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_sub, u16),
            O_Sub_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_sub, u32),
            O_Sub_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_sub, u64),
            O_Sub_U_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_sub_unsigned, i8, u8),
            O_Sub_U_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_sub_unsigned, i16, u16),
            O_Sub_U_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_sub_unsigned, i32, u32),
            O_Sub_U_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, overflowing_sub_unsigned, i64, u64),

            // Saturating Arithmetic
            // ---------------------
            S_Abs_i8 { rd, rs1 } => op1!(rd, rs1, saturating_abs, i8),
            S_Abs_i16 { rd, rs1 } => op1!(rd, rs1, saturating_abs, i16),
            S_Abs_i32 { rd, rs1 } => op1!(rd, rs1, saturating_abs, i32),
            S_Abs_i64 { rd, rs1 } => op1!(rd, rs1, saturating_abs, i64),
            S_Add_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add, i8),
            S_Add_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add, i16),
            S_Add_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add, i32),
            S_Add_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add, i64),
            S_Add_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add, u8),
            S_Add_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add, u16),
            S_Add_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add, u32),
            S_Add_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add, u64),
            S_Add_U_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add_unsigned, i8, u8),
            S_Add_U_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add_unsigned, i16, u16),
            S_Add_U_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add_unsigned, i32, u32),
            S_Add_U_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add_unsigned, i64, u64),
            S_Add_S_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add_signed, u8, i8),
            S_Add_S_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add_signed, u16, i16),
            S_Add_S_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add_signed, u32, i32),
            S_Add_S_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_add_signed, u64, i64),
            S_Div_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_div, i8),
            S_Div_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_div, i16),
            S_Div_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_div, i32),
            S_Div_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_div, i64),
            S_Div_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_div, u8),
            S_Div_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_div, u16),
            S_Div_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_div, u32),
            S_Div_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_div, u64),
            S_Mul_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_mul, i8),
            S_Mul_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_mul, i16),
            S_Mul_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_mul, i32),
            S_Mul_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_mul, i64),
            S_Mul_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_mul, u8),
            S_Mul_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_mul, u16),
            S_Mul_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_mul, u32),
            S_Mul_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_mul, u64),
            S_Neg_i8 { rd, rs1 } => op1!(rd, rs1, saturating_neg, i8),
            S_Neg_i16 { rd, rs1 } => op1!(rd, rs1, saturating_neg, i16),
            S_Neg_i32 { rd, rs1 } => op1!(rd, rs1, saturating_neg, i32),
            S_Neg_i64 { rd, rs1 } => op1!(rd, rs1, saturating_neg, i64),
            S_Pow_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_pow, i8, u32),
            S_Pow_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_pow, i16, u32),
            S_Pow_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_pow, i32, u32),
            S_Pow_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_pow, i64, u32),
            S_Pow_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_pow, u8, u32),
            S_Pow_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_pow, u16, u32),
            S_Pow_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_pow, u32, u32),
            S_Pow_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_pow, u64, u32),
            S_Sub_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_sub, i8),
            S_Sub_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_sub, i16),
            S_Sub_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_sub, i32),
            S_Sub_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_sub, i64),
            S_Sub_u8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_sub, u8),
            S_Sub_u16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_sub, u16),
            S_Sub_u32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_sub, u32),
            S_Sub_u64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_sub, u64),
            S_Sub_U_i8 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_sub_unsigned, i8, u8),
            S_Sub_U_i16 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_sub_unsigned, i16, u16),
            S_Sub_U_i32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_sub_unsigned, i32, u32),
            S_Sub_U_i64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, saturating_sub_unsigned, i64, u64),

            // Floating Point Arithmetic
            // -------------------------
            Abs_f32 { rd, rs1 } => op1!(rd, rs1, abs, f32),
            Abs_f64 { rd, rs1 } => op1!(rd, rs1, abs, f64),
            Add_f32 { rd, rs1, rs2 } => op2!( rd, rs1, rs2 , (+), f32),
            Add_f64 { rd, rs1, rs2 } => op2!( rd, rs1, rs2 , (+), f64),
            Div_f32 { rd, rs1, rs2 } => op2!( rd, rs1, rs2 , (/), f32),
            Div_f64 { rd, rs1, rs2 } => op2!( rd, rs1, rs2 , (/), f64),
            Div_E_f32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, div_euclid, f32),
            Div_E_f64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, div_euclid, f64),
            Log_f32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, log, f32),
            Log_f64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, log, f64),
            Sqrt_f32 { rd, rs1 } => op1!(rd, rs1, sqrt, f32),
            Sqrt_f64 { rd, rs1 } => op1!(rd, rs1, sqrt, f64),
            Mul_f32 { rd, rs1, rs2 } => op2!( rd, rs1, rs2 , (*), f32),
            Mul_f64 { rd, rs1, rs2 } => op2!( rd, rs1, rs2 , (*), f64),
            Neg_f32 { rd, rs1 } => op1!( rd, rs1 , (-), f32),
            Neg_f64 { rd, rs1 } => op1!( rd, rs1 , (-), f64),
            Pow_f32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, powf, f32),
            Pow_f64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, powf, f64),
            Rem_f32 { rd, rs1, rs2 } => op2!( rd, rs1, rs2 , (%), f32),
            Rem_f64 { rd, rs1, rs2 } => op2!( rd, rs1, rs2 , (%), f64),
            Rem_E_f32 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rem_euclid, f32),
            Rem_E_f64 { rd, rs1, rs2 } => op2!(rd, rs1, rs2, rem_euclid, f64),
            Cbrt_f32 { rd, rs1 } => op1!(rd, rs1, cbrt, f32),
            Cbrt_f64 { rd, rs1 } => op1!(rd, rs1, cbrt, f64),
            Sub_f32 { rd, rs1, rs2 } => op2!( rd, rs1, rs2 , (-), f32),
            Sub_f64 { rd, rs1, rs2 } => op2!( rd, rs1, rs2 , (-), f64),

            // Invalid
            // -------
            Invalid(i) => {
                use byte_code::encoding::*;

                println!("Invalid Instruction: {:032b}", i);
                println!("op_code: {}", decode_op_code(i));
                println!("funct3:  {}", decode_funct(i));
                println!("-------------------------------");
                println!("As R-Type:");
                println!("rd:      {:?}", decode_rd(i));
                println!("rs1:     {:?}", decode_rs1(i));
                println!("rs2:     {:?}", decode_rs2(i));
                println!("size:    {}", decode_size(i));
                println!("f:       {}", decode_f(i));
                println!("s:       {}", decode_s(i));
                println!("-------------------------------");
                println!("As I-Type:");
                println!("rd:      {:?}", decode_rd(i));
                println!("imm:     {}", decode_imm(i));
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
