use std::ptr;

use byte_code::{Bit_Width, I_Type, Instruction, Op_Code, R_Type, Register};

use crate::memory::{Memory, Memory_Address};
use crate::program::Program;

pub const REGISTER_COUNT: usize = 64;

pub struct Virtual_Machine {
    registers: Box<[u64; REGISTER_COUNT]>,
    memory: Memory,
}

pub enum Instruction_Pointer {
    Goto(u64),
    Next,
    Halt,
}

trait Operator {
    fn do_i8(vm: &mut Virtual_Machine, a: i8, b: i8) -> i8;
    fn do_i16(vm: &mut Virtual_Machine, a: i16, b: i16) -> i16;
    fn do_i32(vm: &mut Virtual_Machine, a: i32, b: i32) -> i32;
    fn do_i64(vm: &mut Virtual_Machine, a: i64, b: i64) -> i64;
    fn do_u8(vm: &mut Virtual_Machine, a: u8, b: u8) -> u8;
    fn do_u16(vm: &mut Virtual_Machine, a: u16, b: u16) -> u16;
    fn do_u32(vm: &mut Virtual_Machine, a: u32, b: u32) -> u32;
    fn do_u64(vm: &mut Virtual_Machine, a: u64, b: u64) -> u64;
}

macro_rules! operator {
    (Saturating $($operator:ident { $function:ident }),*) => {
        $(
            struct $operator;

            #[rustfmt::skip]
            impl Operator for $operator {
                fn do_i8 (vm: &mut Virtual_Machine, a:i8,  b:i8)  -> i8  { a.$function(b) }
                fn do_i16(vm: &mut Virtual_Machine, a:i16, b:i16) -> i16 { a.$function(b) }
                fn do_i32(vm: &mut Virtual_Machine, a:i32, b:i32) -> i32 { a.$function(b) }
                fn do_i64(vm: &mut Virtual_Machine, a:i64, b:i64) -> i64 { a.$function(b) }
                fn do_u8 (vm: &mut Virtual_Machine, a:u8,  b:u8)  -> u8  { a.$function(b) }
                fn do_u16(vm: &mut Virtual_Machine, a:u16, b:u16) -> u16 { a.$function(b) }
                fn do_u32(vm: &mut Virtual_Machine, a:u32, b:u32) -> u32 { a.$function(b) }
                fn do_u64(vm: &mut Virtual_Machine, a:u64, b:u64) -> u64 { a.$function(b) }
            }
        )*
    };

    (Overflowing $($operator:ident { $function:ident }),*) => {
        $(
            struct $operator;

            // TODO: Set the overflow flag
            #[rustfmt::skip]
            impl Operator for $operator {
                fn do_i8 (vm: &mut Virtual_Machine, a:i8,  b:i8)  -> i8  { a.$function(b).0 }
                fn do_i16(vm: &mut Virtual_Machine, a:i16, b:i16) -> i16 { a.$function(b).0 }
                fn do_i32(vm: &mut Virtual_Machine, a:i32, b:i32) -> i32 { a.$function(b).0 }
                fn do_i64(vm: &mut Virtual_Machine, a:i64, b:i64) -> i64 { a.$function(b).0 }
                fn do_u8 (vm: &mut Virtual_Machine, a:u8,  b:u8)  -> u8  { a.$function(b).0 }
                fn do_u16(vm: &mut Virtual_Machine, a:u16, b:u16) -> u16 { a.$function(b).0 }
                fn do_u32(vm: &mut Virtual_Machine, a:u32, b:u32) -> u32 { a.$function(b).0 }
                fn do_u64(vm: &mut Virtual_Machine, a:u64, b:u64) -> u64 { a.$function(b).0 }
            }
        )*
    }
}

operator! {
    Saturating
    Saturating_Add { saturating_add },
    Saturating_Sub { saturating_sub },
    Saturating_Mul { saturating_mul },
    Saturating_Div { saturating_div }
}

operator! {
    Overflowing
    Overflowing_Add { overflowing_add },
    Overflowing_Sub { overflowing_sub },
    Overflowing_Mul { overflowing_mul },
    Overflowing_Div { overflowing_div }
}

impl Virtual_Machine {
    pub fn new(memory_size: usize) -> Self {
        let mut vm = Self {
            registers: Box::new([0; REGISTER_COUNT]),
            memory: Memory::new(memory_size),
        };

        *vm.register_mut(Register::One) = 1;
        *vm.register_mut(Register::Stack_Pointer) = memory_size as u64;
        *vm.register_mut(Register::Frame_Pointer) = memory_size as u64;
        vm
    }

