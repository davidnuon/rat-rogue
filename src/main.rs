mod scene_manager;
mod scenes;
mod game_state;

use scenes::AvailebleScenes;

use macroquad::prelude::*;
use scene_manager::GameSceneManager;

#[macroquad::main("Rat Rogue")]
async fn main() {
    // Size of the game view
    let view_width = 800;
    let view_height = 600;
    let view_size = vec2(view_width as f32, view_height as f32);
    
    let render_target = render_target(view_width as u32, view_height as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut scene_manager = GameSceneManager::new();
    scene_manager.set_scene(AvailebleScenes::BattleScene);
    
    let global_state = scene_manager.get_state_mut();
    let texture: Texture2D = load_texture("assets/sprites/rat.png").await.unwrap();
    let screen_texture: Texture2D = load_texture("assets/sprites/screen.png").await.unwrap();

    global_state.textures.insert("rat".to_string(), texture);
    global_state.textures.insert("screen".to_string(), screen_texture);

    loop {
        set_camera(&Camera2D {

            // I don't know why this is the correct zoom
            zoom: vec2(2.0/view_size.x, 2.0/view_size.y),

            // Nor why this is the correct target
            target: vec2((view_width/2) as f32, (view_height/2) as f32),
            
            render_target: Some(render_target.clone()),
            ..Default::default()
        });
        scene_manager.update();
        scene_manager.draw();
        set_default_camera();

        let screen_size = vec2(screen_width(), screen_height());
        
        let screen_ratio = screen_size.x / screen_size.y;
        let render_target_ratio: f32 = view_size.x / view_size.y;

        // Determine the scale of the rendered texture
        let scale = if screen_ratio > render_target_ratio {
            screen_height() / view_size.y
        } else {
            screen_width() / view_size.x
        };

        let dest_size = view_size * scale;
        let dest_pos = (screen_size - dest_size)/2.0;

        clear_background(BLUE);
        draw_texture_ex(
            &render_target.texture,
            dest_pos.x, 
            dest_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dest_size),
                ..Default::default()
            },
        );
        next_frame().await
    }
}