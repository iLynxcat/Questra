use std::path::Path;

use raylib::audio::{Music, RaylibAudio};

pub struct MusicTracks<'aud> {
    pub lamentable: Music<'aud>,
    pub summer_night_feast: Music<'aud>,
}

impl<'aud> MusicTracks<'aud> {
    pub fn new(audio: &'aud RaylibAudio) -> Self {
        Self {
            lamentable: load_track("lamentable.mp3", audio),
            summer_night_feast: load_track("summer-night-feast.mp3", audio),
        }
    }
}

fn load_track<'aud>(filename: &'static str, audio: &'aud RaylibAudio) -> Music<'aud> {
    let path = Path::new("res").join("music").join(filename);
    let mut track = audio
        .new_music(path.to_str().unwrap())
        .unwrap_or_else(|_| panic!("Failed to load music {}", filename));
    track.looping = false;

    track
}