    pub fn run_program(&mut self, program: &Program) {
        *self.register_mut(Register::Instruction_Pointer) = program.start as u64;
        self.memory
            .write(program.data.as_slice(), Memory_Address(0));

        while let Some(instruction) = program
            .code
            .get(self.register(Register::Instruction_Pointer) as usize)
        {
            // println!("------------------------------");
            // println!("Instruction: {:?}", instruction);

            match self.execute(instruction) {
                Instruction_Pointer::Halt => break,
                Instruction_Pointer::Next => self.increment_register(Register::Instruction_Pointer),
                Instruction_Pointer::Goto(value) => {
                    *self.register_mut(Register::Instruction_Pointer) = value;
                },
            }
        }
    }

    fn maths<O>(&mut self, r_type: &R_Type)
    where
        O: Operator,
    {
        let source_1 = self.register(r_type.register_1);
        let source_2 = self.register(r_type.register_2);

        let result = if r_type.is_float {
            match r_type.bit_width {
                Bit_Width::Eight => unimplemented!(),
                Bit_Width::Sixteen => unimplemented!(),
                Bit_Width::Thirty_Two => {
                    let source_1 = f32::from_bits(source_1 as u32);
                    let source_2 = f32::from_bits(source_2 as u32);
                    let result = source_1 + source_2;
                    f32::to_bits(result) as u64
                },
                Bit_Width::Sixty_Four => {
                    let source_1 = f64::from_bits(source_1);
                    let source_2 = f64::from_bits(source_2);
                    let result = source_1 + source_2;
                    f64::to_bits(result)
                },
            }
        } else {
            match (r_type.bit_width, r_type.is_signed) {
                (Bit_Width::Eight, true) => O::do_i8(self, source_1 as i8, source_2 as i8) as u64,
                (Bit_Width::Sixteen, true) => {
                    O::do_i16(self, source_1 as i16, source_2 as i16) as u64
                },
                (Bit_Width::Thirty_Two, true) => {
                    O::do_i32(self, source_1 as i32, source_2 as i32) as u64
                },
                (Bit_Width::Sixty_Four, true) => {
                    O::do_i64(self, source_1 as i64, source_2 as i64) as u64
                },
                (Bit_Width::Eight, false) => O::do_u8(self, source_1 as u8, source_2 as u8) as u64,
                (Bit_Width::Sixteen, false) => {
                    O::do_u16(self, source_1 as u16, source_2 as u16) as u64
                },
                (Bit_Width::Thirty_Two, false) => {
                    O::do_u32(self, source_1 as u32, source_2 as u32) as u64
                },
                (Bit_Width::Sixty_Four, false) => {
                    O::do_u64(self, source_1 as u64, source_2 as u64) as u64
                },
            }
        };

        *self.register_mut(r_type.register_0) = result;
    }

