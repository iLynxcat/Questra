use raylib::{
    RaylibHandle, RaylibThread,
    audio::{RaylibAudio, Sound},
    texture::Texture2D,
};

pub struct GameAssets<'aud> {
    pub player_sprite: Texture2D,
    pub crosshair_sprite: Texture2D,
    pub title_sprite: Texture2D,
    pub texture_atlas: Texture2D,

    pub camera_shutter: Sound<'aud>,
}

impl<'aud> GameAssets<'aud> {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread, audio: &'aud RaylibAudio) -> Self {
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

            camera_shutter: audio
                .new_sound("res/sfx/shutter.mp3")
                .expect("Failed to load camera shutter sound"),
        }
    }
}
