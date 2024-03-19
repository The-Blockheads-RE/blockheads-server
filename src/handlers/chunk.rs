pub struct Chunk {
    raw_data: Vec<u8>,
    pub blocks: Vec<Vec<Block>>
}

pub const CHUNK_WIDTH: usize = 32;
pub const CHUNK_HEIGHT: usize = 32;


use std::cell::Cell;


use crate::handlers::block::Block;
use crate::handlers::block::BLOCK_SIZE;


pub struct ChunkCache {
    pub LoadedChunks: Cell<Chunk>
}

impl Chunk {
    pub fn new(&self) {
        
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

        chunk_data.insert(chunk_data.len(), 255);
        chunk_data.insert(chunk_data.len(), 255);
        chunk_data.insert(chunk_data.len(), 255);
        chunk_data.insert(chunk_data.len(), 255);
        chunk_data.insert(chunk_data.len(), 255);


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
            raw_data: raw_data,
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