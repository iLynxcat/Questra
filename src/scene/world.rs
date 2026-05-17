use crate::level::{
    Level, LevelBlock,
    block::{BlockFace, Material},
};
use raylib::{
    RaylibHandle, camera::Camera3D, color::Color, drawing::RaylibDrawHandle, ffi::KeyboardKey,
    math::Vector3, prelude::*,
};

const MOVE_SPEED: f32 = 5.0;

pub struct WorldScene {
    pub is_frozen: bool,
    pub hovered_block: Option<(i32, i32, i32, BlockFace)>,

    camera: Camera3D,
    level: Level,
}

impl WorldScene {
    pub fn new(level: Level) -> Self {
        Self {
            is_frozen: false,
            hovered_block: None,

            camera: Camera3D::orthographic(
                Vector3::new(30.0, 30.0, 30.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                8.0,
            ),
            level,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
            self.is_frozen = !self.is_frozen;
        }

        if self.is_frozen {
            return;
        }

        let mouse = rl.get_mouse_position();
        let ray = rl.get_screen_to_world_ray(mouse, self.camera);

        let mut targeted_block: Option<(f32, &LevelBlock, BlockFace)> = None;

        for block in &self.level.blocks {
            if block.block.material == Material::Air {
                continue;
            }

            let (x, y, z) = (block.x as f32, block.y as f32, block.z as f32);
            let bbox = BoundingBox {
                min: Vector3::new(x - 0.5, y, z - 0.5),
                max: Vector3::new(x + 0.5, y + 1.0, z + 0.5),
            };
            let hit = bbox.get_ray_collision_box(ray);
            if hit.hit {
                if targeted_block.is_none() || hit.distance < targeted_block.as_ref().unwrap().0 {
                    targeted_block = Some((hit.distance, block, face_from_normal(hit.normal)));
                }
            }
        }

        if let Some((_, block, face)) = targeted_block {
            self.hovered_block = Some((block.x, block.y, block.z, face));
        } else {
            self.hovered_block = None;
        }

        if let Some((x, y, z, face)) = self.hovered_block {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                if let Some(block) = self
                    .level
                    .blocks
                    .iter_mut()
                    .find(|b| b.x == x && b.y == y && b.z == z)
                {
                    block.block = Material::Air.default();
                }
            } else if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
                let (ox, oy, oz) = face.offset();
                let (x, y, z) = (x + ox, y + oy, z + oz);

                if let Some(block) = self
                    .level
                    .blocks
                    .iter_mut()
                    .find(|b| b.x == x && b.y == y && b.z == z)
                {
                    block.block = Material::Stone.default();
                }
            }
        }

        let movement_delta: Vector3 = {
            let mut x = 0.0;
            let mut z = 0.0;

            if rl.is_key_down(KeyboardKey::KEY_W) {
                x -= MOVE_SPEED;
                z -= MOVE_SPEED;
            }
            if rl.is_key_down(KeyboardKey::KEY_S) {
                x += MOVE_SPEED;
                z += MOVE_SPEED;
            }
            if rl.is_key_down(KeyboardKey::KEY_A) {
                x -= MOVE_SPEED;
                z += MOVE_SPEED;
            }
            if rl.is_key_down(KeyboardKey::KEY_D) {
                x += MOVE_SPEED;
                z -= MOVE_SPEED;
            }

            let raw = Vector3::new(x, 0.0, z);
            if raw.length() > 0.0 {
                raw.normalized() * MOVE_SPEED * rl.get_frame_time()
            } else {
                raw
            }
        };

        self.camera.position = self.camera.position + movement_delta;
        self.camera.target = self.camera.target + movement_delta;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::SKYBLUE);

        let mut d3 = d.begin_mode3D(&self.camera);

        for LevelBlock { block, x, y, z } in &self.level.blocks {
            let color: Color = match block.material {
                Material::Air => continue, // skip rendering air entirely
                Material::Barrier => continue,
                Material::Dirt => Color::DARKBROWN,
                Material::Grass => Color::FORESTGREEN,
                Material::Stone => Color::LIGHTSLATEGRAY,
            };

            let (x, y, z) = (*x as f32, *y as f32, *z as f32);

            draw_block(&mut d3, x, y, z, color);
        }

        if let Some((x, y, z, ..)) = &self.hovered_block {
            let (x, y, z) = (*x as f32, *y as f32, *z as f32);

            d3.draw_cube_wires(Vector3::new(x, y + 0.5, z), 1.0, 1.0, 1.0, Color::RED);
        }

        // player sprite
        // d3.draw_billboard(&cam, None, center, size, Color::WHITE);

        drop(d3);

        d.draw_text("Questra", 10, 10, 18, Color::WHITE);
        if self.is_frozen {
            d.draw_text("Frozen", 10, 30, 18, Color::WHITE);
        }
    }
}

// TODO: put this in a better place
fn face_from_normal(normal: Vector3) -> BlockFace {
    if normal.y > 0.5 {
        BlockFace::Up
    } else if normal.y < -0.5 {
        BlockFace::Down
    } else if normal.x > 0.5 {
        BlockFace::North
    } else if normal.x < -0.5 {
        BlockFace::East
    } else if normal.z > 0.5 {
        BlockFace::South
    } else {
        BlockFace::West
    }
}

fn draw_block(d: &mut impl RaylibDraw3D, x: f32, y: f32, z: f32, color: Color) {
    let (x0, x1) = (x - 0.5, x + 0.5);
    let (y0, y1) = (y, y + 1.0);
    let (z0, z1) = (z - 0.5, z + 0.5);

    let top = color;
    let side = color.brightness(-0.2);
    let bottom = color.brightness(-0.4);

    // top
    draw_quad(
        d,
        [x0, y1, z1],
        [x1, y1, z1],
        [x1, y1, z0],
        [x0, y1, z0],
        top,
    );
    // bottom
    draw_quad(
        d,
        [x0, y0, z1],
        [x1, y0, z1],
        [x1, y0, z0],
        [x0, y0, z0],
        bottom,
    );
    // front (z+)
    draw_quad(
        d,
        [x0, y0, z1],
        [x1, y0, z1],
        [x1, y1, z1],
        [x0, y1, z1],
        side,
    );
    // back (z-)
    draw_quad(
        d,
        [x1, y0, z0],
        [x0, y0, z0],
        [x0, y1, z0],
        [x1, y1, z0],
        side,
    );
    // right (x+)
    draw_quad(
        d,
        [x1, y0, z1],
        [x1, y0, z0],
        [x1, y1, z0],
        [x1, y1, z1],
        side,
    );
    // left (x-)
    draw_quad(
        d,
        [x0, y0, z0],
        [x0, y0, z1],
        [x0, y1, z1],
        [x0, y1, z0],
        side,
    );
}

fn draw_quad(
    d: &mut impl RaylibDraw3D,
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
    e: [f32; 3],
    color: Color,
) {
    let [a, b, c, e] = [a, b, c, e].map(|v| Vector3::new(v[0], v[1], v[2]));
    d.draw_triangle3D(a, b, c, color);
    d.draw_triangle3D(a, c, e, color);
}
