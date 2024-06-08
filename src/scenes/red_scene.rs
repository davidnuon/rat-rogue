use crate::scene_manager::{
    GameScene,
    GameSceneTransition, GlobalState,
};
use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui,
};

pub struct GameSceneRed {
    pub x: f32,
    pub y: f32,
}

impl GameScene for GameSceneRed {
    fn update(&mut self, global_state: &mut GlobalState) -> GameSceneTransition{
        if is_key_pressed(KeyCode::W) {
            println!("[RED]: W key pressed from ");
            return GameSceneTransition::NextScene(crate::scenes::AvailebleScenes::StartScene);
        }
        GameSceneTransition::NoSceneTransition
    }

    fn draw(&self, global_state: &GlobalState) {
        clear_background(LIGHTGRAY);
        draw_circle(self.x, self.y, 15.0, RED);
        draw_text(&format!("{}", global_state.counter), 20.0, 20.0, 20.0, DARKGRAY);
    }
}