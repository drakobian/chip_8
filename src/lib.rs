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

use rand::Rng;

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
    i: Address,
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

    /// Set registers on the builder
    pub fn registers(&mut self, registers: Registers) -> &mut CPUBuilder {
        self.registers = Some(registers);
        self
    }

    /// Set memory on the builder
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
        // todo: update memory to reserve 0x000 to 0x1FF for interpreter
        // and store some character sprites
        let memory = self.get_memory();

        CPU {
            program_counter: 200,
            registers: self.registers.unwrap_or([0; 16]),
            memory,
            stack: [0; 16],
            stack_pointer: 0,
            i: 0,
        }
    }

    // todo: pull the reserved characters into a separate file
    fn get_memory(&self) -> Memory {
        let mut memory = [0; 0x1000];

        // populate memory w/ reserved characters
        memory[0] = 0xF0;
        memory[1] = 0x90;
        memory[2] = 0x90;
        memory[3] = 0x90;
        memory[4] = 0xF0;

        memory[5] = 0x20;
        memory[6] = 0x60;
        memory[7] = 0x20;
        memory[8] = 0x20;
        memory[9] = 0x70;

        memory[10] = 0xF0;
        memory[11] = 0x10;
        memory[12] = 0xF0;
        memory[13] = 0x80;
        memory[14] = 0xF0;

        memory[15] = 0xF0;
        memory[16] = 0x10;
        memory[17] = 0xF0;
        memory[18] = 0x10;
        memory[19] = 0xF0;

        memory[20] = 0x90;
        memory[21] = 0x90;
        memory[22] = 0xF0;
        memory[23] = 0x10;
        memory[24] = 0x10;

        memory[25] = 0xF0;
        memory[26] = 0x80;
        memory[27] = 0xF0;
        memory[28] = 0x10;
        memory[29] = 0xF0;

        memory[30] = 0xF0;
        memory[31] = 0x80;
        memory[32] = 0xF0;
        memory[33] = 0x90;
        memory[34] = 0xF0;

        memory[35] = 0xF0;
        memory[36] = 0x10;
        memory[37] = 0x20;
        memory[38] = 0x40;
        memory[39] = 0x40;      

        memory[40] = 0xF0;
        memory[41] = 0x90;
        memory[42] = 0xF0;
        memory[43] = 0x90;
        memory[44] = 0xF0;

        memory[45] = 0xF0;
        memory[46] = 0x90;
        memory[47] = 0xF0;
        memory[48] = 0x10;
        memory[49] = 0xF0;

        memory[50] = 0xF0;
        memory[51] = 0x90;
        memory[52] = 0xF0;
        memory[53] = 0x90;
        memory[54] = 0x90;

        memory[55] = 0xE0;
        memory[56] = 0x90;
        memory[57] = 0xE0;
        memory[58] = 0x90;
        memory[59] = 0xE0;

        memory[60] = 0xF0;
        memory[61] = 0x80;
        memory[62] = 0x80;
        memory[63] = 0x80;
        memory[64] = 0xF0;

        memory[65] = 0xE0;
        memory[66] = 0x90;
        memory[67] = 0x90;
        memory[68] = 0x90;
        memory[69] = 0xE0;

        memory[70] = 0xF0;
        memory[71] = 0x80;
        memory[72] = 0xF0;
        memory[73] = 0x80;
        memory[74] = 0xF0;

        memory[75] = 0xF0;
        memory[76] = 0x80;
        memory[77] = 0xF0;
        memory[78] = 0x80;
        memory[79] = 0x80;

        // some interpreter memory is open :)

        // populate rest of memory if any memory was passed in
        for i in 200..4096 {
            memory[i] = self.memory.unwrap_or([0; 4096])[i - 200];
        }

        memory
    }
}

