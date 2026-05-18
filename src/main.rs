use questra::{scene::Scene, state::GameState};
use raylib::{audio::RaylibAudio, ffi::KeyboardKey};

fn main() {
    let (mut rl, thread) = raylib::init() //
        .size(640, 480)
        .title("Questra")
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
        music.update_stream();

        match &mut state.scene {
            Scene::Title(scene) => {
                scene.update(&rl);
                scene.draw(&mut rl.begin_drawing(&thread), &state.assets);
            }
            Scene::World(scene) => {
                scene.update(&rl);
                scene.draw(&mut rl.begin_drawing(&thread), &state.assets);
            }
        }
    }
}
