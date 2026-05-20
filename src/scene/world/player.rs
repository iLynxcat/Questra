use raylib::{
    RaylibHandle,
    color::Color,
    drawing::{RaylibDraw3D, RaylibDrawHandle, RaylibMode3D},
    ffi::KeyboardKey,
    math::{BoundingBox, Matrix, Rectangle, Vector2, Vector3},
};

use crate::{
    assets::GameAssets,
    level::Level,
    scene::{render::lerp_smooth, world::camera::Camera},
};

const MOVEMENT_SPEED: f32 = 5.0;
const MOVEMENT_HALF_LIFE: f32 = 0.1;

const TERMINAL_VELOCITY: f32 = 8.0;
const GRAVITY_HALF_LIFE: f32 = 0.15;

const SPRITE_SIZE: f32 = 64.0;
const MOVE_ANIM_FPS: f32 = 15.0;

const PLAYER_BB_WIDTH: f32 = 0.8;
const PLAYER_BB_DISTANCE_FROM_CENTER: f32 = PLAYER_BB_WIDTH * 0.5;
const PLAYER_BB_HEIGHT: f32 = 1.8;

pub struct Player {
    pub position: Vector3,
    pub velocity: Vector3,

    is_noclip: bool,
    is_grounded: bool,

    walk_animation_frame: i32,
    walk_animation_timer: f32,
}

impl Player {
    pub fn new(position: Vector3) -> Self {
        Self {
            position,
            velocity: Vector3::new(0.0, 0.0, 0.0),

            is_noclip: true,
            is_grounded: true,

            walk_animation_frame: 0,
            walk_animation_timer: 0.0,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, level: &Level) {
        let dt = rl.get_frame_time();

        let input_speed = get_input_direction(&rl);
        let target = input_speed * MOVEMENT_SPEED;

        self.velocity.x = lerp_smooth(self.velocity.x, target.x, MOVEMENT_HALF_LIFE, dt);
        self.velocity.z = lerp_smooth(self.velocity.z, target.z, MOVEMENT_HALF_LIFE, dt);

        let y_target = if self.is_grounded {
            0.0
        } else {
            -TERMINAL_VELOCITY
        };

        self.velocity.y = lerp_smooth(
            self.velocity.y,
            y_target,
            if self.is_noclip {
                0.0
            } else {
                GRAVITY_HALF_LIFE
            },
            dt,
        );

        self.position.x += self.velocity.x * dt;
        for bb in level.overlapping_solid_boxes(&self.get_bounds()) {
            self.position.x = if self.velocity.x > 0.0 {
                bb.min.x - PLAYER_BB_DISTANCE_FROM_CENTER
            } else {
                bb.max.x + PLAYER_BB_DISTANCE_FROM_CENTER
            };
            self.velocity.x = 0.0;
        }

        self.position.z += self.velocity.z * dt;
        for bb in level.overlapping_solid_boxes(&self.get_bounds()) {
            self.position.z = if self.velocity.z > 0.0 {
                bb.min.z - PLAYER_BB_DISTANCE_FROM_CENTER
            } else {
                bb.max.z + PLAYER_BB_DISTANCE_FROM_CENTER
            };
            self.velocity.z = 0.0;
        }

        self.position.y += self.velocity.y * dt;
        self.is_grounded = false;
        for bb in level.overlapping_solid_boxes(&self.get_bounds()) {
            if self.velocity.y <= 0.0 {
                self.position.y = bb.max.y;
                self.is_grounded = true;
            } else {
                self.position.y = bb.min.y - PLAYER_BB_HEIGHT;
            }
            self.velocity.y = 0.0;
        }

        self.walk_animation_timer += rl.get_frame_time();
        if self.walk_animation_timer >= 1.0 / MOVE_ANIM_FPS {
            self.walk_animation_timer = 0.0;
            if input_speed.length() > 0.01 {
                self.walk_animation_frame = if self.walk_animation_frame > 8 {
                    0
                } else {
                    self.walk_animation_frame + 1
                };
            } else {
                self.walk_animation_frame = 0;
            }
        }
    }

    pub fn draw(&self, d3: &mut RaylibMode3D<RaylibDrawHandle>, cam: &Camera, assets: &GameAssets) {
        let frame = self.walk_animation_frame as f32;

        self.draw_bounds(d3);

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

    fn draw_bounds(&self, d3: &mut RaylibMode3D<RaylibDrawHandle>) {
        d3.draw_cube_wires(
            self.position
                .transform_with(Matrix::translate(0.0, PLAYER_BB_HEIGHT * 0.5, 0.0)),
            PLAYER_BB_WIDTH,
            PLAYER_BB_HEIGHT,
            PLAYER_BB_WIDTH,
            Color::BLUE,
        );
    }

    pub fn get_bounds(&self) -> BoundingBox {
        let Vector3 { x, y, z } = self.position;

        BoundingBox::new(
            Vector3::new(x - PLAYER_BB_WIDTH, y, z - PLAYER_BB_WIDTH),
            Vector3::new(
                x + PLAYER_BB_DISTANCE_FROM_CENTER,
                y + PLAYER_BB_HEIGHT,
                z + PLAYER_BB_DISTANCE_FROM_CENTER,
            ),
        )
    }
}

fn get_input_direction(rl: &RaylibHandle) -> Vector3 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    if rl.is_key_down(KeyboardKey::KEY_W) {
        x -= MOVEMENT_SPEED;
        z -= MOVEMENT_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_S) {
        x += MOVEMENT_SPEED;
        z += MOVEMENT_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_A) {
        x -= MOVEMENT_SPEED;
        z += MOVEMENT_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_D) {
        x += MOVEMENT_SPEED;
        z -= MOVEMENT_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_SPACE) {
        y += MOVEMENT_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
        y -= MOVEMENT_SPEED;
    }

    let raw = Vector3::new(x, y, z);
    if raw.length() > 0.0 {
        raw.normalized()
    } else {
        raw
    }
}
