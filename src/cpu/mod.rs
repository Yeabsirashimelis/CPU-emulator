// struct CPU {
//     current_operation: u16, // All CHIP-8 opcodes are  u16 values
//     registers: [u8; 2],     //these 2 registers are sufficient for addition
// }

pub struct CPU {
    pub registers: [u8; 16],
    pub position_in_memory: usize, // usize is great as rust allows it to be used for indexing
    pub memory: [u8; 0x1000],      // 0x1000 - 4096 = 4kb
    pub stack: [u16; 16], // the stacks maximum height is 16. after 16 nested function calls, the program encounters a stack overflow
    pub stack_pointer: usize, // giving the stack_pointer type usize makes it easier to index values within the stack
}

impl CPU {
    pub fn read_opcode(&self) -> u16 {
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

    pub fn run(&mut self) {
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

            let nnn = opcode & 0x0FFF;
            // let kk = (opcode & 0x00FF) as u8

            match (c, x, y, d) {
                (0, 0, 0, 0) => return, //short - circuits the function to terminate execution when the opcode 0X0000 is encountered
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
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

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("stack overflow!")
        }

        /*
          add the current position_in_memory to the stack. this memory adress is 2 bytes higher that the calling location as it is
           incremented within the body of the run() method.
        */
        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize
    }
}
