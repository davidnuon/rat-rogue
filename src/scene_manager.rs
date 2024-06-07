use macroquad::prelude::*;

pub trait GameScene {
    fn update(&mut self) -> GameSceneState;
    fn draw(&self);
}

pub enum GameSceneState {
    NextScene(usize),
    Nothing,
}

pub struct GameSceneManager {
    scenes: Vec<Box<dyn GameScene>>,
    current_scene: usize,
}

impl GameSceneManager {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
            current_scene: 0,
        }
    }

    pub fn add_scene(&mut self, scene: Box<dyn GameScene>) {
        self.scenes.push(scene);
    }

    pub fn set_scene(&mut self, index: usize) {
        self.current_scene = index;
    }

    pub fn update(&mut self) {
        match self.scenes[self.current_scene].update() {
            GameSceneState::NextScene(index) => {
                self.set_scene(index);
            }
            GameSceneState::Nothing => {}
        }
    }

    pub fn draw(&self) {
        self.scenes[self.current_scene].draw();
    }
}
