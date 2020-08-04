use rand::Rng;
use std::fs;

const MEM_SIZE: usize = 4096;
const START_ADDRESS: usize = 0x200;
const FONT_SET_START_ADDRESS: usize = 0x50;
const VIDEO_WIDTH: usize = 64;
const VIDEO_HEIGHT: usize = 32;
const VIDEO_SIZE: usize = VIDEO_WIDTH * VIDEO_HEIGHT;

type Memory = [u8; MEM_SIZE];
type Video = [u32; VIDEO_SIZE];

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct Chip8 {
    pub memory: Memory,
    pub registers: [u8; 16],
    pub index: u16,
    pub pc: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keypad: [u8; 16],
    pub video: Video,
    pub opcode: u16,
    table: Vec<for<'r> fn(&'r mut Chip8)>,
    table0: Vec<for<'r> fn(&'r mut Chip8)>,
    table8: Vec<for<'r> fn(&'r mut Chip8)>,
    tableE: Vec<for<'r> fn(&'r mut Chip8)>,
    tableF: Vec<for<'r> fn(&'r mut Chip8)>,

}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Chip8 {
    pub fn new() -> Self {
        let font_set: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        // init empty memory
        let mut memory: Memory = [0; MEM_SIZE];

        // load the font set into memory
        Self::load_font_set(&mut memory, &font_set);

        // instructions tables
        let mut table: Vec<for<'r> fn(&'r mut Chip8) > = vec!(Self::OP_NULL; 32);
        let mut table0: Vec<for<'r> fn(&'r mut Chip8) > = vec!(Self::OP_NULL; 16);
        let mut table8: Vec<for<'r> fn(&'r mut Chip8) > = vec!(Self::OP_NULL; 16);
        let mut tableE: Vec<for<'r> fn(&'r mut Chip8) > = vec!(Self::OP_NULL; 16);
        let mut tableF: Vec<for<'r> fn(&'r mut Chip8) > = vec!(Self::OP_NULL; 128);

        table[0x0] = Chip8::table0;
		table[0x1] = Chip8::OP_1nnn;
		table[0x2] = Chip8::OP_2nnn;
		table[0x3] = Chip8::OP_3xkk;
		table[0x4] = Chip8::OP_4xkk;
		table[0x5] = Chip8::OP_5xy0;
		table[0x6] = Chip8::OP_6xkk;
		table[0x7] = Chip8::OP_7xkk;
		table[0x8] = Chip8::table8;
		table[0x9] = Chip8::OP_9xy0;
		table[0xA] = Chip8::OP_Annn;
		table[0xB] = Chip8::OP_Bnnn;
		table[0xC] = Chip8::OP_Cxkk;
		table[0xD] = Chip8::OP_Dxyn;
		table[0xE] = Chip8::tableE;
        table[0xF] = Chip8::tableF;
        
        table0[0x0] = Chip8::OP_00E0;
        table0[0xE] = Chip8::OP_00EE;
        
        table8[0x0] = Chip8::OP_8xy0;
		table8[0x1] = Chip8::OP_8xy1;
		table8[0x2] = Chip8::OP_8xy2;
		table8[0x3] = Chip8::OP_8xy3;
		table8[0x4] = Chip8::OP_8xy4;
		table8[0x5] = Chip8::OP_8xy5;
		table8[0x6] = Chip8::OP_8xy6;
		table8[0x7] = Chip8::OP_8xy7;
        table8[0xE] = Chip8::OP_8xyE;
        
        tableE[0x1] = Chip8::OP_ExA1;
        tableE[0xE] = Chip8::OP_Ex9E;
        
        tableF[0x07] = Chip8::OP_Fx07;
		tableF[0x0A] = Chip8::OP_Fx0A;
		tableF[0x15] = Chip8::OP_Fx15;
		tableF[0x18] = Chip8::OP_Fx18;
		tableF[0x1E] = Chip8::OP_Fx1E;
		tableF[0x29] = Chip8::OP_Fx29;
		tableF[0x33] = Chip8::OP_Fx33;
		tableF[0x55] = Chip8::OP_Fx55;
        tableF[0x65] = Chip8::OP_Fx65;
        
        // Null op
        table[0xF + 1] = Chip8::OP_NULL;
        table0[0xE + 1] = Chip8::OP_NULL;
        table8[0xE + 1] = Chip8::OP_NULL;
        tableE[0xE + 1] = Chip8::OP_NULL;
        tableF[0x65 + 1] = Chip8::OP_NULL;


        Chip8 {
            memory: memory,
            registers: [0; 16],
            index: 0,
            pc: START_ADDRESS as u16,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            video: [0; 64 * 32],
            opcode: 0,
            table: table,
            table0: table0,
            table8: table8,
            tableE: tableE,
            tableF: tableF,
        }
    }

    pub fn load_rom(self: &mut Self, path: &String) {
        // load file to byte stream
        let content: Vec<u8> = match fs::read(path) {
            Ok(byte_stream) => byte_stream,
            Err(err) => {
                println!("failed to open file : {}", err);
                let empty = vec![0 as u8];
                empty
            }
        };
        // dump to memory
        for x in 0..content.len() {
            self.memory[START_ADDRESS + (x as usize)] = content[x];
        }
    }

    pub fn rand_gen() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen::<u8>()
    }

    pub fn cycle(self: &mut Self) {
        //debug
        print!("stack : ");
        for x in self.stack.iter(){
            print!("{:X?}|" , x);
        }

        println!("PC : {:X?}", self.pc);
        println!("I : {:X?}", self.index);

        // fetch
        let opcode: u16 = ((self.memory[self.pc as usize] as u16) << 8) | (self.memory[self.pc as usize + 1]) as u16;
        self.opcode = opcode;
        println!("executing {:X?} | table {:X?}", opcode, (opcode & 0xF000) >> 12);
        // increment pc before execute
        self.pc += 2;

        // decode and execute
        self.table[((opcode & 0xF000) >> 12) as usize](self);

        // decrement timer if set
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        // decrement sound timer if set
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn load_font_set(memory: &mut Memory, font_set: &[u8; 80]) {
        // dump font set to memory
        for x in 0..font_set.len() {
            memory[FONT_SET_START_ADDRESS + (x as usize)] = font_set[x]
        }
    }

    // instructions tables

    fn table0(self: &mut Self){
        let instruct = self.table0[(self.opcode & 0x000F) as usize];
        instruct(self);
    }
    fn table8(self: &mut Self){
        let instruct = self.table8[(self.opcode & 0x000F) as usize];
        instruct(self);
    }
    fn tableE(self: &mut Self){
        let instruct = self.tableE[(self.opcode & 0x000F) as usize];
        instruct(self);
    }
    fn tableF(self: &mut Self){
        let instruct = self.tableF[(self.opcode & 0x00FF) as usize];
        instruct(self);
    }
    fn OP_NULL(self:&mut Self) -> (){
        ()
    }
}

