use raylib::{RaylibHandle, camera::Camera3D, drawing::RaylibDrawHandle};

use crate::state::GameState;

pub fn update(_: &RaylibHandle, _: &mut Camera3D, _: &mut GameState) {}

pub fn draw(_: &mut RaylibDrawHandle, _: &Camera3D, _: &GameState) {}
