use std::collections::HashMap;

use crate::level::block::{Block, Material};

pub struct Level {
    pub blocks: HashMap<(i32, i32, i32), Block>,
}

pub const LEVEL_XZ_MIN: i32 = -8;
pub const LEVEL_XZ_MAX: i32 = 8;
pub const LEVEL_Y_MIN: i32 = 0;
pub const LEVEL_Y_MAX: i32 = 127;
const LEVEL_Y_SURFACE: i32 = 12;
const LEVEL_TOTAL_BLOCKS: usize = (LEVEL_XZ_MAX * LEVEL_XZ_MAX * LEVEL_Y_MAX) as usize;

impl Level {
    pub fn new() -> Self {
        let mut blocks: HashMap<(i32, i32, i32), Block> =
            HashMap::with_capacity(LEVEL_TOTAL_BLOCKS);

        // WORLD GENERATION

        for x in LEVEL_XZ_MIN..LEVEL_XZ_MAX {
            for z in LEVEL_XZ_MIN..LEVEL_XZ_MAX {
                for y in LEVEL_Y_MIN..LEVEL_Y_MAX {
                    let block = if y == 0 {
                        Material::Barrier.default()
                    } else if y < LEVEL_Y_SURFACE - 3 {
                        Material::Stone.default()
                    } else if y >= LEVEL_Y_SURFACE - 3 && y < LEVEL_Y_SURFACE {
                        Material::Dirt.default()
                    } else if y == LEVEL_Y_SURFACE {
                        if x > -5 && x < 5 && z > -5 && z < 5 {
                            Material::Water.default()
                        } else {
                            Material::Grass.default()
                        }
                    } else {
                        Material::Air.default()
                    };

                    blocks.insert((x, y, z), block);
                }
            }
        }

        Self { blocks }
    }
}
