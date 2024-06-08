mod red_scene;
mod start_scene;
mod battle_scene;

pub use red_scene::GameSceneRed;
pub use start_scene::FirstGameScene;
pub use battle_scene::BattleScene;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum AvailebleScenes {
    StartScene,
    RedScene,
    BattleScene
}