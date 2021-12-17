mod vector;
mod utils;

use wasm_bindgen::prelude::*;
use vector::Vector2U;
use std::vec::Vec;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[derive(Debug, Clone, Copy)]
pub enum Faces {
    North,
    East,
    South,
    West,
}


#[derive(Debug, Clone, Copy)]
pub enum Cell {
    /// Alive(MaterialId as u8)
    Alive(u8),
    Empty,
}


#[wasm_bindgen]
pub struct Simulator {
    pub size: Vector2U,
    cells: Vec<Cell>,
}


#[wasm_bindgen]
impl Simulator {
    pub fn new(size: Vector2U) -> Self {
        let mut cells = Vec::with_capacity((size.x * size.y) as usize);

        for _ in 0 .. size.x {
            for _ in 0 .. size.y {
                cells.push(Cell::Empty);
            }
        }
        
        return Self {
            size, 
            cells,
        };
    }
    
    pub fn simulate(&mut self) {
        
    }
}