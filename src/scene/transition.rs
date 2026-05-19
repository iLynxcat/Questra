use crate::scene::Scene;

pub enum SceneTransition {
    None,
    To(Scene),
    Quit,
}
