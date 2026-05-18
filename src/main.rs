use questra::{scene::Scene, state::GameState};
use raylib::{audio::RaylibAudio, color::Color, drawing::RaylibDraw, ffi::KeyboardKey};

const TITLE: &str = concat!("Questra ", env!("CARGO_PKG_VERSION"));

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
            state.is_muted = !state.is_muted;
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

        if state.is_muted {
            music.set_volume(0.0);
            d.draw_text("Mute", 10, 460, 18, Color::RED);
        } else {
            music.set_volume(1.0);
        }

        music.update_stream();
    }
}
