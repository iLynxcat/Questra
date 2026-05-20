use std::collections::HashMap;

use raylib::math::BoundingBox;

use crate::{
    level::block::{Block, Material},
    scene::world::collision::get_block_bounds,
};

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
                        if x == LEVEL_XZ_MIN
                            || x == LEVEL_XZ_MAX - 1
                            || z == LEVEL_XZ_MIN
                            || z == LEVEL_XZ_MAX - 1
                        {
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

    pub fn overlapping_solid_boxes(&self, aabb: &BoundingBox) -> Vec<BoundingBox> {
        let x0 = aabb.min.x.floor() as i32;
        let x1 = aabb.max.x.ceil() as i32;
        let y0 = aabb.min.y.floor() as i32;
        let y1 = aabb.max.y.ceil() as i32;
        let z0 = aabb.min.z.floor() as i32;
        let z1 = aabb.max.z.ceil() as i32;

        (x0..=x1)
            .flat_map(|x| {
                (y0..=y1).flat_map(move |y| {
                    (z0..=z1).filter_map(move |z| {
                        self.blocks
                            .get(&(x, y, z))
                            .map_or(None, |b| get_block_bounds(b, (x, y, z)))
                    })
                })
            })
            .collect()
    }
}
