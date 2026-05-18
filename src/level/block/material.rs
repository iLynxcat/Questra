use crate::level::block::{Block, BlockState};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Air = 0,
    Barrier = 1,
    Dirt = 2,
    Grass = 3,
    Stone = 4,
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
            Material::Sign => Block::new(self, BlockState::Sign("".to_string())),
        }
    }
}
