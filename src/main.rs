use questra::{
    scene::{Scene, transition::Transition},
    sound::init_audio,
    state::GameState,
};
use raylib::{audio::Music, color::Color, drawing::RaylibDraw, ffi::KeyboardKey};

const TITLE: &str = concat!("Questra Alpha ", env!("CARGO_PKG_VERSION_PATCH"));

fn main() {
    let (mut rl, thread) = raylib::init() //
        .size(640, 480)
        .title(TITLE)
        .build();

    rl.set_target_fps(60);
    rl.set_exit_key(None);

    let audio = init_audio(0.3);
    let mut state = GameState::load(&mut rl, &thread, &audio);

    let ambience_tracks: Vec<&Music<'_>> = vec![
        &state.assets.music.lamentable,
        &state.assets.music.summer_night_feast,
    ];
    // start at the end of the list so our handler automatically wraps
    let mut ambience_i: usize = ambience_tracks.len() - 1;

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_M) || rl.is_key_pressed_repeat(KeyboardKey::KEY_M) {
            state.is_music_paused = !state.is_music_paused;
            ambience_tracks[ambience_i].play_stream();
        }
        if rl.is_key_pressed(KeyboardKey::KEY_N) || rl.is_key_pressed_repeat(KeyboardKey::KEY_N) {
            ambience_tracks[ambience_i].stop_stream();
            advance_track(&mut ambience_i, ambience_tracks.len());
            ambience_tracks[ambience_i].play_stream();
        }

        let mut d = rl.begin_drawing(&thread);

        let transition = match &mut state.scene {
            Scene::Title(scene) => scene.update(&d),
            Scene::World(scene) => scene.update(&d, &state.assets),
        };
        match transition {
            Transition::To(next) => {
                state.scene = next;
            }
            Transition::Quit => break,
            Transition::None => {}
        };

        match &mut state.scene {
            Scene::Title(scene) => scene.draw(&mut d, &state.assets),
            Scene::World(scene) => scene.draw(&mut d, &state.assets),
        };

        let track = &ambience_tracks[ambience_i];
        if state.is_music_paused {
            d.draw_text("Music Paused", 10, 460, 18, Color::GOLDENROD);

            if track.is_stream_playing() {
                track.pause_stream();
            }

            track.update_stream();
        } else {
            if !track.is_stream_playing() {
                advance_track(&mut ambience_i, ambience_tracks.len());
                let track = ambience_tracks[ambience_i];
                track.play_stream();
                track.update_stream();
            } else {
                track.update_stream();
            }
        }
    }
}

fn advance_track(index: &mut usize, track_count: usize) {
    *index = (*index + 1) % track_count
}
