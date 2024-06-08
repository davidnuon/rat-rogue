mod red_scene;
mod start_scene;

pub use red_scene::GameSceneRed;
pub use start_scene::FirstGameScene;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum AvailebleScenes {
    NoScene = -1,
    StartScene = 1,
    RedScene = 2,
}