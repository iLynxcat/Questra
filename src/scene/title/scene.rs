use raylib::{
    RaylibHandle,
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi,
    math::{Rectangle, Vector2},
};

use crate::{
    assets::GameAssets,
    level::Level,
    scene::{Scene, transition::SceneTransition, world::WorldScene},
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
}

impl TitleScene {
    pub fn new() -> Self {
        Self {
            black_alpha: 1.0,
            fade_time_remain: 0.0,
            fade_dir: -1.0,
            fade_target: None,

            flash_frame: 0,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) -> SceneTransition {
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
        }

        SceneTransition::None
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, assets: &GameAssets) {
        d.clear_background(Color::BLACK);

        if self.black_alpha >= 1.0 {
            return;
        }

        let (w, h) = (assets.title.width as f32, assets.title.height as f32);
        d.draw_texture_rec(
            &assets.title,
            Rectangle::new(0.0, 0.0, w, h),
            Vector2::new((d.get_screen_width() as f32 / 2.0) - (w / 2.0), 24.0),
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
