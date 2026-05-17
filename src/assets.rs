use raylib::{RaylibHandle, RaylibThread, texture::Texture2D};

pub struct GameAssets {
    pub player_sprite: Texture2D,
    pub crosshair_sprite: Texture2D,
}

impl GameAssets {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            player_sprite: rl
                .load_texture(thread, "res/bob.png")
                .expect("Failed to load player sprite"),
            crosshair_sprite: rl
                .load_texture(thread, "res/cursor.png")
                .expect("Failed to load cursor sprite"),
        }
    }
}
