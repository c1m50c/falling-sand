mod vector;
mod utils;
mod cell;

use wasm_bindgen::prelude::*;
use cell::{Cell, CellState};
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


#[wasm_bindgen]
pub struct Simulator {
    pub size: Vector2U,
    cells: Vec<Cell>,
}


impl Simulator {
    #[inline(always)]
    const fn position_to_index(position: Vector2U, size: Vector2U) -> usize {
        return (position.y + (position.x * size.y)) as usize;
    }

    #[inline(always)]
    const fn position_in_bounds(position: Vector2U, size: Vector2U) -> bool {
        if position.x > size.x - 1 || position.y > size.y - 1 { return false; }
        return true;
    }

    #[inline(always)]
    fn set_cell_state(&mut self, position: Vector2U, state: CellState) {
        self.cells[Self::position_to_index(position, self.size)].state = state;
    }

    #[inline(always)]
    fn get_cell_state(&mut self, position: Vector2U) -> CellState {
        return self.cells[Self::position_to_index(position, self.size)].state;
    }
}


#[wasm_bindgen]
impl Simulator {
    pub fn new(size: Vector2U) -> Self {
        let mut cells = Vec::with_capacity((size.x * size.y) as usize);

        for x in 0 .. size.x {
            for y in 0 .. size.y {
                cells.push(Cell::new(CellState::Empty, Vector2U::new(x, y)));
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