    fn execute(&mut self, instruction: &Instruction) -> Instruction_Pointer {
        macro_rules! jump {
            ($op:tt) => {{
                let I_Type {
                    register,
                    immediate_value,
                    ..
                } = *instruction.i_type();

                if self.register(register) $op 0 {
                    Instruction_Pointer::Goto(immediate_value)
                } else {
                    Instruction_Pointer::Next
                }
            }};
        }

        macro_rules! jump_register {
            ($op:tt) => {{
                let R_Type {
                    register_0,
                    register_1,
                    ..
                } = *instruction.r_type();

                if self.register(register_1) $op 0 {
                    Instruction_Pointer::Goto(self.register(register_0))
                } else {
                    Instruction_Pointer::Next
                }
            }};
        }

        match instruction.op_code() {
            // Control Flow
            // ------------
            Op_Code::Halt => Instruction_Pointer::Halt,

            Op_Code::Call => {
                self.call();
                Instruction_Pointer::Goto(instruction.i_type().immediate_value)
            },

            Op_Code::Call_Register => {
                self.call();
                Instruction_Pointer::Goto(self.register(instruction.i_type().register))
            },

            Op_Code::Return => {
                // Reset the stack pointer to the top of the frame.
                self.move_register(Register::Frame_Pointer, Register::Stack_Pointer);

                self.pop(Register::Frame_Pointer); // Restore the previous frame pointer
                self.pop(Register::Instruction_Pointer); // Restore the return address

                Instruction_Pointer::Next
            },

            Op_Code::Jump => Instruction_Pointer::Goto(instruction.i_type().immediate_value),
            Op_Code::Jump_Not_Zero => jump!(!=),
            Op_Code::Jump_Zero => jump!(==),

            Op_Code::Jump_Register => {
                Instruction_Pointer::Goto(self.register(instruction.i_type().register))
            },
            Op_Code::Jump_Not_Zero_Register => jump_register!(!=),
            Op_Code::Jump_Zero_Register => jump_register!(==),

            // Comparison
            // ----------
            Op_Code::Compare_Equal => Instruction_Pointer::Next, // R_Type
            Op_Code::Compare_Not_Equal => Instruction_Pointer::Next, // R_Type
            Op_Code::Compare_Less => Instruction_Pointer::Next,  // R_Type
            Op_Code::Compare_Greater => Instruction_Pointer::Next, // R_Type
            Op_Code::Compare_Less_Equal => Instruction_Pointer::Next, // R_Type
            Op_Code::Compare_Greater_Equal => Instruction_Pointer::Next, // R_Type

            // Arithmetic
            // ----------
            Op_Code::Saturating_Absolute => Instruction_Pointer::Next, // R_Type
            Op_Code::Saturating_Negate => Instruction_Pointer::Next,   // R_Type
            Op_Code::Saturating_Power => Instruction_Pointer::Next,    // R_Type
            Op_Code::Saturating_Remainder => Instruction_Pointer::Next, // R_Type
            Op_Code::Saturating_Add => {
                self.maths::<Saturating_Add>(instruction.r_type());
                Instruction_Pointer::Next
            },

            Op_Code::Saturating_Subtract => {
                self.maths::<Saturating_Sub>(instruction.r_type());
                Instruction_Pointer::Next
            },

            Op_Code::Saturating_Multiply => {
                self.maths::<Saturating_Mul>(instruction.r_type());
                Instruction_Pointer::Next
            },

            Op_Code::Saturating_Divide => {
                self.maths::<Saturating_Div>(instruction.r_type());
                Instruction_Pointer::Next
            },

            Op_Code::Saturating_Modulus => Instruction_Pointer::Next, // R_Type
            Op_Code::Saturating_Shift_Left => Instruction_Pointer::Next, // R_Type
            Op_Code::Saturating_Shift_Right => Instruction_Pointer::Next, // R_Type

            Op_Code::Overflowing_Absolute => Instruction_Pointer::Next, // R_Type
            Op_Code::Overflowing_Negate => Instruction_Pointer::Next,   // R_Type
            Op_Code::Overflowing_Power => Instruction_Pointer::Next,    // R_Type
            Op_Code::Overflowing_Remainder => Instruction_Pointer::Next, // R_Type

            Op_Code::Overflowing_Add => {
                self.maths::<Overflowing_Add>(instruction.r_type());
                Instruction_Pointer::Next
            },

            Op_Code::Overflowing_Subtract => {
                self.maths::<Overflowing_Sub>(instruction.r_type());
                Instruction_Pointer::Next
            },

            Op_Code::Overflowing_Multiply => {
                self.maths::<Overflowing_Mul>(instruction.r_type());
                Instruction_Pointer::Next
            },

            Op_Code::Overflowing_Divide => {
                self.maths::<Overflowing_Div>(instruction.r_type());
                Instruction_Pointer::Next
            },

            Op_Code::Overflowing_Modulus => Instruction_Pointer::Next, // R_Type
            Op_Code::Overflowing_Shift_Left => Instruction_Pointer::Next, // R_Type
            Op_Code::Overflowing_Shift_Right => Instruction_Pointer::Next, // R_Type

            // Bitwise
            // -------
            Op_Code::And => Instruction_Pointer::Next, // R_Type
            Op_Code::Or => Instruction_Pointer::Next,  // R_Type
            Op_Code::Xor => Instruction_Pointer::Next, // R_Type
            Op_Code::Not => Instruction_Pointer::Next, // R_Type
            Op_Code::Rotate_Left => Instruction_Pointer::Next, // R_Type
            Op_Code::Rotate_Right => Instruction_Pointer::Next, // R_Type

            // Memory
            // ------
            Op_Code::Load => {
                let I_Type {
                    register,
                    bit_width,
                    immediate_value,
                } = *instruction.i_type();

                self.load(register, Memory_Address(immediate_value), bit_width);
                Instruction_Pointer::Next
            },

            Op_Code::Load_Immediate => {
                let I_Type {
                    register,
                    bit_width,
                    immediate_value,
                } = *instruction.i_type();

                self.load_immediate(register, immediate_value, bit_width);
                Instruction_Pointer::Next
            },

            Op_Code::Load_Register => {
                let R_Type {
                    register_0,
                    register_1,
                    bit_width,
                    ..
                } = *instruction.r_type();

                let address = Memory_Address(self.register(register_1));
                self.load(register_0, address, bit_width);

                Instruction_Pointer::Next
            },

            Op_Code::Store => {
                let I_Type {
                    register,
                    bit_width,
                    immediate_value: address,
                } = *instruction.i_type();

                self.store(Memory_Address(address), register, bit_width);
                Instruction_Pointer::Next
            },

            Op_Code::Store_Register => {
                let R_Type {
                    register_0,
                    register_1,
                    bit_width,
                    ..
                } = *instruction.r_type();

                let address = Memory_Address(self.register(register_1));
                self.store(address, register_0, bit_width);

                Instruction_Pointer::Next
            },

            Op_Code::Push => {
                let register = instruction.i_type().register;
                self.push(register);
                Instruction_Pointer::Next
            },

            Op_Code::Push_Immediate => {
                let immediate_value = instruction.i_type().immediate_value;
                self.push_immediate(immediate_value);
                Instruction_Pointer::Next
            },

            Op_Code::Pop => {
                let register = instruction.i_type().register;
                self.pop(register);
                Instruction_Pointer::Next
            },

            Op_Code::Move => {
                let R_Type {
                    register_0: destination,
                    register_1: source,
                    ..
                } = *instruction.r_type();

                if !destination.is_readonly() {
                    self.move_register(destination, source);

                    Instruction_Pointer::Next
                } else {
                    Instruction_Pointer::Halt
                }
            },

            // Invalid
            // -------
            Op_Code::Invalid => Instruction_Pointer::Halt,
        }
    }

