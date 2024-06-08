use std::collections::HashMap;
use crate::scenes::AvailebleScenes;
pub struct GameState {
    pub counter: i32,
}
pub trait GameScene {
    fn update(&mut self, gamestate: &mut GameState) -> GameSceneTransition;
    fn draw(&self);
}

pub enum GameSceneTransition {
    NextScene(AvailebleScenes),
    NoSceneTransition,
}

pub struct GameSceneManager {
    scenes: HashMap<AvailebleScenes, Box<dyn GameScene>>,
    current_scene: AvailebleScenes,
    state: GameState,
}

impl GameSceneManager {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            current_scene: AvailebleScenes::NoScene,
            state: GameState { counter: 0 },
        }
    }

    pub fn add_scene(&mut self, name:AvailebleScenes, scene: Box<dyn GameScene>) {
        self.scenes.insert(name, scene);
    }

    pub fn set_scene(&mut self, index: AvailebleScenes) {
        self.current_scene = index;
    }

    pub fn update(&mut self) {
        if let Some(scene) = self.scenes.get_mut(&self.current_scene) {
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
        if let Some(scene) = self.scenes.get(&self.current_scene) {
            scene.draw();
        } else {
            panic!("Scene not found");
        }
    }
}
