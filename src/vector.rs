use wasm_bindgen::prelude::*;


#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Vector2U {
    pub x: u32,
    pub y: u32,
}


#[wasm_bindgen]
impl Vector2U {
    pub fn new(x: u32, y: u32) -> Self {
        return Self {
            x,
            y,
        };
    }
}