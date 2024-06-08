mod scene_manager;
mod scenes;

use scenes::{
    FirstGameScene,
    GameSceneRed,
    AvailebleScenes
};

use macroquad::prelude::*;
use scene_manager::GameSceneManager;

#[macroquad::main("InputKeys")]
async fn main() {
    let mut scene_manager = GameSceneManager::new();

    scene_manager.set_scene(AvailebleScenes::StartScene);

    loop {
        scene_manager.update();
        scene_manager.draw();
        next_frame().await
    }
}