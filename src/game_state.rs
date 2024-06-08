use std::collections::HashMap;
use macroquad::prelude::Texture2D;

pub struct GameState {
    pub counter: i32,
    pub textures: HashMap<String, Texture2D>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            counter: 0,
            textures: HashMap::new(),
        }
    }
}