    fn current_instruction_is_invalid(&self) {
        panic!("Invalid instruction");
    }

    #[inline(always)]
    fn register(&self, register: Register) -> u64 {
        self.registers[register.index()]
    }

    #[inline(always)]
    fn register_mut(&mut self, register: Register) -> &mut u64 {
        &mut self.registers[register.index()]
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

    // TODO: Unify load and load_immediate
    fn load(&mut self, register: Register, address: Memory_Address, size: Bit_Width) {
        if !register.is_readonly() {
            let src = self.memory.slot(address);
            let dst = self.register_mut(register);

            let mut buffer = dst.to_le_bytes();

            unsafe {
                ptr::copy_nonoverlapping(src, buffer.as_mut_ptr(), size.byte_count() as usize)
            };

            *dst = u64::from_le_bytes(buffer);
        }
    }

    fn load_immediate(&mut self, register: Register, value: u64, size: Bit_Width) {
        if !register.is_readonly() {
            let dst = self.register_mut(register);
            let mut buffer = dst.to_le_bytes();

            unsafe {
                ptr::copy_nonoverlapping(
                    &value as *const _ as *const u8,
                    buffer.as_mut_ptr(),
                    size.byte_count() as usize,
                )
            };

            *dst = u64::from_le_bytes(buffer);
        }
    }

    fn store(&mut self, address: Memory_Address, register: Register, size: Bit_Width) {
        let value = self.register(register);
        self.store_immediate(address, value, size)
    }

    fn store_immediate(&mut self, address: Memory_Address, value: u64, size: Bit_Width) {
        let bytes = value.to_le_bytes();
        let dst = self.memory.slot_mut(address);

        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), dst, size.byte_count() as usize) };
    }

    fn push(&mut self, register: Register) {
        self.decrement_register_n(Register::Stack_Pointer, 8);
        self.store(
            Memory_Address(self.register(Register::Stack_Pointer)),
            register,
            Bit_Width::Sixty_Four,
        );
    }

    fn push_immediate(&mut self, value: u64) {
        self.decrement_register_n(Register::Stack_Pointer, 8);
        self.store_immediate(
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

    /// Doesn't check if the destination register is readonly.
    fn move_register(&mut self, destination: Register, source: Register) {
        let source_value = self.register(source);

        *self.register_mut(destination) = source_value;
    }

    fn call(&mut self) {
        // Arguments are already pushed onto the stack.

        self.push(Register::Instruction_Pointer); // The "return" address (actually the address of the call instruction)
        self.push(Register::Frame_Pointer); // The current frame pointer

        // Set the frame pointer to the current stack pointer. We need to do this for two reasons:
        // 1. We need a reference point to access function arguments that are passed on the stack.
        // 2. So that the function being called can use the stack without worrying about cleaning up after itself.
        self.move_register(Register::Frame_Pointer, Register::Stack_Pointer);
    }
}
