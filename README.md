# CHIP-8 Emulator in Rust

A simple CHIP-8 emulator written in Rust. This project demonstrates a minimal CPU
implementation with stack management, registers, and basic opcode execution.

---

## Features

- `2nnn` – Call a subroutine at address `nnn`
- `00EE` – Return from a subroutine
- `8xy4` – Add register `Vy` to `Vx`
- `0000` – Halt execution
- Stack handling for subroutine calls
- Overflow detection using the VF register (`V[F]`)

---

## Memory Setup

Instructions are **loaded manually into memory** in `main.rs`, which allows testing
CPU behavior without a full ROM loader.

Example sequence:

```rust
// CALL function at 0x100
memory[0x000] = 0x21;
memory[0x001] = 0x00;

// CALL function at 0x100 again
memory[0x002] = 0x21;
memory[0x003] = 0x00;

// HALT
memory[0x004] = 0x00;
memory[0x005] = 0x00;

// Function at 0x100
memory[0x100] = 0x80;
memory[0x101] = 0x14;
memory[0x102] = 0x80;
memory[0x103] = 0x14;
memory[0x104] = 0x00;
memory[0x105] = 0xEE;
CPU, Stack & Memory Overview
sql
Copy code
Registers (V0..VF): [ V0=5, V1=10, ..., VF=0 ]

Memory (partial view):
0x000: 21 00  // CALL 0x100
0x002: 21 00  // CALL 0x100
0x004: 00 00  // HALT
0x100: 80 14  // ADD V1 to V0
0x102: 80 14  // ADD V1 to V0
0x104: 00 EE  // RETURN

Stack (used for subroutine calls):
SP=0 -> empty
After first CALL -> 0x002
After second CALL -> 0x004
position_in_memory tracks the next instruction.

stack stores return addresses for subroutine calls.

registers[0..F] hold CPU data. VF is the carry/overflow flag.

Usage
Clone the repository:

bash
Copy code
git clone https://github.com/yourusername/chip8-rust.git
cd chip8-rust
Build and run:

bash
Copy code
cargo run
Expected output:

Copy code
5 + (10 * 2) + (10 * 2) = 45
Project Structure
bash
Copy code
chip8/
├── Cargo.toml
└── src/
    ├── main.rs      # CPU setup and manual instruction loading
    ├── cpu.rs       # CPU struct and opcode implementations
    └── lib.rs       # exposes cpu module