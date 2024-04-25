# Herbal Candy

A register based virtual machine written in Rust.

Current status: **Mostly WIP**

The project is structured as a workspace with the following crates:

- `apps/vm` - The virtual machine. Press Enter to step the VM (The crate name will change at some point)
- `apps/assembler` - A simple assembler that converts a simple assembly language to bytecode.
- `libs/byte_code` - A library that defines the bytecode format. Most.
- `libs/instr_codegen` - A library that defines the instruction set.


# Roadmap

- [x] Define the instruction set
- [x] Define the bytecode format
- [ ] Implement the virtual machine
- [ ] Implement the assembler
- [ ] Implement a simple debugger
