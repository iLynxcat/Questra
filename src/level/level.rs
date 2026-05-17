use crate::level::block::{Block, Material};

pub struct Level {
    pub blocks: Vec<LevelBlock>,
}

#[derive(Debug)]
pub struct LevelBlock {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block: Block,
}

const LEVEL_XZ_MAX: i32 = 16;
const LEVEL_Y_MAX: i32 = 16;
const LEVEL_TOTAL_BLOCKS: usize = (LEVEL_XZ_MAX * LEVEL_XZ_MAX * LEVEL_Y_MAX) as usize;

impl Level {
    pub fn new() -> Self {
        let mut blocks = Vec::with_capacity(LEVEL_TOTAL_BLOCKS);
        for x in 0..LEVEL_XZ_MAX {
            for z in 0..LEVEL_XZ_MAX {
                for y in 0..LEVEL_Y_MAX {
                    let material = if y == 5 {
                        Material::Grass.default()
                    } else if y >= 3 && y < 5 {
                        Material::Dirt.default()
                    } else if y < 3 {
                        Material::Stone.default()
                    } else {
                        Material::Air.default()
                    };

                    blocks.push(LevelBlock {
                        x,
                        y,
                        z,
                        block: material,
                    })
                }
            }
        }

        Self { blocks }
    }
}
