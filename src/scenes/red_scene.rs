use crate::scene_manager::{
    GameScene,
    GameSceneTransition,
};
use macroquad::prelude::*;

pub struct GameSceneRed {
    pub x: f32,
    pub y: f32,
}

impl GameScene for GameSceneRed {
    fn update(&mut self) -> GameSceneTransition{
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
            println!("[RED]: W key pressed from ");
            return GameSceneTransition::NextScene(crate::scenes::AvailebleScenes::StartScene);
        }
        GameSceneTransition::NoSceneTransition
    }

    fn draw(&self) {
        clear_background(LIGHTGRAY);
        draw_circle(self.x, self.y, 15.0, RED);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
    }
}