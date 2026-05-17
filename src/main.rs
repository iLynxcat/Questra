use questra::{scene::Scene, state::GameState};
use raylib::ffi::KeyboardKey;

fn main() {
    let (mut rl, thread) = raylib::init() //
        .size(640, 480)
        .title("Questra")
        .build();

    rl.set_target_fps(60);
    rl.set_exit_key(Some(KeyboardKey::KEY_Q));

    let mut state = GameState::load(&mut rl, &thread);

    while !rl.window_should_close() {
        match &mut state.scene {
            Scene::World(scene) => {
                scene.update(&rl);
                scene.draw(&mut rl.begin_drawing(&thread), &state.assets);
            }
        }
    }
}
