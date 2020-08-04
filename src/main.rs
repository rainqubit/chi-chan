use minifb::{Key, KeyRepeat, Window, WindowOptions, Scale, ScaleMode};
mod chip8;

type Chip8 = chip8::Chip8;

//Graphic setup
const PIXEL_SIZE: usize = 8;
const WIN_WIDTH: usize = 64;
const WIN_HEIGHT: usize = 64;

fn main() {
    let mut chip: Chip8 = Chip8::new();

    // load rom
    let rom_path = String::from("/home/riyan/code/chichan/src/test_opcode.ch8");
    chip.load_rom(&rom_path);
    for x in 0x200..chip.memory.len(){
        print!("|{:X?}|", chip.memory[x as usize]);
    }

    let mut buffer: Vec<u32> = vec![0; WIN_WIDTH * WIN_HEIGHT];

    let mut window_option = WindowOptions::default();
    window_option.scale = Scale::X8;
    window_option.scale_mode = ScaleMode::AspectRatioStretch;

    let mut window = Window::new("Chi-chan", WIN_WIDTH, WIN_HEIGHT, window_option)
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    //window.limit_update_rate(Some(std::time::Duration::from_micros(600000)));

    // main loop
    while window.is_open() && !window.is_key_down(Key::Escape){
        process_input(&window, &mut chip);

        for  p in 0..chip.video.len(){
            buffer[p as usize] = chip.video[p as usize];
        }

        //paint empty background
        for p in buffer.iter_mut(){
            if *p == 0{
                *p = 0x0;
            }
        }

        window
         .update_with_buffer(&buffer, WIN_WIDTH, WIN_HEIGHT)
         .unwrap();
        
         chip.cycle();
    }

}

fn process_input(window: &minifb::Window, chip: &mut Chip8){

    // pressed
    window.get_keys_pressed(KeyRepeat::No).map(|keys| {
        for k in keys {
            match k {
                Key::X => chip.keypad[0] = 1,
                Key::Key1 => chip.keypad[1] = 1,
                Key::Key2 => chip.keypad[2] = 1,
                Key::Key3 => chip.keypad[3] = 1,
                Key::Q => chip.keypad[4] = 1,
                Key::W => chip.keypad[5] = 1,
                Key::E => chip.keypad[6] = 1,
                Key::A => chip.keypad[7] = 1,
                Key::S => chip.keypad[8] = 1,
                Key::D => chip.keypad[9] = 1,
                Key::Z => chip.keypad[0xA] = 1,
                Key::C => chip.keypad[0xB] = 1,
                Key::Key4 => chip.keypad[0xC] = 1,
                Key::R => chip.keypad[0xD] = 1,
                Key::F => chip.keypad[0xE] = 1,
                Key::V => chip.keypad[0xF] = 1,
                _ => (),
            }
        }
    });

    //release
    window.get_keys_released().map(|keys| {
        for k in keys {
            match k {
                Key::X => chip.keypad[0] = 0,
                Key::Key1 => chip.keypad[1] = 0,
                Key::Key2 => chip.keypad[2] = 0,
                Key::Key3 => chip.keypad[3] = 0,
                Key::Q => chip.keypad[4] = 0,
                Key::W => chip.keypad[5] = 0,
                Key::E => chip.keypad[6] = 0,
                Key::A => chip.keypad[7] = 0,
                Key::S => chip.keypad[8] = 0,
                Key::D => chip.keypad[9] = 0,
                Key::Z => chip.keypad[0xA] = 0,
                Key::C => chip.keypad[0xB] = 0,
                Key::Key4 => chip.keypad[0xC] = 0,
                Key::R => chip.keypad[0xD] = 0,
                Key::F => chip.keypad[0xE] = 0,
                Key::V => chip.keypad[0xF] = 0,
                _ => (),
            }
        }
    });
}
