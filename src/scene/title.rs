use raylib::{
    RaylibHandle,
    drawing::RaylibDrawHandle,
    math::{Rectangle, Vector2},
    rgui::RaylibDrawGui,
};

pub struct TitleScene {
    game_transition_frames: usize,
}

impl TitleScene {
    pub fn new() -> Self {
        Self {
            game_transition_frames: 0,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        if self.game_transition_frames > 0 {
            if self.game_transition_frames > 60 {
                return;
            }

            self.game_transition_frames += 1;
            return;
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        if d.gui_button(Rectangle::new(40.0, 20.0, 200.0, 20.0), "Play") {
            self.game_transition_frames = 1;
        }
    }
}
