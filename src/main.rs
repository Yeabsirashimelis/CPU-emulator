struct CPU {
    current_operation: u16, // All CHIP-8 opcodes are  u16 values
    registers: [u8; 2],     //these 2 registers are sufficient for addition
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        // loop {
        let opcode = self.read_opcode();

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
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode), // the full emulator will have many operations
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        //usize is array indexing works with. so x as usize means self.registers[0]
        self.registers[x as usize] += self.registers[y as usize]
    }
}

fn main() {
    //   the process of booting up the CPU consists of writting to the fields of the CPU struct.
    //initializes with no-op(do nothing)
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };

    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 15);
    println!("5 + 10 = {}", cpu.registers[0]);
}
