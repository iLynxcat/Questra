use crate::{
    assets::GameAssets,
    level::{Level, LevelBlock, block::Material},
    scene::world::block::draw_block,
};
use raylib::{
    RaylibHandle, camera::Camera3D, color::Color, drawing::RaylibDrawHandle, ffi::KeyboardKey,
    math::Vector3, prelude::*,
};

const MOVE_SPEED: f32 = 5.0;

pub struct WorldScene {
    pub is_frozen: bool,
    pub is_showing_pause_menu: bool,
    pub hovered_block: Option<(i32, i32, i32, Vector3)>,

    rotation: f32,

    camera: Camera3D,
    level: Level,
}

impl WorldScene {
    pub fn new(level: Level) -> Self {
        Self {
            is_frozen: false,
            is_showing_pause_menu: false,
            hovered_block: None,

            rotation: 0.0,

            camera: Camera3D::orthographic(
                Vector3::new(30.0, 30.0, 30.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                10.0,
            ),
            level,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
            || rl.is_key_pressed_repeat(KeyboardKey::KEY_ESCAPE)
        {
            self.is_frozen = !self.is_frozen;
            self.is_showing_pause_menu = self.is_frozen;
        }

        if self.is_frozen {
            return;
        }

        self.rotation += 1.0;
        if self.rotation >= 360.0 {
            self.rotation = 0.0;
        }

        let mouse = rl.get_mouse_position();
        let ray = rl.get_screen_to_world_ray(mouse, self.camera);

        let mut targeted_block: Option<(f32, &LevelBlock, Vector3)> = None;

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
                    targeted_block = Some((hit.distance, block, hit.normal));
                }
            }
        }

        if let Some((_, block, normal)) = targeted_block {
            self.hovered_block = Some((block.x, block.y, block.z, normal));
        } else {
            self.hovered_block = None;
        }

        if let Some((x, y, z, normal)) = self.hovered_block {
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
                let (ox, oy, oz) = offset_from_normal(normal);
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

    pub fn draw(&self, d: &mut RaylibDrawHandle, assets: &GameAssets) {
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

        d3.draw_billboard_pro(
            &self.camera,
            *assets.player_sprite,
            Rectangle::new(0.0, 0.0, 64.0, 64.0),
            Vector3::new(0.0, 6.5, 0.0),
            Vector3::up(),
            Vector2::new(1.0, 1.0),
            Vector2::zero(),
            self.rotation,
            Color::WHITE,
        );

        drop(d3);

        d.draw_text("Questra", 10, 10, 18, Color::WHITE);
        if self.is_showing_pause_menu {
            d.draw_text("Pause", 10, 34, 18, Color::WHITE);
            d.draw_text("Press Q to quit", 10, 50, 18, Color::WHITE);
        } else if self.is_frozen {
            d.draw_text("Frozen", 10, 34, 18, Color::WHITE);
            d.draw_text("Press Q to quit", 10, 50, 18, Color::WHITE);
        }
    }
}

// TODO: put this in a better place
fn offset_from_normal(normal: Vector3) -> (i32, i32, i32) {
    (
        normal.x.round() as i32,
        normal.y.round() as i32,
        normal.z.round() as i32,
    )
}
