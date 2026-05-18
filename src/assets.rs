use raylib::{RaylibHandle, RaylibThread, texture::Texture2D};

pub struct GameAssets {
    pub player_sprite: Texture2D,
    pub crosshair_sprite: Texture2D,
    pub title_sprite: Texture2D,
    pub texture_atlas: Texture2D,
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
            title_sprite: rl
                .load_texture(thread, "res/title.png")
                .expect("Failed to load title sprite"),
            texture_atlas: rl
                .load_texture(thread, "res/atlas.png")
                .expect("Failed to load texture atlas"),
        }
    }
}
