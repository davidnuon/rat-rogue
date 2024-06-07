mod scene_manager;
mod scenes;

use scenes::{
    FirstGameScene,
    GameSceneRed,
};

use macroquad::prelude::*;
use scene_manager:: {
    GameSceneManager,
};

#[macroquad::main("InputKeys")]
async fn main() {
    let x = screen_width() / 2.0;
    let y = screen_height() / 2.0;
    
    let scene = FirstGameScene { x, y };
    let scene_red = GameSceneRed { x, y };

    let mut scene_manager = GameSceneManager::new();

    scene_manager.add_scene(Box::new(scene));
    scene_manager.add_scene(Box::new(scene_red));

    loop {
        clear_background(LIGHTGRAY);
        scene_manager.update();
        scene_manager.draw();
        next_frame().await
    }
}