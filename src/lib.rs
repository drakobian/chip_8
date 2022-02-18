//! CHIP-8 Emulator
//!
//! Given a set of values in registers and a program in memory,
//! the CPU struct provides a `run` function to
//! emulate a CHIP-8 processor.
//!
//! Reads memory two bytes at a time to form one operation.
//!
//! # Example
//!
//! ```
//! use chip_8::CPUBuilder;
//!
//! let mut registers = [0; 16];
//! registers[0] = 5;
//! registers[1] = 10;
//!
//! let mut memory = [0; 0x1000];
//! // Call the function at memory location `100` (opcode 0x2100)
//! memory[0x000] = 0x21; memory[0x001] = 0x00;
//! // Terminate
//! memory[0x002] = 0x00; memory[0x003] = 0x00;
//!
//! // Add the value in register `1` to register `0` (opcode 0x8014)
//! memory[0x100] = 0x80; memory[0x101] = 0x14;
//! // Return to previous memory location
//! memory[0x102] = 0x00; memory[0x103] = 0xEE;
//!
//! // the program in memory above adds the value of register 1
//! // to the value in register 0
//! let mut cpu = CPUBuilder::new()
//!                 .registers(registers)
//!                 .memory(memory)
//!                 .build();
//!
//! cpu.run();
//!
//! assert_eq!(15, cpu.registers(0));
//! ```

type Address = u16;
type Byte = u8;
type Memory = [Byte; 4096];
type OpCode = u16;
type Registers = [Byte; 16];
type Stack = [u16; 16];

/// Implements a CHIP-8 based CPU
pub struct CPU {
    program_counter: usize,
    registers: Registers,
    memory: Memory,
    stack: Stack,
    stack_pointer: usize,
}

/// Constructs a CPU with defaults, allowing for registers and memory to be
/// optionally set
pub struct CPUBuilder {
    registers: Option<Registers>,
    memory: Option<Memory>,
}

// TODO: link to the 'build' function in the docs for 'new'
impl CPUBuilder {
    /// Makes a new CPUBuilder, defaulting to empty registers and memory
    /// 
    /// call `build` to generate a CPU from this builder
    /// # Examples
    /// ```
    /// use chip_8::CPUBuilder;
    ///
    /// let default_builder = CPUBuilder::new();
    /// ```
    pub fn new() -> CPUBuilder {
        CPUBuilder {
            registers: None,
            memory: None,
        }
    }

    pub fn registers(&mut self, registers: Registers) -> &mut CPUBuilder {
        self.registers = Some(registers);
        self
    }

    pub fn memory(&mut self, memory: Memory) -> &mut CPUBuilder {
        self.memory = Some(memory);
        self
    }

    /// Generates a new CPU from this builder
    ///
    /// Sets registers and memory if those have been passed in
    /// 
    /// or defaults them to [0; 16] and [0; 4096], respectively
    /// 
    /// # Examples
    /// ```
    /// use chip_8::CPUBuilder;
    ///
    /// let default_cpu = CPUBuilder::new().build();
    ///
    /// let mut registers = [0; 16]; registers[5] = 12;
    /// let mut memory = [0; 4096]; memory[100] = 0x80;
    /// let specified_cpu = CPUBuilder::new()
    ///                         .registers(registers)
    ///                         .memory(memory)
    ///                         .build();
    /// ```
    pub fn build(&self) -> CPU {
        CPU {
            program_counter: 0,
            registers: self.registers.unwrap_or([0; 16]),
            memory: self.memory.unwrap_or([0; 0x1000]),
            stack: [0; 16],
            stack_pointer: 0,
        }
    }
}

impl CPU {
    // TODO: add some simple doc examples for doctests
    /// Runs the program set in memory according to the CHIP-8 spec
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

    /// Returns the next two bytes of memory concatenated as a u16
    fn read_opcode(&self) -> OpCode {
        let p = self.program_counter;
        let byte1 = self.memory[p] as OpCode;
        let byte2 = self.memory[p + 1] as OpCode;
        byte1 << 8 | byte2
    }

    /// Moves the program_counter to the given address, maintaining
    /// the old program_counter in the stack.
    ///
    /// # Panics
    ///
    /// Panics if the stack is full
    fn call(&mut self, addr: Address) {
        if self.stack_pointer >= self.stack.len() {
            panic!("Stack overflow")
        }

        self.stack[self.stack_pointer] = self.program_counter as Address;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    /// Moves the program_counter to the previous memory location
    /// on the stack.
    ///
    /// # Panics
    ///
    /// Panics if the stack is empty
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow")
        }

