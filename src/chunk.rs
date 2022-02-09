use cgmath::Vector3;

pub const SIZE: u8 = 16;

pub struct Chunk {
    pub position: Vector3<i32>,
    pub blocks: [[[u16; SIZE as usize]; SIZE as usize]; SIZE as usize],
}

impl Chunk {
    pub fn new_empty(pos: Vector3<i32>) -> Chunk {
        Chunk {
            position: pos * SIZE as i32,
            blocks: [[[0; SIZE as usize]; SIZE as usize]; SIZE as usize],
        }
    }
    pub fn new_filled(pos: Vector3<i32>, block: u16) -> Chunk {
        Chunk {
            position: pos * SIZE as i32,
            blocks: [[[block; SIZE as usize]; SIZE as usize]; SIZE as usize],
        }
    }
    pub fn new_layered(pos: Vector3<i32>, surface: u16, shallow: u16, deep: u16) -> Chunk {
        let mut blocks: [[[u16; SIZE as usize]; SIZE as usize]; SIZE as usize] =
            [[[0; SIZE as usize]; SIZE as usize]; SIZE as usize];
        for x in 0..blocks.len() {
            for y in 0..blocks[x].len() {
                for z in 0..blocks[x][y].len() {
                    if y as u8 >= SIZE - 1 {
                        blocks[x][y][z] = surface;
                    } else if (y as u8) < SIZE - 1 && (y as f32) >= SIZE as f32 / 1.5 {
                        blocks[x][y][z] = shallow;
                    } else {
                        blocks[x][y][z] = deep;
                    }
                }
            }
        }
        Chunk {
            position: pos * SIZE as i32,
            blocks,
        }
    }

    pub fn get_block(&self, pos: Vector3<u8>) -> u16 {
        if pos.x >= SIZE || pos.y >= SIZE || pos.z >= SIZE {
            return 0;
        }
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize]
    }
    pub fn get_block_s(&self, pos: Vector3<i16>) -> u16 {
        if pos.x >= SIZE as i16 || pos.y >= SIZE as i16 || pos.z >= SIZE as i16 || pos.x < 0 || pos.y < 0 || pos.z < 0 {
            return 0;
        }
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize]
    }

    pub fn set_block(&mut self, pos: Vector3<u8>, block: u16) {
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize] = block;
    }
    pub fn set_block_global(&mut self, pos: Vector3<i32>, block: u16) {
        let x = (pos.x - self.position.x) + SIZE as i32 / 2;
        let y = (pos.y - self.position.y) + SIZE as i32 / 2;
        let z = (pos.z - self.position.z) + SIZE as i32 / 2;
        if x >= SIZE as i32 || y >= SIZE as i32 || z >= SIZE as i32 || x < 0 || y < 0 || z < 0 {
            return;
        };
        self.blocks[(x as i32) as usize][(y as i32) as usize][(z as i32) as usize] = block;
    }
}