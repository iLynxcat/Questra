use crate::{
    assets::GameAssets,
    level::Level,
    scene::{Scene, world::WorldScene},
};
use raylib::{RaylibHandle, RaylibThread, audio::RaylibAudio};

pub struct GameState<'aud> {
    pub scene: Scene,
    pub assets: GameAssets<'aud>,

    pub is_music_paused: bool,
}

impl<'aud> GameState<'aud> {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread, audio: &'aud RaylibAudio) -> Self {
        let level = Level::new();

        Self {
            scene: Scene::World(WorldScene::new(level)),
            assets: GameAssets::load(rl, thread, audio),

            is_music_paused: cfg!(debug_assertions),
        }
    }
}
