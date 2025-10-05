// struct CPU {
//     current_operation: u16, // All CHIP-8 opcodes are  u16 values
//     registers: [u8; 2],     //these 2 registers are sufficient for addition
// }

struct CPU {
    registers: [u8; 16],
    position_in_memory: usize, // usize is great as rust allows it to be used for indexing
    memory: [u8; 0x1000],      // 0x1000 - 4096 = 4kb
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        /*
          to create a u16 opcode, we combine two values from memory with the logical OR operation. these need to be cast
            as u16 to start with; otherwise, the left shift sets all the bits to 0.

            just combine the 2 bytes to get the full operation code.
        */
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2; // increments position_in_memory to point to the next instruction

            /*
             select single nibbles with the AND operator(&) to filter bits that should be retained, then shift to move the bits to the lowest
               significant place. Hexadecimal notation is convenient for these operations as each hexadecimal represents 4 bits. A 0xF(1111) value
                selects all bits from a nibble
            */
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;
            // }

            match (c, x, y, d) {
                (0, 0, 0, 0) => return, //short - circuits the function to terminate execution when the opcode 0X0000 is encountered
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode), // the full emulator will have many operations
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        // //usize is array indexing works with. so x as usize means self.registers[0]
        // self.registers[x as usize] += self.registers[y as usize]

        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        /*
         the overflowing_add() method returns (u8, bool). the bool is true when overflow is detected
        */
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;

    // loads opcode 0x8014, which adds register 1 to register 0
    mem[0] = 0x80;
    mem[1] = 0x14;

    // loads opcode 0x8024, which adds register 2 to register 0
    mem[2] = 0x80;
    mem[3] = 0x24;

    // loads opcode 0x8014, which adds register 3 to register 0
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu.run();

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0])
}
