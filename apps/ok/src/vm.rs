use std::ptr;

use crate::byte_code::{Instruction, Register};
use crate::memory::{Memory, Memory_Address};
use crate::program::Program;

pub const REGISTER_COUNT: usize = 64;

pub struct Virtual_Machine {
    registers: Box<[u64; REGISTER_COUNT]>,
    instruction_pointer: u64,
    stack_pointer: u64,
    frame_pointer: u64,
    memory: Memory,
}

impl Virtual_Machine {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: Box::new([0; REGISTER_COUNT]),
            instruction_pointer: 0,
            stack_pointer: memory_size as u64,
            frame_pointer: memory_size as u64,
            memory: Memory::new(memory_size),
        }
    }

    pub fn run_program(&mut self, program: &Program) {
        self.instruction_pointer = program.start as u64;
        self.memory
            .write(program.data.as_slice(), Memory_Address(0));

        while let Some(instruction) = program.code.get(self.instruction_pointer as usize) {
            // println!("------------------------------");
            // println!("Instruction: {:?}", instruction);

            match self.execute(*instruction) {
                Some(next_instruction_pointer) => {
                    self.instruction_pointer = next_instruction_pointer;
                },

                None => break,
            }
        }
    }

    fn execute(&mut self, instruction: Instruction) -> Option<u64> {
        macro_rules! saturating {
            ($operands:expr, $operation:ident as $t:ty) => {{
                if $operands.destination.is_general_purpose() {
                    let source_1 = self.register_value($operands.source_1) as $t;
                    let source_2 = self.register_value($operands.source_2) as $t;

                    let result = source_1.$operation(source_2) as u64;

                    if let Some(destination) = self.register_value_mut($operands.destination) {
                        *destination = result;
                    }

                    Some(self.instruction_pointer + 1)
                } else {
                    self.current_instruction_is_invalid();
                    None
                }
            }};
        }

        macro_rules! overflowing {
            ($operands:expr, $operation:ident as $t:ty) => {{
                if $operands.destination.is_general_purpose() {
                    let source_1 = self.register_value($operands.source_1) as $t;
                    let source_2 = self.register_value($operands.source_2) as $t;

                    // TODO: Should set the overflow flag.
                    let (result, _) = source_1.$operation(source_2);

                    if let Some(destination) = self.register_value_mut($operands.destination) {
                        *destination = result as u64;
                    }

                    // self.set_flag(Flag::Overflow, overflow);
                    Some(self.instruction_pointer + 1)
                } else {
                    self.current_instruction_is_invalid();
                    None
                }
            }};
        }

        macro_rules! load {
            ($register:expr, $address:expr, $size:expr) => {{
                self.load($register, $address, $size);
                Some(self.instruction_pointer + 1)
            }};
        }

        macro_rules! store {
            ($address:expr, $register:expr, $size:expr) => {{
                self.store($address, $register, $size);
                Some(self.instruction_pointer + 1)
            }};
        }

        match instruction {
            Instruction::Halt => None,

            // Saturating i8
            // --------------
            Instruction::Saturating_Add_I8(o) => saturating!(o, saturating_add as i8),
            Instruction::Saturating_Sub_I8(o) => saturating!(o, saturating_sub as i8),
            Instruction::Saturating_Mul_I8(o) => saturating!(o, saturating_mul as i8),
            Instruction::Saturating_Div_I8(o) => saturating!(o, saturating_div as i8),

            // Saturating u8
            // --------------
            Instruction::Saturating_Add_U8(o) => saturating!(o, saturating_add as u8),
            Instruction::Saturating_Sub_U8(o) => saturating!(o, saturating_sub as u8),
            Instruction::Saturating_Mul_U8(o) => saturating!(o, saturating_mul as u8),
            Instruction::Saturating_Div_U8(o) => saturating!(o, saturating_div as u8),

            // Saturating i16
            // --------------
            Instruction::Saturating_Add_I16(o) => saturating!(o, saturating_add as i16),
            Instruction::Saturating_Sub_I16(o) => saturating!(o, saturating_sub as i16),
            Instruction::Saturating_Mul_I16(o) => saturating!(o, saturating_mul as i16),
            Instruction::Saturating_Div_I16(o) => saturating!(o, saturating_div as i16),

            // Saturating u16
            // --------------
            Instruction::Saturating_Add_U16(o) => saturating!(o, saturating_add as u16),
            Instruction::Saturating_Sub_U16(o) => saturating!(o, saturating_sub as u16),
            Instruction::Saturating_Mul_U16(o) => saturating!(o, saturating_mul as u16),
            Instruction::Saturating_Div_U16(o) => saturating!(o, saturating_div as u16),

            // Saturating i32
            // --------------
            Instruction::Saturating_Add_I32(o) => saturating!(o, saturating_add as i32),
            Instruction::Saturating_Sub_I32(o) => saturating!(o, saturating_sub as i32),
            Instruction::Saturating_Mul_I32(o) => saturating!(o, saturating_mul as i32),
            Instruction::Saturating_Div_I32(o) => saturating!(o, saturating_div as i32),

            // Saturating u32
            // --------------
            Instruction::Saturating_Add_U32(o) => saturating!(o, saturating_add as u32),
            Instruction::Saturating_Sub_U32(o) => saturating!(o, saturating_sub as u32),
            Instruction::Saturating_Mul_U32(o) => saturating!(o, saturating_mul as u32),
            Instruction::Saturating_Div_U32(o) => saturating!(o, saturating_div as u32),

            // Saturating i64
            // --------------
            Instruction::Saturating_Add_I64(o) => saturating!(o, saturating_add as i64),
            Instruction::Saturating_Sub_I64(o) => saturating!(o, saturating_sub as i64),
            Instruction::Saturating_Mul_I64(o) => saturating!(o, saturating_mul as i64),
            Instruction::Saturating_Div_I64(o) => saturating!(o, saturating_div as i64),

            // Saturating u64
            // --------------
            Instruction::Saturating_Add_U64(o) => saturating!(o, saturating_add as u64),
            Instruction::Saturating_Sub_U64(o) => saturating!(o, saturating_sub as u64),
            Instruction::Saturating_Mul_U64(o) => saturating!(o, saturating_mul as u64),
            Instruction::Saturating_Div_U64(o) => saturating!(o, saturating_div as u64),

            // Overflowing i8
            // ---------------
            Instruction::Overflowing_Add_I8(o) => overflowing!(o, overflowing_add as i8),
            Instruction::Overflowing_Sub_I8(o) => overflowing!(o, overflowing_sub as i8),
            Instruction::Overflowing_Mul_I8(o) => overflowing!(o, overflowing_mul as i8),
            Instruction::Overflowing_Div_I8(o) => overflowing!(o, overflowing_div as i8),

            // Overflowing u8
            // ---------------
            Instruction::Overflowing_Add_U8(o) => overflowing!(o, overflowing_add as u8),
            Instruction::Overflowing_Sub_U8(o) => overflowing!(o, overflowing_sub as u8),
            Instruction::Overflowing_Mul_U8(o) => overflowing!(o, overflowing_mul as u8),
            Instruction::Overflowing_Div_U8(o) => overflowing!(o, overflowing_div as u8),

            // Overflowing i16
            // ---------------
            Instruction::Overflowing_Add_I16(o) => overflowing!(o, overflowing_add as i16),
            Instruction::Overflowing_Sub_I16(o) => overflowing!(o, overflowing_sub as i16),
            Instruction::Overflowing_Mul_I16(o) => overflowing!(o, overflowing_mul as i16),
            Instruction::Overflowing_Div_I16(o) => overflowing!(o, overflowing_div as i16),

            // Overflowing u16
            // ---------------
            Instruction::Overflowing_Add_U16(o) => overflowing!(o, overflowing_add as u16),
            Instruction::Overflowing_Sub_U16(o) => overflowing!(o, overflowing_sub as u16),
            Instruction::Overflowing_Mul_U16(o) => overflowing!(o, overflowing_mul as u16),
            Instruction::Overflowing_Div_U16(o) => overflowing!(o, overflowing_div as u16),

            // Overflowing i32
            // ---------------
            Instruction::Overflowing_Add_I32(o) => overflowing!(o, overflowing_add as i32),
            Instruction::Overflowing_Sub_I32(o) => overflowing!(o, overflowing_sub as i32),
            Instruction::Overflowing_Mul_I32(o) => overflowing!(o, overflowing_mul as i32),
            Instruction::Overflowing_Div_I32(o) => overflowing!(o, overflowing_div as i32),

            // Overflowing u32
            // ---------------
            Instruction::Overflowing_Add_U32(o) => overflowing!(o, overflowing_add as u32),
            Instruction::Overflowing_Sub_U32(o) => overflowing!(o, overflowing_sub as u32),
            Instruction::Overflowing_Mul_U32(o) => overflowing!(o, overflowing_mul as u32),
            Instruction::Overflowing_Div_U32(o) => overflowing!(o, overflowing_div as u32),

            // Overflowing i64
            // ---------------
            Instruction::Overflowing_Add_I64(o) => overflowing!(o, overflowing_add as i64),
            Instruction::Overflowing_Sub_I64(o) => overflowing!(o, overflowing_sub as i64),
            Instruction::Overflowing_Mul_I64(o) => overflowing!(o, overflowing_mul as i64),
            Instruction::Overflowing_Div_I64(o) => overflowing!(o, overflowing_div as i64),

            // Overflowing u64
            // ---------------
            Instruction::Overflowing_Add_U64(o) => overflowing!(o, overflowing_add as u64),
            Instruction::Overflowing_Sub_U64(o) => overflowing!(o, overflowing_sub as u64),
            Instruction::Overflowing_Mul_U64(o) => overflowing!(o, overflowing_mul as u64),
            Instruction::Overflowing_Div_U64(o) => overflowing!(o, overflowing_div as u64),

            // Load and Store
            // --------------
            Instruction::Load_8(register, address) => load!(register, address, 1),
            Instruction::Load_16(register, address) => load!(register, address, 2),
            Instruction::Load_32(register, address) => load!(register, address, 4),
            Instruction::Load_64(register, address) => load!(register, address, 8),

            Instruction::Store_8(address, register) => store!(address, register, 1),
            Instruction::Store_16(address, register) => store!(address, register, 2),
            Instruction::Store_32(address, register) => store!(address, register, 4),
            Instruction::Store_64(address, register) => store!(address, register, 8),

            Instruction::Push(register) => {
                self.push(register);
                Some(self.instruction_pointer + 1)
            },

            Instruction::Pop(register) => {
                self.pop(register);
                Some(self.instruction_pointer + 1)
            },

            // Register Operations
            // -------------------
            Instruction::Move(destination, source) => {
                if destination.is_general_purpose() {
                    let source_value = self.register_value(source);

                    if let Some(destination) = self.register_value_mut(destination) {
                        *destination = source_value;
                    }

                    Some(self.instruction_pointer + 1)
                } else {
                    None
                }
            },

            // Control Flow
            // ------------
            Instruction::Jump(address) => Some(address),

            Instruction::Jump_Not_Zero(address, register) => {
                if self.register_value(register) != 0 {
                    Some(address)
                } else {
                    Some(self.instruction_pointer + 1)
                }
            },

            Instruction::Call(address) => {
                // Arguments are already pushed onto the stack.

                self.push(Register::Instruction_Pointer); // The "return" address (actually the address of the call instruction)
                self.push(Register::Frame_Pointer); // The current frame pointer

                // Set the frame pointer to the current stack pointer. We need to do this for two reasons:
                // 1. We need a reference point to access function arguments that are passed on the stack.
                // 2. So that the function being called can use the stack without worrying about cleaning up after itself.
                self.frame_pointer = self.stack_pointer;

                Some(address)
            },

            Instruction::Return => {
                // Reset the stack pointer to the top of the frame.
                self.stack_pointer = self.frame_pointer;

                self.pop(Register::Frame_Pointer); // Restore the previous frame pointer
                self.pop(Register::Instruction_Pointer); // Restore the return address

                Some(self.instruction_pointer + 1)
            },
        }
    }

    fn current_instruction_is_invalid(&self) {
        panic!("Invalid instruction");
    }

    fn register_value_mut(&mut self, register: Register) -> Option<&mut u64> {
        use Register::*;

        match register {
            General_Purpose(index) => Some(&mut self.registers[index as usize]),
            Argument_0 => Some(&mut self.registers[0]),
            Argument_1 => Some(&mut self.registers[1]),
            Argument_2 => Some(&mut self.registers[2]),
            Argument_3 => Some(&mut self.registers[3]),
            Return_0 => Some(&mut self.registers[4]),
            Return_1 => Some(&mut self.registers[5]),
            Return_2 => Some(&mut self.registers[6]),
            Return_3 => Some(&mut self.registers[7]),
            Instruction_Pointer => Some(&mut self.instruction_pointer),
            Stack_Pointer => Some(&mut self.stack_pointer),
            Frame_Pointer => Some(&mut self.frame_pointer),
            Zero => None,
            One => None,
        }
    }

    fn register_value(&self, register: Register) -> u64 {
        use Register::*;

        match register {
            General_Purpose(index) => self.registers[index as usize],
            Argument_0 => self.registers[0],
            Argument_1 => self.registers[1],
            Argument_2 => self.registers[2],
            Argument_3 => self.registers[3],
            Return_0 => self.registers[4],
            Return_1 => self.registers[5],
            Return_2 => self.registers[6],
            Return_3 => self.registers[7],
            Instruction_Pointer => self.instruction_pointer,
            Stack_Pointer => self.stack_pointer,
            Frame_Pointer => self.frame_pointer,
            Zero => 0,
            One => 1,
        }
    }

    fn load(&mut self, register: Register, address: Memory_Address, size: usize) {
        let src = self.memory.slot(address);

        if let Some(dst) = self.register_value_mut(register) {
            let mut buffer = dst.to_le_bytes();

            unsafe { ptr::copy_nonoverlapping(src, buffer.as_mut_ptr(), size) };

            *dst = u64::from_le_bytes(buffer);
        }
    }

    fn store(&mut self, address: Memory_Address, register: Register, size: usize) {
        let value = self.register_value(register);
        let bytes = value.to_le_bytes();
        let dst = self.memory.slot_mut(address);

        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), dst, size) };
    }

    fn push(&mut self, register: Register) {
        self.stack_pointer -= 8;
        self.store(Memory_Address(self.stack_pointer), register, 8);
    }

    fn pop(&mut self, register: Register) {
        self.load(register, Memory_Address(self.stack_pointer), 8);
        self.stack_pointer += 8;
    }
}
