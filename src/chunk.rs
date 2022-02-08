pub const SIZE: u8 = 16;

pub struct Chunk {
    pub blocks: [[[u16; SIZE as usize]; SIZE as usize]; SIZE as usize],
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            blocks: [[[3; SIZE as usize]; SIZE as usize]; SIZE as usize],
        }
    }

    pub fn get_block(&self, x: u8, y: u8, z: u8) -> u16 {
        if x >= SIZE || y >= SIZE || z >= SIZE {
            return 0;
        }
        self.blocks[x as usize][y as usize][z as usize]
    }
    pub fn get_block_s(&self, x: i16, y: i16, z: i16) -> u16 {
        if x >= SIZE as i16 || y >= SIZE as i16 || z >= SIZE as i16 || x < 0 || y < 0 || z < 0 {
            return 0;
        }
        self.blocks[x as usize][y as usize][z as usize]
    }

    pub fn set_block(&mut self, x: u8, y: u8, z: u8, block: u16) {
        self.blocks[x as usize][y as usize][z as usize] = block;
    }
}