// Instructions
// note: AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA -rain
// finished, now lets descend into testing hell -rain
#[allow(dead_code)]
#[allow(non_snake_case)]
impl Chip8 {
    /// CLS,
    /// Clear Display
    pub fn OP_00E0(self: &mut Self) {
        self.video = [0 as u32; VIDEO_SIZE];
    }

    /// RET,
    /// Return to previous stack
    pub fn OP_00EE(self: &mut Self) {
        self.sp = self.sp - 1;
        self.pc = self.stack[self.sp as usize];
    }

    /// JMP addr,
    /// Jump to nnn, no stacking
    pub fn OP_1nnn(self: &mut Self) {
        let address: u16 = self.opcode & 0x0FFF;
        self.pc = address;
    }

    /// CALL addr,
    /// Call subroutine at nnn
    pub fn OP_2nnn(self: &mut Self) {
        let address: u16 = self.opcode & 0xFFF as u16;

        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = address;
    }

    /// SE Vx, byte,
    /// Skip next instruction if Vx == kk
    pub fn OP_3xkk(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        if self.registers[vx as usize] == byte {
            self.pc += 2;
        }
    }

    /// SNE Vx, byte,
    /// Skip next instruction if Vx != kk
    pub fn OP_4xkk(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        if self.registers[vx as usize] != byte {
            self.pc += 2;
        }
    }

