use crate::scene_manager::{
    GameScene,
    GameSceneTransition, GameState,
};
use macroquad::prelude::*;

pub struct FirstGameScene {
    pub x: f32,
    pub y: f32,
}

impl GameScene for FirstGameScene {
    fn update(&mut self, game_state: &mut GameState) -> GameSceneTransition {
        game_state.counter += 1;
        
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
            println!("[FGS]: W key pressed from ");
            return GameSceneTransition::NextScene(crate::scenes::AvailebleScenes::RedScene);
        }
        GameSceneTransition::NoSceneTransition
    }

    fn draw(&self) {
        clear_background(LIGHTGRAY);
        draw_circle(self.x, self.y, 15.0, YELLOW);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
    }
}