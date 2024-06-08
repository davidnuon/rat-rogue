use std::collections::HashMap;

pub trait GameScene {
    fn update(&mut self) -> GameSceneTransition;
    fn draw(&self);
}

pub enum GameSceneTransition {
    NextScene(String),
    NoSceneTransition,
}

pub struct GameSceneManager {
    // scenes: Vec<Box<dyn GameScene>>,
    scenes: HashMap<String, Box<dyn GameScene>>,
    current_scene: String,
}

impl GameSceneManager {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            current_scene: "".to_string(),
        }
    }

    pub fn add_scene(&mut self, name:String, scene: Box<dyn GameScene>) {
        self.scenes.insert(name, scene);
    }

    pub fn set_scene(&mut self, index: String) {
        self.current_scene = index;
    }

    pub fn update(&mut self) {
        if let Some(scene) = self.scenes.get_mut(&self.current_scene) {
            match scene.update() {
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