    /// SE Vx, Vy,
    /// Skip next instruction if Vx == Vy
    pub fn OP_5xy0(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.registers[vx as usize] == self.registers[vy as usize] {
            self.pc += 2;
        }
    }

    /// LD Vx, byte,
    /// Set Vx = kk
    pub fn OP_6xkk(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        self.registers[vx as usize] = byte;
    }

    /// ADD Vx, byte,
    /// Set Vx = Vx + kk
    pub fn OP_7xkk(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        self.registers[vx as usize] += byte;
    }

    /// LD Vx, Vy,
    /// Set Vx = Vy
    pub fn OP_8xy0(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] = vy;
    }

    /// OR Vx, Vy,
    /// Set Vx | Vy
    pub fn OP_8xy1(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] |= vy;
    }

    /// AND Vx, Vy,
    /// Set Vx & Vy
    pub fn OP_8xy2(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] &= vy;
    }

    /// XOR Vx, Vy,
    /// Set Vx & Vy
    pub fn OP_8xy3(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        self.registers[vx as usize] ^= vy;
    }

    /// ADD Vx, Vy, set Vf = Carry,
    /// ADD Vx and Vy. if the result greater than 8bit (>255) set Vf to 1, otherwise Vf to 0.
    /// Store the lower 8 bits to Vx
    pub fn OP_8xy4(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;
        let sum: u16 = (self.registers[vx as usize] + self.registers[vy as usize]) as u16;

        if sum > 0xFF {
            self.registers[0xF] = 1; // register 0xf = Vf
        } else {
            self.registers[0xF] = 0; // register 0xf = Vf
        }

        self.registers[vx as usize] = (sum & 0xFF) as u8;
    }

    /// SUB Vx, Vy, set Vf = NOT Borrow.
    /// Set Vx -= Vy, if Vx > Vy set Vf to 1 otherwise 0.
    /// Store result in Vx
    pub fn OP_8xy5(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        // register 0xf = Vf
        if self.registers[vx as usize] > self.registers[vy as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx as usize] -= self.registers[vy as usize];
    }

    /// SHR Vx
    /// Set Vx = Vx >> 1
    /// if Vx is 1, then Vf is set to 1 otherwise 0
    pub fn OP_8xy6(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.registers[0xf] = self.registers[vx as usize] & 0x1;

        self.registers[vx as usize] >>= 1;
    }

    /// SUBN Vx, Vy, set Vf = NOT Borrow.
    /// Set Vy -= Vx, if Vx < Vy set Vf to 1 otherwise 0.
    /// Store result in Vx
    pub fn OP_8xy7(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        // register 0xf = Vf
        if self.registers[vx as usize] < self.registers[vy as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx as usize] = self.registers[vy as usize] - self.registers[vx as usize];
    }

    /// SHL Vx {, Vy}.
    /// Set Vx = Vx << 1.
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
    pub fn OP_8xyE(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.registers[0xF] = ((self.registers[vx as usize] & 0x80) >> 7) as u8;

        self.registers[vx as usize] <<= 1;
    }

    /// SNE Vx, Vy.
    ///* Skip next instruction if Vx != Vy.
    pub fn OP_9xy0(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;

        if self.registers[vx as usize] != self.registers[vy as usize] {
            self.pc += 2;
        }
    }

    /// LD I, addr
    ///* Set I = addr
    pub fn OP_Annn(self: &mut Self) {
        let address: u16 = self.opcode & 0x0FFF;

        self.index = address;
    }

    /// JP V0, addr
    ///* jump to location nnn + V0
    pub fn OP_Bnnn(self: &mut Self) {
        let address: u16 = self.opcode & 0x0FFF;

        self.pc = (self.registers[0] as u16) + address;
    }

    /// RND Vx, byte
    ///* Set Vx = random byte + kk
    pub fn OP_Cxkk(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte: u8 = ((self.opcode & 0x00FF) >> 4) as u8;

        self.registers[vx as usize] = Self::rand_gen() & byte;
    }

    /// DRW Vx, Vy, nibble
    ///* Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    // TODO : how the fuck does this works?
    pub fn OP_Dxyn(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;
        let height: u8 = (self.opcode & 0x000F) as u8;

        // screen wrap
        let xPos: u8 = self.registers[vx as usize] % VIDEO_WIDTH as u8;
        let yPos: u8 = self.registers[vy as usize] % VIDEO_HEIGHT as u8;

        // reset flag
        self.registers[0xF] = 0;

        for row in 0..height {
            let spriteByte: u8 = self.memory[(self.index + row as u16) as usize];

            for col in 0..8 {
                let spritePixel: u8 = spriteByte & (0x80 >> col);
                let screenPixel: &mut u32 = &mut self.video
                    [((yPos + row) as usize) * VIDEO_WIDTH + ((xPos + col) as usize)];

                if spritePixel > 0 {
                    if *screenPixel == 0xFFFFFFFF {
                        self.registers[0xF] = 1;
                    }

                    *screenPixel ^= 0xFFFFFFFF;
                }
            }
        }

        // println!("Video : ");
        // for x in self.video.iter(){
        //     print!("{:X?}|", x);
        // }
    }

    /// SKP Vx
    ///* Skip next instruction if key with the value of Vx is pressed.
    pub fn OP_Ex9E(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let key: u8 = self.registers[vx as usize];

        if self.keypad[key as usize] == 1 {
            self.pc += 2;
        }
    }

    /// SKNP Vx
    ///* Skip next instruction if key with the value of Vx is not pressed.
    pub fn OP_ExA1(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let key: u8 = self.registers[vx as usize];

        if self.keypad[key as usize] != 1 {
            self.pc += 2;
        }
    }

    /// LD Vx, DT
    ///* Set Vx = Delay Timer
    pub fn OP_Fx07(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        self.registers[vx as usize] = self.delay_timer;
    }

    /// LD Vx, K
    ///* Wait for a key press, store value of the key to Vx
    pub fn OP_Fx0A(self: &mut Self) {
        let vx: usize = ((self.opcode & 0x0F00) >> 8) as usize;

        let mut pressedKey: u8 = 16; //not pressed

        for x in 0..self.keypad.len() {
            if self.keypad[x as usize] == 1 {
                pressedKey = x as u8;
                break;
            }
        }

        if pressedKey < 16 {
            // if key pressed
            self.registers[vx] = pressedKey;
        } else {
            self.pc -= 2;
        }
    }

    /// LD DT, Vx
    ///* Set Delay Timer = Vx
    pub fn OP_Fx15(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        self.delay_timer = self.registers[vx as usize];
    }

    /// LD ST, Vx
    ///* Set Sound Timer = Vx
    pub fn OP_Fx18(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.sound_timer = self.registers[vx as usize];
    }

    /// ADD I, Vx
    ///* Set I = I + Vx
    pub fn OP_Fx1E(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        self.index += (self.registers[vx as usize]) as u16;
    }

    /// LD F, Vx
    ///* Set I = location of sprite for digit Vx
    pub fn OP_Fx29(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let digit: u8 = self.registers[vx as usize];

        self.index = (FONT_SET_START_ADDRESS + (5 * digit as usize)) as u16;
    }

    /// LD B, Vx
    ///* Store BCD representation of Vx in memory locations I, I+1, and I+2.
    ///* The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
    pub fn OP_Fx33(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let mut val: u8 = self.registers[vx as usize];

        // Ones
        self.memory[(self.index + 2) as usize] = val % 10;
        val /= 10;

        // Tens
        self.memory[(self.index + 1) as usize] = val % 10;
        val /= 10;

        // Hundreds
        self.memory[(self.index) as usize] = val % 10;
    }

    /// LD [I], Vx
    ///* Store registers from V0 to Vx in memory starting at I
    pub fn OP_Fx55(self: &mut Self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        for i in 0..vx {
            self.memory[(self.index + i as u16) as usize] = self.registers[i as usize];
        }
    }

    /// LD Vx, [I]
    ///* Read registers from V0 to Vx from memory starting at I
    pub fn OP_Fx65(self: &mut Self){
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;

        for x in 0..vx{
            self.registers[x as usize] = self.memory[(self.index + x as u16) as usize];
        }
    }
}


// Tests
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
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
    fn test_OP_00EE() {
        unimplemented!();
    }
}
