use crate::scene_manager::{
    GameScene,
    GameSceneState,
};
use macroquad::prelude::*;

pub struct GameSceneRed {
    pub x: f32,
    pub y: f32,
}

impl GameScene for GameSceneRed {
    fn update(&mut self) -> GameSceneState{
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
        if is_key_pressed(KeyCode::Space) {
            return GameSceneState::NextScene(0);
        }
        GameSceneState::Nothing
    }

    fn draw(&self) {
        clear_background(LIGHTGRAY);
        draw_circle(self.x, self.y, 15.0, RED);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
    }
}