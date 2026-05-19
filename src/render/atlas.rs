use crate::level::block::{BlockFace, Material};

const ATLAS_COLS: u32 = 4;
const ATLAS_ROWS: u32 = 4;

pub fn tile_uvs(col: u32, row: u32) -> (f32, f32, f32, f32) {
    let (ac, ar) = (ATLAS_COLS as f32, ATLAS_ROWS as f32);

    (
        (col + 1) as f32 / ac,
        (row + 1) as f32 / ar,
        (col) as f32 / ac,
        (row) as f32 / ar,
    )
}

pub fn block_tile(mat: Material, face: BlockFace) -> (u32, u32) {
    match mat {
        Material::Dirt => (0, 3),
        Material::Grass => match face {
            BlockFace::Up => (0, 1),
            BlockFace::Down => (0, 3),
            _ => (0, 2),
        },
        Material::Stone => (0, 0),
        Material::Sign => (1, 0),
        Material::Barrier => (3, 3),
        Material::Water => (1, 1),
        _ => (3, 0),
    }
}
