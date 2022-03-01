mod display;

use chip_8::CPUBuilder;
use crate::display::Game;

fn main() {
    let mut memory = [0; 0x1000];

    // set i to 'A' reserved character
    memory[0x000] = 0xA0;
    memory[0x001] = 0x32;

    // call draw
    memory[0x002] = 0xD0;
    memory[0x003] = 0x15;

    // set i to 'B'
    memory[0x004] = 0xA0;
    memory[0x005] = 0x37;

    // move x over
    memory[0x006] = 0x60;
    memory[0x007] = 0x09;

    // call draw
    memory[0x008] = 0xD0;
    memory[0x009] = 0x15;

    // set i to 'C'
    memory[0x00A] = 0xA0;
    memory[0x00B] = 0x3C;

    // move x over
    memory[0x00C] = 0x60;
    memory[0x00D] = 0x12;

    // call draw
    memory[0x00E] = 0xD0;
    memory[0x00F] = 0x15;

    // set i to 'D'
    memory[0x010] = 0xA0;
    memory[0x011] = 0x41;

    // move x over
    memory[0x012] = 0x60;
    memory[0x013] = 0x1B;

    // call draw
    memory[0x014] = 0xD0;
    memory[0x015] = 0x15;

    // set i to 'E'
    memory[0x016] = 0xA0;
    memory[0x017] = 0x46;

    // move x over
    memory[0x018] = 0x60;
    memory[0x019] = 0x24;

    // call draw
    memory[0x01A] = 0xD0;
    memory[0x01B] = 0x15;

    // jump to this instruction (loop forevs)
    // need to jump to the address + 200 to account for reserved........
    memory[0x01C] = 0x10;
    memory[0x01D] = 0xE4;

    let cpu = CPUBuilder::new().memory(memory).build();

    //cpu.run();
    let mut game = Game::new(cpu);
    game.run();
    //Game::run(cpu);
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