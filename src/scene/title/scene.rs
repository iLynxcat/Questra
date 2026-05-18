use raylib::{RaylibHandle, drawing::RaylibDrawHandle};

use crate::{assets::GameAssets, level::Level};

pub struct TitleScene {}

impl TitleScene {
    pub fn new(_level: Level) -> Self {
        Self {}
    }

    pub fn update(&mut self, _rl: &RaylibHandle) {}

    pub fn draw(&self, _d: &mut RaylibDrawHandle, _assets: &GameAssets) {}
}
