use std::ops::Add;

use crate::{
    assets::GameAssets,
    level::{
        Level, LevelBlock,
        block::{BlockState, Material},
    },
    scene::world::{block::draw_block, player::Player},
};
use raylib::{
    RaylibHandle, camera::Camera3D, color::Color, drawing::RaylibDrawHandle, ffi::KeyboardKey,
    math::Vector3, prelude::*,
};

pub struct WorldScene {
    pub is_frozen: bool,
    pub is_showing_pause_menu: bool,
    pub is_showing_cursor: bool,
    pub hovered_block: Option<(i32, i32, i32, Vector3)>,

    sign_text: Option<String>,
    player: Player,
    camera: Camera3D,
    level: Level,
}

const PLAYER_CAMERA_OFFSET: Vector3 = Vector3::new(45.0, 30.0, 45.0);

impl WorldScene {
    pub fn new(level: Level) -> Self {
        Self {
            is_frozen: false,
            is_showing_pause_menu: false,
            is_showing_cursor: false,
            hovered_block: None,

            sign_text: None,
            player: Player::new(Vector3::new(0.0, 7.0, 0.0)),
            camera: Camera3D::orthographic(
                PLAYER_CAMERA_OFFSET,
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
            self.is_showing_pause_menu = !self.is_showing_pause_menu;
            self.is_frozen = self.is_showing_pause_menu;
        }

        if self.is_frozen {
            return;
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
            self.sign_text = if let BlockState::Sign(text) = &block.block.state {
                Some(text.to_string())
            } else {
                None
            };
        } else {
            self.hovered_block = None;
            self.sign_text = None;
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

        if rl.is_key_pressed(KeyboardKey::KEY_C) {
            self.is_showing_cursor = !self.is_showing_cursor;
        }

        self.player.update(&rl);

        self.camera.position = self.player.position.add(PLAYER_CAMERA_OFFSET);
        self.camera.target = self.player.position;
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
                Material::Sign => Color::DARKBLUE,
            };

            let (x, y, z) = (*x as f32, *y as f32, *z as f32);

            draw_block(&mut d3, x, y, z, color);
        }

        if let Some((x, y, z, ..)) = &self.hovered_block {
            let (x, y, z) = (*x as f32, *y as f32, *z as f32);

            d3.draw_cube_wires(Vector3::new(x, y + 0.5, z), 1.0, 1.0, 1.0, Color::RED);
        }

        self.player.draw(&mut d3, &self.camera, &assets);

        drop(d3);

        if self.is_showing_cursor {
            let mut b = d.begin_blend_mode(BlendMode::BLEND_CUSTOM);
            unsafe {
                raylib_sys::rlSetBlendFactors(
                    ffi::RL_ONE_MINUS_DST_COLOR as i32,
                    ffi::RL_ONE_MINUS_SRC_ALPHA as i32,
                    ffi::RL_FUNC_ADD as i32,
                );
            }

            b.draw_texture(&assets.crosshair_sprite, 288, 208, Color::WHITE);
            drop(b);
        }

        if self.is_showing_pause_menu {
            d.draw_text("Pause", 10, 4, 18, Color::WHITE);
            d.draw_text("Press Q to quit", 10, 20, 18, Color::WHITE);
        } else if self.is_frozen {
            d.draw_text("Frozen", 10, 4, 18, Color::WHITE);
            d.draw_text("Press Q to quit", 10, 20, 18, Color::WHITE);
        }

        if let Some(text) = &self.sign_text {
            d.draw_text(&text, 15, 132, 18, Color::BLACK.alpha(0.5));
            d.draw_text(&text, 10, 128, 18, Color::WHITE);
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
