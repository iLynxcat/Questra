use raylib::ffi;
use std::collections::HashMap;
use std::mem;

use crate::level::block::{BlockFace, Material};

const ATLAS_COLS: u32 = 4;
const ATLAS_ROWS: u32 = 4;

fn tile_uvs(col: u32, row: u32, atlas_cols: u32, atlas_rows: u32) -> (f32, f32, f32, f32) {
    let (ac, ar) = (atlas_cols as f32, atlas_rows as f32);
    (
        (col + 1) as f32 / ac,
        (row + 1) as f32 / ar,
        (col) as f32 / ac,
        (row) as f32 / ar,
    )
}

fn block_tile(mat: Material, face: BlockFace) -> (u32, u32) {
    match mat {
        Material::Dirt => (0, 3),
        Material::Grass => match face {
            BlockFace::Up => (0, 1),
            BlockFace::Down => (0, 3),
            _ => (0, 2),
        },
        Material::Stone => (0, 0),
        Material::Sign => (1, 0),
        _ => (3, 0),
    }
}

#[derive(Default)]
pub struct MeshData {
    pub positions: Vec<f32>, // 3 floats per vertex (x, y, z)
    pub texcoords: Vec<f32>, // 2 floats per vertex (x, y; in atlas)
    pub colors: Vec<u8>,     // 4 bytes per vertex (r, g, b, a)
}

pub fn upload_mesh(data: MeshData) -> ffi::Mesh {
    unsafe {
        let mut mesh: ffi::Mesh = mem::zeroed();
        mesh.vertexCount = (data.positions.len() / 3) as i32;
        mesh.triangleCount = (data.positions.len() / 9) as i32;

        let mut pos = data.positions.into_boxed_slice();
        let mut uvs = data.texcoords.into_boxed_slice();
        let mut col = data.colors.into_boxed_slice();

        mesh.vertices = pos.as_mut_ptr();
        mesh.texcoords = uvs.as_mut_ptr();
        mesh.colors = col.as_mut_ptr();

        mem::forget(pos);
        mem::forget(uvs);
        mem::forget(col);

        ffi::UploadMesh(&mut mesh, false);
        mesh
    }
}

fn is_solid(world: &HashMap<(i32, i32, i32), Material>, pos: (i32, i32, i32)) -> bool {
    matches!(world.get(&pos), Some(b) if *b != Material::Air)
}

pub fn build_mesh(world: &HashMap<(i32, i32, i32), Material>) -> MeshData {
    let mut data = MeshData::default();

    for (&(x, y, z), &material) in world {
        if material == Material::Air || material == Material::Barrier {
            continue;
        }

        let (xi, yi, zi) = (x as i32, y as i32, z as i32);
        let (fx, fy, fz) = (x as f32, y as f32, z as f32);

        let candidates = [
            (BlockFace::Up, xi, yi + 1, zi, 1.0f32),
            (BlockFace::Down, xi, yi - 1, zi, 0.6),
            (BlockFace::North, xi, yi, zi + 1, 0.8),
            (BlockFace::South, xi, yi, zi - 1, 0.8),
            (BlockFace::East, xi + 1, yi, zi, 0.9),
            (BlockFace::West, xi - 1, yi, zi, 0.9),
        ];

        for (face, nx, ny, nz, brightness) in candidates {
            if is_solid(world, (nx, ny, nz)) {
                continue;
            }
            let (tc, tr) = block_tile(material, face);
            push_face(
                &mut data,
                face,
                fx,
                fy,
                fz,
                tile_uvs(tc, tr, ATLAS_COLS, ATLAS_ROWS),
                brightness,
            );
        }
    }

    data
}

fn push_face(
    data: &mut MeshData,
    face: BlockFace,
    x: f32,
    y: f32,
    z: f32,
    uvs: (f32, f32, f32, f32),
    brightness: f32,
) {
    let (x0, x1) = (x - 0.5, x + 0.5);
    let (y0, y1) = (y, y + 1.0);
    let (z0, z1) = (z - 0.5, z + 0.5);

    let verts: [[f32; 3]; 4] = match face {
        BlockFace::Up => [[x0, y1, z1], [x1, y1, z1], [x1, y1, z0], [x0, y1, z0]],
        BlockFace::Down => [[x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1]],
        BlockFace::North => [[x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1]],
        BlockFace::South => [[x1, y0, z0], [x0, y0, z0], [x0, y1, z0], [x1, y1, z0]],
        BlockFace::East => [[x1, y0, z1], [x1, y0, z0], [x1, y1, z0], [x1, y1, z1]],
        BlockFace::West => [[x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0]],
    };

    let (u0, v0, u1, v1) = uvs;
    let uv_corners = [[u0, v0], [u1, v0], [u1, v1], [u0, v1]];

    let c = (brightness * 255.0) as u8;

    for &i in &[0usize, 1, 2, 0, 2, 3] {
        data.positions.extend_from_slice(&verts[i]);
        data.texcoords.extend_from_slice(&uv_corners[i]);
        data.colors.extend_from_slice(&[c, c, c, 255]);
    }
}

pub fn make_model(mesh: ffi::Mesh, texture: &raylib::texture::Texture2D) -> ffi::Model {
    unsafe {
        let model = ffi::LoadModelFromMesh(mesh);
        let maps = std::slice::from_raw_parts_mut((*model.materials).maps, 12);
        maps[ffi::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].texture = *texture.as_ref();
        model
    }
}