        self.stack_pointer -= 1;
        let mem = self.stack[self.stack_pointer];
        self.program_counter = mem as usize;
    }

    /// Increments the value in register `x` by the value in register `y`
    ///
    /// If this operation overflows the register size, the overflow register
    ///
    /// `0xF` is set to `1`
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

    /// A convenience method for retrieving the value of a specific register
    /// # Examples
    /// ```
    /// use chip_8::CPUBuilder;
    ///
    /// let mut registers = [0; 16]; registers[5] = 12;
    /// let cpu = CPUBuilder::new().registers(registers).build();
    /// assert_eq!(cpu.registers(5), 12);
    /// ```
    pub fn registers(&self, ind: usize) -> Byte {
        self.registers[ind]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_creates_cpu() {
        let cb = CPUBuilder::new();
        let cpu = cb.build();
        assert_eq!(cpu.registers, [0; 16]);
        assert_eq!(cpu.memory, [0; 0x1000]);
        assert_eq!(cpu.program_counter, 0);
        assert_eq!(cpu.stack_pointer, 0);
        assert_eq!(cpu.stack, [0; 16]);
    }

    #[test]
    fn builder_options_creates_cpu() {
        let mut registers = [0; 16];
        registers[5] = 10;

        let mut memory = [0; 4096];
        memory[0x001] = 0x80;

        let cpu = CPUBuilder::new()
            .registers(registers)
            .memory(memory)
            .build();

        assert_eq!(cpu.registers(5), 10);
        assert_eq!(cpu.memory[0x001], 0x80);
        assert_eq!(cpu.program_counter, 0);
        assert_eq!(cpu.stack_pointer, 0);
        assert_eq!(cpu.stack, [0; 16]);
    }

    #[test]
    fn registers_gets_register_at_index() {
        let mut registers = [0; 16];
        registers[3] = 3;
        let cpu = CPUBuilder::new().registers(registers).build();
        for i in 0..16 {
            assert_eq!(cpu.registers(i), if i == 3 { 3 } else { 0 });
        }
    }

    #[test]
    fn add_xy_adds_registers_no_overflow() {
        let mut registers = [0; 16];
        registers[0] = 3;
        registers[1] = 5;
        let mut cpu = CPUBuilder::new().registers(registers).build();
        cpu.add_xy(0, 1);

        assert_eq!(8, cpu.registers(0));
        assert_eq!(5, cpu.registers(1));
        assert_eq!(0, cpu.registers(15));
    }

    #[test]
    fn add_xy_adds_registers_overflow() {
        let mut registers = [0; 16];
        registers[0] = 255;
        registers[1] = 1;
        let mut cpu = CPUBuilder::new().registers(registers).build();
        cpu.add_xy(0, 1);

        assert_eq!(0, cpu.registers(0));
        assert_eq!(1, cpu.registers(15));
    }

    #[test]
    fn read_opcode_concats_next_two_bytes() {
        let byte1 = 0x81;
        let byte2 = 0x56;
        let start = 0x123;
        let mut memory = [0; 0x1000];
        memory[start] = byte1;
        memory[start + 1] = byte2;
        let mut cpu = CPUBuilder::new().memory(memory).build();
        cpu.program_counter = start;

        let expected = ((memory[start] as u16) << 8 | (memory[start + 1] as u16)) as u16;
        assert_eq!(expected, cpu.read_opcode());
    }

    #[test]
    #[should_panic(expected = "Stack overflow")]
    fn call_can_overflow_stack() {
        let mut cpu = CPUBuilder::new().build();
        cpu.stack_pointer = 16;

        cpu.call(0x100);
        assert_eq!(false, true, "Expected the stack to overflow")
    }

    #[test]
    fn call_sets_stack_and_pointers() {
        let start = 5;
        let pc = 0x100;
        let addr = 200;

        let mut cpu = CPUBuilder::new().build();
        cpu.stack_pointer = start;
        cpu.program_counter = pc;

        cpu.call(addr);

        assert_eq!(cpu.stack[start], pc as u16);
        assert_eq!(cpu.stack_pointer, start + 1);
        assert_eq!(cpu.program_counter, addr as usize);
    }

    #[test]
    #[should_panic(expected = "Stack underflow")]
    fn ret_can_underflow_stack() {
        let mut cpu = CPUBuilder::new().build();

        cpu.ret();
        assert_eq!(false, true, "Expected the stack to underflow")
    }

    #[test]
    fn ret_sets_pointers() {
        let start = 5;
        let pc = 0x100;

        let mut cpu = CPUBuilder::new().build();
        cpu.stack_pointer = start;
        cpu.stack[start - 1] = pc;

        cpu.ret();

        assert_eq!(cpu.stack_pointer, start - 1);
        assert_eq!(cpu.program_counter, pc as usize);
    }
}
