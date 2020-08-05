use coffee::graphics::{Color, Frame, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

mod chip8;

mod test_chip8;

type Chip8 = chip8::Chip8;

//Graphic setup
// const PIXEL_SIZE: usize = 8;
const WIN_WIDTH: usize = 64;
const WIN_HEIGHT: usize = 32;

fn main() -> Result<()>{
    let mut chip: Chip8 = Chip8::new();

    // load rom
    let bc_test = String::from("/home/riyan/code/chichan/src/BC_test.ch8");
    let op_test = String::from("/home/riyan/code/chichan/src/test_opcode.ch8");
    let tetris = String::from("/home/riyan/code/chichan/src/tetris.ch8");
    let rom_path = op_test;
    chip.load_rom(&rom_path);

    let mut buffer: Vec<u32> = vec![0; WIN_WIDTH * WIN_HEIGHT];

    Display::run(WindowSettings {
        title: String::from("Chi-chan"),
        size: (600, 320),
        resizable: false,
        fullscreen: false,
        maximized: false,
    })

}


struct Display {}

impl Game for Display {
    
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Display>{
        Task::succeed(|| Display {})
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer){
        frame.clear(Color::WHITE);
    }
}