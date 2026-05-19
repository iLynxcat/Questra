use questra::{scene::Scene, sound::init_audio, state::GameState};
use raylib::{audio::Music, color::Color, drawing::RaylibDraw, ffi::KeyboardKey};

const TITLE: &str = concat!("Questra Alpha ", env!("CARGO_PKG_VERSION_PATCH"));

fn main() {
    let (mut rl, thread) = raylib::init() //
        .size(640, 480)
        .title(TITLE)
        .build();

    rl.set_target_fps(60);
    rl.set_exit_key(Some(KeyboardKey::KEY_Q));

    let audio = init_audio(0.3);
    let mut state = GameState::load(&mut rl, &thread, &audio);

    let ambience_tracks: Vec<&Music<'_>> = vec![&state.assets.music.lamentable];
    let mut current_ambience: usize = 0;

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_M) || rl.is_key_pressed_repeat(KeyboardKey::KEY_M) {
            state.is_music_paused = !state.is_music_paused;
        }

        let mut d = rl.begin_drawing(&thread);

        match &mut state.scene {
            Scene::Title(scene) => {
                scene.update(&d);
                scene.draw(&mut d, &state.assets);
            }
            Scene::World(scene) => {
                scene.update(&d, &state.assets);
                scene.draw(&mut d, &state.assets);
            }
        }

        let track = &ambience_tracks[current_ambience];
        if state.is_music_paused {
            d.draw_text("Music Paused", 10, 460, 18, Color::GOLDENROD);

            if track.is_stream_playing() {
                track.pause_stream();
            }

            track.update_stream();
        } else {
            if (track.get_time_played() / track.get_time_length()) >= 1.0 {
                track.stop_stream();
                current_ambience = (current_ambience + 1) % ambience_tracks.len();
                let track = ambience_tracks[current_ambience];
                track.play_stream();
                track.update_stream();
            } else {
                if !track.is_stream_playing() {
                    track.play_stream();
                }
                track.update_stream();
            }
        }
    }
}
