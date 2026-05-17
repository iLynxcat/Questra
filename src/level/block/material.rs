use crate::level::block::{Block, BlockState};

#[derive(Clone, Copy, PartialEq)]
pub enum Material {
    Air = 0,
    Dirt = 1,
    Grass = 2,
    Stone = 3,
}

impl Material {
    pub fn default(self) -> Block {
        match self {
            Material::Air => Block::new(self, BlockState::None),
            Material::Dirt => Block::new(self, BlockState::None),
            Material::Grass => Block::new(self, BlockState::None),
            Material::Stone => Block::new(self, BlockState::None),
        }
    }
}
