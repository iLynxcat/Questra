use crate::scene::*;

pub enum Scene {
    Title(TitleScene),
    World(WorldScene),
}
