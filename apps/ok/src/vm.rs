use core::slice;
use std::{mem, ptr};

use crate::byte_code::{Micro_Op, Register};
use crate::memory::{Memory, Memory_Address};
use crate::program::Program;

pub struct Virtual_Machine {
    registers: Box<[u64; u8::MAX as usize]>,
    instruction_pointer: usize,
    stack_pointer: usize,
    pub memory: Memory,
}

impl Virtual_Machine {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: Box::new([0; u8::MAX as usize]),
            instruction_pointer: 0,
            stack_pointer: memory_size,
            memory: Memory::new(memory_size),
        }
    }

    pub fn current_instruction_is_invalid(&self) {
        panic!("Invalid instruction");
    }

    pub fn general_purpose_register_value_mut(&mut self, index: u8) -> &mut u64 {
        &mut self.registers[index as usize]
    }

    /// Mutable access is only given to general purpose registers.
    pub fn register_value_mut(&mut self, register: Register) -> Option<&mut u64> {
        match register {
            Register::General_Purpose(index) => self.registers.get_mut(index as usize),

            _ => None,
        }
    }

    pub fn register_value(&self, register: Register) -> u64 {
        match register {
            Register::General_Purpose(index) => self.registers[index as usize],
            Register::Instruction_Pointer => self.instruction_pointer as u64,
            Register::Stack_Pointer => self.stack_pointer as u64,
        }
    }

    pub fn run_program(&mut self, program: &Program) {
        self.instruction_pointer = program.start;
        self.memory
            .write(program.data.as_slice(), Memory_Address(0));

        while let Some(instruction) = program.code.get(self.instruction_pointer) {
            self.execute(*instruction);
            self.instruction_pointer += 1;
        }
    }

    fn execute(&mut self, instruction: Micro_Op) {
        macro_rules! saturating {
            ($operands:expr, $operation:ident as $t:ty) => {{
                if let Some(destination_index) = $operands.destination.as_general_purpose() {
                    let source_1 = self.register_value($operands.source_1) as $t;
                    let source_2 = self.register_value($operands.source_2) as $t;

                    let result = source_1.$operation(source_2) as u64;

                    *self.general_purpose_register_value_mut(destination_index) = result;
                } else {
                    self.current_instruction_is_invalid();
                }
            }};
        }

        macro_rules! overflowing {
            ($operands:expr, $operation:ident as $t:ty) => {{
                if let Some(destination_index) = $operands.destination.as_general_purpose() {
                    let source_1 = self.register_value($operands.source_1) as $t;
                    let source_2 = self.register_value($operands.source_2) as $t;

                    // TODO: Should set the overflow flag.
                    let (result, _) = source_1.$operation(source_2);

                    *self.general_purpose_register_value_mut(destination_index) = result as u64;

                    // self.set_flag(Flag::Overflow, overflow);
                } else {
                    self.current_instruction_is_invalid();
                }
            }};
        }

        macro_rules! load {
            ($register:expr, $address:expr, $size:expr) => {{
                let src = self.memory.slot($address);

                if let Some(r) = self.register_value_mut($register) {
                    // We need to make sure that we're only updating the bytes that we're interested in.
                    let mut buffer = r.to_le_bytes();

                    unsafe { ptr::copy_nonoverlapping(src, buffer.as_mut_ptr(), $size) };

                    *r = u64::from_le_bytes(buffer);
                }
            }};
        }

        macro_rules! store {
            ($address:expr, $register:expr, $size:expr) => {{
                let value = self.register_value($register);
                let bytes = value.to_le_bytes();
                let dst = self.memory.slot_mut($address);

                unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), dst, $size) }
            }};
        }

        match instruction {
            // Saturating i32
            // --------------
            Micro_Op::Saturating_Add_I32(o) => saturating!(o, saturating_add as i32),
            Micro_Op::Saturating_Sub_I32(o) =>                saturating!(o, saturating_sub as i32)
            ,
            Micro_Op::Saturating_Mul_I32(o) => saturating!(o, saturating_mul as i32) ,
            Micro_Op::Saturating_Div_I32(o) => saturating!(o, saturating_div as i32) ,

            // Saturating u32
            // --------------
            Micro_Op::Saturating_Add_U32(o) =>
                saturating!(o, saturating_add as u32)
            ,
            Micro_Op::Saturating_Sub_U32(o) =>
                saturating!(o, saturating_sub as u32)
            ,
            Micro_Op::Saturating_Mul_U32(o) =>
                saturating!(o, saturating_mul as u32)
            ,
            Micro_Op::Saturating_Div_U32(o) =>
                saturating!(o, saturating_div as u32)
            ,

            // Saturating i64
            // --------------
            Micro_Op::Saturating_Add_I64(o) =>
                saturating!(o, saturating_add as i64)
            ,
            Micro_Op::Saturating_Sub_I64(o) =>
                saturating!(o, saturating_sub as i64)
            ,
            Micro_Op::Saturating_Mul_I64(o) =>
                saturating!(o, saturating_mul as i64)
            ,
            Micro_Op::Saturating_Div_I64(o) =>
                saturating!(o, saturating_div as i64)
            ,

            // Saturating u64
            // --------------
            Micro_Op::Saturating_Add_U64(o) =>
                saturating!(o, saturating_add as u64)
            ,
            Micro_Op::Saturating_Sub_U64(o) =>
                saturating!(o, saturating_sub as u64)
            ,
            Micro_Op::Saturating_Mul_U64(o) =>
                saturating!(o, saturating_mul as u64)
            ,
            Micro_Op::Saturating_Div_U64(o) =>
                saturating!(o, saturating_div as u64)
            ,

            // Overflowing i32
            // ---------------
            Micro_Op::Overflowing_Add_I32(o) =>
                overflowing!(o, overflowing_add as i32)
            ,
            Micro_Op::Overflowing_Sub_I32(o) =>
                overflowing!(o, overflowing_sub as i32)
            ,
            Micro_Op::Overflowing_Mul_I32(o) =>
                overflowing!(o, overflowing_mul as i32)
            ,
            Micro_Op::Overflowing_Div_I32(o) =>
                overflowing!(o, overflowing_div as i32)
            ,

            // Overflowing u32
            // ---------------
            Micro_Op::Overflowing_Add_U32(o) =>
                overflowing!(o, overflowing_add as u32)
            ,
            Micro_Op::Overflowing_Sub_U32(o) =>
                overflowing!(o, overflowing_sub as u32)
            ,
            Micro_Op::Overflowing_Mul_U32(o) =>
                overflowing!(o, overflowing_mul as u32)
            ,
            Micro_Op::Overflowing_Div_U32(o) =>
                overflowing!(o, overflowing_div as u32)
            ,

            // Overflowing i64
            // ---------------
            Micro_Op::Overflowing_Add_I64(o) =>
                overflowing!(o, overflowing_add as i64)
            ,
            Micro_Op::Overflowing_Sub_I64(o) =>
                overflowing!(o, overflowing_sub as i64)
            ,
            Micro_Op::Overflowing_Mul_I64(o) =>
                overflowing!(o, overflowing_mul as i64)
            ,
            Micro_Op::Overflowing_Div_I64(o) =>
                overflowing!(o, overflowing_div as i64)
            ,

            // Overflowing u64
            // ---------------
            Micro_Op::Overflowing_Add_U64(o) =>
                overflowing!(o, overflowing_add as u64)
            ,
            Micro_Op::Overflowing_Sub_U64(o) =>
                overflowing!(o, overflowing_sub as u64)
            ,
            Micro_Op::Overflowing_Mul_U64(o) =>
                overflowing!(o, overflowing_mul as u64)
            ,
            Micro_Op::Overflowing_Div_U64(o) =>
                overflowing!(o, overflowing_div as u64)
            ,

            // Load and Store
            // --------------
            Micro_Op::Load_8(register, address) => load!(register, address, 1),
            Micro_Op::Load_16(register, address) => load!(register, address, 2),
            Micro_Op::Load_32(register, address) => load!(register, address, 4),
            Micro_Op::Load_64(register, address) => load!(register, address, 8),

            Micro_Op::Store_8(address, register) => store!(address, register, 1),
            Micro_Op::Store_16(address, register) => store!(address, register, 2),
            Micro_Op::Store_32(address, register) => store!(address, register, 4),
            Micro_Op::Store_64(address, register) => store!(address, register, 8),

            Micro_Op::Push(register) => {
                self.stack_pointer -= 8;
                store!(Memory_Address(self.stack_pointer), register, 8);
            },

            Micro_Op::Pop(register) => {
                load!(register, Memory_Address(self.stack_pointer), 8);
                self.stack_pointer += 8;
            },

            // Branching
            // ---------
            Micro_Op::Jump(address) => {
                self.instruction_pointer = address;
            },
        }
    }
}
