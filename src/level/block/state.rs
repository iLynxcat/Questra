#[derive(Clone, Debug)]
pub enum BlockState {
    None,
    Sign(String),
    LiquidLevel(f32),
}
