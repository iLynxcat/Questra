use std::ops::Add;

use crate::{
    assets::GameAssets,
    level::{
        Level,
        block::{Block, BlockState, Material},
    },
    render::mesh::{build_mesh, make_model, upload_mesh},
    scene::{
        Scene,
        title::TitleScene,
        transition::SceneTransition,
        world::{
            camera::{Camera, CameraDirection},
            player::Player,
        },
    },
};
use raylib::{
    RaylibHandle,
    color::Color,
    drawing::{RaylibBlendModeExt, RaylibDraw, RaylibDraw3D, RaylibDrawHandle, RaylibMode3DExt},
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
    "esc = pause",
    "[ = zoom +",
    "] = zoom +",
    "M = music on/off",
    "N = next track",
    ">/< = change angle",
    "Q = back to title",
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
    level_mesh_opaque: Option<ffi::Model>,
    level_mesh_water: Option<ffi::Model>,
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
            level_mesh_opaque: None,
            level_mesh_water: None,
            level_mesh_is_dirty: true,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, assets: &GameAssets) -> SceneTransition {
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
            || rl.is_key_pressed_repeat(KeyboardKey::KEY_ESCAPE)
        {
            self.is_showing_pause_menu = !self.is_showing_pause_menu;
            self.is_frozen = self.is_showing_pause_menu;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
            return SceneTransition::To(Scene::Title(TitleScene::new()));
        }

        if self.is_frozen {
            return SceneTransition::None;
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
            assets.sfx.camera_shutter.set_pitch(1.0);
            assets.sfx.camera_shutter.play();
            if self.camera.fovy_destination < ZOOM_FOVY_MAX {
                self.camera.fovy_destination += ZOOM_FOVY_INCREMENT;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT_BRACKET) {
            assets.sfx.camera_shutter.set_pitch(1.2);
            assets.sfx.camera_shutter.play();
            if self.camera.fovy_destination > ZOOM_FOVY_MIN {
                self.camera.fovy_destination -= ZOOM_FOVY_INCREMENT;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
            assets.sfx.camera_shutter.set_pitch(0.7);
            assets.sfx.camera_shutter.play();
            self.camera.direction = self.camera.direction.get_prev();
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            assets.sfx.camera_shutter.set_pitch(0.7);
            assets.sfx.camera_shutter.play();
            self.camera.direction = self.camera.direction.get_next();
        }

        self.player.update(&rl, &self.level);
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

        if self.level_mesh_is_dirty {
            let (opaque_mesh, water_mesh) = build_mesh(&self.level.blocks);
            self.level_mesh_opaque =
                Some(make_model(upload_mesh(opaque_mesh), &assets.texture_atlas));
            self.level_mesh_water =
                Some(make_model(upload_mesh(water_mesh), &assets.texture_atlas));
            self.level_mesh_is_dirty = false;
        }

        SceneTransition::None
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, assets: &GameAssets) {
        d.clear_background(Color::SKYBLUE);

        let mut d3 = d.begin_mode3D(&self.camera.raycam);

        if let Some(opaque) = self.level_mesh_opaque {
            draw_mesh(opaque, self.is_showing_wireframe, 1.0);
        }

        self.player.draw(&mut d3, &self.camera, &assets);

        if let Some(water) = self.level_mesh_water {
            draw_mesh(water, self.is_showing_wireframe, 0.3);
        }

        if let Some((x, y, z, ..)) = &self.hovered_block {
            let (x, y, z) = (*x as f32, *y as f32, *z as f32);

            d3.draw_cube_wires(Vector3::new(x, y + 0.5, z), 1.0, 1.0, 1.0, Color::RED);
        }

        drop(d3);

        if self.is_showing_debug {
            self.draw_crosshair(d, &assets);
            self.draw_debug_text(d);
        }

        if self.is_showing_pause_menu {
            draw_frozen_text(d, "Paused");
        } else if self.is_frozen {
            draw_frozen_text(d, "Frozen");
        }

        if let Some(text) = &self.sign_text {
            d.draw_text(&text, 15, 132, 18, Color::BLACK.alpha(0.5));
            d.draw_text(&text, 10, 128, 18, Color::WHITE);
        }
    }

    fn draw_crosshair(&self, d: &mut RaylibDrawHandle, assets: &GameAssets) {
        let (screen_w, screen_h) = (d.get_screen_width(), d.get_screen_height());

        let mut b = d.begin_blend_mode(BlendMode::BLEND_CUSTOM);
        unsafe {
            raylib_sys::rlSetBlendFactors(
                ffi::RL_ONE_MINUS_DST_COLOR as i32,
                ffi::RL_ONE_MINUS_SRC_ALPHA as i32,
                ffi::RL_FUNC_ADD as i32,
            );
        }

        let txt = &assets.crosshair_sprite;

        b.draw_texture(
            txt,
            (screen_w / 2) - (txt.width / 2),
            (screen_h / 2) - (txt.height / 2),
            Color::WHITE,
        );
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
            if let Some(model) = self.level_mesh_opaque.take() {
                ffi::UnloadModel(model);
            }
            if let Some(model) = self.level_mesh_water.take() {
                ffi::UnloadModel(model);
            }
        }
    }
}

pub fn draw_mesh(model: ffi::Model, as_wireframe: bool, alpha: f32) {
    const MODEL_POS: ffi::Vector3 = ffi::Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    unsafe {
        let tint: ffi::Color = Color::WHITE.alpha(alpha).into();

        if alpha < 1.0 {
            ffi::rlDisableDepthMask();
            ffi::BeginBlendMode(BlendMode::BLEND_ALPHA as i32);
        }

        if as_wireframe {
            ffi::DrawModelWires(model, MODEL_POS, 1.0, tint);
        } else {
            ffi::DrawModel(model, MODEL_POS, 1.0, tint);
        }

        if alpha < 1.0 {
            ffi::EndBlendMode();
            ffi::rlEnableDepthMask();
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

fn draw_frozen_text(d: &mut RaylibDrawHandle, label: &'static str) {
    d.draw_text(label, 10, 4, 18, Color::WHITE);
    let mut i = 0;
    HELP_TEXTS.iter().for_each(|t| {
        d.draw_text(*t, 10, 24 + (i * 18), 18, Color::WHITE);
        i += 1;
    });
}
