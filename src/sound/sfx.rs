use raylib::audio::{RaylibAudio, Sound};

pub struct SoundEffects<'aud> {
    pub camera_shutter: Sound<'aud>,
}

impl<'aud> SoundEffects<'aud> {
    pub fn new(audio: &'aud RaylibAudio) -> Self {
        Self {
            camera_shutter: audio
                .new_sound("res/sfx/shutter.mp3")
                .expect("Failed to load sfx/shutter.mp3"),
        }
    }
}
