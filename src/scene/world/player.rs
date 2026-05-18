use raylib::{
    RaylibHandle,
    color::Color,
    drawing::{RaylibDraw3D, RaylibDrawHandle, RaylibMode3D},
    ffi::KeyboardKey,
    math::{Rectangle, Vector2, Vector3},
};

use crate::{assets::GameAssets, scene::world::camera::Camera};

const MOVE_SPEED: f32 = 5.0;

pub struct Player {
    pub position: Vector3,

    walk_animation_frame: i32,
}

impl Player {
    pub fn new(position: Vector3) -> Self {
        Self {
            position,
            walk_animation_frame: 0,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        let movement = get_movement_delta(&rl);
        self.position = self.position.add(movement);
        if movement.length() > 0.01 {
            self.walk_animation_frame = if self.walk_animation_frame > 8 {
                0
            } else {
                self.walk_animation_frame + 1
            };
        } else {
            self.walk_animation_frame = 0;
        }
    }

    pub fn draw(&self, d3: &mut RaylibMode3D<RaylibDrawHandle>, cam: &Camera, assets: &GameAssets) {
        let frame = self.walk_animation_frame as f32;

        d3.draw_billboard_pro(
            cam.raycam,
            *assets.player_sprite,
            Rectangle::new(frame * SPRITE_SIZE, 0.0, SPRITE_SIZE, SPRITE_SIZE),
            self.position,
            Vector3::up(),
            Vector2::new(2.0, 2.0),
            Vector2::new(1.0, 0.0),
            0.0,
            Color::WHITESMOKE,
        );
    }
}

const SPRITE_SIZE: f32 = 64.0;

fn get_movement_delta(rl: &RaylibHandle) -> Vector3 {
    let mut x = 0.0;
    let mut y = 0.0;
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
    if rl.is_key_down(KeyboardKey::KEY_SPACE) {
        y += MOVE_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
        y -= MOVE_SPEED;
    }

    let raw = Vector3::new(x, y, z);
    if raw.length() > 0.0 {
        raw.normalized() * MOVE_SPEED * rl.get_frame_time()
    } else {
        raw
    }
}
