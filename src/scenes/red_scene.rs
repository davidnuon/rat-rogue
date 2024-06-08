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
    x: f32,
    y: f32,
}

impl GameSceneRed {
    pub fn new() -> Self {
        Self {
            x: 100.0,
            y: 100.0,
        }
    }
}

impl GameScene for GameSceneRed {
    fn update(&mut self, global_state: &mut GlobalState) -> GameSceneTransition{
        global_state.counter += 1;
        if is_key_pressed(KeyCode::Space) {
            println!("[RED]: W key pressed from ");
            return GameSceneTransition::NextScene(crate::scenes::AvailebleScenes::StartScene);
        }
        GameSceneTransition::NoSceneTransition
    }

    fn draw(&self, global_state: &GlobalState) {
        clear_background(LIGHTGRAY);
        draw_circle(self.x, self.y, 15.0, RED);
        draw_text(&format!("{}", global_state.counter), 20.0, 20.0, 20.0, DARKGRAY);
        widgets::Window::new(hash!(), vec2(400., 200.), vec2(320., 400.))
        .label("Shop")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            for i in 0..30 {
                Group::new(hash!("shop", i), Vec2::new(300., 80.)).ui(ui, |ui| {
                    ui.label(Vec2::new(10., 10.), &format!("Item N {}", global_state.counter));
                    ui.label(Vec2::new(260., 40.), "10/10");
                    ui.label(Vec2::new(200., 58.), &format!("{} kr", 800));
                });
            }
        });
        
    }
}