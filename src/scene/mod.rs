use crate::scene::world::WorldScene;

pub mod title;
pub mod world;

pub enum Scene {
    // Title(TitleScene),
    World(WorldScene),
}
