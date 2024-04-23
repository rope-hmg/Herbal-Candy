# Herbal Candy

A register based virtual machine written in Rust.

Current status: **Very WIP**

The project is structured as a workspace with the following crates:

- `apps/vm` - The virtual machine. Press Enter to step the VM (The crate name will change at some point)
- `apps/assembler` - A simple assembler that converts a simple assembly language to bytecode.
- `libs/byte_code` - A library that defines the bytecode format.
