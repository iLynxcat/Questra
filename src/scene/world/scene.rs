use std::ops::Add;

use crate::{
    assets::GameAssets,
    level::{
        Level,
        block::{Block, BlockState, Material},
    },
    scene::world::{
        build_mesh,
        camera::{Camera, CameraDirection},
        make_model,
        player::Player,
        upload_mesh,
    },
};
use raylib::{
    RaylibHandle,
    color::Color,
    drawing::{
        RaylibBlendModeExt, RaylibDraw, RaylibDraw3D, RaylibDrawHandle, RaylibMode3D,
        RaylibMode3DExt,
    },
    ffi,
    math::{BoundingBox, Vector3},
};
use raylib_sys::{BlendMode, KeyboardKey, MouseButton};

const PLAYER_CAMERA_OFFSET_XZ: f32 = 45.0;
const PLAYER_CAMERA_OFFSET_Y: f32 = 15.0;

const ZOOM_FOVY_MIN: f32 = 5.0;
const ZOOM_FOVY_DEFAULT: f32 = 15.0;
const ZOOM_FOVY_MAX: f32 = 30.0;
const ZOOM_FOVY_INCREMENT: f32 = 5.0;

const HELP_TEXTS: [&'static str; 7] = [
    "Z = debug",
    "X = toggle wireframe",
    "[ = zoom out",
    "] = zoom in",
    "M = toggle mute",
    "V = switch view",
    "Q = quit",
];

pub struct WorldScene {
    is_frozen: bool,
    is_showing_pause_menu: bool,
    is_showing_debug: bool,
    is_showing_wireframe: bool,
    hovered_block: Option<(i32, i32, i32, Vector3)>,

    fps: u32,

    sign_text: Option<String>,
    player: Player,
    camera: Camera,
    level: Level,
    level_mesh: Option<ffi::Model>,
    level_mesh_is_dirty: bool,
}

