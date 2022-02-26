use chip_8::CPUBuilder;

fn main() {
    let mut memory = [0; 0x1000];

    // set i to 100
    memory[0x000] = 0xA1;
    memory[0x001] = 0x00;

    // call draw
    memory[0x002] = 0xD0;
    memory[0x003] = 0x15;

    memory[0x100] = 0xFF;
    memory[0x101] = 0x81;
    memory[0x102] = 0xFF;
    memory[0x103] = 0x81;
    memory[0x104] = 0x81;

    let mut cpu = CPUBuilder::new().memory(memory).build();

    cpu.run();
}
/*fn main() {
    let mut registers = [0; 16];
    registers[0] = 5;
    registers[1] = 10;

    let mut memory = [0; 0x1000];
    memory[0x000] = 0x21;
    memory[0x001] = 0x00;
    memory[0x002] = 0x21;
    memory[0x003] = 0x00;
    memory[0x004] = 0x00;
    memory[0x005] = 0x00;

    memory[0x100] = 0x80;
    memory[0x101] = 0x14;
    memory[0x102] = 0x80;
    memory[0x103] = 0x14;
    memory[0x104] = 0x00;
    memory[0x105] = 0xEE;

    let mut cpu = CPUBuilder::new()
        .registers(registers)
        .memory(memory)
        .build();

    cpu.run();

    assert_eq!(cpu.registers(0), 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers(0));
}
*/