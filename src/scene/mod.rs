use crate::scene::{title::TitleScene, world::WorldScene};

pub mod title;
pub mod world;

pub enum Scene {
    Title(TitleScene),
    World(WorldScene),
}
