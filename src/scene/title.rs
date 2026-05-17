use raylib::{RaylibHandle, camera::Camera3D, drawing::RaylibDrawHandle};

use crate::state::GameState;

pub fn update(rl: &RaylibHandle, cam: &mut Camera3D, state: &mut GameState) {}

pub fn draw(d: &mut RaylibDrawHandle, cam: &Camera3D, state: &GameState) {}
