use std::path::Path;

use raylib::audio::{RaylibAudio, Sound};

pub struct SoundEffects<'aud> {
    pub camera_shutter: Sound<'aud>,
}

impl<'aud> SoundEffects<'aud> {
    pub fn new(audio: &'aud RaylibAudio) -> Self {
        Self {
            camera_shutter: load_effect("shutter.mp3", audio),
        }
    }
}

fn load_effect<'aud>(filename: &'static str, audio: &'aud RaylibAudio) -> Sound<'aud> {
    let path = Path::new("res").join("sfx").join(filename);

    audio
        .new_sound(path.to_str().unwrap())
        .unwrap_or_else(|_| panic!("Failed to load sound {}", filename))
}
