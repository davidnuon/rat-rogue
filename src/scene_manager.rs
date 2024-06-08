use crate::scenes::{
    AvailebleScenes,
    FirstGameScene,
    GameSceneRed,
    BattleScene,
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
    scene_stack: Vec<Box<dyn GameScene>>,
    state: GameState,
}

impl GameSceneManager {
    pub fn new() -> Self {
        Self {
            scene_stack: Vec::new(),
            state: GameState::new(),
        }
    }

    pub fn set_scene(&mut self, index: AvailebleScenes) {
        let next_scene: Box<dyn GameScene> = match index {
            AvailebleScenes::StartScene => Box::new(FirstGameScene::new()),
            AvailebleScenes::RedScene => Box::new(GameSceneRed::new()),
            AvailebleScenes::BattleScene => Box::new(BattleScene::new()),
        };

        if self.scene_stack.len() > 0 {
            self.scene_stack.pop();
        }
        self.scene_stack.push(next_scene);
    }

    pub fn update(&mut self) {
        if let Some(scene) = self.scene_stack.last_mut() {
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
        if let Some(scene) = self.scene_stack.last() {
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
