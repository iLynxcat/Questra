use std::collections::HashMap;

use crate::level::block::{Block, BlockFace, BlockState, Material};

pub fn block_height(block: &Block) -> f32 {
    match block.state {
        BlockState::LiquidLevel(level) => 0.9 * level,
        _ => 1.0,
    }
}

pub fn is_solid_for(
    world: &HashMap<(i32, i32, i32), Block>,
    pos: (i32, i32, i32),
    current: Material,
    face: BlockFace,
) -> bool {
    match world.get(&pos) {
        None => false,
        Some(b) => match b.material {
            Material::Air => false,
            Material::Water => match face {
                _ => current == Material::Water, // only cull water-water
            },
            _ => true,
        },
    }
}
