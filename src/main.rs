use coffee::graphics::{Color, Frame, Window, WindowSettings, Rectangle, Shape, Mesh};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

mod chip8;

mod test_chip8;

type Chip8 = chip8::Chip8;

//Graphic setup
// const PIXEL_SIZE: usize = 8;
const WIN_WIDTH: usize = 64;
const WIN_HEIGHT: usize = 32;
const VIDEO_WIDTH: usize = 32;
const VIDEO_HEIGHT: usize = 64;

fn main() -> Result<()>{
    // let mut chip: Chip8 = Chip8::new();

    // load rom
    // let bc_test = String::from("/home/riyan/code/chichan/src/BC_test.ch8");
    // let op_test = String::from("/home/riyan/code/chichan/src/test_opcode.ch8");
    // let tetris = String::from("/home/riyan/code/chichan/src/tetris.ch8");
    // let rom_path = op_test;
    // chip.load_rom(&rom_path);

    // let mut buffer: Vec<u32> = vec![0; WIN_WIDTH * WIN_HEIGHT];

    Display::run(WindowSettings {
        title: String::from("Chi-chan"),
        size: (600, 320),
        resizable: false,
        fullscreen: false,
        maximized: false,
    })

}


struct Display {
    chip: Chip8,
}

impl Game for Display {
    
    type Input = ();
    type LoadingScreen = ();
    const TICKS_PER_SECOND: u16 = 20;

    fn load(_window: &Window) -> Task<Display>{
        let mut chip = Chip8::new();
        let _bc_test = String::from("/home/riyan/code/chichan/src/BC_test.ch8");
        let _op_test = String::from("/home/riyan/code/chichan/src/test_opcode.ch8");
        let _tetris = String::from("/home/riyan/code/chichan/src/tetris.ch8");
        let rom_path = _bc_test;

        chip.load_rom(&rom_path);

        Task::succeed(|| 
            Display {
            chip: chip,
        })
    }

    fn update(&mut self, _window: &Window){
        self.chip.cycle();
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer){
        frame.clear(Color::BLACK);

        let mut mesh = Mesh::new_with_tolerance(0.0);

        let chip_buffer = self.chip.video_to_2d();
        let pixel_scale = 8;
        let black = Color::new(0.0, 0.0, 0.0, 1.0);
        let white = Color::new(1.0, 1.0, 1.0, 1.0);

        for y in 0..VIDEO_HEIGHT {
            for x in 0..VIDEO_WIDTH {
                let pixel = chip_buffer[x][y];

                let rect = Shape::Rectangle(Rectangle {
                    x: (x * pixel_scale) as f32,
                    y: (y * pixel_scale) as f32,
                    height: pixel_scale as f32,
                    width: pixel_scale as f32,
                });

                if pixel > 0 {
                    mesh.fill(rect, white);
                } else {
                    mesh.fill(rect, black);
                }
            }
        }

        mesh.draw(&mut frame.as_target());

    }
}