use crate::scenes::{
    AvailebleScenes,
    FirstGameScene,
    GameSceneRed,
};

pub use crate::game_state::GameState;

pub trait GameScene {
    fn update(&mut self, global_state: &mut GameState) -> GameSceneTransition;
    fn draw(&self, global_state: &GameState);
}

pub enum GameSceneTransition {
    NextScene(AvailebleScenes),
    NoSceneTransition,
}

pub struct GameSceneManager {
    current_scene: Option<Box<dyn GameScene>>,
    state: GameState,
}

impl GameSceneManager {
    pub fn new() -> Self {
        Self {
            current_scene: None,
            state: GameState { counter: 0 },
        }
    }

    pub fn set_scene(&mut self, index: AvailebleScenes) {
        self.current_scene = match index {
            AvailebleScenes::StartScene => Some(Box::new(FirstGameScene::new())),
            AvailebleScenes::RedScene => Some(Box::new(GameSceneRed::new())),
        };
    }

    pub fn update(&mut self) {
        if let Some(scene) = self.current_scene.as_mut() {
            match scene.update(&mut self.state) {
                GameSceneTransition::NextScene(index) => {
                    self.set_scene(index);
                }
                GameSceneTransition::NoSceneTransition => {}
            }
        } else {
            panic!("Scene not found");
        }
    }

    pub fn draw(&self) {
        if let Some(scene) = self.current_scene.as_ref() {
            scene.draw(&self.state);
        } else {
            panic!("Scene not found");
        }
    }

    #[allow(dead_code)]
    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    #[allow(dead_code)]
    pub fn get_state_mut(&mut self) -> &mut GameState {
        &mut self.state
    }
}
