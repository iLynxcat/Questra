use raylib::{
    RaylibHandle,
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle, RaylibMode3DExt},
    ffi,
    math::{Rectangle, Vector2, Vector3},
};

use crate::{
    assets::GameAssets,
    level::Level,
    render::mesh::{build_mesh, make_model, upload_mesh},
    scene::{
        Scene,
        title::camera::Camera,
        transition::SceneTransition,
        world::{WorldScene, draw_mesh},
    },
};

const TEXT_FLASH_FRAMES: u8 = 16;

enum FadeTarget {
    Game,
    Quit,
}

pub struct TitleScene {
    black_alpha: f32,
    fade_time_remain: f32,
    /// Set to:
    /// -1.0 = fade in, 1.0 = fade out, 0.0 = idle.
    fade_dir: f32,
    fade_target: Option<FadeTarget>,

    flash_frame: u8,

    level: Level,
    camera: Camera,

    mesh_opaque: Option<ffi::Model>,
    mesh_water: Option<ffi::Model>,
    should_recompute_meshes: bool,
}

impl TitleScene {
    pub fn new() -> Self {
        Self {
            black_alpha: 1.0,
            fade_time_remain: 0.0,
            fade_dir: -1.0,
            fade_target: None,

            flash_frame: 0,
            level: Level::new(),
            camera: Camera::new(
                Vector3::new(0.0, 13.8, 0.0),
                Vector3::new(0.0, 11.2, -14.0),
                90.0,
            ),

            mesh_opaque: None,
            mesh_water: None,
            should_recompute_meshes: true,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, assets: &GameAssets) -> SceneTransition {
        if self.should_recompute_meshes {
            let (opaque_mesh, water_mesh) = build_mesh(&self.level.blocks);
            self.mesh_opaque = Some(make_model(upload_mesh(opaque_mesh), &assets.texture_atlas));
            self.mesh_water = Some(make_model(upload_mesh(water_mesh), &assets.texture_atlas));
            self.should_recompute_meshes = false;
        }

        if self.fade_dir != 0.0 {
            const FPS: f32 = 15.0;
            const ONE_FRAME: f32 = 1.0 / FPS;

            self.fade_time_remain += rl.get_frame_time();

            if self.fade_time_remain >= ONE_FRAME {
                self.fade_time_remain -= ONE_FRAME;
                self.black_alpha = (self.black_alpha + self.fade_dir / FPS).clamp(0.0, 1.0);

                if self.black_alpha <= 0.0 {
                    self.fade_dir = 0.0;
                } else if self.black_alpha >= 1.0 && self.fade_dir == 1.0 {
                    return match self.fade_target {
                        Some(FadeTarget::Game) => {
                            SceneTransition::To(Scene::World(WorldScene::new(Level::new())))
                        }
                        Some(FadeTarget::Quit) => SceneTransition::Quit,
                        None => SceneTransition::None,
                    };
                }
            }
        } else if rl.is_key_pressed(ffi::KeyboardKey::KEY_SPACE) {
            self.start_fade_out(FadeTarget::Game);
        } else if rl.is_key_pressed(ffi::KeyboardKey::KEY_Q) {
            self.start_fade_out(FadeTarget::Quit);
        } else {
            const ONE_FRAME: f32 = 1.0 / 15.0;
            self.fade_time_remain += rl.get_frame_time();
            if self.fade_time_remain >= ONE_FRAME {
                self.fade_time_remain -= ONE_FRAME;
                self.flash_frame = (self.flash_frame + 1) % TEXT_FLASH_FRAMES;
            }

            self.camera.update(rl);
        }

        if !assets.sfx.waves_ambience.is_playing() {
            assets
                .sfx
                .waves_ambience
                .set_pitch(rand::random_range(0.7..1.3));
            assets.sfx.waves_ambience.play();
        }

        SceneTransition::None
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, assets: &GameAssets) {
        d.clear_background(Color::LIGHTSKYBLUE);

        if self.black_alpha >= 1.0 {
            return;
        }

        let d3 = d.begin_mode3D(self.camera.raycam);

        if let Some(opaque) = self.mesh_opaque {
            draw_mesh(opaque, false, 1.0);
        }

        if let Some(water) = self.mesh_water {
            draw_mesh(water, false, 0.5);
        }

        drop(d3);

        let (title_w, title_h) = (assets.title.width as f32, assets.title.height as f32);
        d.draw_texture_rec(
            &assets.title,
            Rectangle::new(0.0, 0.0, title_w, title_h),
            Vector2::new((d.get_screen_width() as f32 / 2.0) - (title_w / 2.0), 24.0),
            Color::WHITE,
        );

        const START_TEXT: &str = "Press SPACE to start";
        const START_TEXT_SIZE: i32 = 20;
        let text_w = d.measure_text(START_TEXT, START_TEXT_SIZE);

        d.draw_text(
            START_TEXT,
            (d.get_screen_width() - text_w) / 2,
            360,
            START_TEXT_SIZE,
            Color::WHITE.alpha(self.get_text_alpha()),
        );

        if self.black_alpha > 0.0 {
            d.draw_rectangle(
                0,
                0,
                d.get_screen_width(),
                d.get_screen_height(),
                Color::BLACK.alpha(self.black_alpha),
            );
        }
    }

    fn start_fade_out(&mut self, target: FadeTarget) {
        self.fade_dir = 1.0;
        self.fade_target = Some(target);
    }

    fn get_text_alpha(&self) -> f32 {
        const PHASE: u8 = TEXT_FLASH_FRAMES / 4;
        let frame = self.flash_frame;

        match frame / PHASE {
            0 => 1.0,
            1 => 1.0 - (frame % PHASE) as f32 / PHASE as f32,
            2 => (frame % PHASE) as f32 / PHASE as f32,
            _ => 1.0,
        }
    }
}
