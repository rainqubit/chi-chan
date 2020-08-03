mod chip8;

type Chip8 = chip8::Chip8;

fn main() {
    let chip: Chip8 = Chip8::new();

    //memory print test
    for x in 0x50..0xA0 {
        println!("{}", chip.memory[x]);
    }
}