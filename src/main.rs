mod display;

use chip_8::CPUBuilder;
use crate::display::Game;

fn main() {
    let mut memory = [0; 0x1000];

    // set i to 100
    memory[0x000] = 0xA1;
    memory[0x001] = 0x00;

    // call draw
    memory[0x002] = 0xD0;
    memory[0x003] = 0x15;

    // set i to 200
    memory[0x004] = 0xA2;
    memory[0x005] = 0x00;

    // move x over
    memory[0x006] = 0x60;
    memory[0x007] = 0x09;

    // call draw
    memory[0x008] = 0xD0;
    memory[0x009] = 0x15;

    // set i to 300
    memory[0x00A] = 0xA3;
    memory[0x00B] = 0x00;

    // move x over
    memory[0x00C] = 0x60;
    memory[0x00D] = 0x12;

    // call draw
    memory[0x00E] = 0xD0;
    memory[0x00F] = 0x15;

    // set i to 400
    memory[0x010] = 0xA4;
    memory[0x011] = 0x00;

    // move x over
    memory[0x012] = 0x60;
    memory[0x013] = 0x1B;

    // call draw
    memory[0x014] = 0xD0;
    memory[0x015] = 0x15;

    // set i to 500
    memory[0x016] = 0xA5;
    memory[0x017] = 0x00;

    // move x over
    memory[0x018] = 0x60;
    memory[0x019] = 0x24;

    // call draw
    memory[0x01A] = 0xD0;
    memory[0x01B] = 0x15;

    // reset x
    memory[0x01C] = 0x60;
    memory[0x01D] = 0x00;
    
    // jump to this instruction (loop forevs)
    memory[0x01E] = 0x10;
    memory[0x01F] = 0x1E;

    // D
    memory[0x100] = 0xFC; // 11111100
    memory[0x101] = 0x82; // 10000010     
    memory[0x102] = 0x81; // 10000001
    memory[0x103] = 0x82; // 10000010
    memory[0x104] = 0xFC; // 11111100

    // R
    memory[0x200] = 0xFF;
    memory[0x201] = 0x81;
    memory[0x202] = 0xFF;
    memory[0x203] = 0x84;
    memory[0x204] = 0x82;

    // A
    memory[0x300] = 0xFF;
    memory[0x301] = 0x81;
    memory[0x302] = 0xFF;
    memory[0x303] = 0x81; // 10000100
    memory[0x304] = 0x81; // 10000010

    // K
    memory[0x400] = 0x81; // 10000001
    memory[0x401] = 0x82; // 10000010
    memory[0x402] = 0xFC; // 11111100
    memory[0x403] = 0x82; // 10000010
    memory[0x404] = 0x81; // 10000001

    // E
    memory[0x500] = 0xFF; // 11111111
    memory[0x501] = 0x08; // 10000000
    memory[0x502] = 0xFF; // 11111111
    memory[0x503] = 0x08; // 10000000
    memory[0x504] = 0xFF; // 11111111

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