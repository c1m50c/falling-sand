use wasm_bindgen::prelude::*;
pub(crate) mod cell_chunk;
pub(crate) mod cell;


#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Vector2U {
    pub x: u32,
    pub y: u32,
}


#[wasm_bindgen]
impl Vector2U {
    #[inline]
    pub fn new(x: u32, y: u32) -> Self {
        return Self {
            x,
            y,
        }
    }
}