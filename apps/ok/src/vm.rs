use std::ptr;

use crate::byte_code::{Micro_Op, Register};
use crate::memory::{Memory, Memory_Address};
use crate::program::Program;

pub const REGISTER_COUNT: usize = 64;

pub struct Virtual_Machine {
    registers: Box<[u64; REGISTER_COUNT]>,
    instruction_pointer: usize,
    stack_pointer: usize,
    frame_pointer: usize,
    memory: Memory,
}

/// Not what it seems?
enum Value_Mut<'a> {
    General_Purpose(&'a mut u64),
    Instruction_Pointer(&'a mut usize),
    Stack_Pointer(&'a mut usize),
    Frame_Pointer(&'a mut usize),
    None,
}

impl Virtual_Machine {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: Box::new([0; REGISTER_COUNT]),
            instruction_pointer: 0,
            stack_pointer: memory_size,
            frame_pointer: memory_size,
            memory: Memory::new(memory_size),
        }
    }

    pub fn run_program(&mut self, program: &Program) {
        self.instruction_pointer = program.start;
        self.memory
            .write(program.data.as_slice(), Memory_Address(0));

        while let Some(instruction) = program.code.get(self.instruction_pointer) {
            println!("------------------------------");
            println!("Instruction: {:?}", instruction);

            match self.execute(*instruction) {
                Some(next_instruction_pointer) => {
                    self.instruction_pointer = next_instruction_pointer;
                },

                None => break,
            }
        }
    }

    fn execute(&mut self, instruction: Micro_Op) -> Option<usize> {
        macro_rules! saturating {
            ($operands:expr, $operation:ident as $t:ty) => {{
                if $operands.destination.is_general_purpose() {
                    let source_1 = self.register_value($operands.source_1) as $t;
                    let source_2 = self.register_value($operands.source_2) as $t;

                    let result = source_1.$operation(source_2) as u64;

                    if let Value_Mut::General_Purpose(destination) =
                        self.register_value_mut($operands.destination)
                    {
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

                    if let Value_Mut::General_Purpose(destination) =
                        self.register_value_mut($operands.destination)
                    {
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
            Micro_Op::Halt => None,

            // Saturating i32
            // --------------
            Micro_Op::Saturating_Add_I32(o) => saturating!(o, saturating_add as i32),
            Micro_Op::Saturating_Sub_I32(o) => saturating!(o, saturating_sub as i32),
            Micro_Op::Saturating_Mul_I32(o) => saturating!(o, saturating_mul as i32),
            Micro_Op::Saturating_Div_I32(o) => saturating!(o, saturating_div as i32),

            // Saturating u32
            // --------------
            Micro_Op::Saturating_Add_U32(o) => saturating!(o, saturating_add as u32),
            Micro_Op::Saturating_Sub_U32(o) => saturating!(o, saturating_sub as u32),
            Micro_Op::Saturating_Mul_U32(o) => saturating!(o, saturating_mul as u32),
            Micro_Op::Saturating_Div_U32(o) => saturating!(o, saturating_div as u32),

            // Saturating i64
            // --------------
            Micro_Op::Saturating_Add_I64(o) => saturating!(o, saturating_add as i64),
            Micro_Op::Saturating_Sub_I64(o) => saturating!(o, saturating_sub as i64),
            Micro_Op::Saturating_Mul_I64(o) => saturating!(o, saturating_mul as i64),
            Micro_Op::Saturating_Div_I64(o) => saturating!(o, saturating_div as i64),

            // Saturating u64
            // --------------
            Micro_Op::Saturating_Add_U64(o) => saturating!(o, saturating_add as u64),
            Micro_Op::Saturating_Sub_U64(o) => saturating!(o, saturating_sub as u64),
            Micro_Op::Saturating_Mul_U64(o) => saturating!(o, saturating_mul as u64),
            Micro_Op::Saturating_Div_U64(o) => saturating!(o, saturating_div as u64),

            // Overflowing i32
            // ---------------
            Micro_Op::Overflowing_Add_I32(o) => overflowing!(o, overflowing_add as i32),
            Micro_Op::Overflowing_Sub_I32(o) => overflowing!(o, overflowing_sub as i32),
            Micro_Op::Overflowing_Mul_I32(o) => overflowing!(o, overflowing_mul as i32),
            Micro_Op::Overflowing_Div_I32(o) => overflowing!(o, overflowing_div as i32),

            // Overflowing u32
            // ---------------
            Micro_Op::Overflowing_Add_U32(o) => overflowing!(o, overflowing_add as u32),
            Micro_Op::Overflowing_Sub_U32(o) => overflowing!(o, overflowing_sub as u32),
            Micro_Op::Overflowing_Mul_U32(o) => overflowing!(o, overflowing_mul as u32),
            Micro_Op::Overflowing_Div_U32(o) => overflowing!(o, overflowing_div as u32),

            // Overflowing i64
            // ---------------
            Micro_Op::Overflowing_Add_I64(o) => overflowing!(o, overflowing_add as i64),
            Micro_Op::Overflowing_Sub_I64(o) => overflowing!(o, overflowing_sub as i64),
            Micro_Op::Overflowing_Mul_I64(o) => overflowing!(o, overflowing_mul as i64),
            Micro_Op::Overflowing_Div_I64(o) => overflowing!(o, overflowing_div as i64),

            // Overflowing u64
            // ---------------
            Micro_Op::Overflowing_Add_U64(o) => overflowing!(o, overflowing_add as u64),
            Micro_Op::Overflowing_Sub_U64(o) => overflowing!(o, overflowing_sub as u64),
            Micro_Op::Overflowing_Mul_U64(o) => overflowing!(o, overflowing_mul as u64),
            Micro_Op::Overflowing_Div_U64(o) => overflowing!(o, overflowing_div as u64),

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
                self.push(register);
                Some(self.instruction_pointer + 1)
            },

            Micro_Op::Pop(register) => {
                self.pop(register);
                Some(self.instruction_pointer + 1)
            },

            // Register Operations
            // -------------------
            Micro_Op::Move(destination, source) => {
                if destination.is_general_purpose() {
                    let source_value = self.register_value(source);

                    if let Value_Mut::General_Purpose(destination) =
                        self.register_value_mut(destination)
                    {
                        *destination = source_value;
                    }

                    Some(self.instruction_pointer + 1)
                } else {
                    None
                }
            },

            // Control Flow
            // ------------
            Micro_Op::Jump(address) => Some(address),

            Micro_Op::Jump_Not_Zero(address, register) => {
                if self.register_value(register) != 0 {
                    Some(address)
                } else {
                    Some(self.instruction_pointer + 1)
                }
            },

            Micro_Op::Call(address) => {
                // Arguments are already pushed onto the stack.

                self.push(Register::Instruction_Pointer); // The "return" address (actually the address of the call instruction)
                self.push(Register::Frame_Pointer); // The current frame pointer

                // Set the frame pointer to the current stack pointer. We need to do this for two reasons:
                // 1. We need a reference point to access function arguments that are passed on the stack.
                // 2. So that the function being called can use the stack without worrying about cleaning up after itself.
                self.frame_pointer = self.stack_pointer;

                Some(address)
            },

            Micro_Op::Return => {
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

    fn register_value_mut(&mut self, register: Register) -> Value_Mut {
        use Register::*;

        match register {
            General_Purpose(index) => {
                Value_Mut::General_Purpose(&mut self.registers[index as usize])
            },
            // TODO: Find a way to keep this in sync with the `register.as_general_purpose()` method.
            Argument_0 => Value_Mut::General_Purpose(&mut self.registers[0]),
            Argument_1 => Value_Mut::General_Purpose(&mut self.registers[1]),
            Argument_2 => Value_Mut::General_Purpose(&mut self.registers[2]),
            Argument_3 => Value_Mut::General_Purpose(&mut self.registers[3]),
            Return_0 => Value_Mut::General_Purpose(&mut self.registers[4]),
            Return_1 => Value_Mut::General_Purpose(&mut self.registers[5]),
            Return_2 => Value_Mut::General_Purpose(&mut self.registers[6]),
            Return_3 => Value_Mut::General_Purpose(&mut self.registers[7]),
            Instruction_Pointer => Value_Mut::Instruction_Pointer(&mut self.instruction_pointer),
            Stack_Pointer => Value_Mut::Stack_Pointer(&mut self.stack_pointer),
            Frame_Pointer => Value_Mut::Frame_Pointer(&mut self.frame_pointer),
            Zero => Value_Mut::None,
            One => Value_Mut::None,
        }
    }

    fn register_value(&self, register: Register) -> u64 {
        use Register::*;

        match register {
            General_Purpose(index) => self.registers[index as usize],
            // TODO: Find a way to keep this in sync with the `register.as_general_purpose()` method.
            Argument_0 => self.registers[0],
            Argument_1 => self.registers[1],
            Argument_2 => self.registers[2],
            Argument_3 => self.registers[3],
            Return_0 => self.registers[4],
            Return_1 => self.registers[5],
            Return_2 => self.registers[6],
            Return_3 => self.registers[7],
            Instruction_Pointer => self.instruction_pointer as u64,
            Stack_Pointer => self.stack_pointer as u64,
            Frame_Pointer => self.frame_pointer as u64,
            Zero => 0,
            One => 1,
        }
    }

    fn load(&mut self, register: Register, address: Memory_Address, size: usize) {
        let src = self.memory.slot(address);

        macro_rules! write_to_register {
            ($dst:expr, $t:ty) => {{
                // We need to make sure that we're only updating the bytes that we're interested in.
                let mut buffer = $dst.to_le_bytes();

                unsafe { ptr::copy_nonoverlapping(src, buffer.as_mut_ptr(), size) };

                *$dst = <$t>::from_le_bytes(buffer);
            }};
        }

        match self.register_value_mut(register) {
            Value_Mut::General_Purpose(r) => write_to_register!(r, u64),
            Value_Mut::Instruction_Pointer(r) => write_to_register!(r, usize),
            Value_Mut::Stack_Pointer(r) => write_to_register!(r, usize),
            Value_Mut::Frame_Pointer(r) => write_to_register!(r, usize),
            Value_Mut::None => {},
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