impl CPU {
    // TODO: add some simple doc examples for doctests
    /// Runs the program set in memory according to the CHIP-8 spec
    pub fn run(&mut self, screen: &mut [[bool; 64]; 32]) -> Option<()> {
        //loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;

            let c = ((opcode & 0xF000) >> 12) as Byte;
            let x = ((opcode & 0x0F00) >> 08) as Byte;
            let y = ((opcode & 0x00F0) >> 04) as Byte;
            let d = ((opcode & 0x000F) >> 00) as Byte;
            let nnn = opcode & 0x0FFF;
            let nn = opcode & 0x00FF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => return None,
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x1, _, _, _) => self.jump(nnn),
                (0x2, _, _, _) => self.call(nnn),
                (0x3, _, _, _) => self.skip_equal(x, nn),
                (0x4, _, _, _) => self.skip_not_equal(x, nn),
                (0x5, _, _, _) => self.skip_equal_reg(x, y),
                (0x6, _, _, _) => self.set_register(x, nn),
                (0x7, _, _, _) => self.add(x, nn),
                (0x8, _, _, 0) => self.assign(x, y),
                (0x8, _, _, 0x1) => self.or(x, y),
                (0x8, _, _, 0x2) => self.and(x, y),
                (0x8, _, _, 0x3) => self.xor(x, y),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                (0x8, _, _, 0x5) => self.sub_xy(x, y),
                (0x8, _, _, 0x6) => self.shift_right(x),
                (0x8, _, _, 0x7) => self.sub_n(x, y),
                (0x8, _, _, 0xE) => self.shift_left(x),
                (0x9, _, _, 0) => self.skip_not_equal_reg(x, y),
                (0xA, _, _, _) => self.set_i(nnn),
                (0xB, _, _, _) => self.jump_reg(nnn),
                (0xC, _, _, _) => self.rand(nn),
                (0xF, _, 0x1, 0xE) => self.set_i_reg(x),
                (0xF, _, 0x3, 0x3) => self.bcd(x),
                (0xF, _, 0x5, 0x5) => self.reg_dump(x),
                (0xF, _, 0x6, 0x5) => self.reg_load(x),
                (0xD, _, _, _) => self.draw(x, y, d, screen),
                _ => todo!("opcode {:04x}", opcode),
            };
        Some(())
        //}
    }

    /// Draws a sprite at coordinate (VX, VY) that has a width 
    /// of 8 pixels and a height of N pixels. Each row of 8 pixels 
    /// is read as bit-coded starting from memory location I; I value 
    /// does not change after the execution of this instruction. As 
    /// described above, VF is set to 1 if any screen pixels are flipped 
    /// from set to unset when the sprite is drawn, and to 0 if that does not happen

    // todo: implement wrapping for indices outside of screen (? not sure if needed)
    fn draw(&mut self, x: Byte, y: Byte, d: Byte, screen: &mut [[bool; 64]; 32]) {
        let bits = self.get_display_bits(d);
        let x_coord = self.registers[x as usize] as usize;
        let y_coord = self.registers[y as usize] as usize;

        // we have a vec of byte strings to write, and we know the coordinate
        // (vx, vy) to start at.

        // so for each byte string in bits
            // and for each character in each byte string
                // update screen accordingly..
        // byte_string_ind indicates which row we're on
        let mut flip_vf = false;
        for (byte_string_ind, byte_string) in bits.iter().enumerate() {
            // and char_ind indicates column
            for (char_ind, char) in byte_string.chars().enumerate() {
                let previous = screen[y_coord + byte_string_ind][x_coord + char_ind]; 

                if char == '1' {
                    screen[y_coord + byte_string_ind][x_coord + char_ind] ^= true;
                } else {
                    screen[y_coord + byte_string_ind][x_coord + char_ind] ^= false;
                }

                // if a bit was set before, and just got unset, need to flip vf at end
                if previous && !screen[y_coord + byte_string_ind][x_coord + char_ind] {
                    flip_vf = true;
                }
            }
        }

        if flip_vf {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    /// Gets the bytes required for `draw` and returns as bit strings
    // Todo: make this more rusty! (will Clippy help?)
    // Pretty sure could do this in a more functional/iterator style
    fn get_display_bits(&self, d: Byte) -> Vec<String> {
        let mut bits = vec![];

        for i in 0..(d as usize) {
            let byte = self.memory[self.i as usize + i];
            bits.push(format!("{:b}", byte));
        }

        bits
    }

    /// Returns the next two bytes of memory concatenated as a u16
    fn read_opcode(&self) -> OpCode {
        let p = self.program_counter;
        let byte1 = self.memory[p] as OpCode;
        let byte2 = self.memory[p + 1] as OpCode;
        byte1 << 8 | byte2
    }

    /// Moves the program_counter to the given address
    fn jump(&mut self, addr: Address) {
        self.program_counter = addr as usize;
    }

    /// Moves the program_counter to the given address + registers[0]
    fn jump_reg(&mut self, addr: Address) {
        self.program_counter = self.registers[0] as usize + addr as usize;
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
    /// If this operation overflows the register size, the borrow register
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

    /// Decrements the value in register `x` by the value in register `y`
    ///
    /// If this operation _does not_ underflow the register, the 'borrow' register
    ///
    /// `0xF` is set to `1`
    fn sub_xy(&mut self, x: Byte, y: Byte) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, overflow) = arg1.overflowing_sub(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 0;
        } else {
            self.registers[0xF] = 1;
        }
    }

    /// Sets register[x] = register[y] - register[x]
    ///
    /// If this operation _does not_ underflow the register, the 'borrow' register
    ///
    /// `0xF` is set to `1`
    fn sub_n(&mut self, x: Byte, y: Byte) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, overflow) = arg2.overflowing_sub(arg1);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 0;
        } else {
            self.registers[0xF] = 1;
        }
    }

    /// Skips the next instruction if registers[x] equals NN
    fn skip_equal(&mut self, x: Byte, nn: u16) {
        if self.registers[x as usize] == nn as Byte {
            self.program_counter += 2;
        }
    }

    /// Skips the next instruction if registers[x] does not equal NN
    fn skip_not_equal(&mut self, x: Byte, nn: u16) {
        if self.registers[x as usize] != nn as Byte {
            self.program_counter += 2;
        }
    }

    /// Skips the next instruction if registers[x] equals registers[y]
    fn skip_equal_reg(&mut self, x: Byte, y: Byte) {
        if self.registers[x as usize] == self.registers[y as usize] {
            self.program_counter += 2;
        }
    }

    /// Skips the next instruction if registers[x] does not equal registers[y]
    fn skip_not_equal_reg(&mut self, x: Byte, y: Byte) {
        if self.registers[x as usize] != self.registers[y as usize] {
            self.program_counter += 2;
        }
    }

    /// Sets registers[x] to nn
    fn set_register(&mut self, x: Byte, nn: u16) {
        self.registers[x as usize] = nn as Byte;
    }

    /// Adds nn to register[x]
    fn add(&mut self, x: Byte, nn: u16) {
        // TODO: handle overflow?
        self.registers[x as usize] += nn as u8;
    }

    /// Sets register[x] to the value in register[y]
    fn assign(&mut self, x: Byte, y: Byte) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    /// Sets register[x] to register[x] bitwise OR register[y]
    fn or(&mut self, x: Byte, y: Byte) {
        self.registers[x as usize] |= self.registers[y as usize];
    }

    /// Sets register[x] to register[x] bitwise AND register[y]
    fn and(&mut self, x: Byte, y: Byte) {
        self.registers[x as usize] &= self.registers[y as usize];
    }

    /// Sets register[x] to register[x] bitwise XOR register[y]
    fn xor(&mut self, x: Byte, y: Byte) {
        self.registers[x as usize] ^= self.registers[y as usize];
    }

    /// Stores the least signifcant bit of register[x] in the borrow register
    /// 
    /// and then shifts register[x] right 1
    fn shift_right(&mut self, x: Byte) {
        let least_sig = self.registers[x as usize] & 0b00000001;
        self.registers[0xF] = least_sig;
        self.registers[x as usize] >>= 1;
    }

    /// Stores the most signifcant bit of register[x] in the borrow register
    /// 
    /// and then shifts register[x] right 1
    fn shift_left(&mut self, x: Byte) {
        let most_sig = self.registers[x as usize] & 0b10000000;
        self.registers[0xF] = most_sig >> 7;
        self.registers[x as usize] <<= 1;
    }

    /// Sets the I register
    fn set_i(&mut self, addr: Address) {
        self.i = addr;
    }

    /// Sets the I register from another register
    fn set_i_reg(&mut self, x: Byte) {
        self.i += self.registers[x as usize] as u16;
    }

    /// Sets v0 to some random number (1-255) AND nn
    fn rand(&mut self, nn: u16) {
        let mut rng = rand::thread_rng();
        self.registers[0] = (nn & rng.gen_range(1.0..256.0) as u16) as u8;
    }

    /// Stores from V0 to VX (including VX) in memory, starting at address I
    fn reg_dump(&mut self, x: Byte) {
        for ind in 0..=(x as usize) {
            self.memory[self.i as usize + ind] = self.registers[ind];
        }
    }

    /// Fills from V0 to VX (including VX) in memory, starting at address I
    fn reg_load(&mut self, x: Byte) {
        for ind in 0..=(x as usize) {
            self.registers[ind] = self.memory[self.i as usize + ind];
        }
    }

    /// Stores the binary-coded decimal representation of VX in memory starting at address I
    fn bcd(&mut self, x: Byte) {
        let hundreds = self.registers[x as usize] / 100;
        let tens = (self.registers[x as usize] / 10) % 10;
        let ones = self.registers[x as usize] % 10;

        self.memory[self.i as usize + 0] = hundreds as Byte;
        self.memory[self.i as usize + 1] = tens as Byte;
        self.memory[self.i as usize + 2] = ones as Byte; 
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
    fn jump_sets_program_counter() {
        let mut cpu = CPUBuilder::new().build();
        cpu.program_counter = 0x100;
        cpu.jump(0x200);

        assert_eq!(cpu.program_counter, 0x200);
    }

    #[test]
    fn jump_reg_sets_program_counter() {
        let mut cpu = CPUBuilder::new().build();
        cpu.registers[0] = 0x0FF;
        cpu.jump_reg(0x100);

        assert_eq!(cpu.program_counter, 0x1FF);
    }

    #[test]
    fn skip_equal_sets_program_counter_when_equal() {
        let mut cpu = CPUBuilder::new().build();
        cpu.program_counter = 0x100;
        cpu.registers[2] = 8;
        cpu.skip_equal(2, 8);

        assert_eq!(cpu.program_counter, 0x102);
    }

    #[test]
    fn skip_equal_continues_when_not_equal() {
        let mut cpu = CPUBuilder::new().build();
        cpu.program_counter = 0x100;
        cpu.registers[2] = 7;
        cpu.skip_equal(2, 8);

        assert_eq!(cpu.program_counter, 0x100);
    }

    #[test]
    fn skip_not_equal_continues_when_equal() {
        let mut cpu = CPUBuilder::new().build();
        cpu.program_counter = 0x100;
        cpu.registers[2] = 8;
        cpu.skip_not_equal(2, 8);

        assert_eq!(cpu.program_counter, 0x100);
    }

    #[test]
    fn skip_not_equal_sets_program_counter_when_not_equal() {
        let mut cpu = CPUBuilder::new().build();
        cpu.program_counter = 0x100;
        cpu.registers[2] = 7;
        cpu.skip_not_equal(2, 8);

        assert_eq!(cpu.program_counter, 0x102);
    }

    #[test]
    fn skip_equal_reg_sets_program_counter_when_equal() {
        let mut cpu = CPUBuilder::new().build();
        cpu.program_counter = 0x100;
        cpu.registers[2] = 8;
        cpu.registers[7] = 8;
        cpu.skip_equal_reg(2, 7);

        assert_eq!(cpu.program_counter, 0x102);
    }

    #[test]
    fn skip_equal_reg_continues_when_not_equal() {
        let mut cpu = CPUBuilder::new().build();
        cpu.program_counter = 0x100;
        cpu.registers[2] = 7;
        cpu.registers[7] = 2;
        cpu.skip_equal_reg(2, 7);

        assert_eq!(cpu.program_counter, 0x100);
    }

    #[test]
    fn skip_not_equal_reg_continues_when_equal() {
        let mut cpu = CPUBuilder::new().build();
        cpu.program_counter = 0x100;
        cpu.registers[2] = 8;
        cpu.registers[7] = 8;
        cpu.skip_not_equal_reg(2, 7);

        assert_eq!(cpu.program_counter, 0x100);
    }

    #[test]
    fn skip_not_equal_reg_sets_program_counter_when_not_equal() {
        let mut cpu = CPUBuilder::new().build();
        cpu.program_counter = 0x100;
        cpu.registers[2] = 7;
        cpu.registers[7] = 2;
        cpu.skip_not_equal_reg(2, 7);

        assert_eq!(cpu.program_counter, 0x102);
    }

    #[test]
    fn set_register_sets_register() {
        let mut cpu = CPUBuilder::new().build();
        cpu.set_register(2, 7);

        assert_eq!(cpu.registers[2], 7);
    }

    #[test]
    fn add_increments_register() {
        let mut cpu = CPUBuilder::new().build();
        cpu.add(3, 5);
        cpu.add(3, 1);

        assert_eq!(cpu.registers[3], 6);
    }

    #[test]
    fn assign_sets_register_from_other_register() {
        let mut cpu = CPUBuilder::new().build();
        cpu.registers[1] = 6;
        cpu.registers[10] = 4;
        cpu.assign(1, 10);

        assert_eq!(cpu.registers[1], 4);
        assert_eq!(cpu.registers[10], 4);
    }

    #[test]
    fn or_sets_register_from_other_register() {
        let mut cpu = CPUBuilder::new().build();
        cpu.registers[2] = 0x001;
        cpu.registers[5] = 0x010;
        cpu.or(2, 5);

        assert_eq!(cpu.registers[2], 0x011);
        assert_eq!(cpu.registers[5], 0x010);
    }

    #[test]
    fn and_sets_register_from_other_register() {
        let mut cpu = CPUBuilder::new().build();
        cpu.registers[2] = 0x011;
        cpu.registers[5] = 0x010;
        cpu.and(2, 5);

        assert_eq!(cpu.registers[2], 0x010);
        assert_eq!(cpu.registers[5], 0x010);
    }

    #[test]
    fn xor_sets_register_from_other_register() {
        let mut cpu = CPUBuilder::new().build();
        cpu.registers[2] = 0x011;
        cpu.registers[5] = 0x010;
        cpu.xor(2, 5);

        assert_eq!(cpu.registers[2], 0x001);
        assert_eq!(cpu.registers[5], 0x010);
    }

    #[test]
    fn sub_xy_subtracts_registers_no_underflow() {
        let mut registers = [0; 16];
        registers[0] = 5;
        registers[1] = 3;
        let mut cpu = CPUBuilder::new().registers(registers).build();
        cpu.sub_xy(0, 1);

        assert_eq!(2, cpu.registers(0));
        assert_eq!(3, cpu.registers(1));
        assert_eq!(1, cpu.registers(15));
    }

    #[test]
    fn sub_xy_subtracts_registers_underflow() {
        let mut registers = [0; 16];
        registers[0] = 0;
        registers[1] = 1;
        let mut cpu = CPUBuilder::new().registers(registers).build();
        cpu.sub_xy(0, 1);

        assert_eq!(255, cpu.registers(0));
        assert_eq!(0, cpu.registers(15));
    }

    #[test]
    fn shift_right_halves_register_and_stores_in_borrow_register() {
        let mut cpu = CPUBuilder::new().build();
        cpu.registers[3] = 0x011;
        cpu.registers[5] = 0x0F0;

        cpu.shift_right(3);
        assert_eq!(cpu.registers[3], 0x008);
        assert_eq!(cpu.registers[0xF], 1);

        cpu.shift_right(5);
        assert_eq!(cpu.registers[5], 0x078);
        assert_eq!(cpu.registers[0xF], 0);
    }

    #[test]
    fn shift_left_doubles_register_and_stores_in_borrow_register() {
        let mut cpu = CPUBuilder::new().build();
        cpu.registers[3] = 0b01111111;

        cpu.shift_left(3);
        assert_eq!(cpu.registers[3], 0b11111110);
        assert_eq!(cpu.registers[0xF], 0);
    }

    #[test]
    fn subn_subtracts_registers_no_borrow() {
        let mut cpu = CPUBuilder::new().build();
        cpu.registers[5] = 9;
        cpu.registers[2] = 10;
        cpu.sub_n(5, 2);

        assert_eq!(cpu.registers[5], 1);
        assert_eq!(cpu.registers[0xF], 1);
    }

    #[test]
    fn subn_subtracts_registers_with_borrow() {
        let mut cpu = CPUBuilder::new().build();
        cpu.registers[5] = 1;
        cpu.registers[2] = 0;
        cpu.sub_n(5, 2);

        assert_eq!(cpu.registers[5], 255);
        assert_eq!(cpu.registers[0xF], 0);
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

    #[test]
    fn set_i_sets_i_register() {
        let mut cpu = CPUBuilder::new().build();
        cpu.set_i(512);

        assert_eq!(cpu.i, 512);
    }

    #[test]
    fn rand_sets_first_register() {
        let mut cpu = CPUBuilder::new().build();
        cpu.rand(0xFF);

        assert_ne!(cpu.registers[0], 0);
    }

    #[test]
    fn set_i_reg_sets_i_from_register() {
        let mut cpu = CPUBuilder::new().build();
        cpu.registers[4] = 18;
        cpu.i = 22;
        cpu.set_i_reg(4);

        assert_eq!(cpu.i, 40);
    }

    #[test]
    fn reg_dump_sets_memory_from_registers() {
        let mut cpu = CPUBuilder::new().build();
        cpu.i = 0x100;
        cpu.registers[0] = 0x80;
        cpu.registers[1] = 0x14;
        cpu.registers[2] = 0x77;
        cpu.registers[3] = 0xEE;

        cpu.reg_dump(2);
        assert_eq!(cpu.memory[0x100], 0x80);
        assert_eq!(cpu.memory[0x101], 0x14);
        assert_eq!(cpu.memory[0x102], 0x77);
        assert_eq!(cpu.memory[0x103], 0);

        cpu.reg_dump(3);
        assert_eq!(cpu.memory[0x100], 0x80);
        assert_eq!(cpu.memory[0x101], 0x14);
        assert_eq!(cpu.memory[0x102], 0x77);
        assert_eq!(cpu.memory[0x103], 0xEE);
    }

    #[test]
    fn reg_load_sets_registers_from_memory() {
        let mut cpu = CPUBuilder::new().build();
        cpu.i = 0x100;
        cpu.memory[0x100] = 0x80;
        cpu.memory[0x101] = 0x14;
        cpu.memory[0x102] = 0x77;
        cpu.memory[0x103] = 0xEE;

        cpu.reg_load(2);
        assert_eq!(cpu.registers[0], 0x80);
        assert_eq!(cpu.registers[1], 0x14);
        assert_eq!(cpu.registers[2], 0x77);
        assert_eq!(cpu.registers[3], 0);

        cpu.reg_load(3);
        assert_eq!(cpu.registers[0], 0x80);
        assert_eq!(cpu.registers[1], 0x14);
        assert_eq!(cpu.registers[2], 0x77);
        assert_eq!(cpu.registers[3], 0xEE);
    }

    #[test]
    fn bcd_sets_memory_from_binary_coded_register() {
        let mut cpu = CPUBuilder::new().build();
        cpu.registers[3] = 213;
        cpu.registers[7] = 176;
        cpu.registers[11] = 54;
        cpu.registers[13] = 1;

        cpu.i = 0x100;
        cpu.bcd(3);
        assert_eq!(cpu.memory[cpu.i as usize + 0], 2);
        assert_eq!(cpu.memory[cpu.i as usize + 1], 1);
        assert_eq!(cpu.memory[cpu.i as usize + 2], 3);

        cpu.i = 0x120;
        cpu.bcd(7);
        assert_eq!(cpu.memory[cpu.i as usize + 0], 1);
        assert_eq!(cpu.memory[cpu.i as usize + 1], 7);
        assert_eq!(cpu.memory[cpu.i as usize + 2], 6);

        cpu.i = 0x140;
        cpu.bcd(11);
        assert_eq!(cpu.memory[cpu.i as usize + 0], 0);
        assert_eq!(cpu.memory[cpu.i as usize + 1], 5);
        assert_eq!(cpu.memory[cpu.i as usize + 2], 4);

        cpu.i = 0x160;
        cpu.bcd(13);
        assert_eq!(cpu.memory[cpu.i as usize + 0], 0);
        assert_eq!(cpu.memory[cpu.i as usize + 1], 0);
        assert_eq!(cpu.memory[cpu.i as usize + 2], 1);
    }

    #[test]
    fn get_display_bits_reads_from_memory_as_bits() {
        let mut cpu = CPUBuilder::new().build();
        cpu.i = 0x100;
        cpu.memory[0x100] = 0xFF;
        cpu.memory[0x101] = 0x81;
        cpu.memory[0x102] = 0xFF;
        cpu.memory[0x103] = 0x81;
        cpu.memory[0x104] = 0x81;
        let bits = cpu.get_display_bits(5);

        assert_eq!(bits, vec![
            String::from("11111111"),
            String::from("10000001"),
            String::from("11111111"),
            String::from("10000001"),
            String::from("10000001"),
        ]);
    }

    // Todo: maybe find a way to unit test display opcodes
}
