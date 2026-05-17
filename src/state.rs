use crate::{
    assets::GameAssets,
    level::{Level, block::Material},
    scene::{Scene, world::WorldScene},
};
use raylib::{RaylibHandle, RaylibThread};

pub struct GameState {
    pub scene: Scene,
    pub assets: GameAssets,
}

impl GameState {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let level = Level::new();

        Self {
            scene: Scene::World(WorldScene::new(level)),
            assets: GameAssets::load(rl, thread),
        }
    }
}
