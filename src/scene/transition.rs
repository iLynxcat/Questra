use crate::scene::Scene;

pub enum Transition {
    None,
    To(Scene),
}
