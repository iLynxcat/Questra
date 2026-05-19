use crate::{
    assets::GameAssets,
    scene::{Scene, title::TitleScene},
};
use raylib::{RaylibHandle, RaylibThread, audio::RaylibAudio};

pub struct GameState<'aud> {
    pub scene: Scene,
    pub assets: GameAssets<'aud>,

    pub is_music_paused: bool,
}

impl<'aud> GameState<'aud> {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread, audio: &'aud RaylibAudio) -> Self {
        Self {
            scene: Scene::Title(TitleScene::new()),
            assets: GameAssets::load(rl, thread, audio),

            is_music_paused: cfg!(debug_assertions),
        }
    }
}
