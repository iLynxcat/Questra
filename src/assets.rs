use raylib::{RaylibHandle, RaylibThread, audio::RaylibAudio, texture::Texture2D};

use crate::sound::{music::MusicTracks, sfx::SoundEffects};

pub struct GameAssets<'aud> {
    pub title: Texture2D,
    pub player_sprite: Texture2D,
    pub crosshair_sprite: Texture2D,
    pub texture_atlas: Texture2D,

    pub music: MusicTracks<'aud>,
    pub sfx: SoundEffects<'aud>,
}

impl<'aud> GameAssets<'aud> {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread, audio: &'aud RaylibAudio) -> Self {
        Self {
            title: rl
                .load_texture(thread, "res/title.png")
                .expect("Failed to load title sprite"),
            player_sprite: rl
                .load_texture(thread, "res/bob.png")
                .expect("Failed to load player sprite"),
            crosshair_sprite: rl
                .load_texture(thread, "res/cursor.png")
                .expect("Failed to load cursor sprite"),
            texture_atlas: rl
                .load_texture(thread, "res/atlas.png")
                .expect("Failed to load texture atlas"),

            music: MusicTracks::new(audio),
            sfx: SoundEffects::new(audio),
        }
    }
}
