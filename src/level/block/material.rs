use raylib::audio::Sound;

use crate::{
    level::block::{Block, BlockState},
    sound::sfx::SoundEffects,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Air = 0,
    Barrier = 1,
    Dirt = 2,
    Grass = 3,
    Stone = 4,
    Water = 10,
    Sign = 15,
}

impl Material {
    pub fn default(self) -> Block {
        match self {
            Material::Air => Block::new(self, BlockState::None),
            Material::Barrier => Block::new(self, BlockState::None),
            Material::Dirt => Block::new(self, BlockState::None),
            Material::Grass => Block::new(self, BlockState::None),
            Material::Stone => Block::new(self, BlockState::None),
            Material::Water => Block::new(self, BlockState::LiquidLevel(1.0)),
            Material::Sign => Block::new(self, BlockState::Sign("".to_string())),
        }
    }

    pub fn placement_sound<'sfx>(&self, sfx: &'sfx SoundEffects<'sfx>) -> &'sfx Sound<'sfx> {
        match self {
            _ => &sfx.stone_place,
        }
    }

    pub fn destroy_sound<'sfx>(&self, sfx: &'sfx SoundEffects<'sfx>) -> &'sfx Sound<'sfx> {
        match self {
            _ => &sfx.stone_destroy,
        }
    }
}
