use crate::level::block::{material::Material, state::BlockState};

#[derive(Clone, Copy)]
pub struct Block {
    pub material: Material,
    pub state: BlockState,
}

impl Block {
    pub const fn new(material: Material, state: BlockState) -> Self {
        Block { material, state }
    }
}
