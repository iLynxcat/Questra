use questra::{
    level::{Level, block::Material},
    scene::{Scene, world::WorldScene},
    state::GameState,
};

fn main() {
    let (mut rl, thread) = raylib::init() //
        .size(640, 480)
        .title("Questra")
        .build();

    rl.set_target_fps(60);

    let mut state = GameState::load(&mut rl, &thread);

    while !rl.window_should_close() {
        match &mut state.scene {
            Scene::World(scene) => {
                scene.update(&rl);
                scene.draw(&mut rl.begin_drawing(&thread));
            }
        }
    }
}
