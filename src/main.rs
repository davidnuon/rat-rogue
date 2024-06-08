mod scene_manager;
mod scenes;

use scenes::AvailebleScenes;

use macroquad::prelude::*;
use scene_manager::GameSceneManager;

#[macroquad::main("InputKeys", "Camera")]
async fn main() {
    let render_target = render_target(800, 600);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut scene_manager = GameSceneManager::new();
    
    scene_manager.set_scene(AvailebleScenes::StartScene);
    
    loop {
        set_camera(&Camera2D {
            zoom: vec2(1.0/800., 1.0/600.),
            target: vec2(800.0, 600.0),
            render_target: Some(render_target.clone()),
            // viewport: Some((0, 0, 800, 600)),
            ..Default::default()
        });
        scene_manager.update();
        scene_manager.draw();
        set_default_camera();

        // Draw render target
        // Fit an 800x600 render target into the screen

        let ratio = screen_width() / screen_height();
        let render_target_ratio = 800.0 / 600.0;
        let scale = if ratio > render_target_ratio {
            screen_height() / 600.0
        } else {
            screen_width() / 800.0
        };

        let x_pos = (screen_width() - 800.0 * scale) / 2.0;
        let y_pos = (screen_height() - 600.0 * scale) / 2.0;
        let dest_size = vec2(800.0 * scale, 600.0 * scale);
        clear_background(BLUE);
        draw_texture_ex(
            &render_target.texture,
            x_pos, 
            y_pos,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dest_size),
                ..Default::default()
            },
        );
        next_frame().await
    }
}