impl WorldScene {
    pub fn new(level: Level) -> Self {
        let player_position = Vector3::new(0.0, 13.0, 0.0);

        Self {
            is_frozen: false,
            is_showing_pause_menu: false,
            is_showing_debug: false,
            is_showing_wireframe: false,
            hovered_block: None,

            fps: 0,

            sign_text: None,
            player: Player::new(player_position),
            camera: Camera::new(
                Vector3::new(
                    PLAYER_CAMERA_OFFSET_XZ,
                    PLAYER_CAMERA_OFFSET_Y,
                    PLAYER_CAMERA_OFFSET_XZ,
                ) + player_position,
                player_position,
                CameraDirection::PlusXPlusZ,
                ZOOM_FOVY_DEFAULT,
            ),
            level,
            level_mesh: None,
            level_mesh_is_dirty: true,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, assets: &GameAssets) {
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
        let ray = rl.get_screen_to_world_ray(mouse, self.camera.raycam);

        let mut targeted_block: Option<(f32, (i32, i32, i32), &Block, Vector3)> = None;

        for (&(x, y, z), block) in &self.level.blocks {
            if block.material == Material::Air {
                continue;
            }

            let (x, y, z) = (x as f32, y as f32, z as f32);
            let bbox = BoundingBox {
                min: Vector3::new(x - 0.5, y, z - 0.5),
                max: Vector3::new(x + 0.5, y + 1.0, z + 0.5),
            };
            let hit = bbox.get_ray_collision_box(ray);
            if hit.hit {
                if targeted_block.is_none() || hit.distance < targeted_block.as_ref().unwrap().0 {
                    targeted_block = Some((
                        hit.distance,
                        (x.round() as i32, y.round() as i32, z.round() as i32),
                        &block,
                        hit.normal,
                    ));
                }
            }
        }

        if let Some((_, (x, y, z), block, normal)) = targeted_block {
            self.hovered_block = Some((x, y, z, normal));
            self.sign_text = if let BlockState::Sign(text) = &block.state {
                Some(text.to_string())
            } else {
                None
            };
        } else {
            self.hovered_block = None;
            self.sign_text = None;
        }

        if let Some((hx, hy, hz, normal)) = self.hovered_block {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                if self
                    .level
                    .blocks
                    .get(&(hx, hy, hz))
                    .filter(|b| b.material == Material::Barrier)
                    .is_none()
                {
                    self.level
                        .blocks
                        .insert((hx, hy, hz), Material::Air.default());
                    self.level_mesh_is_dirty = true;
                }
            } else if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
                let (ox, oy, oz) = offset_from_normal(normal);
                let (nx, ny, nz) = (hx + ox, hy + oy, hz + oz);
                self.level
                    .blocks
                    .insert((nx, ny, nz), Material::Stone.default());

                self.level_mesh_is_dirty = true;
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_Z) {
            self.is_showing_debug = !self.is_showing_debug;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_X) {
            self.is_showing_wireframe = !self.is_showing_wireframe;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT_BRACKET) {
            assets.camera_shutter.set_pitch(1.2);
            assets.camera_shutter.play();
            if self.camera.fovy_destination < ZOOM_FOVY_MAX {
                self.camera.fovy_destination += ZOOM_FOVY_INCREMENT;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT_BRACKET) {
            assets.camera_shutter.set_pitch(0.9);
            assets.camera_shutter.play();
            if self.camera.fovy_destination > ZOOM_FOVY_MIN {
                self.camera.fovy_destination -= ZOOM_FOVY_INCREMENT;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_V) {
            assets.camera_shutter.set_pitch(0.7);
            assets.camera_shutter.play();
            self.camera.direction = self.camera.direction.get_next();
        }

        self.player.update(&rl);
        self.fps = rl.get_fps();

        self.camera.position_destination = self.player.position.add(Vector3::new(
            match self.camera.direction {
                CameraDirection::PlusXPlusZ => PLAYER_CAMERA_OFFSET_XZ,
                CameraDirection::PlusXMinusZ => PLAYER_CAMERA_OFFSET_XZ,
                CameraDirection::MinusXMinusZ => -PLAYER_CAMERA_OFFSET_XZ,
                CameraDirection::MinusXPlusZ => -PLAYER_CAMERA_OFFSET_XZ,
            },
            PLAYER_CAMERA_OFFSET_Y,
            match self.camera.direction {
                CameraDirection::PlusXPlusZ => PLAYER_CAMERA_OFFSET_XZ,
                CameraDirection::MinusXPlusZ => PLAYER_CAMERA_OFFSET_XZ,
                CameraDirection::MinusXMinusZ => -PLAYER_CAMERA_OFFSET_XZ,
                CameraDirection::PlusXMinusZ => -PLAYER_CAMERA_OFFSET_XZ,
            },
        ));
        self.camera.target_destination = self.player.position.add(Vector3::new(0.0, 1.0, 0.0));

        self.camera.update(&rl);
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, assets: &GameAssets) {
        d.clear_background(Color::SKYBLUE);

        let mut d3 = d.begin_mode3D(&self.camera.raycam);

        self.draw_world_mesh(&mut d3, &assets);

        if let Some((x, y, z, ..)) = &self.hovered_block {
            let (x, y, z) = (*x as f32, *y as f32, *z as f32);

            d3.draw_cube_wires(Vector3::new(x, y + 0.5, z), 1.0, 1.0, 1.0, Color::RED);
        }

        self.player.draw(&mut d3, &self.camera, &assets);

        drop(d3);

        if self.is_showing_debug {
            self.draw_crosshair(d, &assets);
            self.draw_debug_text(d);
        }

        if self.is_showing_pause_menu {
            self.draw_frozen_text(d, "Paused");
        } else if self.is_frozen {
            self.draw_frozen_text(d, "Frozen");
        }

        if let Some(text) = &self.sign_text {
            d.draw_text(&text, 15, 132, 18, Color::BLACK.alpha(0.5));
            d.draw_text(&text, 10, 128, 18, Color::WHITE);
        }
    }

    fn draw_world_mesh(&mut self, _d3: &mut RaylibMode3D<RaylibDrawHandle>, assets: &GameAssets) {
        if self.level_mesh_is_dirty {
            self.level_mesh = Some(make_model(
                upload_mesh(build_mesh(&self.level.to_material_map())),
                &assets.texture_atlas,
            ));
            self.level_mesh_is_dirty = false;
        }

        if let Some(model) = self.level_mesh {
            unsafe {
                let pos = ffi::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let white = ffi::Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                };

                if self.is_showing_wireframe {
                    ffi::DrawModelWires(model, pos, 1.0, white);
                } else {
                    ffi::DrawModel(model, pos, 1.0, white);
                }
            }
        }
    }

    fn draw_frozen_text(&self, d: &mut RaylibDrawHandle, label: &'static str) {
        d.draw_text(label, 10, 4, 18, Color::WHITE);
        let mut i = 0;
        HELP_TEXTS.iter().for_each(|t| {
            d.draw_text(*t, 10, 24 + (i * 18), 18, Color::WHITE);
            i += 1;
        });
    }

    fn draw_crosshair(&self, d: &mut RaylibDrawHandle, assets: &GameAssets) {
        let mut b = d.begin_blend_mode(BlendMode::BLEND_CUSTOM);
        unsafe {
            raylib_sys::rlSetBlendFactors(
                ffi::RL_ONE_MINUS_DST_COLOR as i32,
                ffi::RL_ONE_MINUS_SRC_ALPHA as i32,
                ffi::RL_FUNC_ADD as i32,
            );
        }

        b.draw_texture(&assets.crosshair_sprite, 304, 224, Color::WHITE);
        drop(b);
    }

    fn draw_debug_text(&self, d: &mut RaylibDrawHandle) {
        let version_line = format!("Questra Alpha {}", env!("CARGO_PKG_VERSION_PATCH"));
        let fps_line = format!("{} FPS", self.fps);
        let direction_line = format!("Facing: {}", self.camera.direction);
        let location_line = {
            let (x, y, z) = (
                self.player.position.x.floor(),
                self.player.position.y.floor(),
                self.player.position.z.floor(),
            );
            format!("Player {} {} {}", x, y, z)
        };
        let hovering_line = self
            .hovered_block
            .map(|(x, y, z, ..)| format!("Hover {} {} {}", x, y, z));

        let mut next_y = 4;
        d.draw_text(&version_line, 128, next_y, 18, Color::WHITE);
        next_y += 16;
        d.draw_text(&fps_line, 128, next_y, 18, Color::WHITE);
        next_y += 16;
        d.draw_text(&location_line, 128, next_y, 18, Color::WHITE);
        next_y += 16;
        d.draw_text(&direction_line, 128, next_y, 18, Color::WHITE);
        if let Some(hovering_line) = hovering_line {
            next_y += 16;
            d.draw_text(&hovering_line, 128, next_y, 18, Color::WHITE);
        }
    }
}

impl Drop for WorldScene {
    fn drop(&mut self) {
        unsafe {
            if let Some(model) = self.level_mesh.take() {
                ffi::UnloadModel(model);
            }
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
