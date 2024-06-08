use macroquad::prelude::*;

use crate::scene_manager::{
    GameScene,
    GameSceneTransition, GameState,
};

pub struct BattleScene {
    x: f32,
    y: f32,
}

impl BattleScene {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl GameScene for BattleScene {
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

    fn draw(&self, global_state: &GameState) {
        clear_background(LIGHTGRAY);

        let rat_texture = global_state.textures.get("rat").unwrap();
        let screen_texture = global_state.textures.get("screen").unwrap();
        draw_texture(rat_texture, 0.0, 0.0, WHITE);
        draw_texture(screen_texture, 0.0, 0.0, WHITE);

        draw_circle(self.x, self.y, 10.0, GREEN);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
    }
}