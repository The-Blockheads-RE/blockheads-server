pub struct Chunk {
    pub blocks: Vec<Vec<Block>>
}

pub const CHUNK_WIDTH: usize = 32;
pub const CHUNK_HEIGHT: usize = 32;


use std::cell::Cell;


use crate::handlers::block::Block;
use crate::handlers::block::BLOCK_SIZE;


pub struct ChunkCache {
    pub LoadedChunks: Vec<Vec<Chunk>>
}

impl Chunk {
    pub fn new() -> Self { // Creates an empty chunk, filled with just air and light.
        let mut blocks = Vec::new();

        let mut y = 0;
        while y < CHUNK_HEIGHT {
            let mut x = 0;
            let mut row = Vec::new();

            while x < CHUNK_WIDTH {
                let block = Block::new();
                block.type_index.set(2);
                block.sun_light.set(255);
                block.light.set(255);

                row.insert(x, block);

                x += 1;
            }
            blocks.insert(y, row);

            y += 1;
        }

        return Self {
            blocks: blocks
        };
    }
    pub fn encode(&self) -> Vec<u8> {
        let mut chunk_data = Vec::new();

        for y in 0..=self.blocks.len()-1 {
            for x in 0..=self.blocks[y].len()-1 {
                let block = self.get_block(x, y).unwrap();
                let block_data = block.encode();

                for byte in block_data {
                    chunk_data.insert(chunk_data.len(), byte)
                };
            }
        };

        let mut padding = [255; 5].to_vec(); // Required for the client to accept the chunk
        chunk_data.append(&mut padding);

        return chunk_data;
    }
    pub fn decode(raw_data: Vec<u8>) -> Self {
        let mut blocks = Vec::new();
        blocks.insert(0, Vec::new());

        let mut index = 0;
        let mut block_y_index = 0;
        let mut block_x_index = 0;
        while index < (CHUNK_HEIGHT * CHUNK_WIDTH) * BLOCK_SIZE {
            if block_y_index > CHUNK_WIDTH - 1 {
                block_x_index += 1;
                block_y_index = 0;
                if block_x_index < CHUNK_HEIGHT {
                    println!("insert index {block_x_index}");
                    blocks.insert(block_x_index, Vec::new());
                } else {
                    break;
                }
            }

            let data = raw_data[index..index+BLOCK_SIZE-1].to_vec();
            let block = Block::decode(data);
            //println!("[{block_x_index}][{block_y_index}]");
            blocks[block_x_index].insert(block_y_index, block);
            //println!("{:02x}", block.type_index);
            //println!("{}", index);
            index += BLOCK_SIZE;
            block_y_index += 1;
        }

        //println!("{:#?}", blocks);

        return Chunk {
            blocks: blocks
        }
    }
}

impl Chunk {
    pub fn get_block(&self, x: usize, y: usize) -> Option<&Block> {
        let blocks_y: &Vec<Block> = match self.blocks.get(y) {
            Some(blocks_y) => blocks_y,
            None => return None
        };

        return match blocks_y.get(x) {
            Some(block) => {
                return Some(block); // it works i think so like
            },
            None => None
        };
    }

    pub fn print(&self) {
        let mut x: usize = 0;

        while x < CHUNK_WIDTH - 1 {
            let mut y: usize = 0;

            while y < CHUNK_HEIGHT - 1 {
                let block = &self.blocks[x][y];
                //println!("visibility: {}, type: {}, subtype: {}, backwall: {}", block.visibility, block.get_name(), block.sub_type, block.back_wall_type_index);
                y += 1
            }
            x += 1
        }
    }
}