use questra::{scene::Scene, state::GameState};
use raylib::{audio::RaylibAudio, color::Color, drawing::RaylibDraw, ffi::KeyboardKey};

const TITLE: &str = concat!("Questra Alpha ", env!("CARGO_PKG_VERSION_PATCH"));

fn main() {
    let (mut rl, thread) = raylib::init() //
        .size(640, 480)
        .title(TITLE)
        .build();

    let Ok(audio) = RaylibAudio::init_audio_device() else {
        panic!("could not init audio device!");
    };

    audio.set_master_volume(0.3);
    let Ok(mut music) = audio.new_music("res/music/tdhr-summer-night-feast-2025.mp3") else {
        panic!("failed to load music!")
    };

    music.looping = true;
    music.play_stream();

    rl.set_target_fps(60);
    rl.set_exit_key(Some(KeyboardKey::KEY_Q));

    let mut state = GameState::load(&mut rl, &thread);

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
                scene.update(&d);
                scene.draw(&mut d, &state.assets);
            }
        }

        if state.is_music_paused {
            d.draw_text("Music Paused", 10, 460, 18, Color::GOLDENROD);

            if music.is_stream_playing() {
                music.pause_stream();
            }
        } else if !state.is_music_paused && !music.is_stream_playing() {
            music.play_stream();
        }

        music.update_stream();
    }
}
