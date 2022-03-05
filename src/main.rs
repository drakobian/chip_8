mod display;

use chip_8::CPUBuilder;
use crate::display::Game;

use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("./roms/sierpinski.ch8")?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    let mut memory = [0; 0x1000];
    
    // Read file into vector.
    reader.read_to_end(&mut buffer)?;
    
    for (ind, value) in buffer.iter().enumerate() {
        memory[ind] = *value;
    }

    let cpu = CPUBuilder::new().memory(memory).build();
    let mut game = Game::new(cpu);
    game.run();

    Ok(())
}