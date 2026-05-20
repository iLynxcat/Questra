use raylib::math::{BoundingBox, Vector3};

use crate::level::block::{Block, Material};

pub fn get_block_bounds(block: &Block, (x, y, z): (i32, i32, i32)) -> Option<BoundingBox> {
    let (x, y, z) = (x as f32, y as f32, z as f32);

    match block.material {
        Material::Water => None,
        Material::Sign => None,
        Material::Air => None,
        _ => Some(BoundingBox::new(
            Vector3::new(x, y, z),
            Vector3::new(x + 1.0, y + 1.0, z + 1.0),
        )),
    }
}
