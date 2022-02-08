type Address = u16;
type Byte = u8;
type Memory = [Byte; 0x1000];
type OpCode = u16;
type Registers = [Byte; 16];
type Stack = [u16; 16];

pub struct CPU {
    program_counter: usize,
    registers: Registers,
    memory: Memory,
    stack: Stack,
    stack_pointer: usize,
}


impl CPU {
    pub fn new(registers: Registers, memory: Memory) -> Self {
        CPU {
            program_counter: 0,
            registers,
            memory,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;

            let c = ((opcode & 0xF000) >> 12) as Byte;
            let x = ((opcode & 0x0F00) >> 08) as Byte;
            let y = ((opcode & 0x00F0) >> 04) as Byte;
            let d = ((opcode & 0x000F) >> 00) as Byte;
            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => break,
                (0x2, _, _, _) => self.call(nnn),
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn read_opcode(&self) -> OpCode {
        let p = self.program_counter;
        let byte1 = self.memory[p] as OpCode;
        let byte2 = self.memory[p+1] as OpCode;
        byte1 << 8 | byte2
    }

    fn call(&mut self, addr: Address) {
        if self.stack_pointer > self.stack.len() {
            panic!("Stack overflow")
        }

        self.stack[self.stack_pointer] = self.program_counter as Address;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack.len() == 0 {
            panic!("Stack underflow")
        }

        self.stack_pointer -= 1;
        let mem = self.stack[self.stack_pointer];
        self.program_counter = mem as usize;
    }

    fn add_xy(&mut self, x: Byte, y: Byte) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    // Getters
    pub fn registers(&self, ind: usize) -> Byte {
        self.registers[ind]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_cpu() {
        let memory = [0; 0x1000];
        let registers = [0; 16];
        let cpu = CPU::new(registers, memory);
        assert_eq!(cpu.registers, registers);
        assert_eq!(cpu.memory, memory);
        assert_eq!(cpu.program_counter, 0);
        assert_eq!(cpu.stack_pointer, 0);
        assert_eq!(cpu.stack, [0; 16]);
    }

    #[test]
    fn registers_gets_register_at_index() {
        let memory = [0; 0x1000];
        let mut registers = [0; 16];
        registers[3] = 3;
        let cpu = CPU::new(registers, memory);
        for i in 0..16 {
            assert_eq!(cpu.registers(i), if i == 3 { 3 } else { 0 });
        }
    }
}