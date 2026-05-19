use raylib::audio::{Music, RaylibAudio};

pub struct MusicTracks<'aud> {
    pub lamentable: Music<'aud>,
}

impl<'aud> MusicTracks<'aud> {
    pub fn new(audio: &'aud RaylibAudio) -> Self {
        Self {
            lamentable: audio
                .new_music("res/music/lamentable.mp3")
                .expect("Failed to load music/lamentable.mp3"),
        }
    }
}
