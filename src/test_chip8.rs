// Tests
#[allow(dead_code)]
#[cfg(test)]
#[allow(non_snake_case)]
mod test_chip8 {
    use super::super::chip8::Chip8;

    const MEM_SIZE: usize = 4096;
    const START_ADDRESS: usize = 0x200;
    const FONT_SET_START_ADDRESS: usize = 0x50;
    const VIDEO_WIDTH: usize = 32;
    const VIDEO_HEIGHT: usize = 64;
    const VIDEO_SIZE: usize = VIDEO_WIDTH * VIDEO_HEIGHT;
     
    #[test]
    fn test_OP_00E0() {
        let mut chip = Chip8::new();
        Chip8::OP_00E0(&mut chip);
        let empty_video = [0 as u32; VIDEO_SIZE];

        let mut equal = true;

        for x in 0..chip.video.len() {
            if chip.video[x as usize] != empty_video[x as usize] {
                equal = false;
                break;
            }
        }

        assert!(equal);
    }

    #[test]
    fn test_OP_8xy0() {
        //init
        let mut chip = Chip8::new();
        chip.registers[0x1] = 2;
        chip.registers[0x3] = 5;
        chip.opcode = 0x8130;
        //test
        chip.OP_8xy0();

        //assert
        assert_eq!(chip.registers[0x1], chip.registers[0x3]);
    }

    #[test]
    fn test_OP_8xy1() {
        //init
        let mut chip = Chip8::new();
        chip.registers[0x1] = 0b0011;
        chip.registers[0x2] = 0b1100;
        chip.opcode = 0x8121;

        //test
        chip.OP_8xy1();

        //assert
        assert_eq!(0b1111, chip.registers[0x1]);
    }

    #[test]
    fn test_OP_8xy2() {
        //init
        let mut chip = Chip8::new();
        chip.registers[0x1] = 0b0011;
        chip.registers[0x3] = 0b0001;
        chip.opcode = 0x8132;

        //test
        chip.OP_8xy2();
        let vx = chip.registers[0x1];
        let res: u8 = 0b0001;


        //assert
        assert_eq!(vx, res);
        

    }

    #[test]
    fn test_OP_8xy3() {
        //init
        let mut chip = Chip8::new();
        chip.registers[0x1] = 0b0011;
        chip.registers[0x3] = 0b0001;
        chip.opcode = 0x8133;

        //test
        chip.OP_8xy3();
        let vx = chip.registers[0x1];
        let res: u8 = 0b0010;
        assert_eq!(vx, res);
    }

    #[test]
    fn test_OP_fx55() {
        //init
        let mut chip = Chip8::new();
        chip.registers[0x0] = 0x0;
        chip.registers[0x1] = 0xA;
        chip.opcode = 0xF355;
        chip.index = 1000;

        //test
        chip.OP_Fx55();
        let v0 = chip.registers[0x0];
        let v1 = chip.registers[0x1];
        
        let m0 = chip.memory[chip.index as usize];
        let m1 = chip.memory[chip.index as usize + 1];

        assert_eq!(v0, m0);
        assert_eq!(v1, m1);
    }

    // #[test]
    // fn test_OP_fx33() {
    //     unimplemented!();
    // }
}
