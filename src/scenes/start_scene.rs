use crate::scene_manager::{
    GameScene,
    GameSceneTransition
};
use macroquad::prelude::*;

use crate::game_state::GameState;

pub struct FirstGameScene {
    x: f32,
    y: f32,
}

impl FirstGameScene {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl GameScene for FirstGameScene {
    fn update(&mut self, global_state: &mut GameState) -> GameSceneTransition {
        global_state.counter += 1;
        
        if is_key_down(KeyCode::Right) {
            self.x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            self.x -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            self.y += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            self.y -= 1.0;
        }
        if is_key_down(KeyCode::Escape) {
            std::process::exit(0);
        }
        if is_key_pressed(KeyCode::W) {
            return GameSceneTransition::NextScene(crate::scenes::AvailebleScenes::RedScene);
        }
        GameSceneTransition::NoSceneTransition
    }

    fn draw(&self, _global_state: &GameState) {
        clear_background(LIGHTGRAY);
        draw_circle(self.x, self.y, 10.0, YELLOW);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
    }
}